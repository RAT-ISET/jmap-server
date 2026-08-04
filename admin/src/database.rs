// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /admin/src/io/database.rs
// Database linker.

use jmap_core::conf::ConfigDatabase;
use sqlx::{SqlitePool, query};
use std::env::current_dir;
use tracing::debug;
use jmap_core::account::AccountTable;
use jmap_core::database::{insert_token, read_all, read_item};
use jmap_core::email::EmailTable;

pub async fn init(config: &ConfigDatabase) -> Result<SqlitePool, sqlx::Error> {
    debug!("Read path: {}/{}", current_dir()?.display(), &config.file);
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(&config.file)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    insert_disable(&pool).await?;
    insert_system(&pool).await?;
    insert_email("sys".to_string(), 0, &pool).await?;
    Ok(pool)
}

pub async fn test_init(source: &SqlitePool) -> Result<(), sqlx::Error> {
    insert_account("Tester 1".to_string(), &source).await?;
    insert_account("Tester 2".to_string(), &source).await?;
    let id_1 = read_item::<AccountTable>(vec![("username", "Tester 1".to_string())], source).await?.id;
    let id_2 = read_item::<AccountTable>(vec![("username", "Tester 2".to_string())], source).await?.id;
    insert_email("test1a".to_string(), id_1, &source).await?;
    insert_email("test1b".to_string(), id_1, &source).await?;
    insert_email("test2".to_string(), id_2, &source).await?;
    let email_1 = read_all::<EmailTable>(vec![("owner", id_1.to_string())], source).await?;
    let email_1a = email_1.get(0).unwrap().id;
    let email_1b = email_1.get(1).unwrap().id;
    let email_2 = read_item::<EmailTable>(vec![("owner", id_2.to_string())], source).await?.id;
    insert_token("test1token".to_string(), &id_1, vec![(&email_1a, true, false), (&email_1b, true, false)], &source).await?;
    insert_token("test2token".to_string(), &id_2, vec![(&email_2, true, false), (&email_1b, false, true)], &source).await?;
    Ok(())
}

async fn insert_disable(source: &SqlitePool) -> Result<(), sqlx::Error> {
    query("INSERT INTO Account (id, username) VALUES (-1, \"REMOVED\")")
        .execute(source)
        .await?;
    Ok(())
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
    email: &i64,
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
