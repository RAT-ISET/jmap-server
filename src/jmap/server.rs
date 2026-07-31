// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/jmap/server.rs
// JMAP server.

use crate::conf::ConfigJmap;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::sync::Arc;
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
    fn new(config: ConfigJmap) -> Result<Self, serde_json::Error> {
        trace!("Initializing JMAP Server State");
        let base = config.base_url.clone();
        let session = config.base_url.clone() + &config.session.clone();
        Ok(JmapServerState {
            base,
            well_known: Json(JmapServerStateWellKnown { session }),
        })
    }
}

pub struct JmapServer {
    data: Arc<JmapServerState>,
    conf: ConfigJmap,
}

impl JmapServer {
    #[instrument]
    pub fn new(config: ConfigJmap) -> Result<Self, serde_json::Error> {
        debug!("Initializing JMAP Server");
        Ok(JmapServer {
            data: Arc::new(JmapServerState::new(config.clone())?),
            conf: config,
        })
    }

    pub fn router(&self) -> Router {
        trace!("Add the router /.well-known/jmap");
        Router::new()
            .route(&self.conf.well_known, get(jmap_well_known))
            .with_state(self.data.clone())
    }
}

async fn jmap_well_known(state: State<Arc<JmapServerState>>) -> Json<JmapServerStateWellKnown> {
    state.well_known.clone()
}
