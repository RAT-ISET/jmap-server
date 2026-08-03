// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /server/src/main.rs
// Main of the server.

use axum::Router;
use axum::routing::get;
use jmap_core::conf::Config;
use jmap_core::database;
use std::env;
use std::fs::read_to_string;
use std::process::exit;
use std::sync::Arc;
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, fmt};

mod http;
mod jmap;
mod model;

#[tokio::main]
async fn main() {
    jmap_core::init_log();
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
    let database = database::open(&config.database).await.unwrap();

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
