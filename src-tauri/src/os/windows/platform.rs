use std::path::{Path, PathBuf};

use crate::core::config::TargetConfig;
use crate::platform::PlatformAdapter;

/// Not implemented. macOS is the only supported platform.
pub struct WindowsPlatform;

fn unsupported() -> ! {
    panic!("Unsupported platform: only macOS is supported")
}

impl PlatformAdapter for WindowsPlatform {
    fn home_dir(&self) -> PathBuf {
        unsupported()
    }

    fn default_new_target_config(&self) -> TargetConfig {
        unsupported()
    }

    fn default_target_configs(&self) -> Vec<TargetConfig> {
        unsupported()
    }

    fn symlink_path(&self, _source: &Path, _target: &Path) -> Result<(), String> {
        unsupported()
    }

    fn is_symlink(&self, _path: &Path) -> Result<bool, String> {
        unsupported()
    }
}
