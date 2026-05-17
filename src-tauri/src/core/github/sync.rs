use std::collections::{BTreeMap, BTreeSet};

use crate::core::constants::{INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::sync::run_sync;

use super::api::{GithubClient, GithubRepo};
use super::auth::{get_github_sync_status, read_token};
use super::constants::{
    DEFAULT_REPO_NAME, MANIFEST_FILE, REMOTE_ROOT, SKILL_FILE, SKILL_METADATA_FILE,
};
use super::files::{
    collect_local_files, rebuild_skill_index_from_disk, remove_local_file, write_local_file,
};
use super::manifest::manifest;
use super::merge::merge_remote_local;
use super::paths::{remote_path, validate_sync_path};
use super::settings::{
    read_local_settings, save_github_repo, save_pending_conflicts, update_last_sync,
    write_local_settings, GithubRepoSettings,
};
use super::types::{
    GithubAutoSyncResult, GithubErrorCode, GithubSyncStatus, SyncConflictResolution, SyncResult,
    SyncState,
};

fn github_client() -> AppResult<GithubClient> {
    let token = read_token()?
        .ok_or_else(|| AppError::new(GithubErrorCode::NotConnected, "GitHub is not connected"))?;
    GithubClient::new(token)
}

pub fn has_local_changes(settings: &GithubRepoSettings) -> bool {
    collect_local_files().is_ok_and(|files| files != settings.baseline_files)
}

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

fn push_all(client: &GithubClient, settings: &GithubRepoSettings) -> AppResult<String> {
    let files = collect_local_files()?;
    let mut changed_files = Vec::new();

    for (path, content) in &files {
        let path = remote_path(path);
        let existing = client.read_file(&settings.repo_owner, &settings.repo_name, &path)?;
        if existing
            .as_ref()
            .is_some_and(|(existing_content, _sha)| existing_content == content)
        {
            continue;
        }
        changed_files.push((path, content.clone(), existing.map(|(_, sha)| sha)));
    }

    let manifest_path = remote_path(MANIFEST_FILE);
    let existing_manifest =
        client.read_file(&settings.repo_owner, &settings.repo_name, &manifest_path)?;

    if changed_files.is_empty() && existing_manifest.is_some() {
        return client.branch_head(
            &settings.repo_owner,
            &settings.repo_name,
            &settings.default_branch,
        );
    }

    for (path, content, sha) in changed_files {
        client.write_file(
            &settings.repo_owner,
            &settings.repo_name,
            &path,
            &content,
            sha.as_deref(),
            &sync_commit_message(&path),
        )?;
    }

    client.write_file(
        &settings.repo_owner,
        &settings.repo_name,
        &manifest_path,
        &manifest(&files)?,
        existing_manifest.as_ref().map(|(_, sha)| sha.as_str()),
        "chore(sync): update manifest",
    )?;

    client.branch_head(
        &settings.repo_owner,
        &settings.repo_name,
        &settings.default_branch,
    )
}

fn sync_commit_message(path: &str) -> String {
    if path == remote_path(INSTRUCTIONS_FILE).as_str() {
        return "chore(sync): update instructions".into();
    }

    if let Some(rest) = path.strip_prefix(&format!("{REMOTE_ROOT}/{SKILLS_DIR}/")) {
        if let Some((skill_id, file_name)) = rest.split_once('/') {
            return match file_name {
                SKILL_FILE => format!("chore(sync): update skill {skill_id}"),
                SKILL_METADATA_FILE => format!("chore(sync): update skill metadata {skill_id}"),
                _ => format!("chore(sync): update {rest}"),
            };
        }
    }

    if path == remote_path(MANIFEST_FILE).as_str() {
        return "chore(sync): update manifest".into();
    }

    format!("chore(sync): update {path}")
}

fn read_remote_files(
    client: &GithubClient,
    settings: &GithubRepoSettings,
    head: &str,
) -> AppResult<BTreeMap<String, String>> {
    let paths = client.tree_paths(&settings.repo_owner, &settings.repo_name, head)?;
    let mut remote_paths = BTreeSet::new();
    for path in paths {
        let Some(path) = path
            .strip_prefix(&format!("{REMOTE_ROOT}/"))
            .map(str::to_string)
        else {
            continue;
        };
        if path == MANIFEST_FILE || validate_sync_path(&path).is_err() {
            continue;
        }
        remote_paths.insert(path);
    }
    let mut files = BTreeMap::new();

    for path in remote_paths {
        let full_path = remote_path(&path);
        if let Some((content, _sha)) =
            client.read_file(&settings.repo_owner, &settings.repo_name, &full_path)?
        {
            files.insert(path, content);
        }
    }

    Ok(files)
}

pub(super) fn setup_github_sync_blocking() -> AppResult<GithubSyncStatus> {
    let client = github_client()?;
    let repo = ensure_repo(&client, DEFAULT_REPO_NAME)?;
    let head = client
        .branch_head(&repo.owner.login, &repo.name, &repo.default_branch)
        .ok();
    save_repo_settings(&repo, head)?;

    let settings = read_local_settings()?
        .github
        .ok_or_else(|| AppError::unknown("GitHub repo settings were not saved"))?;
    let head = push_all(&client, &settings)?;
    update_last_sync(head, collect_local_files()?)?;
    get_github_sync_status()
}

fn not_connected_result() -> SyncResult {
    SyncResult {
        status: SyncState::NotConnected,
        conflicts: Vec::new(),
    }
}

pub(super) fn sync_now_blocking() -> AppResult<SyncResult> {
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

pub(super) fn sync_auto_blocking() -> AppResult<GithubAutoSyncResult> {
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

    let result = sync_now_blocking()?;
    Ok(GithubAutoSyncResult {
        status: get_github_sync_status()?,
        result: Some(result),
    })
}

pub(super) fn resolve_sync_conflict_blocking(
    path: String,
    resolution: SyncConflictResolution,
) -> AppResult<()> {
    let mut settings = read_local_settings()?;
    let github = settings
        .github
        .as_mut()
        .ok_or_else(|| AppError::new(GithubErrorCode::NotConnected, "GitHub is not connected"))?;
    let conflict = github
        .pending_conflicts
        .get(&path)
        .cloned()
        .ok_or_else(|| {
            AppError::new(
                GithubErrorCode::RemotePathInvalid,
                format!("GitHub conflict was not found: {path}"),
            )
        })?;

    let baseline_content = conflict.remote_content.clone();
    let content = match resolution {
        SyncConflictResolution::Local => conflict.local_content,
        SyncConflictResolution::Remote => conflict.remote_content,
    };

    match content {
        Some(content) => write_local_file(&path, &content)?,
        None => remove_local_file(&path)?,
    }

    match baseline_content {
        Some(content) => {
            github.baseline_files.insert(path.clone(), content);
        }
        None => {
            github.baseline_files.remove(&path);
        }
    }
    github.pending_conflicts.remove(&path);
    write_local_settings(&settings)?;

    rebuild_skill_index_from_disk()?;
    run_sync()
}
