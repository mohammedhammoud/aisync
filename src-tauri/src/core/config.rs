use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Emitter};

use crate::core::constants::{APP_NAME, CONFIG_FILE, INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::events::CONFIGS_CHANGED;
use crate::core::path_safety::{app_root, validate_id};
use crate::core::sync;
use crate::platform::platform;

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
}

pub fn config_path() -> std::path::PathBuf {
    app_root().join(CONFIG_FILE)
}

pub fn skills_dir() -> std::path::PathBuf {
    app_root().join(SKILLS_DIR)
}

pub fn instructions_path() -> std::path::PathBuf {
    app_root().join(INSTRUCTIONS_FILE)
}

pub fn ensure_root(root: &Path) -> AppResult<()> {
    fs::create_dir_all(root.join(SKILLS_DIR)).map_err(AppError::io)?;
    Ok(())
}

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

pub fn read_config() -> AppResult<Config> {
    let path = config_path();
    if !path.exists() {
        return Ok(default_config(&app_root()));
    }

    let content = fs::read_to_string(path).map_err(AppError::io)?;
    serde_json::from_str(&content).map_err(AppError::json)
}

pub fn write_config(config: &Config) -> AppResult<()> {
    let root = app_root();
    ensure_root(&root)?;
    let content = serde_json::to_string_pretty(config).map_err(AppError::json)?;
    fs::write(config_path(), format!("{content}\n")).map_err(AppError::io)
}

fn emit_configs_changed(app: &AppHandle) -> AppResult<()> {
    app.emit(CONFIGS_CHANGED, ()).map_err(AppError::emit)
}

#[tauri::command]
#[specta::specta]
pub fn get_config(config_id: String) -> AppResult<TargetConfig> {
    validate_id(&config_id)?;
    read_config()?
        .configs
        .into_iter()
        .find(|target| target.id == config_id)
        .ok_or_else(|| AppError::new(ConfigErrorCode::ConfigNotFound, "Config not found"))
}

#[tauri::command]
#[specta::specta]
pub fn create_config(app: AppHandle, target: TargetConfig) -> AppResult<()> {
    validate_id(&target.id)?;
    let mut config = read_config()?;
    if config.configs.iter().any(|item| item.id == target.id) {
        return Err(AppError::new(
            ConfigErrorCode::ConfigAlreadyExists,
            "Config already exists",
        ));
    }
    config.configs.push(target);
    config
        .configs
        .sort_by(|left, right| left.name.cmp(&right.name));
    write_config(&config)?;
    let _ = sync::run_sync();
    emit_configs_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn update_config(app: AppHandle, config_id: String, target: TargetConfig) -> AppResult<()> {
    validate_id(&config_id)?;
    validate_id(&target.id)?;
    let mut config = read_config()?;

    if config_id != target.id && config.configs.iter().any(|item| item.id == target.id) {
        return Err(AppError::new(
            ConfigErrorCode::ConfigAlreadyExists,
            "Config already exists",
        ));
    }

    let Some(existing) = config.configs.iter_mut().find(|item| item.id == config_id) else {
        return Err(AppError::new(
            ConfigErrorCode::ConfigNotFound,
            "Config not found",
        ));
    };
    *existing = target;
    config
        .configs
        .sort_by(|left, right| left.name.cmp(&right.name));
    write_config(&config)?;
    let _ = sync::run_sync();
    emit_configs_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn delete_config(app: AppHandle, config_id: String) -> AppResult<()> {
    validate_id(&config_id)?;
    let mut config = read_config()?;
    config.configs.retain(|target| target.id != config_id);
    write_config(&config)?;
    let _ = sync::run_sync();
    emit_configs_changed(&app)
}

#[tauri::command]
#[specta::specta]
pub fn get_configs() -> AppResult<Vec<TargetConfig>> {
    Ok(read_config()?.configs)
}

#[tauri::command]
#[specta::specta]
pub fn get_globals() -> AppResult<Globals> {
    Ok(Globals {
        app_name: APP_NAME.into(),
        setup_path: read_config()?.setup_path,
    })
}

#[tauri::command]
#[specta::specta]
pub fn get_defaults() -> AppResult<Defaults> {
    Ok(Defaults {
        setup_path: app_root().to_string_lossy().to_string(),
        new_target_config: default_new_target_config(),
    })
}

#[cfg(test)]
mod tests {
    use crate::core::config::{default_new_target_config, get_configs, get_defaults};
    use crate::core::path_safety::validate_id;
    use crate::test_support::{
        default_instructions, default_skills, initialize_setup, set_home, temp_root, test_lock,
    };

    #[test]
    fn setup_creates_explicit_config_instructions_and_default_skills() {
        let _guard = test_lock();
        let root = temp_root("setup");
        set_home(&root);

        let config = initialize_setup(root.to_string_lossy().to_string(), true, true).unwrap();

        assert_eq!(
            config
                .configs
                .iter()
                .map(|target| target.id.as_str())
                .collect::<Vec<_>>(),
            vec!["codex", "copilot", "pi"]
        );
        assert!(root.join("instructions.md").exists());
        assert_eq!(
            std::fs::read_to_string(root.join("instructions.md")).unwrap(),
            default_instructions()
        );
        assert_eq!(
            config
                .skills
                .iter()
                .map(|skill| skill.id.as_str())
                .collect::<Vec<_>>(),
            vec!["audit", "commit", "debug", "refactor"]
        );
        let default_skills = default_skills().unwrap();
        assert_eq!(
            config.skills,
            default_skills
                .iter()
                .map(|skill| skill.metadata.clone())
                .collect::<Vec<_>>()
        );
        for default_skill in default_skills {
            let skill_dir = root.join("skills").join(&default_skill.metadata.id);
            assert_eq!(
                std::fs::read_to_string(skill_dir.join("SKILL.md")).unwrap(),
                default_skill.content
            );
            let metadata_content =
                std::fs::read_to_string(skill_dir.join("metadata.json")).unwrap();
            assert_eq!(
                metadata_content,
                format!(
                    "{}\n",
                    serde_json::to_string_pretty(&default_skill.metadata).unwrap()
                )
            );
            let metadata: crate::core::config::SkillMetadata =
                serde_json::from_str(&metadata_content).unwrap();
            assert_eq!(metadata, default_skill.metadata);
        }
        assert!(root.join("config.json").exists());
    }

    #[test]
    fn embedded_default_skill_metadata_parses() {
        let defaults = default_skills().unwrap();

        assert_eq!(
            defaults
                .iter()
                .map(|skill| skill.metadata.id.as_str())
                .collect::<Vec<_>>(),
            vec!["audit", "commit", "debug", "refactor"]
        );

        for default_skill in defaults {
            let skill_id = &default_skill.metadata.id;
            assert!(
                default_skill.content.starts_with("---\n"),
                "{skill_id} SKILL.md should start with YAML frontmatter"
            );
            assert!(
                default_skill.content[4..].contains("\n---"),
                "{skill_id} SKILL.md should close YAML frontmatter"
            );
            assert!(
                default_skill.content.contains(&format!("name: {skill_id}")),
                "{skill_id} SKILL.md should include matching name frontmatter"
            );
            assert!(
                default_skill.content.contains("description:"),
                "{skill_id} SKILL.md should include description frontmatter"
            );
        }
    }

    #[test]
    fn setup_without_explicit_path_uses_app_root() {
        let _guard = test_lock();
        let root = temp_root("default-root");
        set_home(&root);

        let config = initialize_setup(String::new(), false, false).unwrap();

        assert_eq!(config.setup_path, root.to_string_lossy());
        assert!(root.join("config.json").exists());
    }

    #[test]
    fn app_defaults_return_resolved_root_and_rust_owned_new_config() {
        let _guard = test_lock();
        let root = temp_root("defaults");
        set_home(&root);

        let defaults = get_defaults().unwrap();

        assert_eq!(defaults.setup_path, root.to_string_lossy());
        assert_eq!(defaults.new_target_config, default_new_target_config());
    }

    #[test]
    fn loaded_config_round_trips() {
        let _guard = test_lock();
        let root = temp_root("load");
        set_home(&root);
        initialize_setup(root.to_string_lossy().to_string(), false, false).unwrap();

        let configs = get_configs().unwrap();

        assert_eq!(configs.len(), 3);
    }

    #[test]
    fn invalid_config_ids_are_rejected() {
        let _guard = test_lock();

        assert!(validate_id("../bad").is_err());
    }
}
