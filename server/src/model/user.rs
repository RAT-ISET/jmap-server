// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /server/src/jmap/account
// JMAP user.

use crate::jmap::SERVER_ERROR;
use axum::response::{IntoResponse, Response};
use jmap_core::account::{UserItem, UserTable};
use jmap_core::database::read_where_item;
use sqlx::SqlitePool;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum UsersError {
    #[error("User not found")]
    NotFoundUser(#[from] sqlx::Error),
    #[error("Database error")]
    DatabaseFailure,
}

impl IntoResponse for UsersError {
    fn into_response(self) -> Response {
        match self {
            UsersError::NotFoundUser(_) | UsersError::DatabaseFailure => {
                error!("{}", self);
                SERVER_ERROR.into_response()
            }
        }
    }
}

pub async fn user_from(user_id: i64, database: &SqlitePool) -> Result<UserItem, UsersError> {
    Ok(read_where_item::<UserTable>("id", user_id.to_string(), database).await?)
}
