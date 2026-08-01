// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/jmap/user.rs
// JMAP user.

use crate::io::database::{DatabaseTable, read_item};
use crate::jmap::server::SERVER_ERROR;
use axum::response::{IntoResponse, Response};
use sqlx::{Error, SqlitePool};
use thiserror::Error;
use tracing::error;

#[derive(sqlx::FromRow)]
pub struct UserItem {
    pub id: i64,
    pub username: String,
}

pub struct UserTable;
impl DatabaseTable for UserTable {
    type Item = UserItem;
    const TABLE_NAME: &'static str = "users";
    const COLUMN_NAME: &'static str = "*";
}

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

impl UserItem {
    pub async fn from(user_id: i64, database: &SqlitePool) -> Result<Self, UsersError> {
        Ok(read_item::<UserTable>("id", user_id.to_string(), database).await?)
    }
}
