use tauri::AppHandle;

use tauri::Emitter;

use crate::core::errors::AppResult;
use crate::core::events::{CONFIGS_CHANGED, SKILLS_CHANGED};

use super::types::{ForceLinkTarget, LinkStatus};

#[tauri::command]
#[specta::specta]
pub fn get_link_status() -> AppResult<Vec<LinkStatus>> {
    super::local::get_link_status()
}

#[tauri::command]
#[specta::specta]
pub fn force_link_target(app: AppHandle, target: ForceLinkTarget) -> AppResult<()> {
    super::local::force_link_target(target)?;
    let _ = app.emit(CONFIGS_CHANGED, ());
    let _ = app.emit(SKILLS_CHANGED, ());
    Ok(())
}
