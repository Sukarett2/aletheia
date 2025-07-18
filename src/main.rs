// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

#![warn(clippy::pedantic)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::option_if_let_else)]
#![deny(clippy::allow_attributes_without_reason)]
#![deny(clippy::string_to_string)]
#![deny(clippy::get_unwrap)]
#![allow(clippy::unreadable_literal, reason = "'Readable' literals are ugly")]

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli_helpers;
mod commands;
mod config;
mod dirs;
mod file;
mod gamedb;
mod infer;
mod operations;
mod scanner;
mod ui;

#[cfg(all(feature = "updater", not(debug_assertions)))]
mod updater;

use commands::{Args, Command};

fn main() {
    env_logger::init();

    log::info!("Aletheia v{} (Flatpak: {})", env!("CARGO_PKG_VERSION"), cfg!(target_os = "linux") && std::env::var("FLATPAK_ID").is_ok() && std::fs::exists("/.flatpak-info").unwrap_or(false));

    let config = config::Config::load();
    let mut args = std::env::args().skip(1);

    if let Some(cmd) = args.next() {
        let args = Args::parse(&args.collect::<Vec<String>>());
        match cmd.as_str() {
            "backup" => commands::Backup::run(args, &config),
            "restore" => commands::Restore::run(args, &config),
            #[cfg(all(feature = "updater", not(debug_assertions)))]
            "update" => commands::Update::run(args, &config),
            "update_gamedb" => commands::UpdateGameDb::run(args, &config),
            "update_custom_gamedbs" => commands::UpdateCustom::run(args, &config),
            _ => eprintln!("Command not found.")
        }
    } else {
        ui::run(&config);
    }
}
