use std::collections::BTreeMap;

use crate::core::errors::{AppError, AppResult};

use crate::core::github::api::{GithubClient, GithubRepo};
use crate::core::github::auth::get_github_sync_status;
use crate::core::github::constants::DEFAULT_REPO_NAME;
use crate::core::github::files::collect_local_files;
use crate::core::github::settings::{
    read_local_settings, save_github_repo, update_last_sync, GithubRepoSettings,
};
use crate::core::github::types::GithubSyncStatus;

use super::github_client;
use super::push::push_all;

fn ensure_repo(client: &GithubClient, repo_name: &str) -> AppResult<GithubRepo> {
    let user = client.current_user()?;
    if let Some(repo) = client.get_repo(&user.login, repo_name)? {
        return Ok(repo);
    }
    client.create_private_repo(repo_name)
}

fn save_repo_settings(repo: &GithubRepo, head: Option<String>) -> AppResult<()> {
    save_github_repo(GithubRepoSettings {
        repo_owner: repo.owner.login.clone(),
        repo_name: repo.name.clone(),
        default_branch: repo.default_branch.clone(),
        last_synced_commit_sha: head,
        last_synced_at: Some(chrono::Utc::now().to_rfc3339()),
        baseline_files: BTreeMap::new(),
        pending_conflicts: BTreeMap::new(),
    })
}

pub fn setup_github_sync_locked() -> AppResult<GithubSyncStatus> {
    let client = github_client()?;
    let repo = ensure_repo(&client, DEFAULT_REPO_NAME)?;
    let head = client.branch_head(&repo.owner.login, &repo.name, &repo.default_branch)?;
    save_repo_settings(&repo, Some(head))?;

    let settings = read_local_settings()?
        .github
        .ok_or_else(|| AppError::unknown("GitHub repo settings were not saved"))?;
    let head = push_all(&client, &settings)?;
    update_last_sync(head, collect_local_files()?)?;
    get_github_sync_status()
}
