use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::config::{instructions_path, read_config, skills_dir, Config, TargetConfig};
use crate::core::constants::{BACKUPS_DIR, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::{app_root, expand_home};
use crate::platform::platform;

fn timestamp() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}-{:09}", duration.as_secs(), duration.subsec_nanos())
}

fn is_owned_skill_symlink(path: &Path) -> AppResult<bool> {
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
    Ok(link.starts_with(app_root().join(SKILLS_DIR)))
}

fn backup_target(target: &Path) -> AppResult<()> {
    if !target.exists() && fs::symlink_metadata(target).is_err() {
        return Ok(());
    }
    let backup = target
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(BACKUPS_DIR)
        .join(timestamp())
        .join(target.file_name().unwrap_or_default());
    copy_path(target, &backup)
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

fn remove_path(path: &Path) -> AppResult<()> {
    let metadata = fs::symlink_metadata(path).map_err(AppError::io)?;
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path).map_err(AppError::io)
    } else {
        fs::remove_file(path).map_err(AppError::io)
    }
}

fn sync_for_config(target_config: &TargetConfig, config: &Config) -> AppResult<()> {
    let skills_path = expand_home(&target_config.skills_path);
    let instructions_target = expand_home(&target_config.instructions_path);

    for skill in &config.skills {
        if !skill.enabled {
            continue;
        }
        let source = skills_dir().join(&skill.id);
        let target = skills_path.join(&skill.id);
        backup_target(&target)?;
        if target.exists() || fs::symlink_metadata(&target).is_ok() {
            remove_path(&target)?;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        symlink_path(&source, &target)?;
    }

    backup_target(&instructions_target)?;
    if instructions_target.exists() || fs::symlink_metadata(&instructions_target).is_ok() {
        remove_path(&instructions_target)?;
    }
    if let Some(parent) = instructions_target.parent() {
        fs::create_dir_all(parent).map_err(AppError::io)?;
    }
    symlink_path(&instructions_path(), &instructions_target)?;

    if skills_path.exists() {
        for entry in fs::read_dir(&skills_path).map_err(AppError::io)? {
            let entry = entry.map_err(AppError::io)?;
            let stale_id = entry.file_name().to_string_lossy().to_string();
            let target = entry.path();
            if is_owned_skill_symlink(&target)?
                && !config
                    .skills
                    .iter()
                    .any(|skill| skill.id == stale_id && skill.enabled)
            {
                fs::remove_file(target).map_err(AppError::io)?;
            }
        }
    }

    Ok(())
}

pub fn run_sync() -> AppResult<()> {
    let config = read_config()?;
    for target_config in &config.configs {
        if !target_config.enabled {
            continue;
        }
        sync_for_config(target_config, &config)?;
    }
    Ok(())
}
