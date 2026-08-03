// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/jmap/token.rs
// JMAP token.

use crate::database::{DatabaseTable, read_all};
use crate::grant::{GrantItem, GrantTable};
use sqlx::SqlitePool;
use std::fmt::Display;

#[derive(sqlx::FromRow)]
pub struct TokenItem {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
}

pub struct TokenTable;
impl DatabaseTable for TokenTable {
    type Item = TokenItem;
    const TABLE_NAME: &'static str = "Token";
    const COLUMN_NAME: &'static str = "*";
}

pub struct TokenItemDisplayer<'a> {
    item: &'a TokenItem,
    grants: Vec<GrantItem>,
}

impl<'a> TokenItemDisplayer<'a> {
    pub async fn new(item: &'a TokenItem, source: &SqlitePool) -> Result<Self, sqlx::Error> {
        let grants =
            read_all::<GrantTable>(vec![("token_id", item.id.to_string())], source).await?;
        Ok(Self { item, grants })
    }
}

impl Display for TokenItemDisplayer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[T{}] Owner: {}", self.item.id, self.item.user_id)?;
        for grant in &self.grants {
            writeln!(f, "  {}", grant)?;
        }
        Ok(())
    }
}
