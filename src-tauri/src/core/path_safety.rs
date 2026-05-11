use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::{Path, PathBuf};

use crate::core::constants::APP_NAME;
use crate::core::errors::{AppError, AppResult};
use crate::platform::platform;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PathErrorCode {
    InvalidIdLength,
    InvalidIdFormat,
    PathResolveRootFailed,
    PathResolvePathFailed,
    PathResolveParentFailed,
    PathEscapesRoot,
}

pub fn app_root() -> PathBuf {
    platform().app_root()
}

pub fn expand_home(input: &str) -> PathBuf {
    platform().expand_home(input)
}

pub fn validate_id(id: &str) -> AppResult<()> {
    if id.is_empty() || id.len() > 96 {
        return Err(AppError::new(
            PathErrorCode::InvalidIdLength,
            "Invalid ID length",
        ));
    }

    let valid = id
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || matches!(char, '-' | '_'));

    if valid {
        Ok(())
    } else {
        Err(AppError::new(
            PathErrorCode::InvalidIdFormat,
            "Invalid ID. Use letters, numbers, hyphen, or underscore.",
        ))
    }
}

pub fn assert_child(root: &Path, path: &Path) -> AppResult<()> {
    let root = root.canonicalize().map_err(|error| {
        AppError::new(
            PathErrorCode::PathResolveRootFailed,
            format!("Cannot resolve root: {error}"),
        )
    })?;
    let path = if path.exists() {
        path.canonicalize().map_err(|error| {
            AppError::new(
                PathErrorCode::PathResolvePathFailed,
                format!("Cannot resolve path: {error}"),
            )
        })?
    } else {
        let parent = path
            .parent()
            .ok_or_else(|| {
                AppError::new(PathErrorCode::PathResolveParentFailed, "Path has no parent")
            })?
            .canonicalize()
            .map_err(|error| {
                AppError::new(
                    PathErrorCode::PathResolveParentFailed,
                    format!("Cannot resolve parent: {error}"),
                )
            })?;
        parent.join(path.file_name().ok_or_else(|| {
            AppError::new(PathErrorCode::PathResolvePathFailed, "Path has no name")
        })?)
    };

    if path.starts_with(root) {
        Ok(())
    } else {
        Err(AppError::new(
            PathErrorCode::PathEscapesRoot,
            format!("Path escapes {APP_NAME} root"),
        ))
    }
}
