// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use crate::config::Config;
use crate::gamedb;
use crate::infer::Launcher;
use crate::infer::launchers::Heroic;
use crate::operations::restore_game;
use crate::utils;
use std::fs::File;

#[cfg(all(unix, not(target_os = "macos")))]
use crate::infer::launchers::Lutris;

pub fn restore(launcher: &str, config: &Config) {
    let game = match launcher.to_lowercase().as_str() {
        "heroic" => Heroic::get_game(),
        #[cfg(all(unix, not(target_os = "macos")))]
        "lutris" => Lutris::get_game(),
        _ => {
            log::warn!("Backup was ran with infer using an unsupported launcher.");
            return;
        }
    };

    if let Some(game) = game {
        let game_dir = config.save_dir.join(utils::sanitize_game_name(&game.name).as_ref());

        if !game_dir.exists() || !game_dir.is_dir() {
            log::warn!("No backups found for {}.", game.name);
            return;
        }

        let manifest_path = game_dir.join("aletheia_manifest.yaml");

        if !manifest_path.exists() {
            log::error!("{} is missing a manifest file.", game.name);
            return;
        }

        let Ok(manifest) = serde_yaml::from_reader::<File, gamedb::Manifest>(File::open(manifest_path).unwrap()) else {
            log::error!("Failed to parse {}'s manifest.", game_dir.file_name().unwrap().display());
            return;
        };

        if let Err(e) = restore_game(&game_dir, &manifest, &gamedb::get_installed_games(), config) {
            log::error!("Failed to restore {}: {}", game.name, e);
        } else {
            log::info!("Restore up {}.", game.name);
        }
    }
}
