// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /server/src/jmap/token.rs
// JMAP token.

use crate::jmap::{JmapServerState, SERVER_ERROR};
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use jmap_core::database::{DatabaseTable, read_all};
use jmap_core::token::*;
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, warn};

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Missed authorization")]
    MissedAuthorization,
    #[error("Invalid authorization")]
    InvalidAuthorization,
    #[error("User not found")]
    NotFoundUser,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        match self {
            TokenError::MissedAuthorization
            | TokenError::InvalidAuthorization
            | TokenError::NotFoundUser => {
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
            .ok_or(TokenError::MissedAuthorization)?
            .to_str()
            .map_err(|_| TokenError::InvalidAuthorization)?
            .strip_prefix("Bearer ")
            .ok_or(TokenError::InvalidAuthorization)?
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
        let users = read_all::<TokenTable>("token", token, &state.database).await?;
        if users.is_empty() {
            Err(TokenError::NotFoundUser)
        } else {
            Ok(TokenList(users))
        }
    }
}
