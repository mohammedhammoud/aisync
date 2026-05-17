use std::fs;

use tauri::{AppHandle, Emitter};

use crate::core::config::{skills_dir, validate_skill_metadata, SkillMetadata};
use crate::core::errors::{AppError, AppResult};
use crate::core::events::SKILLS_CHANGED;
use crate::core::github::events::request_auto_sync;
use crate::core::path_safety::{app_root, assert_child, validate_id};
use crate::core::sync;

use super::frontmatter::{compose_skill_content, parse_skill_content};
use super::store::{
    delete_skill_record, skill_dir_exists, validate_skill_body, write_skill_content,
};
use super::types::{SkillEditorRecord, SkillErrorCode};

#[tauri::command]
#[specta::specta]
pub fn get_skills() -> AppResult<Vec<crate::core::config::SkillMetadata>> {
    Ok(crate::core::config::read_config()?.skills)
}

#[tauri::command]
#[specta::specta]
pub fn get_skill(skill_id: String) -> AppResult<SkillEditorRecord> {
    validate_id(&skill_id)?;
    let dir = skills_dir().join(&skill_id);
    assert_child(&app_root(), &dir)?;
    let content = fs::read_to_string(dir.join("SKILL.md")).map_err(AppError::io)?;
    let metadata_content = fs::read_to_string(dir.join("metadata.json")).map_err(AppError::io)?;
    let metadata = serde_json::from_str(&metadata_content).map_err(AppError::json)?;
    let parsed = parse_skill_content(&content);

    Ok(SkillEditorRecord {
        metadata,
        body: parsed.body,
        frontmatter_lines: parsed.frontmatter_lines,
    })
}

fn emit_skills_changed(app: &AppHandle) -> AppResult<()> {
    app.emit(SKILLS_CHANGED, ()).map_err(AppError::emit)
}

#[tauri::command]
#[specta::specta]
pub fn create_skill(
    app: AppHandle,
    skill_id: String,
    body: String,
    metadata: SkillMetadata,
    frontmatter_lines: Vec<String>,
) -> AppResult<()> {
    validate_id(&skill_id)?;
    validate_skill_metadata(&metadata)?;
    validate_skill_body(&body)?;
    if skill_id != metadata.id {
        return Err(AppError::new(
            SkillErrorCode::IdMismatch,
            "Skill ID mismatch",
        ));
    }
    if skill_dir_exists(&skill_id) {
        return Err(AppError::new(
            SkillErrorCode::AlreadyExists,
            "Skill already exists",
        ));
    }
    let content = compose_skill_content(&body, &metadata, &frontmatter_lines)?;
    write_skill_content(skill_id, content, metadata)?;
    let _ = sync::run_sync();
    let _ = request_auto_sync();
    emit_skills_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn update_skill(
    app: AppHandle,
    skill_id: String,
    body: String,
    metadata: SkillMetadata,
    frontmatter_lines: Vec<String>,
) -> AppResult<()> {
    validate_id(&skill_id)?;
    validate_skill_metadata(&metadata)?;
    validate_skill_body(&body)?;
    if !skill_dir_exists(&skill_id) {
        return Err(AppError::new(SkillErrorCode::NotFound, "Skill not found"));
    }
    if skill_id != metadata.id && skill_dir_exists(&metadata.id) {
        return Err(AppError::new(
            SkillErrorCode::AlreadyExists,
            "Skill already exists",
        ));
    }

    let content = compose_skill_content(&body, &metadata, &frontmatter_lines)?;
    write_skill_content(metadata.id.clone(), content, metadata.clone())?;

    if skill_id != metadata.id {
        delete_skill_record(skill_id)?;
    }

    let _ = sync::run_sync();
    let _ = request_auto_sync();
    emit_skills_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn delete_skill(app: AppHandle, skill_id: String) -> AppResult<()> {
    delete_skill_record(skill_id)?;
    let _ = sync::run_sync();
    let _ = request_auto_sync();
    emit_skills_changed(&app)
}
