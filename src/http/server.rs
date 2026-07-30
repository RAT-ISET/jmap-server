// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/http/server.rs
// HTTP server.

use axum::Router;
use tracing::{info, instrument, trace};

pub struct HttpServer<'a> {
    port: &'a u16,
    host: &'a str,
    router: Router,
}

impl<'a> HttpServer<'a> {
    #[instrument(skip(router))]
    pub fn new(port: &'a u16, host: &'a str, router: Router) -> Self {
        trace!("Create a new http server");
        HttpServer { port, host, router }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        info!("Starting http server");
        let listener = tokio::net::TcpListener::bind((self.host, *self.port)).await?;
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
