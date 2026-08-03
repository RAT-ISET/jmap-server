// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/jmap/email.rs
// JMAP email.

use crate::database::DatabaseTable;
use thiserror::Error;

#[derive(sqlx::FromRow)]
pub struct EmailItem {
    pub id: i64,
    pub owner: i64,
    pub name: String,
}

pub struct EmailTable;
impl DatabaseTable for EmailTable {
    type Item = EmailItem;
    const TABLE_NAME: &'static str = "Email";
    const COLUMN_NAME: &'static str = "*";
}
