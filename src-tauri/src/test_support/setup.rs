use std::fs;
use std::path::Path;

use crate::core::config::{default_config, ensure_root, Config, TargetConfig};
use crate::core::constants::{CONFIG_FILE, INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::path_safety::{app_root, expand_home};

use super::defaults::{default_instructions, default_skills};

pub(crate) fn target_config(root: &Path) -> TargetConfig {
    TargetConfig {
        id: "test".into(),
        name: "Test".into(),
        skills_path: root.join("skills").to_string_lossy().to_string(),
        instructions_path: root.join("AGENTS.md").to_string_lossy().to_string(),
        enabled: true,
    }
}

pub(crate) fn initialize_setup(
    setup_path: String,
    install_default_skills: bool,
    install_default_instructions: bool,
) -> Result<Config, String> {
    let root = if setup_path.trim().is_empty() {
        app_root()
    } else {
        expand_home(&setup_path)
    };

    ensure_root(&root).map_err(|error| error.message)?;
    let mut config = default_config(&root);

    if install_default_instructions {
        fs::write(root.join(INSTRUCTIONS_FILE), default_instructions())
            .map_err(|error| error.to_string())?;
    } else if !root.join(INSTRUCTIONS_FILE).exists() {
        fs::write(root.join(INSTRUCTIONS_FILE), "").map_err(|error| error.to_string())?;
    }

    if install_default_skills {
        for skill in default_skills()? {
            fs::create_dir_all(root.join(SKILLS_DIR).join(&skill.metadata.id))
                .map_err(|error| error.to_string())?;
            fs::write(
                root.join(SKILLS_DIR)
                    .join(&skill.metadata.id)
                    .join("SKILL.md"),
                skill.content,
            )
            .map_err(|error| error.to_string())?;
            fs::write(
                root.join(SKILLS_DIR)
                    .join(&skill.metadata.id)
                    .join("metadata.json"),
                format!(
                    "{}\n",
                    serde_json::to_string_pretty(&skill.metadata)
                        .map_err(|error| error.to_string())?
                ),
            )
            .map_err(|error| error.to_string())?;
            config.skills.push(skill.metadata);
        }
    }

    let content = serde_json::to_string_pretty(&config).map_err(|error| error.to_string())?;
    fs::write(root.join(CONFIG_FILE), format!("{content}\n")).map_err(|error| error.to_string())?;
    Ok(config)
}
