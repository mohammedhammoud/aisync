use std::fs;
use std::path::Path;

use crate::core::constants::SKILLS_DIR;
use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::app_root;
use crate::platform::platform;

pub fn is_owned_skill_symlink(path: &Path) -> AppResult<bool> {
    is_owned_symlink_under(path, &app_root().join(SKILLS_DIR))
}

pub fn is_owned_instruction_symlink(path: &Path) -> AppResult<bool> {
    is_owned_symlink_under(path, &app_root())
}

fn is_owned_symlink_under(path: &Path, root: &Path) -> AppResult<bool> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return Ok(false),
    };
    if !metadata.file_type().is_symlink()
        || !platform().is_symlink(path).map_err(AppError::unknown)?
    {
        return Ok(false);
    }
    let link = fs::read_link(path).map_err(AppError::io)?;
    Ok(link.starts_with(root))
}

pub fn refuse_non_owned_existing_target(target: &Path, is_owned: bool) -> AppResult<()> {
    if (target.exists() || fs::symlink_metadata(target).is_ok()) && !is_owned {
        return Err(AppError::unknown(format!(
            "Refusing to replace non-AISync target: {}",
            target.to_string_lossy()
        )));
    }
    Ok(())
}
