use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs;
use tauri::{AppHandle, Emitter};

use crate::core::config::{read_config, skills_dir, write_config, SkillMetadata};
use crate::core::errors::{AppError, AppResult};
use crate::core::events::SKILLS_CHANGED;
use crate::core::path_safety::{app_root, assert_child, validate_id};
use crate::core::sync;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SkillErrorCode {
    SkillNotFound,
    SkillAlreadyExists,
    SkillIdMismatch,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillEditorRecord {
    pub metadata: SkillMetadata,
    pub body: String,
    pub frontmatter_lines: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedSkillContent {
    pub body: String,
    pub frontmatter_lines: Vec<String>,
}

pub fn parse_skill_content(content: &str) -> ParsedSkillContent {
    if !content.starts_with("---\n") {
        return ParsedSkillContent {
            body: content.to_string(),
            frontmatter_lines: Vec::new(),
        };
    }

    let Some(relative_closing_delimiter_index) = content[4..].find("\n---") else {
        return ParsedSkillContent {
            body: content.to_string(),
            frontmatter_lines: Vec::new(),
        };
    };
    let closing_delimiter_index = relative_closing_delimiter_index + 4;

    let mut body_start = closing_delimiter_index + "\n---".len();
    if content[body_start..].starts_with("\r\n") {
        body_start += 2;
    } else if content[body_start..].starts_with('\n') {
        body_start += 1;
    }

    if content[body_start..].starts_with("\r\n") {
        body_start += 2;
    } else if content[body_start..].starts_with('\n') {
        body_start += 1;
    }

    ParsedSkillContent {
        body: content[body_start..].to_string(),
        frontmatter_lines: content[4..closing_delimiter_index]
            .split('\n')
            .map(|line| line.strip_suffix('\r').unwrap_or(line).to_string())
            .collect(),
    }
}

fn yaml_string(value: &str) -> AppResult<String> {
    serde_json::to_string(value).map_err(AppError::json)
}

pub fn compose_skill_content(
    body: &str,
    metadata: &SkillMetadata,
    frontmatter_lines: &[String],
) -> AppResult<String> {
    let mut frontmatter = vec![
        "---".to_string(),
        format!("name: {}", metadata.id),
        format!("description: {}", yaml_string(&metadata.description)?),
    ];
    frontmatter.extend(
        frontmatter_lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim_start();
                !(trimmed.starts_with("name")
                    && trimmed["name".len()..].trim_start().starts_with(':')
                    || trimmed.starts_with("description")
                        && trimmed["description".len()..].trim_start().starts_with(':'))
            })
            .cloned(),
    );
    frontmatter.push("---".to_string());

    Ok(format!("{}\n\n{}", frontmatter.join("\n"), body))
}

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

pub fn write_skill_content(
    skill_id: String,
    content: String,
    metadata: SkillMetadata,
) -> AppResult<()> {
    validate_id(&skill_id)?;
    if skill_id != metadata.id {
        return Err(AppError::new(
            SkillErrorCode::SkillIdMismatch,
            "Skill ID mismatch",
        ));
    }

    let dir = skills_dir().join(&skill_id);
    fs::create_dir_all(&dir).map_err(AppError::io)?;
    assert_child(&app_root(), &dir)?;
    fs::write(dir.join("SKILL.md"), content).map_err(AppError::io)?;
    fs::write(
        dir.join("metadata.json"),
        format!(
            "{}\n",
            serde_json::to_string_pretty(&metadata).map_err(AppError::json)?
        ),
    )
    .map_err(AppError::io)?;
    upsert_skill_metadata(metadata)
}

fn skill_dir_exists(skill_id: &str) -> bool {
    skills_dir().join(skill_id).exists()
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
    if dir.exists() {
        fs::remove_dir_all(dir).map_err(AppError::io)?;
    }
    remove_skill_metadata(&skill_id)
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
    if skill_id != metadata.id {
        return Err(AppError::new(
            SkillErrorCode::SkillIdMismatch,
            "Skill ID mismatch",
        ));
    }
    if skill_dir_exists(&skill_id) {
        return Err(AppError::new(
            SkillErrorCode::SkillAlreadyExists,
            "Skill already exists",
        ));
    }
    let content = compose_skill_content(&body, &metadata, &frontmatter_lines)?;
    write_skill_content(skill_id, content, metadata)?;
    let _ = sync::run_sync();
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
    validate_id(&metadata.id)?;
    if !skill_dir_exists(&skill_id) {
        return Err(AppError::new(
            SkillErrorCode::SkillNotFound,
            "Skill not found",
        ));
    }
    if skill_id != metadata.id && skill_dir_exists(&metadata.id) {
        return Err(AppError::new(
            SkillErrorCode::SkillAlreadyExists,
            "Skill already exists",
        ));
    }

    let content = compose_skill_content(&body, &metadata, &frontmatter_lines)?;
    write_skill_content(metadata.id.clone(), content, metadata.clone())?;

    if skill_id != metadata.id {
        delete_skill_record(skill_id)?;
    }

    let _ = sync::run_sync();
    emit_skills_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn delete_skill(app: AppHandle, skill_id: String) -> AppResult<()> {
    delete_skill_record(skill_id)?;
    let _ = sync::run_sync();
    emit_skills_changed(&app)
}
