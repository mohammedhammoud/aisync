use crate::core::errors::{AppError, AppResult};
use crate::core::path_safety::validate_id;

use super::types::{SkillMetadata, TargetConfig};

pub(crate) fn validate_skill_metadata(metadata: &SkillMetadata) -> AppResult<()> {
    validate_id(&metadata.id)?;
    validate_string_limit("skill name", &metadata.name, 120)?;
    validate_string_limit("skill description", &metadata.description, 2_000)?;
    if metadata.tags.len() > 50 {
        return Err(AppError::unknown("Skill has too many tags"));
    }
    for tag in &metadata.tags {
        validate_string_limit("skill tag", tag, 80)?;
    }
    Ok(())
}

pub(crate) fn validate_target_config(target: &TargetConfig) -> AppResult<()> {
    validate_id(&target.id)?;
    validate_string_limit("config name", &target.name, 120)?;
    validate_string_limit("skills path", &target.skills_path, 4_096)?;
    validate_string_limit("instructions path", &target.instructions_path, 4_096)?;
    Ok(())
}

fn validate_string_limit(label: &str, value: &str, max_bytes: usize) -> AppResult<()> {
    if value.len() > max_bytes {
        return Err(AppError::unknown(format!(
            "{label} is too large; maximum is {max_bytes} bytes"
        )));
    }
    Ok(())
}
