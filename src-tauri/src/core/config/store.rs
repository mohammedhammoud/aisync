use std::fs;

use crate::core::errors::{AppError, AppResult};
use crate::core::fs_utils::write_json_pretty;
use crate::core::path_safety::app_root;

use super::defaults::default_config;
use super::paths::{config_path, ensure_root};
use super::types::Config;

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
    write_json_pretty(&config_path(), config)
}
