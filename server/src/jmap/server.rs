// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /server/src/jmap/server.rs
// JMAP server.

use crate::jmap::apis;
use axum::Router;
use axum::http::{HeaderName, StatusCode, header};
use axum::response::Redirect;
use axum::routing::get;
use jmap_core::conf::ConfigJmap;
use sqlx::SqlitePool;
use std::sync::Arc;
use tracing::{debug, instrument, trace};

pub const SERVER_ERROR: (StatusCode, [(HeaderName, &'static str); 1], &'static str) = (
    StatusCode::INTERNAL_SERVER_ERROR,
    [(header::CONTENT_TYPE, "application/json")],
    "{\"type\": \"serverFail\",\"description\": \"Internal server error\"}",
);

pub struct JmapServerState {
    pub base: String,
    pub database: SqlitePool,
}

impl JmapServerState {
    #[instrument]
    fn new(config: ConfigJmap, database: SqlitePool) -> Result<Self, serde_json::Error> {
        trace!("Initializing JMAP Server State");
        let base = config.base_url.clone();
        Ok(JmapServerState { base, database })
    }
}

pub struct JmapServer {
    data: Arc<JmapServerState>,
    conf: ConfigJmap,
}

impl JmapServer {
    #[instrument]
    pub fn new(config: ConfigJmap, database: SqlitePool) -> Result<Self, serde_json::Error> {
        debug!("Initializing JMAP Server");
        Ok(JmapServer {
            data: Arc::new(JmapServerState::new(config.clone(), database)?),
            conf: config,
        })
    }

    pub fn router(&self) -> Router {
        trace!("Add the router /.well-known/jmap");
        Router::new()
            .route(
                "/.well-known/jmap",
                get(Redirect::temporary(
                    (self.conf.root.clone() + "/session").as_str(),
                )),
            )
            .route(
                (self.conf.root.clone() + "/session").as_str(),
                get(apis::session::handle),
            )
            .with_state(self.data.clone())
    }
}
