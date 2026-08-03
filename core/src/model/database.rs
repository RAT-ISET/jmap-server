// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/io/database.rs
// Database linker.

use crate::conf::ConfigDatabase;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, QueryBuilder, Sqlite, SqlitePool};
use std::env::current_dir;
use tracing::debug;

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

fn read_query<T>(filter: Vec<(&str, String)>, source: &SqlitePool) -> QueryBuilder<Sqlite>
where
    T: DatabaseTable,
{
    let mut builder = QueryBuilder::new(get_command::<T>());
    if !filter.is_empty() {
        builder.push("WHERE ");
        let mut separated = builder.separated(" AND ");
        filter.iter().for_each(|(k, v)| {
            separated.push(k).push(" = ").push_bind(v);
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
    Ok(read_query::<T>(filter, source)
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
    Ok(read_query::<T>(filter, source)
        .build_query_as::<T::Item>()
        .fetch_all(source)
        .await?)
}
