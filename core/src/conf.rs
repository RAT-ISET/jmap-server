// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /core/src/conf.rs
// JMAP server configuration.

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigHttp {
    #[allow(unused)]
    pub host: String,
    pub bind: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigJmap {
    pub base_url: String,
    pub root: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ConfigDatabase {
    pub file: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub http: ConfigHttp,
    pub jmap: ConfigJmap,
    pub database: ConfigDatabase,
}
