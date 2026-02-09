// SPDX-FileCopyrightText: 2025-2026 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use std::borrow::Cow;

const INVALID_CHARS: &[char] = &[':', '/', '\\'];

pub fn sanitize_game_name(name: &str) -> Cow<'_, str> {
    if name.contains(INVALID_CHARS) {
        Cow::Owned(name.replace(INVALID_CHARS, ""))
    } else {
        Cow::Borrowed(name)
    }
}
