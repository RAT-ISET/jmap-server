// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/jmap/user.rs
// JMAP user.

use crate::database::DatabaseTable;

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
