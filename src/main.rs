use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use dotenvy::dotenv;
use flate2::read::GzDecoder;
use maxminddb::Reader;
use serde_json::{Value, json};
use std::{env, net::IpAddr, path::PathBuf, sync::Arc};
use tar::Archive;
use tokio::{fs, signal};
use tokio_cron_scheduler::{Job, JobScheduler};

// Shared application state
#[derive(Clone)]
struct AppState {
    db_reader: Arc<Reader<Vec<u8>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    init_database().await?;

    // Load GeoIP database
    let db_content = fs::read("./data/GeoLite2-City.mmdb").await?;
    let reader = Reader::from_source(db_content)?;
    let state = AppState {
        db_reader: Arc::new(reader),
    };

    // Configure API routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/{ip}", get(ip_handler))
        // .route("/me", get(me_handler))           // TODO: Fetch own IP
        // .route("/database", post(db_handler))    // TODO: Add CORS to this endpoint
        .with_state(state);

    // Schedule weekly automatic update
    let mut sched = JobScheduler::new().await?;
    let cron_expr = "0 0 0 * * 0"; // Every Sunday at midnight UTC

    sched
        .add(Job::new_async(cron_expr, |_uuid, _| {
            Box::pin(async {
                log::info!("Updating GeoLite2 DB...");
                match update_database().await {
                    Ok(_) => log::info!("Database update successful"),
                    Err(e) => log::error!("DB update failed: {}", e),
                }
            })
        })?)
        .await?;

    sched.start().await?;

    // Start web server
    let port = env::var("PORT")
        .map(|p| p.parse().unwrap_or(1208))
        .unwrap_or(1208);

    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Server running on {}", address);

    // Handle concurrent server and scheduler shutdown
    tokio::select! {
        _ = axum::serve(listener, app) => {},
        _ = shutdown_signal() => {
            log::info!("Initiating graceful shutdown...");
            sched.shutdown().await?;
        }
    }

    Ok(())
}

// Endpoint handlers
async fn root_handler() -> &'static str {
    "GeoIP 🍡"
}

async fn me_handler(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ip = "127.0.0.1".parse().unwrap(); // TODO: Fetch own IP
    handle_ip_lookup(State(state), ip).await
}

async fn ip_handler(
    State(state): State<AppState>,
    Path(ip_str): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ip = ip_str.parse().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "Invalid IP address" })),
        )
    })?;

    handle_ip_lookup(State(state), ip).await
}

// Manual database update handler
async fn db_handler(State(_): State<AppState>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    log::info!("Manual database update requested");
    match update_database().await {
        Ok(_) => Ok(Json(
            json!({ "success": true, "message": "Update initiated" }),
        )),
        Err(e) => {
            log::error!("Update failed: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "Update failed" })),
            ))
        }
    }
}

// Common IP lookup logic
async fn handle_ip_lookup(
    State(state): State<AppState>,
    ip: IpAddr,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    state
        .db_reader
        .lookup::<maxminddb::geoip2::City>(ip)
        .map(|city| Json(json!(city)))
        .map_err(|e| {
            log::error!("Lookup error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "message": "IP lookup failed" })),
            )
        })
}

// Database management
async fn init_database() -> Result<(), Box<dyn std::error::Error>> {
    if !PathBuf::from("./data/GeoLite2-City.mmdb").exists() {
        log::info!("Downloading initial database...");
        update_database().await?;
    }
    Ok(())
}

async fn update_database() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let account_id = env::var("ACCOUNT_ID")?;
    let license_key = env::var("LICENSE_KEY")?;

    // Download compressed file
    let content = reqwest::Client::new()
        .get("https://download.maxmind.com/geoip/databases/GeoLite2-City/download?suffix=tar.gz")
        .basic_auth(&account_id, Some(&license_key))
        .send()
        .await?
        .bytes()
        .await?;

    // Extract and update file
    fs::create_dir_all("./data").await?;
    let tar = GzDecoder::new(&content[..]);
    let mut archive = Archive::new(tar);
    archive.unpack("./dist")?;

    // Move to final location
    let mut dir = fs::read_dir("./dist").await?;
    let entry = dir.next_entry().await?.ok_or("Empty archive")?;
    fs::copy(
        format!("{}/GeoLite2-City.mmdb", entry.path().display()),
        "./data/GeoLite2-City.mmdb",
    )
    .await?;

    // Cleanup temporary files
    fs::remove_dir_all("./dist").await?;
    log::info!("Database update completed");
    Ok(())
}

// Shutdown signal handling
async fn shutdown_signal() {
    let ctrl_c = async { signal::ctrl_c().await.unwrap() };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => log::info!("CTRL+C received"),
        _ = terminate => log::info!("SIGTERM received"),
    }

    log::info!("Shutting down...");
}
