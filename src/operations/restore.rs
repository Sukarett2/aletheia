// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use crate::config::Config;
use crate::dirs::expand_path;
use crate::file::hash_file;
use crate::gamedb::Manifest;
use crate::scanner::Game;
use std::fs::{copy, create_dir_all};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Game not found")]
    GameNotFound,
    #[error("{0} is missing or corrupted")]
    MissingOrCorruptedFiles(String)
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn restore_game(game_dir: &Path, manifest: &Manifest, installed_games: &[Game], config: &Config) -> Result<bool> {
    let steam_id = config.steam_account_id.as_deref();
    let game_name = &manifest.name;

    let game = installed_games.iter().find(|g| g.name == *game_name).ok_or(Error::GameNotFound)?;

    for file in &manifest.files {
        let src_file = game_dir.join(Path::new(&file.path).file_name().unwrap());

        if !src_file.exists() || hash_file(&src_file) != file.hash {
            return Err(Error::MissingOrCorruptedFiles(src_file.file_name().unwrap().to_string_lossy().to_string()));
        }
    }

    for file in &manifest.files {
        let file_path = Path::new(&file.path);

        #[cfg(unix)]
        let expanded = expand_path(file_path, game.installation_dir.as_deref(), game.prefix.as_deref(), steam_id);

        #[cfg(windows)]
        let expanded = expand_path(file_path, game.installation_dir.as_deref(), steam_id);

        let src_file = game_dir.join(file_path.file_name().unwrap());

        if expanded.exists() && hash_file(&expanded) == file.hash {
            continue;
        }

        create_dir_all(expanded.parent().unwrap()).unwrap();
        copy(&src_file, &expanded).unwrap();
    }

    Ok(true)
}
