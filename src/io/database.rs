// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /src/io/database.rs
// Database linker.

use std::env::current_dir;
use crate::conf::ConfigDatabase;
use sqlx::SqlitePool;
use tracing::debug;

pub async fn init(config: &ConfigDatabase) -> Result<(), sqlx::Error> {
    debug!("Read path: {}/{}", current_dir()?.display(), &config.file);
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(&config.file).create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("./tests/database/migrations")
        .run(&pool)
        .await?;
    Ok(())
}
