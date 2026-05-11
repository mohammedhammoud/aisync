use std::path::{Path, PathBuf};

use crate::core::config::TargetConfig;
use crate::core::constants::{APP_ROOT_DIR, HOME_ENV};

pub trait PlatformAdapter {
    fn home_dir(&self) -> PathBuf;

    fn app_root(&self) -> PathBuf {
        if let Ok(path) = std::env::var(HOME_ENV) {
            return self.expand_home(&path);
        }

        self.home_dir().join(APP_ROOT_DIR)
    }

    fn expand_home(&self, input: &str) -> PathBuf {
        if input == "~" {
            return self.home_dir();
        }

        if let Some(rest) = input.strip_prefix("~/") {
            return self.home_dir().join(rest);
        }

        PathBuf::from(input)
    }

    fn default_new_target_config(&self) -> TargetConfig;
    fn default_target_configs(&self) -> Vec<TargetConfig>;
    fn symlink_path(&self, source: &Path, target: &Path) -> Result<(), String>;
    fn is_symlink(&self, path: &Path) -> Result<bool, String>;
}

#[cfg(windows)]
static WINDOWS_PLATFORM: crate::os::windows_platform::WindowsPlatform =
    crate::os::windows_platform::WindowsPlatform;

#[cfg(target_os = "macos")]
static MACOS_PLATFORM: crate::os::macos_platform::MacOsPlatform =
    crate::os::macos_platform::MacOsPlatform;

#[cfg(all(unix, not(target_os = "macos")))]
static LINUX_PLATFORM: crate::os::linux_platform::LinuxPlatform =
    crate::os::linux_platform::LinuxPlatform;

#[cfg(target_os = "macos")]
pub fn platform() -> &'static dyn PlatformAdapter {
    &MACOS_PLATFORM
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn platform() -> &'static dyn PlatformAdapter {
    &LINUX_PLATFORM
}

#[cfg(windows)]
pub fn platform() -> &'static dyn PlatformAdapter {
    &WINDOWS_PLATFORM
}
