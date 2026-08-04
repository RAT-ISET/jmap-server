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

// TODO(change_owner): Change the owner for email.
// TODO(delete_token): Delete the token.
// TODO(add_token_from_other): Add the token by API or other method.
