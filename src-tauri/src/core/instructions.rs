use std::fs;

use crate::core::config::instructions_path;
use crate::core::errors::{AppError, AppResult};
use crate::core::sync;

#[tauri::command]
#[specta::specta]
pub fn read_instructions() -> AppResult<String> {
    let path = instructions_path();
    if !path.exists() {
        return Ok(String::new());
    }
    fs::read_to_string(path).map_err(AppError::io)
}

#[tauri::command]
#[specta::specta]
pub fn write_instructions(content: String) -> AppResult<()> {
    let path = instructions_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(AppError::io)?;
    }
    fs::write(path, content).map_err(AppError::io)?;
    let _ = sync::run_sync();
    Ok(())
}
