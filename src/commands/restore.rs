// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use super::{Args, Command};
use crate::cli_helpers::ensure_steam_account_selected;
use crate::config::Config;
use crate::gamedb;
use crate::infer;
use crate::operations::restore_game;
use std::fs::File;

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

        for game in std::fs::read_dir(&config.save_dir).unwrap() {
            let game_dir = game.unwrap().path();
            let is_dir = game_dir.is_dir();
            let game_name = game_dir.file_name().unwrap().to_string_lossy();

            if !is_dir || game_name.starts_with('.') {
                continue;
            }

            let manifest_path = game_dir.join("aletheia_manifest.yaml");

            if !manifest_path.exists() {
                eprintln!("{game_name} is missing a manifest file.");
                continue;
            }

            let Ok(manifest) = serde_yaml::from_reader::<File, gamedb::Manifest>(File::open(manifest_path).unwrap()) else {
                eprintln!("Failed to parse {}'s manifest.", game_dir.file_name().unwrap().display());
                continue;
            };

            if !args.positional.is_empty() && !args.positional.contains(&manifest.name) {
                continue;
            }

            if let Err(e) = restore_game(&game_dir, &manifest, &installed_games, config) {
                println!("Failed to restore {}: {e}", manifest.name);
            } else {
                println!("Restored {}.", manifest.name);
            }
        }
    }
}
