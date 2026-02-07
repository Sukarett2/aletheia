// SPDX-FileCopyrightText: 2026 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use crate::ui::app::FirstTimeSetup;
use slint::ComponentHandle;

pub fn run_first_time_setup() {
    let first_time_setup = FirstTimeSetup::new().unwrap();

    slint::set_xdg_app_id("moe.spencer.Aletheia").unwrap();
    first_time_setup.run().unwrap();
}
