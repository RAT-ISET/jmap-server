// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /server/src/jmap/apis/session.rs
// JMAP session response.

use crate::jmap::server::JmapServerState;
use crate::model::token::TokenList;
use crate::model::user::user_from;
use axum::Json;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ResponseBody {
    capabilities: HashMap<String, serde_json::Value>,
    accounts: HashMap<String, serde_json::Value>,
    #[serde(alias = "primaryAccounts")]
    primary_accounts: HashMap<String, serde_json::Value>,
}

pub async fn handle(state: State<Arc<JmapServerState>>, users: TokenList) -> impl IntoResponse {
    let mut capabilities = HashMap::new();
    capabilities.insert(
        "urn:ietf:params:jmap:core".to_string(),
        serde_json::json!({
            "maxSizeUpload": 5000000,
            "maxConcurrentUpload": 4
        }),
    );
    capabilities.insert(
        "urn:ietf:params:jmap:mail".to_string(),
        serde_json::json!({}),
    );
    let mut accounts = HashMap::new();
    for token in users.0 {
        let username = match user_from(token.user_id, &state.database).await {
            Ok(user) => user.username,
            Err(e) => return e.into_response(),
        };
        let mut account_capabilities = HashMap::new();
        account_capabilities.insert(
            "urn:ietf:params:jmap:mail".to_string(),
            serde_json::json!({}),
        );
        accounts.insert(
            "A".to_string() + &token.user_id.to_string(),
            serde_json::json!({
                "name": username,
                "isPersonal": true,
                "isReadOnly": false,
                "accountCapabilities": account_capabilities,
            }),
        );
    }
    let mut primary_accounts = HashMap::new();
    primary_accounts.insert(
        "urn:ietf:params:jmap:mail".to_string(),
        serde_json::json!({}),
    );
    (
        [(header::CACHE_CONTROL, "no-store")],
        Json(ResponseBody {
            capabilities,
            accounts,
            primary_accounts,
        }),
    )
        .into_response()
}
