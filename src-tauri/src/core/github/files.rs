use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::config::{instructions_path, read_config, skills_dir, write_config};
use crate::core::constants::{INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::fs_utils::write_atomic;
use crate::core::path_safety::validate_id;

use super::constants::{SKILL_FILE, SKILL_METADATA_FILE};
use super::paths::{invalid_remote_path, validate_sync_path, SyncPath};

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
            let skill_file = entry.path().join(SKILL_FILE);
            let metadata_file = entry.path().join(SKILL_METADATA_FILE);
            if skill_file.exists() != metadata_file.exists() {
                return Err(AppError::new(
                    crate::core::skills::SkillErrorCode::NotFound,
                    format!("Skill {skill_id} is missing SKILL.md or metadata.json"),
                ));
            }
            for (file_name, path) in [
                (SKILL_FILE, skill_file),
                (SKILL_METADATA_FILE, metadata_file),
            ] {
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
    match SyncPath::parse(path)? {
        SyncPath::Instructions => Ok(instructions_path()),
        SyncPath::SkillFile {
            skill_id,
            file_name,
        } => Ok(skills_dir().join(skill_id).join(file_name)),
    }
}

pub fn write_local_file(path: &str, content: &str) -> AppResult<()> {
    let target = local_file_path(path)?;
    write_atomic(&target, content)
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
    write_atomic(&target, content)
}

pub fn rebuild_skill_index_from_disk() -> AppResult<()> {
    let mut config = read_config()?;
    let mut skills = Vec::new();
    let dir = skills_dir();

    if dir.exists() {
        for entry in fs::read_dir(dir).map_err(AppError::io)? {
            let entry = entry.map_err(AppError::io)?;
            let skill_id = entry.file_name().to_string_lossy().to_string();
            validate_id(&skill_id)?;
            let metadata_path = entry.path().join(SKILL_METADATA_FILE);
            if metadata_path.exists() {
                let content = fs::read_to_string(metadata_path).map_err(AppError::io)?;
                let metadata: crate::core::config::SkillMetadata =
                    serde_json::from_str(&content).map_err(AppError::json)?;
                if metadata.id != skill_id {
                    return Err(AppError::new(
                        crate::core::skills::SkillErrorCode::IdMismatch,
                        "Skill metadata ID does not match directory name",
                    ));
                }
                skills.push(metadata);
            }
        }
    }

    skills.sort_by(|left: &crate::core::config::SkillMetadata, right| left.name.cmp(&right.name));
    config.skills = skills;
    write_config(&config)
}
