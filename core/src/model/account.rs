// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/jmap/account.rs
// JMAP account.

use crate::database::DatabaseTable;

#[derive(sqlx::FromRow)]
pub struct UserItem {
    pub id: i64,
    pub username: String,
}

pub struct UserTable;
impl DatabaseTable for UserTable {
    type Item = UserItem;
    const TABLE_NAME: &'static str = "User";
    const COLUMN_NAME: &'static str = "*";
}
