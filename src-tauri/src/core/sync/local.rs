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
use super::types::{ForceLinkTarget, LinkState, LinkStatus};

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

fn target_exists(path: &Path) -> bool {
    path.exists() || fs::symlink_metadata(path).is_ok()
}

fn sync_link(source: &Path, target: &Path, allow_replace_unowned: bool) -> AppResult<()> {
    let is_owned = if source == instructions_path() {
        is_owned_instruction_symlink(target)?
    } else {
        is_owned_skill_symlink(target)?
    };
    if !allow_replace_unowned {
        refuse_non_owned_existing_target(target, is_owned)?;
    }
    backup_target(target)?;
    if target_exists(target) {
        remove_path(target)?;
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(AppError::io)?;
    }
    symlink_path(source, target)
}

fn link_state(
    source: &Path,
    target: &Path,
    is_owned: impl Fn(&Path) -> AppResult<bool>,
) -> AppResult<Option<LinkState>> {
    if !target_exists(target) {
        return Ok(Some(LinkState::Missing));
    }
    if !is_owned(target)? {
        return Ok(Some(LinkState::Blocked));
    }
    let link = fs::read_link(target).map_err(AppError::io)?;
    if link == source {
        Ok(None)
    } else {
        Ok(Some(LinkState::Missing))
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
        sync_link(&source, &target, false)?;
    }

    sync_link(&instructions_path(), &instructions_target, false)?;

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

pub fn get_link_status() -> AppResult<Vec<LinkStatus>> {
    let config = read_config()?;
    let mut statuses = Vec::new();

    for target_config in &config.configs {
        if !target_config.enabled {
            continue;
        }

        let skills_path = expand_home(&target_config.skills_path);
        for skill in &config.skills {
            if !skill.enabled {
                continue;
            }
            let source = skills_dir().join(&skill.id);
            let target = skills_path.join(&skill.id);
            if let Some(state) = link_state(&source, &target, is_owned_skill_symlink)? {
                statuses.push(LinkStatus::Skill {
                    config_name: target_config.name.clone(),
                    skill_id: skill.id.clone(),
                    state,
                    target_path: target.to_string_lossy().to_string(),
                });
            }
        }

        let source = instructions_path();
        let target = expand_home(&target_config.instructions_path);
        if let Some(state) = link_state(&source, &target, is_owned_instruction_symlink)? {
            statuses.push(LinkStatus::Instructions {
                config_name: target_config.name.clone(),
                state,
                target_path: target.to_string_lossy().to_string(),
            });
        }
    }

    Ok(statuses)
}

pub fn force_link_target(target: ForceLinkTarget) -> AppResult<()> {
    let config = read_config()?;

    match target {
        ForceLinkTarget::Skill {
            config_name,
            skill_id,
            target_path,
        } => {
            let requested_target = expand_home(&target_path);
            for target_config in &config.configs {
                if !target_config.enabled || target_config.name != config_name {
                    continue;
                }

                let skills_path = expand_home(&target_config.skills_path);
                for skill in &config.skills {
                    if !skill.enabled || skill.id != skill_id {
                        continue;
                    }
                    let source = skills_dir().join(&skill.id);
                    let link_target = skills_path.join(&skill.id);
                    if link_target == requested_target {
                        sync_link(&source, &link_target, true)?;
                        return Ok(());
                    }
                }
            }
        }
        ForceLinkTarget::Instructions {
            config_name,
            target_path,
        } => {
            let requested_target = expand_home(&target_path);
            for target_config in &config.configs {
                if !target_config.enabled || target_config.name != config_name {
                    continue;
                }

                let source = instructions_path();
                let link_target = expand_home(&target_config.instructions_path);
                if link_target == requested_target {
                    sync_link(&source, &link_target, true)?;
                    return Ok(());
                }
            }
        }
    }

    Err(AppError::unknown("Link target not found"))
}
