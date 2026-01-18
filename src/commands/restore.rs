// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Args, Command};
use crate::cli_helpers::ensure_steam_account_selected;
use crate::config::Config;
use crate::gamedb;
use crate::infer;
use crate::operations::restore_game;

pub struct Restore;

impl Command for Restore {
    fn run(args: Args, config: &Config) {
        if !config.save_dir.exists() {
            eprintln!("Backup directory doesn't exist.");
            return;
        }

        let installed_games = gamedb::get_installed_games();

        if config.steam_account_id.is_none() && installed_games.iter().any(|g| g.source == "Steam") {
            ensure_steam_account_selected(config);
        }

        if let Some(launcher) = args.get_flag_value("infer") {
            infer::restore(launcher, config);
            return;
        }

        for game in &installed_games {
            if !args.positional.is_empty() && !args.positional.contains(&game.name) {
                continue;
            }

            if let Err(e) = restore_game(game, config) {
                eprintln!("Failed to restore {}: {e}", game.name);
            } else {
                println!("Restored {}.", game.name);
            }
        }
    }
}
