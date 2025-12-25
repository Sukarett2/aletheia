// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use crate::config::Config;
use crate::dirs::{expand_path, shrink_path};
use crate::file::hash_file;
use crate::gamedb::{FileMetadata, GameDbEntry, Manifest};
use crate::scanner::Game;
use crate::utils;
use glob::glob;
use std::fs::{File, copy, create_dir_all, metadata};
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to create backup directory: {0}")]
    DirectoryCreation(#[from] std::io::Error),
    #[error("Malformed manifest file")]
    MalformedManifest
}

pub type Result<T> = core::result::Result<T, Error>;

pub fn backup_game(game: &Game, config: &Config, entry: &GameDbEntry) -> Result<bool> {
    let steam_id = config.steam_account_id.as_deref();
    let backup_folder = config.save_dir.join(utils::sanitize_game_name(&game.name).as_ref());
    let manifest_path = backup_folder.join("aletheia_manifest.yaml");
    let existing_manifest = manifest_path
        .exists()
        .then(|| File::open(&manifest_path).unwrap())
        .map(|file| serde_yaml::from_reader::<File, Manifest>(file).map_err(|_| Error::MalformedManifest))
        .transpose()?;

    let mut changed = false;
    let mut game_files: Vec<FileMetadata> = vec![];
    let mut paths = vec![];

    #[cfg(windows)]
    if let Some(ref windows_paths) = entry.files.windows {
        paths.extend(windows_paths);
    }

    #[cfg(unix)]
    if game.prefix.is_some()
        && let Some(ref windows_paths) = entry.files.windows
    {
        paths.extend(windows_paths);
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    if let Some(ref linux_paths) = entry.files.linux {
        paths.extend(linux_paths);
    }

    #[cfg(target_os = "macos")]
    if let Some(ref mac_paths) = entry.files.mac {
        paths.extend(mac_paths);
    }

    let mut files = vec![];

    for path in paths {
        #[cfg(unix)]
        let expanded = expand_path(Path::new(path), game.installation_dir.as_deref(), game.prefix.as_deref(), steam_id);

        #[cfg(windows)]
        let expanded = expand_path(Path::new(path), game.installation_dir.as_deref(), steam_id);

        let found_paths = glob(&expanded.to_string_lossy()).unwrap();

        for file in found_paths {
            let file = file.unwrap();

            if file.is_dir() {
                log::warn!("Found {} while backing up {}. Glob patterns should match files only.", file.display(), game.name);
                continue;
            }

            if file.file_name().unwrap() == "steam_autocloud.vdf" {
                continue;
            }

            files.push(file);
        }
    }

    if files.is_empty() {
        return Ok(false);
    }

    create_dir_all(&backup_folder)?;

    for file in files {
        #[cfg(unix)]
        let shrunk_file = shrink_path(file.as_path(), game.installation_dir.as_deref(), game.prefix.as_deref(), steam_id);

        #[cfg(windows)]
        let shrunk_file = shrink_path(file.as_path(), game.installation_dir.as_deref(), steam_id);

        let shrunk_file_path = shrunk_file.to_string_lossy();
        let should_backup = existing_manifest.as_ref().is_none_or(|manifest| {
            manifest.files.iter().find(|m| m.path == shrunk_file_path).is_none_or(|existing| {
                metadata(&file).unwrap().modified().unwrap() > existing.modified && existing.hash != hash_file(&file)
            })
        });

        if should_backup {
            let file_metadata = process_file(&file, &backup_folder.join(file.file_name().unwrap()), game, steam_id);

            game_files.push(file_metadata);
            changed = true;
        } else {
            let existing_metadata = existing_manifest.as_ref().unwrap().files.iter().find(|m| m.path == shrunk_file_path).unwrap();

            game_files.push(existing_metadata.to_owned());
        }
    }

    if !changed {
        return Ok(false);
    }

    let game_metadata = Manifest { name: game.name.clone(), files: game_files };

    serde_yaml::to_writer(File::create(&manifest_path).unwrap(), &game_metadata).unwrap();

    Ok(true)
}

fn process_file(file_path: &Path, dest: &Path, game: &Game, steam_id: Option<&str>) -> FileMetadata {
    copy(file_path, dest).unwrap();

    let file_metadata = metadata(file_path).unwrap();

    FileMetadata {
        modified: file_metadata.modified().unwrap(),
        #[cfg(unix)]
        path: shrink_path(file_path, game.installation_dir.as_deref(), game.prefix.as_deref(), steam_id).to_string_lossy().to_string(),
        #[cfg(windows)]
        path: shrink_path(file_path, game.installation_dir.as_deref(), steam_id).to_string_lossy().to_string(),
        hash: hash_file(file_path),
        size: file_metadata.len()
    }
}
