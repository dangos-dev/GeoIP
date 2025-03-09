# Rust GeoIP Service
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

A high-performance IP geolocation service powered by MaxMind's GeoLite2 database, built with Rust and Axum.


[![Deploy on Railway](https://railway.com/button.svg)](https://railway.com/template/xMaTor?referralCode=Al2B-n)


## Features

- 🌍 IP Geolocation Lookups
- ⚡ Async Architecture
- 🔄 Automatic Weekly Database Updates
- 📦 Self-Contained Database Management
- 🔒 Graceful Shutdown Handling
- 📈 Built-in Logging

## Installation

1. Clone repository:
    ```bash
    git clone https://github.com/yourusername/geoip-service.git
    cd geoip-service
    ```
2. Create .env file:
    ```bash
    cp .env.example .env
    ```

3. Build and run:
    ```bash
    cargo build --release
    cargo run --release
    ```

## Configuration
Required environment variables:
- `ACCOUNT_ID`: MaxMind account ID
- `LICENSE_KEY`: MaxMind license key

## API Endpoints
- `GET /`: Service status check
- `GET /me`: Lookup your own IP
- `GET /{ip}`: Lookup any IPv4/IPv6 address
- `POST /database`: Trigger manual database update (disabled on this repo)

## Scheduled Updates
Automatic weekly updates occur every Sunday at 00:00 UTC. Manual updates can be triggered via the API.

## License
MIT License

### Acknowledgements
- MaxMind GeoLite2 Database
- Axum Web Framework
- Tokio Runtime