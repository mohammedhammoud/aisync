use crate::core::errors::{AppError, AppResult};

use super::events::{emit_current_status, emit_sync_failed, emit_sync_finished, emit_sync_started};
use super::sync::{resolve_sync_conflict_blocking, setup_github_sync_blocking, sync_now_blocking};
use super::types::{GithubSyncStatus, SyncConflictResolution, SyncResult};

#[tauri::command]
#[specta::specta]
pub async fn setup_github_sync(app: tauri::AppHandle) -> AppResult<GithubSyncStatus> {
    emit_sync_started(&app, true);
    let result = tauri::async_runtime::spawn_blocking(setup_github_sync_blocking)
        .await
        .map_err(|error| AppError::unknown(error.to_string()))?;
    match &result {
        Ok(_status) => emit_sync_finished(&app, None, true),
        Err(error) => emit_sync_failed(&app, error.message.clone(), true),
    }
    result
}

#[tauri::command]
#[specta::specta]
pub fn resolve_sync_conflict(
    app: tauri::AppHandle,
    path: String,
    resolution: SyncConflictResolution,
) -> AppResult<()> {
    let result = resolve_sync_conflict_blocking(path, resolution);
    emit_current_status(&app);
    result
}

#[tauri::command]
#[specta::specta]
pub async fn sync_github_now(app: tauri::AppHandle) -> AppResult<SyncResult> {
    emit_sync_started(&app, true);
    let result = tauri::async_runtime::spawn_blocking(sync_now_blocking)
        .await
        .map_err(|error| AppError::unknown(error.to_string()))?;
    match &result {
        Ok(sync_result) => emit_sync_finished(&app, Some(sync_result.clone()), true),
        Err(error) => emit_sync_failed(&app, error.message.clone(), true),
    }
    result
}
