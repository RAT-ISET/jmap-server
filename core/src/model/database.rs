// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/io/database.rs
// Database linker.

use crate::conf::ConfigDatabase;
use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, QueryBuilder, SqlitePool};
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

fn get_command_where<T>(name: &str) -> String
where
    T: DatabaseTable,
{
    format!(
        "SELECT {} FROM {} WHERE {} = ",
        T::COLUMN_NAME,
        T::TABLE_NAME,
        name
    )
}

fn get_command<T>() -> String
where
    T: DatabaseTable,
{
    format!("SELECT {} FROM {}", T::COLUMN_NAME, T::TABLE_NAME)
}

pub async fn read_where_item<T>(
    name: &str,
    value: String,
    source: &SqlitePool,
) -> Result<T::Item, sqlx::Error>
where
    T: DatabaseTable,
{
    let result = QueryBuilder::new(get_command_where::<T>(name))
        .push_bind(value)
        .build_query_as::<T::Item>()
        .fetch_one(source)
        .await?;
    Ok(result)
}

pub async fn read_where_all<T>(
    name: &str,
    value: String,
    source: &SqlitePool,
) -> Result<Vec<T::Item>, sqlx::Error>
where
    T: DatabaseTable,
{
    let result = QueryBuilder::new(get_command_where::<T>(name))
        .push_bind(value)
        .build_query_as::<T::Item>()
        .fetch_all(source)
        .await?;
    Ok(result)
}

pub async fn read_all<T>(source: &SqlitePool) -> Result<Vec<T::Item>, sqlx::Error>
where
    T: DatabaseTable,
{
    let result = QueryBuilder::new(get_command::<T>())
        .build_query_as::<T::Item>()
        .fetch_all(source)
        .await?;
    Ok(result)
}
