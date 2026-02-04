// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

slint::include_modules!();

use super::handlers::{games, settings};
use crate::archive::{ArchiveReader, Error as ArchiveError};
use crate::config::Config as AletheiaConfig;
use crate::gamedb;
use crate::operations::{RestoreError, restore_game};
use std::cell::RefCell;
use std::path::Path;
use std::process::Command;
use std::rc::Rc;

#[cfg(all(feature = "updater", not(debug_assertions)))]
use crate::updater;

pub fn run(config: &AletheiaConfig) {
    #[cfg(all(feature = "updater", not(debug_assertions)))]
    if config.check_for_updates
        && let Ok(updater::UpdateStatus::Available(release)) = updater::check()
    {
        let updater_window = Updater::new().unwrap();
        let updater_logic = updater_window.global::<UpdaterLogic>();

        slint::set_xdg_app_id("moe.spencer.Aletheia").unwrap();

        updater_logic.set_current_version(env!("CARGO_PKG_VERSION").into());
        updater_logic.set_new_version(release.tag_name.into());
        updater_logic.set_changelog(release.body.into());

        updater_logic.on_skip_update({
            let updater_window = updater_window.as_weak().unwrap();

            move || updater_window.window().hide().unwrap()
        });

        updater_logic.on_download_update({
            let updater_window = updater_window.as_weak().unwrap();

            move || {
                open_url(&release.url);
                updater_window.window().hide().unwrap();
            }
        });

        updater_window.run().unwrap();

        if updater_logic.get_downloading() {
            return;
        }
    }

    let app = App::new().unwrap();
    let app_weak = app.as_weak();
    let cfg = Rc::new(RefCell::new(config.clone()));

    slint::set_xdg_app_id("moe.spencer.Aletheia").unwrap();

    setup_app_handlers(&app);
    games::setup(&app_weak, &cfg);
    settings::setup(&app_weak, &cfg);

    app.run().unwrap();
}

pub fn run_restore_dialog(config: &AletheiaConfig, archive_path: &str) {
    let archive_path = Path::new(archive_path);
    if !archive_path.exists() {
        return;
    }

    let Ok(reader) = ArchiveReader::open(archive_path) else {
        return;
    };

    let cfg = Rc::new(RefCell::new(config.clone()));
    let restore_dialog = RestoreDialog::new().unwrap();
    let restore_logic = restore_dialog.global::<RestoreLogic>();

    restore_logic.on_cancel({
        let restore_weak = restore_dialog.as_weak().unwrap();

        move || {
            restore_weak.hide().unwrap();
        }
    });

    restore_logic.on_restore({
        let restore_weak = restore_dialog.as_weak().unwrap();

        move || {
            let restore_logic = restore_weak.global::<RestoreLogic>();
            let installed_games = gamedb::get_installed_games();
            let game_name = restore_logic.get_game_name();
            let game_name = game_name.as_str();

            let Some(game) = installed_games.iter().find(|g| g.name == game_name) else {
                restore_logic.set_error("GAME_NOT_INSTALLED".into());
                return;
            };

            if let Err(RestoreError::Archive(e)) = restore_game(game, &cfg.borrow()) {
                let error_message = match e {
                    ArchiveError::ChecksumMismatch(..) | ArchiveError::FileNotFound(_) => "ARCHIVE_CORRUPTED",
                    ArchiveError::InvalidArchive | ArchiveError::Serialization(_) => "INVALID_ARCHIVE",
                    ArchiveError::Io(_) => "IO_ERROR",
                    ArchiveError::UnsupportedVersion(_) => "UNSUPPORTED_ARCHIVE_VERSION"
                };

                restore_logic.set_error(error_message.into());
            } else {
                restore_weak.hide().unwrap();
            }
        }
    });

    restore_logic.set_game_name(reader.game.into());
    slint::set_xdg_app_id("moe.spencer.Aletheia").unwrap();

    restore_dialog.run().unwrap();
}

fn setup_app_handlers(app: &App) {
    let app_logic = app.global::<AppLogic>();

    app_logic.set_version(env!("CARGO_PKG_VERSION").into());
    app_logic.on_open_url(|url| open_url(&url));
}

fn open_url(url: &str) {
    #[cfg(all(unix, not(target_os = "macos")))]
    Command::new("xdg-open").arg(url).spawn().ok();

    #[cfg(target_os = "macos")]
    Command::new("open").arg(url).spawn().ok();

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        Command::new("cmd").args(["/c", "start", url]).creation_flags(0x08000000).spawn().ok();
    }
}
