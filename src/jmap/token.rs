// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/jmap/token.rs
// JMAP token.

use crate::io::database::{DatabaseTable, read_all};
use crate::jmap::server::{JmapServerState, SERVER_ERROR};
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, warn};

#[derive(sqlx::FromRow)]
pub struct TokenItem {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
}

pub struct TokenTable;
impl DatabaseTable for TokenTable {
    type Item = TokenItem;
    const TABLE_NAME: &'static str = "users";
    const COLUMN_NAME: &'static str = "*";
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Missed authorization")]
    MissedAuthorization,
    #[error("User not found")]
    NotFoundUser,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        match self {
            TokenError::MissedAuthorization | TokenError::NotFoundUser => {
                warn!("{}", self);
                (
                    StatusCode::UNAUTHORIZED,
                    [(header::WWW_AUTHENTICATE, "Bearer")],
                )
                    .into_response()
            }
            TokenError::Database(status) => {
                error!("{}", status);
                SERVER_ERROR.into_response()
            }
        }
    }
}

pub struct BearerToken(String);
fn get_token(parts: &mut Parts) -> Result<BearerToken, TokenError> {
    Ok(BearerToken(
        parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|auth| auth.to_str().ok())
            .ok_or(TokenError::MissedAuthorization)?
            .to_string(),
    ))
}
impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = TokenError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        get_token(parts)
    }
}

pub struct TokenList(pub Vec<TokenItem>);
impl FromRequestParts<Arc<JmapServerState>> for TokenList {
    type Rejection = TokenError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<JmapServerState>,
    ) -> Result<Self, Self::Rejection> {
        let token = get_token(parts)?.0;
        Ok(TokenList(
            read_all::<TokenTable>("token", token, &state.database).await?,
        ))
    }
}
