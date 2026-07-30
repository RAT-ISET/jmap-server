// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/http/server.rs
// HTTP server.

use axum::Router;
use axum::routing::get;
use tracing::{info, instrument, trace};

pub async fn http_server_start() {
    info!("Starting http server");
    let app = Router::new().route("/", get("Hello, world!"));
    HttpServer::new(8080, "0.0.0.0", app).run().await.unwrap();
}

struct HttpServer {
    port: u16,
    host: &'static str,
    router: Router,
}

impl HttpServer {
    #[instrument(skip(router))]
    fn new(port: u16, host: &'static str, router: Router) -> Self {
        trace!("Create a new http server");
        HttpServer { port, host, router }
    }

    async fn run(self) -> Result<(), std::io::Error> {
        info!("Starting http server");
        let listener = tokio::net::TcpListener::bind(
            self.host.to_string() + ":" + self.port.to_string().as_str(),
        )
        .await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
