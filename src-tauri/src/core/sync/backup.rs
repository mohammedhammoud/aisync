use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::constants::BACKUPS_DIR;
use crate::core::errors::{AppError, AppResult};
use crate::core::fs_utils::write_json_pretty;
use crate::core::path_safety::app_root;
use crate::platform::platform;

fn timestamp() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}-{:09}", duration.as_secs(), duration.subsec_nanos())
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BackupManifest {
    original_path: String,
    backup_path: String,
    reason: &'static str,
    timestamp: String,
    app_version: &'static str,
}

pub fn backup_target(target: &Path) -> AppResult<()> {
    if !target.exists() && fs::symlink_metadata(target).is_err() {
        return Ok(());
    }
    let backup_timestamp = timestamp();
    let backup = app_root()
        .join(BACKUPS_DIR)
        .join(&backup_timestamp)
        .join(target.file_name().unwrap_or_default());
    copy_path(target, &backup)?;
    let manifest = BackupManifest {
        original_path: target.to_string_lossy().to_string(),
        backup_path: backup.to_string_lossy().to_string(),
        reason: "target replaced by AISync symlink sync",
        timestamp: backup_timestamp,
        app_version: env!("CARGO_PKG_VERSION"),
    };
    write_json_pretty(&backup.with_file_name("manifest.json"), &manifest)
}

fn symlink_path(source: &Path, target: &Path) -> AppResult<()> {
    platform()
        .symlink_path(source, target)
        .map_err(AppError::unknown)
}

fn copy_path(source: &Path, target: &Path) -> AppResult<()> {
    let metadata = fs::symlink_metadata(source).map_err(AppError::io)?;
    if metadata.file_type().is_symlink() {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        let link = fs::read_link(source).map_err(AppError::io)?;
        symlink_path(&link, target)?;
    } else if metadata.is_dir() {
        fs::create_dir_all(target).map_err(AppError::io)?;
        for entry in fs::read_dir(source).map_err(AppError::io)? {
            let entry = entry.map_err(AppError::io)?;
            copy_path(&entry.path(), &target.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        fs::copy(source, target).map_err(AppError::io)?;
    }
    Ok(())
}
