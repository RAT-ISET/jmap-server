// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/jmap/server.rs
// JMAP server.

use axum::{Json, Router};
use axum::extract::State;
use axum::routing::get;
use std::sync::Arc;
use serde::Serialize;
use tracing::{debug, instrument, trace};

#[derive(Serialize, Clone)]
struct JmapServerStateWellKnown {
    session: String,
}

struct JmapServerState {
    base: String,
    well_known: Json<JmapServerStateWellKnown>,
}

impl JmapServerState {
    #[instrument]
    fn new(base: String, session: String) -> Result<Self, serde_json::Error> {
        trace!("Initializing JMAP Server State");
        Ok(JmapServerState {
            base,
            well_known: Json(JmapServerStateWellKnown { session }),
        })
    }
}

pub struct JmapServer {
    data: Arc<JmapServerState>,
}

impl JmapServer {
    #[instrument]
    pub fn new(base: String, session: String) -> Result<Self, serde_json::Error> {
        debug!("Initializing JMAP Server");
        Ok(JmapServer {
            data: Arc::new(JmapServerState::new(base, session)?),
        })
    }

    pub fn router(&self) -> Router {
        trace!("Add the router /.well-known/jmap");
        Router::new()
            .route("/.well-known/jmap", get(jmap_well_known))
            .with_state(self.data.clone())
    }
}

async fn jmap_well_known(state: State<Arc<JmapServerState>>) -> Json<JmapServerStateWellKnown> {
    state.well_known.clone()
}
