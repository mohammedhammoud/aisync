use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::config::{instructions_path, read_config, skills_dir, write_config};
use crate::core::constants::{INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::validate_id;

use super::constants::{SKILL_FILE, SKILL_METADATA_FILE};
use super::paths::{invalid_remote_path, validate_sync_path};

pub fn collect_local_files() -> AppResult<BTreeMap<String, String>> {
    let mut files = BTreeMap::new();
    let instructions = instructions_path();
    files.insert(
        INSTRUCTIONS_FILE.to_string(),
        if instructions.exists() {
            fs::read_to_string(instructions).map_err(AppError::io)?
        } else {
            String::new()
        },
    );

    let skills = skills_dir();
    if skills.exists() {
        for entry in fs::read_dir(skills).map_err(AppError::io)? {
            let entry = entry.map_err(AppError::io)?;
            if !entry.file_type().map_err(AppError::io)?.is_dir() {
                continue;
            }
            let skill_id = entry.file_name().to_string_lossy().to_string();
            validate_id(&skill_id)?;
            for file_name in [SKILL_FILE, SKILL_METADATA_FILE] {
                let path = entry.path().join(file_name);
                if path.exists() {
                    files.insert(
                        format!("{SKILLS_DIR}/{skill_id}/{file_name}"),
                        fs::read_to_string(path).map_err(AppError::io)?,
                    );
                }
            }
        }
    }

    Ok(files)
}

pub fn local_file_path(path: &str) -> AppResult<PathBuf> {
    validate_sync_path(path)?;
    if path == INSTRUCTIONS_FILE {
        return Ok(instructions_path());
    }

    let rest = path
        .strip_prefix(&format!("{SKILLS_DIR}/"))
        .ok_or_else(|| invalid_remote_path(path))?;
    Ok(skills_dir().join(rest))
}

pub fn write_local_file(path: &str, content: &str) -> AppResult<()> {
    let target = local_file_path(path)?;
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(AppError::io)?;
    }
    fs::write(target, content).map_err(AppError::io)
}

pub fn remove_local_file(path: &str) -> AppResult<()> {
    let target = local_file_path(path)?;
    if target.exists() {
        fs::remove_file(target).map_err(AppError::io)?;
    }
    Ok(())
}

pub fn write_conflict_file(
    root: &Path,
    path: &str,
    name: &str,
    content: Option<&String>,
) -> AppResult<()> {
    let Some(content) = content else {
        return Ok(());
    };
    validate_sync_path(path)?;
    if !matches!(name, "local" | "remote") {
        return Err(invalid_remote_path(name));
    }
    let target = root.join(format!("{path}.{name}"));
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(AppError::io)?;
    }
    fs::write(target, content).map_err(AppError::io)
}

pub fn rebuild_skill_index_from_disk() -> AppResult<()> {
    let mut config = read_config()?;
    let mut skills = Vec::new();
    let dir = skills_dir();

    if dir.exists() {
        for entry in fs::read_dir(dir).map_err(AppError::io)? {
            let entry = entry.map_err(AppError::io)?;
            let metadata_path = entry.path().join(SKILL_METADATA_FILE);
            if metadata_path.exists() {
                let content = fs::read_to_string(metadata_path).map_err(AppError::io)?;
                skills.push(serde_json::from_str(&content).map_err(AppError::json)?);
            }
        }
    }

    skills.sort_by(|left: &crate::core::config::SkillMetadata, right| left.name.cmp(&right.name));
    config.skills = skills;
    write_config(&config)
}
