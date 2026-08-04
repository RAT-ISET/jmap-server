// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /admin/src/io/database.rs
// Database linker.

use jmap_core::conf::ConfigDatabase;
use jmap_core::database::read_item;
use jmap_core::token::TokenTable;
use sqlx::{SqlitePool, query};
use std::env::current_dir;
use tracing::debug;

pub async fn init(config: &ConfigDatabase) -> Result<SqlitePool, sqlx::Error> {
    debug!("Read path: {}/{}", current_dir()?.display(), &config.file);
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&config.file)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    insert_system(&pool).await?;
    insert_email("sys".to_string(), 0, &pool).await?;
    Ok(pool)
}

async fn insert_system(source: &SqlitePool) -> Result<(), sqlx::Error> {
    query("INSERT INTO Account (id, username) VALUES (0, \"System\")")
        .execute(source)
        .await?;
    Ok(())
}

pub async fn insert_account(user_name: String, source: &SqlitePool) -> Result<(), sqlx::Error> {
    query("INSERT INTO Account (username) VALUES (?)")
        .bind(user_name)
        .execute(source)
        .await?;
    Ok(())
}

pub async fn insert_email(
    email: String,
    user_id: i64,
    source: &SqlitePool,
) -> Result<(), sqlx::Error> {
    query("INSERT INTO Email (name, owner) VALUES (?, ?)")
        .bind(email)
        .bind(user_id)
        .execute(source)
        .await?;
    Ok(())
}

pub async fn trans_email(
    email: i64,
    new_owner: i64,
    source: &SqlitePool,
) -> Result<(), sqlx::Error> {
    query("UPDATE Email SET owner = ? WHERE id = ?")
        .bind(new_owner)
        .bind(email)
        .execute(source)
        .await?;
    query("DELETE FROM TokenGrant WHERE email_id = ?")
        .bind(email)
        .execute(source)
        .await?;
    Ok(())
}

// TODO(add_token_from_other): Add the token by API or other method.
