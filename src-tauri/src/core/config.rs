pub(crate) mod commands;
mod defaults;
mod paths;
mod store;
mod types;
mod validation;

#[allow(unused_imports)]
pub use commands::{
    create_config, delete_config, get_config, get_configs, get_defaults, get_globals, update_config,
};
#[allow(unused_imports)]
pub use defaults::{default_config, default_new_target_config, default_target_configs};
#[allow(unused_imports)]
pub use paths::{config_path, ensure_root, instructions_path, skills_dir};
pub use store::{read_config, write_config};
#[allow(unused_imports)]
pub use types::{Config, ConfigErrorCode, Defaults, Globals, SkillMetadata, TargetConfig};
pub(crate) use validation::validate_skill_metadata;

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
