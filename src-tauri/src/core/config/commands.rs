use tauri::{AppHandle, Emitter};

use crate::core::constants::APP_NAME;
use crate::core::errors::{AppError, AppResult};
use crate::core::events::CONFIGS_CHANGED;
use crate::core::path_safety::{app_root, validate_id};
use crate::core::sync;
use crate::core::update::check_available_update;

use super::defaults::default_new_target_config;
use super::store::{read_config, write_config};
use super::types::{ConfigErrorCode, Defaults, Globals, TargetConfig};
use super::validation::validate_target_config;

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
    validate_target_config(&target)?;
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
    validate_target_config(&target)?;
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
    let old_len = config.configs.len();
    config.configs.retain(|target| target.id != config_id);
    if config.configs.len() == old_len {
        return Err(AppError::new(
            ConfigErrorCode::ConfigNotFound,
            "Config not found",
        ));
    }
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
        available_update: check_available_update(),
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
