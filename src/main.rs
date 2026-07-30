// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/main.rs
// Main of the project.

use axum::Router;
use axum::routing::get;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

mod http;
mod jmap;

#[tokio::main]
async fn main() {
    init_log();
    info!("Server starting");
    let jmap_server =
        jmap::JmapServer::new("http://localhost:8080".to_string(), "http://localhost:8080".to_string()).unwrap();
    http::HttpServer::new(
        &8080u16,
        "0.0.0.0",
        Router::new()
            .route("/", get("Hello, world!"))
            .merge(jmap_server.router()),
    )
    .run()
    .await
    .unwrap();
}

fn init_log() {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}
