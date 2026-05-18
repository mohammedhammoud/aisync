use std::fs;
use std::path::{Path, PathBuf};

use crate::core::constants::{CONFIG_FILE, INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::app_root;

pub fn config_path() -> PathBuf {
    app_root().join(CONFIG_FILE)
}

pub fn skills_dir() -> PathBuf {
    app_root().join(SKILLS_DIR)
}

pub fn instructions_path() -> PathBuf {
    app_root().join(INSTRUCTIONS_FILE)
}

pub fn ensure_root(root: &Path) -> AppResult<()> {
    fs::create_dir_all(root.join(SKILLS_DIR)).map_err(AppError::io)?;
    Ok(())
}
