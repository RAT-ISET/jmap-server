// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/http/server.rs
// HTTP server.

use axum::Router;
use axum::routing::get;
use tracing::info;

pub async fn http_server_start(){
    info!("Starting http server");
    let app = Router::new().route("/", get("Hello, world!"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("Server is listening on: {}, port 8080", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
