// SPDX-FileCopyrightText: 2025-2026 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

mod app;
mod first_time_setup;
mod handlers;
mod restore_dialog;

pub use app::run;
pub use first_time_setup::run_first_time_setup;
pub use restore_dialog::run_restore_dialog;
