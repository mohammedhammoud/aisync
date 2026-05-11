#[cfg(all(unix, not(target_os = "macos")))]
#[path = "linux/platform.rs"]
pub mod linux_platform;

#[cfg(target_os = "macos")]
#[path = "macos/platform.rs"]
pub mod macos_platform;

#[cfg(windows)]
#[path = "windows/platform.rs"]
pub mod windows_platform;
