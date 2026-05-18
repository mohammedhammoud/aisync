use std::time::Duration;

use serde::Deserialize;
use specta::Type;

const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/mohammedhammoud/ai-sync/releases/latest";
const RELEASES_URL: &str = "https://github.com/mohammedhammoud/ai-sync/releases/latest";

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AvailableUpdate {
    pub version: String,
    pub download_url: String,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: Option<String>,
    html_url: Option<String>,
}

fn parse_stable_version(version: &str) -> Option<[u32; 3]> {
    let version = version.trim();
    let version = version.strip_prefix("aisync-").unwrap_or(version);
    let version = version.strip_prefix('v').unwrap_or(version);
    let parts = version
        .split('.')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;

    match parts.as_slice() {
        [major, minor, patch] => Some([*major, *minor, *patch]),
        _ => None,
    }
}

pub fn is_newer_stable_version(candidate: &str, current: &str) -> bool {
    let Some(candidate_parts) = parse_stable_version(candidate) else {
        return false;
    };
    let Some(current_parts) = parse_stable_version(current) else {
        return false;
    };

    candidate_parts > current_parts
}

pub fn check_available_update() -> Option<AvailableUpdate> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(1))
        .build()
        .ok()?;
    let response = client
        .get(LATEST_RELEASE_URL)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "AISync")
        .send()
        .ok()?;

    if !response.status().is_success() {
        return None;
    }

    let release = response.json::<GithubRelease>().ok()?;
    let version = release.tag_name?;

    if !is_newer_stable_version(&version, env!("CARGO_PKG_VERSION")) {
        return None;
    }

    Some(AvailableUpdate {
        version,
        download_url: release.html_url.unwrap_or_else(|| RELEASES_URL.into()),
    })
}

#[cfg(test)]
mod tests {
    use super::is_newer_stable_version;

    #[test]
    fn detects_newer_stable_versions() {
        assert!(is_newer_stable_version("aisync-v0.3.0", "0.2.2"));
        assert!(is_newer_stable_version("v0.3.0", "0.2.2"));
        assert!(is_newer_stable_version("0.2.3", "0.2.2"));
    }

    #[test]
    fn ignores_same_older_and_prerelease_versions() {
        assert!(!is_newer_stable_version("v0.2.2", "0.2.2"));
        assert!(!is_newer_stable_version("v0.2.1", "0.2.2"));
        assert!(!is_newer_stable_version("v0.3.0-beta.1", "0.2.2"));
    }
}
