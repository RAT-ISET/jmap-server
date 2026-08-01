// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/main.rs
// Main of the project.

use crate::conf::Config;
use axum::Router;
use axum::routing::get;
use std::env;
use std::fs::read_to_string;
use std::process::exit;
use std::sync::Arc;
use tracing::{debug, error, info, trace};
use tracing_subscriber::{EnvFilter, fmt};

mod conf;
mod http;
mod io;
mod jmap;

#[tokio::main]
async fn main() {
    init_log();
    debug!("Logging started");

    let config_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            error!("Missed parameter of the configure file >> Usage: JMAPs <config.toml>");
            exit(1);
        }
    };
    debug!("Configuration loading");
    let config: Arc<Config> =
        Arc::new(toml::from_str(read_to_string(config_path).unwrap().as_str()).unwrap());

    debug!("Database loading");
    let database = io::database::init(&config.database).await.unwrap();

    info!("Server starting");
    let jmap_server = jmap::JmapServer::new(config.jmap.clone(), database).unwrap();
    http::HttpServer::new(
        &config.http.port,
        &config.http.bind,
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
