use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::config::{instructions_path, read_config, skills_dir, Config, TargetConfig};
use crate::core::constants::{BACKUPS_DIR, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::{app_root, expand_home};
use crate::platform::platform;

fn timestamp() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}-{:09}", duration.as_secs(), duration.subsec_nanos())
}

fn is_owned_skill_symlink(path: &Path) -> AppResult<bool> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return Ok(false),
    };
    if !metadata.file_type().is_symlink()
        || !platform().is_symlink(path).map_err(AppError::unknown)?
    {
        return Ok(false);
    }
    let link = fs::read_link(path).map_err(AppError::io)?;
    Ok(link.starts_with(app_root().join(SKILLS_DIR)))
}

fn backup_target(target: &Path) -> AppResult<()> {
    if !target.exists() && fs::symlink_metadata(target).is_err() {
        return Ok(());
    }
    let backup = target
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(BACKUPS_DIR)
        .join(timestamp())
        .join(target.file_name().unwrap_or_default());
    copy_path(target, &backup)
}

fn symlink_path(source: &Path, target: &Path) -> AppResult<()> {
    platform()
        .symlink_path(source, target)
        .map_err(AppError::unknown)
}

fn copy_path(source: &Path, target: &Path) -> AppResult<()> {
    let metadata = fs::symlink_metadata(source).map_err(AppError::io)?;
    if metadata.file_type().is_symlink() {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        let link = fs::read_link(source).map_err(AppError::io)?;
        symlink_path(&link, target)?;
    } else if metadata.is_dir() {
        fs::create_dir_all(target).map_err(AppError::io)?;
        for entry in fs::read_dir(source).map_err(AppError::io)? {
            let entry = entry.map_err(AppError::io)?;
            copy_path(&entry.path(), &target.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        fs::copy(source, target).map_err(AppError::io)?;
    }
    Ok(())
}

fn remove_path(path: &Path) -> AppResult<()> {
    let metadata = fs::symlink_metadata(path).map_err(AppError::io)?;
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(path).map_err(AppError::io)
    } else {
        fs::remove_file(path).map_err(AppError::io)
    }
}

fn sync_for_config(target_config: &TargetConfig, config: &Config) -> AppResult<()> {
    let skills_path = expand_home(&target_config.skills_path);
    let instructions_target = expand_home(&target_config.instructions_path);

    for skill in &config.skills {
        if !skill.enabled {
            continue;
        }
        let source = skills_dir().join(&skill.id);
        let target = skills_path.join(&skill.id);
        backup_target(&target)?;
        if target.exists() || fs::symlink_metadata(&target).is_ok() {
            remove_path(&target)?;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(AppError::io)?;
        }
        symlink_path(&source, &target)?;
    }

    backup_target(&instructions_target)?;
    if instructions_target.exists() || fs::symlink_metadata(&instructions_target).is_ok() {
        remove_path(&instructions_target)?;
    }
    if let Some(parent) = instructions_target.parent() {
        fs::create_dir_all(parent).map_err(AppError::io)?;
    }
    symlink_path(&instructions_path(), &instructions_target)?;

    if skills_path.exists() {
        for entry in fs::read_dir(&skills_path).map_err(AppError::io)? {
            let entry = entry.map_err(AppError::io)?;
            let stale_id = entry.file_name().to_string_lossy().to_string();
            let target = entry.path();
            if is_owned_skill_symlink(&target)?
                && !config
                    .skills
                    .iter()
                    .any(|skill| skill.id == stale_id && skill.enabled)
            {
                fs::remove_file(target).map_err(AppError::io)?;
            }
        }
    }

    Ok(())
}

pub fn run_sync() -> AppResult<()> {
    let config = read_config()?;
    for target_config in &config.configs {
        if !target_config.enabled {
            continue;
        }
        sync_for_config(target_config, &config)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
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
}
