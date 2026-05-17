use std::fs;

use crate::core::config::instructions_path;
use crate::core::errors::{AppError, AppResult};
use crate::core::fs_utils::write_atomic;
use crate::core::github::events::request_auto_sync;
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
    const MAX_INSTRUCTIONS_BYTES: usize = 1_000_000;
    if content.len() > MAX_INSTRUCTIONS_BYTES {
        return Err(AppError::unknown(format!(
            "Instructions are too large; maximum is {MAX_INSTRUCTIONS_BYTES} bytes"
        )));
    }
    let path = instructions_path();
    write_atomic(&path, content)?;
    let _ = sync::run_sync();
    let _ = request_auto_sync();
    Ok(())
}
