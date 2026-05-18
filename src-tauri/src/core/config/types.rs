use serde::{Deserialize, Serialize};
use specta::Type;

use super::defaults::default_target_configs;
use crate::core::update::AvailableUpdate;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConfigErrorCode {
    ConfigNotFound,
    ConfigAlreadyExists,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TargetConfig {
    pub id: String,
    pub name: String,
    pub skills_path: String,
    pub instructions_path: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub setup_path: String,
    #[serde(default = "default_target_configs")]
    pub configs: Vec<TargetConfig>,
    #[serde(default)]
    pub skills: Vec<SkillMetadata>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Defaults {
    pub setup_path: String,
    pub new_target_config: TargetConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Globals {
    pub app_name: String,
    pub setup_path: String,
    pub available_update: Option<AvailableUpdate>,
}
