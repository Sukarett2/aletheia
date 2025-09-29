// SPDX-FileCopyrightText: 2025 Spencer
// SPDX-License-Identifier: AGPL-3.0-only

use reqwest::{blocking::Client, header};
use semver::Version;

const RELEASES_URL: &str = "https://api.github.com/repos/Spencer-0003/aletheia/releases";
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Clone, serde::Deserialize)]
pub struct Release {
    pub body: String,
    pub tag_name: String,
    #[serde(rename = "prerelease")]
    #[expect(clippy::struct_field_names, reason = "API consistency")]
    pub pre_release: bool,
    #[serde(rename = "html_url")]
    pub url: String
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to deserialize: {0}")]
    Deserialize(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error)
}

pub type Result<T> = core::result::Result<T, Error>;

pub enum UpdateStatus {
    UpToDate,
    Available(Release)
}

pub fn check() -> Result<UpdateStatus> {
    let client = Client::new();
    let response = client.get(RELEASES_URL).header(header::USER_AGENT, USER_AGENT).send()?.error_for_status()?;

    let releases: Vec<Release> = serde_json::from_reader(response)?;
    let Some(latest_release) = releases.into_iter().find(|r| !r.pre_release) else {
        return Ok(UpdateStatus::UpToDate);
    };

    let current_version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    let new_version = Version::parse(&latest_release.tag_name).unwrap();

    if current_version < new_version {
        Ok(UpdateStatus::Available(latest_release))
    } else {
        Ok(UpdateStatus::UpToDate)
    }
}
