use std::fs;

use crate::core::config::write_config;
use crate::core::constants::{BACKUPS_DIR, CONFIG_FILE};
use crate::core::skills::delete_skill_record;
use crate::core::sync::run_sync;
use crate::platform::platform;
use crate::test_support::{initialize_setup, set_home, target_config, temp_root, test_lock};

#[test]
fn symlink_sync_backs_up_targets() {
    let _guard = test_lock();
    let root = temp_root("sync-copy");
    let target = temp_root("sync-copy-target");
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
    run_sync().unwrap();

    assert!(target
        .join("skills")
        .join("audit")
        .join("SKILL.md")
        .exists());
    assert!(target.join("skills").join(BACKUPS_DIR).exists());
    assert!(target.join("AGENTS.md").exists());
}

#[test]
fn legacy_backup_false_and_copy_mode_config_is_ignored() {
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

    run_sync().unwrap();

    assert!(target.join("skills").join(BACKUPS_DIR).exists());
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
