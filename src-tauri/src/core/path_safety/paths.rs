use std::path::{Path, PathBuf};

use crate::core::constants::APP_NAME;
use crate::core::errors::{AppError, AppResult};
use crate::platform::platform;

use super::types::PathErrorCode;

pub fn app_root() -> PathBuf {
    platform().app_root()
}

pub fn expand_home(input: &str) -> PathBuf {
    platform().expand_home(input)
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
