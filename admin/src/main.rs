// ==============================================================
// Copyright (c) 2026 Team ISET
// Licensed under the MIT.
// https://github.com/RAT-ISET/jmap-server
// ==============================================================
// Path /admin/src/main.rs
// Main of the project.

use crate::database::init;
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use jmap_core::account::AccountTable;
use jmap_core::conf::Config;
use jmap_core::database::{open, read_all};
use jmap_core::email::EmailTable;
use jmap_core::token::{TokenItemDisplayer, TokenTable};
use std::env;
use std::fs::read_to_string;
use std::sync::Arc;
use tracing::{debug, error, info};

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
            .arg(
                Arg::new("path")
                    .help("Path of the configuration file")
                    .value_name("PATH")
                    .required_unless_present("version"),
            )
            .subcommand(
                Command::new("init").about("Initialize the SQLite file")
            )
            .subcommand(
                Command::new("add").about("Add account or email")
                    .subcommand(
                        Command::new("account").about("Add the account").arg(
                            Arg::new("name")
                                .help("Account name")
                                .value_name("NAME")
                                .required(true),
                        )
                    )
                    .subcommand(
                        Command::new("email").about("Add the email").arg(
                            Arg::new("name")
                                .help("Email name")
                                .value_name("NAME")
                                .required(true),
                        ).arg(
                            Arg::new("owner")
                                .value_parser(value_parser!(i64))
                                .help("Email owner ID")
                                .value_name("OWNER")
                                .required(true),
                        )
                    )
            )
            .subcommand(
                Command::new("list").about("List account, email or token")
                    .subcommand(
                        Command::new("account").about("List the account").arg(
                            Arg::new("username")
                                .help("Account name")
                                .long("name")
                                .value_name("NAME"),
                        ).arg(
                            Arg::new("id")
                                .help("Account ID")
                                .long("id")
                                .value_name("ID"),
                        )
                    )
                    .subcommand(
                        Command::new("email").about("List the email").arg(
                            Arg::new("name")
                                .help("Email name")
                                .long("name")
                                .value_name("NAME")
                        ).arg(
                            Arg::new("id")
                                .help("Email ID")
                                .long("id")
                                .value_name("ID"),
                        ).arg(
                            Arg::new("owner")
                                .help("Email owner")
                                .long("owner")
                                .value_name("OWNER"),
                        )
                    )
                    .subcommand(
                        Command::new("token").about("List the token").arg(
                            Arg::new("token")
                                .help("Token value")
                                .long("value")
                                .value_name("VALUE")
                        ).arg(
                            Arg::new("id")
                                .help("Token ID")
                                .long("id")
                                .value_name("ID"),
                        ).arg(
                            Arg::new("user_id")
                                .help("Token owner")
                                .long("owner")
                                .value_name("OWNER"),
                        )
                    )
            )
            .after_help("Author: Team ISET <team@ratiset.org>\n\nLicense:\n  Copyright (c) 2026 Team ISET\n  Licensed under the MIT.\n\n  https://github.com/RAT-ISET/jmap-server")
            .arg_required_else_help(true)
            .get_matches();
    info!(target: "arg", argv = ?env::args().collect::<Vec<_>>(), "exec");
    if matches.get_flag("version") {
        println!(
            "ISET JMAP Server Account Manager {}\nCopyright (c) 2026 Team ISET\nLicensed under the MIT.",
            env!("CARGO_PKG_VERSION")
        );
        return;
    }
    let config_path = match matches.get_one::<String>("path") {
        Some(p) => p,
        None => {
            error!("Missed path");
            return;
        }
    };
    debug!("Configuration loading");
    let config: Arc<Config> =
        Arc::new(toml::from_str(read_to_string(config_path).unwrap().as_str()).unwrap());
    match matches.subcommand() {
        Some(("init", c)) => {
            init(&config.database).await.unwrap();
        }
        Some(("add", sub)) => {
            let source = open(&config.database).await.unwrap();
            match sub.subcommand() {
                Some(("account", c)) => database::insert_account(
                    c.get_one::<String>("name").unwrap().to_string(),
                    &source,
                )
                .await
                .unwrap(),
                Some(("email", c)) => database::insert_email(
                    c.get_one::<String>("name").unwrap().to_string(),
                    *c.get_one::<i64>("owner").unwrap(),
                    &source,
                )
                .await
                .unwrap(),
                _ => {
                    error!("No subcommand provided");
                    return;
                }
            }
        }
        Some(("list", sub)) => {
            let source = open(&config.database).await.unwrap();
            match sub.subcommand() {
                Some(("account", c)) => read_all::<AccountTable>(build_query(c), &source)
                    .await
                    .unwrap()
                    .iter()
                    .for_each(|item| println!("{}", item)),
                Some(("email", c)) => read_all::<EmailTable>(build_query(c), &source)
                    .await
                    .unwrap()
                    .iter()
                    .for_each(|item| println!("{}", item)),
                Some(("token", c)) => {
                    for item in read_all::<TokenTable>(build_query(c), &source)
                        .await
                        .unwrap()
                    {
                        println!("{}", TokenItemDisplayer::new(&item, &source).await.unwrap())
                    }
                }
                _ => {
                    error!("No subcommand provided");
                    return;
                }
            }
        }
        _ => {
            error!("No subcommand provided");
            return;
        }
    }
}

fn build_query(matches: &ArgMatches) -> Vec<(&str, String)> {
    let mut result = Vec::new();
    for id in matches.ids() {
        let name = id.as_str();
        if let Some(value) = matches.get_one::<String>(name) {
            result.push((name, value.to_string()))
        }
    }
    result
}

// TODO(create_token): Create the token.
// TODO(link_token): Link the token and email.
// TODO(delete_token): Delete the token.

// TODO(init_test): Add the test initialization.
