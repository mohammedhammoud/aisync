use crate::core::errors::AppResult;

use crate::core::github::settings::{clear_github_repo, read_local_settings};
use crate::core::github::sync::has_local_changes;
use crate::core::github::types::{GithubLoginStart, GithubSyncStatus};

use super::device_flow;
use super::keychain::{delete_token, read_token};

#[tauri::command]
#[specta::specta]
pub fn start_github_login(app: tauri::AppHandle) -> AppResult<GithubLoginStart> {
    device_flow::start_github_login(app)
}

#[tauri::command]
#[specta::specta]
pub fn logout_github() -> AppResult<()> {
    delete_token()?;
    clear_github_repo()
}

#[tauri::command]
#[specta::specta]
pub fn get_github_sync_status() -> AppResult<GithubSyncStatus> {
    let settings = read_local_settings()?;
    let has_token = read_token()?.is_some();
    let github = settings.github;

    let has_local_changes = github.as_ref().is_some_and(has_local_changes);

    Ok(GithubSyncStatus {
        connected: has_token && github.is_some(),
        repo_owner: github.as_ref().map(|item| item.repo_owner.clone()),
        repo_name: github.as_ref().map(|item| item.repo_name.clone()),
        default_branch: github.as_ref().map(|item| item.default_branch.clone()),
        last_synced_commit_sha: github
            .as_ref()
            .and_then(|item| item.last_synced_commit_sha.clone()),
        last_synced_at: github.as_ref().and_then(|item| item.last_synced_at.clone()),
        has_token,
        has_local_changes,
    })
}
