// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use std::borrow::Cow;

pub fn sanitize_game_name(name: &str) -> Cow<'_, str> {
    if name.contains(':') {
        // NTFS doesn't support : and this makes sense on Unix for cross-OS syncing
        Cow::Owned(name.replace(':', ""))
    } else {
        Cow::Borrowed(name)
    }
}
