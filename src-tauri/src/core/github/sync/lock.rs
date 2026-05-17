use std::sync::Mutex;

use crate::core::errors::{AppError, AppResult};

static SYNC_LOCK: Mutex<()> = Mutex::new(());

pub fn with_sync_lock<T>(operation: impl FnOnce() -> AppResult<T>) -> AppResult<T> {
    let _guard = SYNC_LOCK
        .lock()
        .map_err(|error| AppError::unknown(error.to_string()))?;
    operation()
}
