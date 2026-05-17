use crate::core::constants::{INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::validate_id;

use super::constants::{REMOTE_ROOT, SKILL_FILE, SKILL_METADATA_FILE};
use super::types::GithubErrorCode;

pub fn remote_path(local_path: &str) -> String {
    format!("{REMOTE_ROOT}/{local_path}")
}

pub fn invalid_remote_path(path: &str) -> AppError {
    AppError::new(
        GithubErrorCode::RemotePathInvalid,
        format!("Invalid GitHub sync path: {path}"),
    )
}

pub fn validate_sync_path(path: &str) -> AppResult<()> {
    if path == INSTRUCTIONS_FILE {
        return Ok(());
    }

    let Some(rest) = path.strip_prefix(&format!("{SKILLS_DIR}/")) else {
        return Err(invalid_remote_path(path));
    };
    let Some((skill_id, file_name)) = rest.split_once('/') else {
        return Err(invalid_remote_path(path));
    };
    if rest.contains("../") || skill_id.contains('/') || skill_id == "." || skill_id == ".." {
        return Err(invalid_remote_path(path));
    }
    validate_id(skill_id)?;
    if matches!(file_name, SKILL_FILE | SKILL_METADATA_FILE) {
        Ok(())
    } else {
        Err(invalid_remote_path(path))
    }
}

#[cfg(test)]
mod tests {
    use super::validate_sync_path;

    #[test]
    fn validates_allowed_sync_paths() {
        assert!(validate_sync_path("instructions.md").is_ok());
        assert!(validate_sync_path("skills/audit/SKILL.md").is_ok());
        assert!(validate_sync_path("skills/audit/metadata.json").is_ok());
    }

    #[test]
    fn rejects_unsafe_sync_paths() {
        for path in [
            "../instructions.md",
            "/tmp/instructions.md",
            "skills/../audit/SKILL.md",
            "skills/audit/../../config.json",
            "skills//audit/SKILL.md",
            "skills/audit\\SKILL.md",
            "skills/bad.id/SKILL.md",
            "skills/audit/extra.txt",
            "skills/audit/SKILL.md/extra",
            "config.json",
        ] {
            assert!(validate_sync_path(path).is_err(), "{path}");
        }
    }
}
