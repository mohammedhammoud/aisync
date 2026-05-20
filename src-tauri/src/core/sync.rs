mod backup;
pub mod commands;
mod local;
mod ownership;
mod types;

pub use local::run_sync;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::core::config::write_config;
    use crate::core::constants::{BACKUPS_DIR, CONFIG_FILE};
    use crate::core::skills::delete_skill_record;
    use crate::core::sync::run_sync;
    use crate::core::sync::types::ForceLinkTarget;
    use crate::platform::platform;
    use crate::test_support::{initialize_setup, set_home, target_config, temp_root, test_lock};

    #[test]
    fn symlink_sync_refuses_non_owned_targets() {
        let _guard = test_lock();
        let root = temp_root("sync-refuse");
        let target = temp_root("sync-refuse-target");
        set_home(&root);
        let mut config = initialize_setup(root.to_string_lossy().to_string(), true, true).unwrap();
        config.configs = vec![target_config(&target)];
        write_config(&config).unwrap();

        fs::create_dir_all(target.join("skills").join("audit")).unwrap();
        fs::write(
            target.join("skills").join("audit").join("SKILL.md"),
            "local",
        )
        .unwrap();

        let error = run_sync().unwrap_err();

        assert!(error
            .message
            .contains("Refusing to replace non-AISync target"));
        assert!(!root.join(BACKUPS_DIR).exists());
        assert_eq!(
            fs::read_to_string(target.join("skills").join("audit").join("SKILL.md")).unwrap(),
            "local"
        );
    }

    #[test]
    fn legacy_backup_false_and_copy_mode_config_does_not_bypass_safety() {
        let _guard = test_lock();
        let root = temp_root("sync-legacy-backup");
        let target = temp_root("sync-legacy-backup-target");
        set_home(&root);
        let config = initialize_setup(root.to_string_lossy().to_string(), true, true).unwrap();
        let mut value = serde_json::to_value(config).unwrap();
        value["configs"] = serde_json::json!([{
            "id": "test",
            "name": "Test",
            "skillsPath": target.join("skills").to_string_lossy(),
            "instructionsPath": target.join("AGENTS.md").to_string_lossy(),
            "mode": "copy",
            "backup": false,
            "enabled": true
        }]);
        fs::create_dir_all(&root).unwrap();
        fs::write(
            root.join(CONFIG_FILE),
            format!("{}\n", serde_json::to_string_pretty(&value).unwrap()),
        )
        .unwrap();

        fs::create_dir_all(target.join("skills").join("audit")).unwrap();
        fs::write(
            target.join("skills").join("audit").join("SKILL.md"),
            "local",
        )
        .unwrap();

        let error = run_sync().unwrap_err();

        assert!(error
            .message
            .contains("Refusing to replace non-AISync target"));
    }

    #[test]
    fn force_link_target_uses_requested_kind_when_paths_overlap() {
        let _guard = test_lock();
        let root = temp_root("sync-force-kind");
        let target = temp_root("sync-force-kind-target");
        set_home(&root);
        let mut config = initialize_setup(root.to_string_lossy().to_string(), true, true).unwrap();
        let mut target_config = target_config(&target);
        target_config.instructions_path = target
            .join("skills")
            .join("audit")
            .to_string_lossy()
            .to_string();
        config.configs = vec![target_config.clone()];
        write_config(&config).unwrap();

        super::local::force_link_target(ForceLinkTarget::Instructions {
            config_name: target_config.name,
            target_path: target_config.instructions_path,
        })
        .unwrap();

        assert_eq!(
            fs::read_link(target.join("skills").join("audit")).unwrap(),
            root.join("instructions.md")
        );
    }

    #[test]
    fn symlink_sync_and_owned_stale_removal_are_safe() {
        let _guard = test_lock();
        let root = temp_root("sync-link");
        let target = temp_root("sync-link-target");
        set_home(&root);
        let mut config = initialize_setup(root.to_string_lossy().to_string(), true, true).unwrap();
        config.configs = vec![target_config(&target)];
        write_config(&config).unwrap();

        run_sync().unwrap();
        let audit_link = target.join("skills").join("audit");
        assert!(fs::symlink_metadata(&audit_link)
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(fs::symlink_metadata(target.join("AGENTS.md"))
            .unwrap()
            .file_type()
            .is_symlink());

        delete_skill_record("audit".into()).unwrap();
        let external = temp_root("external");
        fs::create_dir_all(&external).unwrap();
        fs::write(external.join("SKILL.md"), "external").unwrap();
        platform()
            .symlink_path(&external, &target.join("skills").join("external"))
            .unwrap();

        run_sync().unwrap();

        assert!(!audit_link.exists());
        assert!(target.join("skills").join("external").exists());
    }
}
