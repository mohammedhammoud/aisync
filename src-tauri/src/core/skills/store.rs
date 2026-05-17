use std::fs;

use crate::core::config::{
    read_config, skills_dir, validate_skill_metadata, write_config, SkillMetadata,
};
use crate::core::errors::{AppError, AppResult};
use crate::core::fs_utils::{write_atomic, write_json_pretty};
use crate::core::path_safety::{app_root, assert_child, validate_id};

use super::types::SkillErrorCode;

pub fn write_skill_content(
    skill_id: String,
    content: String,
    metadata: SkillMetadata,
) -> AppResult<()> {
    validate_id(&skill_id)?;
    validate_skill_metadata(&metadata)?;
    validate_skill_body(&content)?;
    if skill_id != metadata.id {
        return Err(AppError::new(
            SkillErrorCode::IdMismatch,
            "Skill ID mismatch",
        ));
    }

    let dir = skills_dir().join(&skill_id);
    fs::create_dir_all(&dir).map_err(AppError::io)?;
    assert_child(&app_root(), &dir)?;
    write_atomic(&dir.join("SKILL.md"), content)?;
    write_json_pretty(&dir.join("metadata.json"), &metadata)?;
    upsert_skill_metadata(metadata)
}

pub fn skill_dir_exists(skill_id: &str) -> bool {
    skills_dir().join(skill_id).exists()
}

pub fn validate_skill_body(body: &str) -> AppResult<()> {
    const MAX_SKILL_BODY_BYTES: usize = 1_000_000;
    if body.len() > MAX_SKILL_BODY_BYTES {
        return Err(AppError::unknown(format!(
            "Skill body is too large; maximum is {MAX_SKILL_BODY_BYTES} bytes"
        )));
    }
    Ok(())
}

fn upsert_skill_metadata(metadata: SkillMetadata) -> AppResult<()> {
    validate_id(&metadata.id)?;
    let mut config = read_config()?;
    if let Some(existing) = config
        .skills
        .iter_mut()
        .find(|skill| skill.id == metadata.id)
    {
        *existing = metadata;
    } else {
        config.skills.push(metadata);
    }
    config
        .skills
        .sort_by(|left, right| left.name.cmp(&right.name));
    write_config(&config)
}

fn remove_skill_metadata(skill_id: &str) -> AppResult<()> {
    let mut config = read_config()?;
    config.skills.retain(|skill| skill.id != skill_id);
    write_config(&config)
}

pub fn delete_skill_record(skill_id: String) -> AppResult<()> {
    validate_id(&skill_id)?;
    let dir = skills_dir().join(&skill_id);
    assert_child(&app_root(), &dir)?;
    if !dir.exists() {
        return Err(AppError::new(SkillErrorCode::NotFound, "Skill not found"));
    }
    fs::remove_dir_all(dir).map_err(AppError::io)?;
    remove_skill_metadata(&skill_id)
}
