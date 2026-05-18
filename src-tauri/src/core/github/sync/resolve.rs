use crate::core::errors::{AppError, AppResult};
use crate::core::sync::run_sync;

use crate::core::github::files::{
    rebuild_skill_index_from_disk, remove_local_file, write_local_file,
};
use crate::core::github::settings::{read_local_settings, write_local_settings};
use crate::core::github::types::{GithubErrorCode, SyncConflictResolution};

pub fn resolve_sync_conflict_locked(
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
