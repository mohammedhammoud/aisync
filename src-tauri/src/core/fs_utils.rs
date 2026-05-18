use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::errors::{AppError, AppResult};

pub(crate) fn write_atomic(path: &Path, content: impl AsRef<[u8]>) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(AppError::io)?;
    }

    let tmp_path = temp_path(path);
    fs::write(&tmp_path, content).map_err(AppError::io)?;
    fs::rename(&tmp_path, path).map_err(|error| {
        let _ = fs::remove_file(&tmp_path);
        AppError::io(error)
    })
}

pub(crate) fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> AppResult<()> {
    let content = serde_json::to_string_pretty(value).map_err(AppError::json)?;
    write_atomic(path, format!("{content}\n"))
}

fn temp_path(path: &Path) -> PathBuf {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("tmp");
    path.with_extension(format!("{extension}.{}.tmp", uuid::Uuid::new_v4()))
}
