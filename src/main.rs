// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/main.rs
// Main of the project.

use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

mod http;

#[tokio::main]
async fn main() {
    init_log();
    info!("Server starting");
    http::server::http_server_start().await;
}

fn init_log() {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
}
