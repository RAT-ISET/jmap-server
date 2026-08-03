// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/jmap/grant.rs
// JMAP token grant.

use crate::database::DatabaseTable;

#[derive(sqlx::FromRow)]
pub struct GrantItem {
    pub token_id: i64,
    pub email_id: i64,
    pub is_personal: bool,
    pub is_read_only: bool,
}

pub struct GrantTable;
impl DatabaseTable for GrantTable {
    type Item = GrantItem;
    const TABLE_NAME: &'static str = "TokenGrant";
    const COLUMN_NAME: &'static str = "*";
}
