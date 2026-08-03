// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/jmap/account.rs
// JMAP account.

use crate::database::DatabaseTable;
use std::fmt::Display;

#[derive(sqlx::FromRow)]
pub struct AccountItem {
    pub id: i64,
    pub username: String,
}

pub struct AccountTable;
impl DatabaseTable for AccountTable {
    type Item = AccountItem;
    const TABLE_NAME: &'static str = "Account";
    const COLUMN_NAME: &'static str = "*";
}

impl Display for AccountItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[A{}] Name: {}", self.id, self.username)
    }
}
