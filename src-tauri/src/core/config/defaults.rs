use std::path::Path;

use crate::platform::platform;

use super::types::{Config, TargetConfig};

pub fn default_new_target_config() -> TargetConfig {
    platform().default_new_target_config()
}

pub fn default_target_configs() -> Vec<TargetConfig> {
    platform().default_target_configs()
}

pub fn default_config(root: &Path) -> Config {
    Config {
        setup_path: root.to_string_lossy().to_string(),
        configs: default_target_configs(),
        skills: Vec::new(),
    }
}
