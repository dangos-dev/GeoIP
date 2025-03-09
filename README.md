# Rust GeoIP Service

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

A high-performance IP geolocation service powered by MaxMind's GeoLite2 database, built with Rust and Axum.

[![Deploy on Railway](https://railway.com/button.svg)](https://railway.com/template/y3QoLi?referralCode=Al2B-n)

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
    git clone https://github.com/dangos-dev/GeoIP.git
    cd GeoIP
    ```
2. Create .env file.

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
- `GET /{ip}`: Lookup any IPv4/IPv6 address
- `GET /me`: Lookup your own IP (TODO)
- `POST /database`: Trigger manual database update (TODO)

## Scheduled Updates

Automatic weekly updates occur every Sunday at 00:00 UTC. Manual updates can be triggered via the API.

## License

MIT License

### Acknowledgements

- MaxMind GeoLite2 Database
- Axum Web Framework
- Tokio Runtime