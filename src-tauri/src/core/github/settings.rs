use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::core::errors::{AppError, AppResult};
use crate::platform::platform;

use super::types::SyncConflict;

const SETTINGS_FILE: &str = "github.json";

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GithubLocalSettings {
    pub device_id: String,
    pub github: Option<GithubRepoSettings>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepoSettings {
    pub repo_owner: String,
    pub repo_name: String,
    pub default_branch: String,
    pub last_synced_commit_sha: Option<String>,
    pub last_synced_at: Option<String>,
    #[serde(default)]
    pub baseline_files: BTreeMap<String, String>,
    #[serde(default)]
    pub pending_conflicts: BTreeMap<String, SyncConflict>,
}

fn settings_dir() -> PathBuf {
    platform().local_settings_dir()
}

fn settings_path() -> PathBuf {
    settings_dir().join(SETTINGS_FILE)
}

pub fn read_local_settings() -> AppResult<GithubLocalSettings> {
    let path = settings_path();
    if !path.exists() {
        return Ok(GithubLocalSettings::default());
    }

    let content = fs::read_to_string(path).map_err(AppError::io)?;
    serde_json::from_str(&content).map_err(AppError::json)
}

pub fn write_local_settings(settings: &GithubLocalSettings) -> AppResult<()> {
    fs::create_dir_all(settings_dir()).map_err(AppError::io)?;
    let content = serde_json::to_string_pretty(settings).map_err(AppError::json)?;
    fs::write(settings_path(), format!("{content}\n")).map_err(AppError::io)
}

pub fn get_or_create_device_id() -> AppResult<String> {
    let mut settings = read_local_settings()?;
    if settings.device_id.trim().is_empty() {
        settings.device_id = Uuid::new_v4().to_string();
        write_local_settings(&settings)?;
    }
    Ok(settings.device_id)
}

pub fn save_github_repo(repo: GithubRepoSettings) -> AppResult<()> {
    let mut settings = read_local_settings()?;
    if settings.device_id.trim().is_empty() {
        settings.device_id = Uuid::new_v4().to_string();
    }
    settings.github = Some(repo);
    write_local_settings(&settings)
}

pub fn update_last_sync(
    commit_sha: String,
    baseline_files: BTreeMap<String, String>,
) -> AppResult<()> {
    let mut settings = read_local_settings()?;
    let Some(github) = settings.github.as_mut() else {
        return Ok(());
    };
    github.last_synced_commit_sha = Some(commit_sha);
    github.last_synced_at = Some(chrono::Utc::now().to_rfc3339());
    github.baseline_files = baseline_files;
    github.pending_conflicts.clear();
    write_local_settings(&settings)
}

pub fn save_pending_conflicts(conflicts: &[SyncConflict]) -> AppResult<()> {
    let mut settings = read_local_settings()?;
    let Some(github) = settings.github.as_mut() else {
        return Ok(());
    };
    github.pending_conflicts = conflicts
        .iter()
        .map(|conflict| (conflict.path.clone(), conflict.clone()))
        .collect();
    write_local_settings(&settings)
}

pub fn clear_github_repo() -> AppResult<()> {
    let mut settings = read_local_settings()?;
    settings.github = None;
    write_local_settings(&settings)
}
