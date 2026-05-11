use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Emitter};

use crate::core::constants::{APP_NAME, CONFIG_FILE, INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::events::CONFIGS_CHANGED;
use crate::core::path_safety::{app_root, validate_id};
use crate::core::sync;
use crate::platform::platform;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfigErrorCode {
    ConfigNotFound,
    ConfigAlreadyExists,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TargetConfig {
    pub id: String,
    pub name: String,
    pub skills_path: String,
    pub instructions_path: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub setup_path: String,
    #[serde(default = "default_target_configs")]
    pub configs: Vec<TargetConfig>,
    pub skills: Vec<SkillMetadata>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Defaults {
    pub setup_path: String,
    pub new_target_config: TargetConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Globals {
    pub app_name: String,
    pub setup_path: String,
}

pub fn config_path() -> std::path::PathBuf {
    app_root().join(CONFIG_FILE)
}

pub fn skills_dir() -> std::path::PathBuf {
    app_root().join(SKILLS_DIR)
}

pub fn instructions_path() -> std::path::PathBuf {
    app_root().join(INSTRUCTIONS_FILE)
}

pub fn ensure_root(root: &Path) -> AppResult<()> {
    fs::create_dir_all(root.join(SKILLS_DIR)).map_err(AppError::io)?;
    Ok(())
}

pub fn default_new_target_config() -> TargetConfig {
    platform().default_new_target_config()
}

pub fn default_target_configs() -> Vec<TargetConfig> {
    platform().default_target_configs()
}

pub fn default_config(root: &Path) -> Config {
    Config {
        setup_path: root.to_string_lossy().to_string(),
        configs: default_target_configs(),
        skills: Vec::new(),
    }
}

pub fn read_config() -> AppResult<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(default_config(&app_root()));
    }

    let content = fs::read_to_string(path).map_err(AppError::io)?;
    serde_json::from_str(&content).map_err(AppError::json)
}

pub fn write_config(config: &Config) -> AppResult<()> {
    let root = app_root();
    ensure_root(&root)?;
    let content = serde_json::to_string_pretty(config).map_err(AppError::json)?;
    fs::write(config_path(), format!("{content}\n")).map_err(AppError::io)
}

fn emit_configs_changed(app: &AppHandle) -> AppResult<()> {
    app.emit(CONFIGS_CHANGED, ()).map_err(AppError::emit)
}

#[tauri::command]
#[specta::specta]
pub fn get_config(config_id: String) -> AppResult<TargetConfig> {
    validate_id(&config_id)?;
    read_config()?
        .configs
        .into_iter()
        .find(|target| target.id == config_id)
        .ok_or_else(|| AppError::new(ConfigErrorCode::ConfigNotFound, "Config not found"))
}

#[tauri::command]
#[specta::specta]
pub fn create_config(app: AppHandle, target: TargetConfig) -> AppResult<()> {
    validate_id(&target.id)?;
    let mut config = read_config()?;
    if config.configs.iter().any(|item| item.id == target.id) {
        return Err(AppError::new(
            ConfigErrorCode::ConfigAlreadyExists,
            "Config already exists",
        ));
    }
    config.configs.push(target);
    config
        .configs
        .sort_by(|left, right| left.name.cmp(&right.name));
    write_config(&config)?;
    let _ = sync::run_sync();
    emit_configs_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn update_config(app: AppHandle, config_id: String, target: TargetConfig) -> AppResult<()> {
    validate_id(&config_id)?;
    validate_id(&target.id)?;
    let mut config = read_config()?;

    if config_id != target.id && config.configs.iter().any(|item| item.id == target.id) {
        return Err(AppError::new(
            ConfigErrorCode::ConfigAlreadyExists,
            "Config already exists",
        ));
    }

    let Some(existing) = config.configs.iter_mut().find(|item| item.id == config_id) else {
        return Err(AppError::new(
            ConfigErrorCode::ConfigNotFound,
            "Config not found",
        ));
    };
    *existing = target;
    config
        .configs
        .sort_by(|left, right| left.name.cmp(&right.name));
    write_config(&config)?;
    let _ = sync::run_sync();
    emit_configs_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn delete_config(app: AppHandle, config_id: String) -> AppResult<()> {
    validate_id(&config_id)?;
    let mut config = read_config()?;
    config.configs.retain(|target| target.id != config_id);
    write_config(&config)?;
    let _ = sync::run_sync();
    emit_configs_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn get_configs() -> AppResult<Vec<TargetConfig>> {
    Ok(read_config()?.configs)
}

#[tauri::command]
#[specta::specta]
pub fn get_globals() -> AppResult<Globals> {
    Ok(Globals {
        app_name: APP_NAME.into(),
        setup_path: read_config()?.setup_path,
    })
}

#[tauri::command]
#[specta::specta]
pub fn get_defaults() -> AppResult<Defaults> {
    Ok(Defaults {
        setup_path: app_root().to_string_lossy().to_string(),
        new_target_config: default_new_target_config(),
    })
}
