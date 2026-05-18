use std::fs;
use std::path::Path;

use crate::core::config::{instructions_path, read_config, skills_dir, Config, TargetConfig};
use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::expand_home;
use crate::platform::platform;

use super::backup::backup_target;
use super::ownership::{
    is_owned_instruction_symlink, is_owned_skill_symlink, refuse_non_owned_existing_target,
};

fn symlink_path(source: &Path, target: &Path) -> AppResult<()> {
    platform()
        .symlink_path(source, target)
        .map_err(AppError::unknown)
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
        let is_owned = is_owned_skill_symlink(&target)?;
        refuse_non_owned_existing_target(&target, is_owned)?;
        backup_target(&target)?;
        if target.exists() || fs::symlink_metadata(&target).is_ok() {
            remove_path(&target)?;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        symlink_path(&source, &target)?;
    }

    let is_owned = is_owned_instruction_symlink(&instructions_target)?;
    refuse_non_owned_existing_target(&instructions_target, is_owned)?;
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
