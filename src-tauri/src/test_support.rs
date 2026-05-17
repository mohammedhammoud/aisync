use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::config::{default_config, ensure_root, Config, SkillMetadata, TargetConfig};
use crate::core::constants::{CONFIG_FILE, HOME_ENV, INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::path_safety::{app_root, expand_home};

static TEST_LOCK: Mutex<()> = Mutex::new(());

pub(crate) struct DefaultSkill {
    pub metadata: SkillMetadata,
    pub content: &'static str,
}

struct EmbeddedDefaultSkill {
    metadata: &'static str,
    content: &'static str,
}

const DEFAULT_SKILLS: &[EmbeddedDefaultSkill] = &[
    EmbeddedDefaultSkill {
        metadata: include_str!("core/defaults/skills/audit/metadata.json"),
        content: include_str!("core/defaults/skills/audit/SKILL.md"),
    },
    EmbeddedDefaultSkill {
        metadata: include_str!("core/defaults/skills/commit/metadata.json"),
        content: include_str!("core/defaults/skills/commit/SKILL.md"),
    },
    EmbeddedDefaultSkill {
        metadata: include_str!("core/defaults/skills/debug/metadata.json"),
        content: include_str!("core/defaults/skills/debug/SKILL.md"),
    },
    EmbeddedDefaultSkill {
        metadata: include_str!("core/defaults/skills/refactor/metadata.json"),
        content: include_str!("core/defaults/skills/refactor/SKILL.md"),
    },
];

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

pub(crate) fn target_config(root: &Path) -> TargetConfig {
    TargetConfig {
        id: "test".into(),
        name: "Test".into(),
        skills_path: root.join("skills").to_string_lossy().to_string(),
        instructions_path: root.join("AGENTS.md").to_string_lossy().to_string(),
        enabled: true,
    }
}

pub(crate) fn default_instructions() -> &'static str {
    include_str!("core/defaults/instructions.md")
}

pub(crate) fn default_skills() -> Result<Vec<DefaultSkill>, String> {
    DEFAULT_SKILLS
        .iter()
        .map(|skill| {
            Ok(DefaultSkill {
                metadata: serde_json::from_str(skill.metadata)
                    .map_err(|error| error.to_string())?,
                content: skill.content,
            })
        })
        .collect()
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
