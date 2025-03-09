use axum::{Router, routing::get};
use std::env;
use tokio::net::TcpListener;

async fn hello() -> &'static str {
    "Hello World! 🍡"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    // Get port from environment or default to 1208
    let port = env::var("PORT")
        .map(|p| p.parse().unwrap_or(1208))
        .unwrap_or(1208);

    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&address).await.unwrap();

    println!("Server running on {}", address);
    axum::serve(listener, app).await.unwrap();
}
