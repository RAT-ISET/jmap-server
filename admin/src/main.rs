// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /admin/src/main.rs
// Main of the project.

use std::env;
use std::fs::read_to_string;
use std::sync::Arc;
use clap::{Arg, ArgAction, Command};
use tracing::{debug, error, info};
use jmap_core::conf::Config;
use crate::database::init;

mod database;

#[tokio::main]
async fn main() {
    jmap_core::init_log();
    let matches =
        Command::new("ISET JMAP Server Account Manager")
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .about("ISET JMAP Server Account Manager")
            .disable_version_flag(true)
            .arg(
                Arg::new("version")
                    .long("version")
                    .short('V')
                    .help("Show version information")
                    .action(ArgAction::SetTrue),
            )
            .subcommand(
                Command::new("init").about("Initialize the SQLite file").arg(
                    Arg::new("path")
                        .help("Path of the configuration file")
                        .default_value("/etc/padeploy/conf.toml")
                        .value_name("PATH")
                        .required(true),
                ),
    )
        .after_help("Author: Team ISET <team@ratiset.org>\n\nLicense:\n  Copyright (c) 2026 Team ISET\n  Licensed under the MIT.\n\n  https://github.com/RAT-ISET/jmap-server")
        .arg_required_else_help(true)
        .get_matches();
    info!(target: "arg", argv = ?std::env::args().collect::<Vec<_>>(), "exec");
    if matches.get_flag("version") {
        println!(
            "PA Deploy Client {}\nCopyright (c) 2026 Phiarc Team and St Rangeset\nLicensed under the GPLv3 or later License.",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    match matches.subcommand() {
        Some(("init", c)) => {
            let config_path = match c
                .get_one::<String>("path"){
                Some(p) => p,
                None => {
                    error!("Missed path");
                    return;
                }
            };
            debug!("Configuration loading");
            let config: Arc<Config> =
                Arc::new(toml::from_str(read_to_string(config_path).unwrap().as_str()).unwrap());

            init(&config.database).await.unwrap();
        }
        _ => {
            error!("No subcommand provided");
            return;
        }
    }
}
