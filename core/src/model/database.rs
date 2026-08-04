// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/io/database.rs
// Database linker.

use crate::conf::ConfigDatabase;
use sqlx::sqlite::SqliteRow;
use sqlx::{query, FromRow, QueryBuilder, Sqlite, SqlitePool};
use std::env::current_dir;
use tracing::debug;
use crate::token::TokenTable;

pub async fn open(config: &ConfigDatabase) -> Result<SqlitePool, sqlx::Error> {
    debug!("Read path: {}/{}", current_dir()?.display(), &config.file);
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&config.file);
    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}

pub trait DatabaseTable {
    type Item: Send + Unpin + for<'a> FromRow<'a, SqliteRow>;
    const TABLE_NAME: &'static str;
    const COLUMN_NAME: &'static str;
}

fn get_command<T>() -> String
where
    T: DatabaseTable,
{
    format!("SELECT {} FROM {}", T::COLUMN_NAME, T::TABLE_NAME)
}

fn read_query<T>(filter: Vec<(&str, String)>) -> QueryBuilder<Sqlite>
where
    T: DatabaseTable,
{
    let mut builder = QueryBuilder::new(get_command::<T>());
    if !filter.is_empty() {
        builder.push(" WHERE ");
        let mut is_first = true;
        filter.iter().for_each(|(k, v)| {
            if !is_first {
                builder.push(" AND ");
            } else {
                is_first = false
            }
            builder.push(k).push(" = ").push_bind(v);
        });
    }
    builder
}

pub async fn read_item<T>(
    filter: Vec<(&str, String)>,
    source: &SqlitePool,
) -> Result<T::Item, sqlx::Error>
where
    T: DatabaseTable,
{
    Ok(read_query::<T>(filter)
        .build_query_as::<T::Item>()
        .fetch_one(source)
        .await?)
}

pub async fn read_all<T>(
    filter: Vec<(&str, String)>,
    source: &SqlitePool,
) -> Result<Vec<T::Item>, sqlx::Error>
where
    T: DatabaseTable,
{
    Ok(read_query::<T>(filter)
        .build_query_as::<T::Item>()
        .fetch_all(source)
        .await?)
}

pub async fn insert_token(
    token: String,
    owner: &i64,
    permission: Vec<(&i64, bool, bool)>,
    source: &SqlitePool,
) -> Result<(), sqlx::Error> {
    query("INSERT INTO Token (token, user_id) VALUES (?, ?)")
        .bind(&token)
        .bind(owner)
        .execute(source)
        .await?;
    let token_id = read_item::<TokenTable>(vec![("token", token)], source)
        .await?
        .id;
    for item in permission {
        query("INSERT INTO TokenGrant (token_id, email_id, is_personal, is_read_only) VALUES (?, ?, ?, ?)").bind(token_id).bind(item.0).bind(item.1).bind(item.2).execute(source).await?;
    }
    Ok(())
}

pub async fn delete_token(
    id: i64,
    source: &SqlitePool,
) -> Result<(), sqlx::Error> {
    query("DELETE FROM TokenGrant WHERE token_id = ?").bind(id).execute(source).await?;
    query("DELETE FROM Token WHERE id = ?").bind(id).execute(source).await?;
    Ok(())
}
