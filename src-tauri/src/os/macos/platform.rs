use std::fs;
use std::path::{Path, PathBuf};

use crate::core::config::TargetConfig;
use crate::platform::PlatformAdapter;

pub struct MacOsPlatform;

impl PlatformAdapter for MacOsPlatform {
    fn home_dir(&self) -> PathBuf {
        std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    fn default_new_target_config(&self) -> TargetConfig {
        TargetConfig {
            id: "new-config".into(),
            name: "Custom configuration".into(),
            skills_path: "~/.codex/skills".into(),
            instructions_path: "~/.codex/AGENTS.md".into(),
            enabled: true,
        }
    }

    fn default_target_configs(&self) -> Vec<TargetConfig> {
        vec![
            TargetConfig {
                id: "codex".into(),
                name: "Codex".into(),
                skills_path: "~/.codex/skills".into(),
                instructions_path: "~/.codex/AGENTS.md".into(),
                enabled: true,
            },
            TargetConfig {
                id: "copilot".into(),
                name: "Copilot".into(),
                skills_path: "~/.copilot/skills".into(),
                instructions_path: "~/.copilot/copilot-instructions.md".into(),
                enabled: true,
            },
            TargetConfig {
                id: "pi".into(),
                name: "Pi".into(),
                skills_path: "~/.pi/agent/skills".into(),
                instructions_path: "~/.pi/agent/APPEND_SYSTEM.md".into(),
                enabled: true,
            },
        ]
    }

    fn symlink_path(&self, source: &Path, target: &Path) -> Result<(), String> {
        std::os::unix::fs::symlink(source, target).map_err(|error| error.to_string())
    }

    fn is_symlink(&self, path: &Path) -> Result<bool, String> {
        Ok(fs::symlink_metadata(path)
            .map_err(|error| error.to_string())?
            .file_type()
            .is_symlink())
    }
}
