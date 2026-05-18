mod lock;
mod pull;
mod push;
mod resolve;
mod setup;

use crate::core::errors::{AppError, AppResult};

use super::api::GithubClient;
use super::auth::{get_github_sync_status, read_token};
use super::files::collect_local_files;
use super::merge::merge_remote_local;
use super::settings::{
    read_local_settings, save_pending_conflicts, update_last_sync, GithubRepoSettings,
};
use super::types::{
    GithubAutoSyncResult, GithubErrorCode, GithubSyncStatus, SyncConflictResolution, SyncResult,
    SyncState,
};

use lock::with_sync_lock;
use pull::read_remote_files;
use push::push_all;
use resolve::resolve_sync_conflict_locked;
use setup::setup_github_sync_locked;

fn github_client() -> AppResult<GithubClient> {
    let token = read_token()?
        .ok_or_else(|| AppError::new(GithubErrorCode::NotConnected, "GitHub is not connected"))?;
    GithubClient::new(token)
}

pub fn has_local_changes(settings: &GithubRepoSettings) -> bool {
    collect_local_files().is_ok_and(|files| files != settings.baseline_files)
}

pub(super) fn setup_github_sync_blocking() -> AppResult<GithubSyncStatus> {
    with_sync_lock(setup_github_sync_locked)
}

fn not_connected_result() -> SyncResult {
    SyncResult {
        status: SyncState::NotConnected,
        conflicts: Vec::new(),
    }
}

pub(super) fn sync_now_blocking() -> AppResult<SyncResult> {
    with_sync_lock(sync_now_blocking_locked)
}

pub(super) fn sync_auto_blocking() -> AppResult<GithubAutoSyncResult> {
    with_sync_lock(sync_auto_blocking_locked)
}

fn sync_auto_blocking_locked() -> AppResult<GithubAutoSyncResult> {
    let Some(settings) = read_local_settings()?.github else {
        return Ok(GithubAutoSyncResult {
            status: get_github_sync_status()?,
            result: None,
        });
    };

    if read_token()?.is_none()
        || !has_local_changes(&settings)
        || !settings.pending_conflicts.is_empty()
    {
        return Ok(GithubAutoSyncResult {
            status: get_github_sync_status()?,
            result: None,
        });
    }

    let result = sync_now_blocking_locked()?;
    Ok(GithubAutoSyncResult {
        status: get_github_sync_status()?,
        result: Some(result),
    })
}

fn sync_now_blocking_locked() -> AppResult<SyncResult> {
    let client = github_client()?;
    let Some(settings) = read_local_settings()?.github else {
        return Ok(not_connected_result());
    };

    let before = client.branch_head(
        &settings.repo_owner,
        &settings.repo_name,
        &settings.default_branch,
    )?;
    let remote_files = read_remote_files(&client, &settings, &before)?;
    let local_files = collect_local_files()?;
    let merge = merge_remote_local(&settings, &local_files, &remote_files)?;
    if !merge.conflicts.is_empty() {
        save_pending_conflicts(&merge.conflicts)?;
        return Ok(SyncResult {
            status: SyncState::Conflict,
            conflicts: merge.conflicts,
        });
    }

    let head = push_all(&client, &settings)?;
    update_last_sync(head.clone(), collect_local_files()?)?;

    Ok(SyncResult {
        status: if head != before {
            SyncState::Pushed
        } else if merge.changed_local_files {
            SyncState::Pulled
        } else {
            SyncState::UpToDate
        },
        conflicts: Vec::new(),
    })
}

pub(super) fn resolve_sync_conflict_blocking(
    path: String,
    resolution: SyncConflictResolution,
) -> AppResult<()> {
    with_sync_lock(|| resolve_sync_conflict_locked(path, resolution))
}
