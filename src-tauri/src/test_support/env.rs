use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::constants::HOME_ENV;

static TEST_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn test_lock() -> MutexGuard<'static, ()> {
    TEST_LOCK.lock().unwrap()
}

pub(crate) fn temp_root(name: &str) -> PathBuf {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    std::env::temp_dir().join(format!(
        "aisync-{name}-{}-{}",
        std::process::id(),
        duration.as_nanos()
    ))
}

pub(crate) fn set_home(path: &Path) {
    std::env::set_var(HOME_ENV, path);
}
