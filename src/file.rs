// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use blake3::Hasher;
use std::fs::File;
use std::io::{BufReader, copy};
use std::path::Path;

pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

pub fn hash_file(file_path: &Path) -> String {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();

    copy(&mut reader, &mut hasher).unwrap();

    hasher.finalize().to_hex().to_string()
}
