// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/jmap/token.rs
// JMAP token.

use crate::database::DatabaseTable;
use thiserror::Error;

#[derive(sqlx::FromRow)]
pub struct TokenItem {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
}

pub struct TokenTable;
impl DatabaseTable for TokenTable {
    type Item = TokenItem;
    const TABLE_NAME: &'static str = "tokens";
    const COLUMN_NAME: &'static str = "*";
}
