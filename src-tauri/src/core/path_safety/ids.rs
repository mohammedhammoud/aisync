use crate::core::errors::{AppError, AppResult};

use super::types::PathErrorCode;

pub fn validate_id(id: &str) -> AppResult<()> {
    if id.is_empty() || id.len() > 96 {
        return Err(AppError::new(
            PathErrorCode::InvalidIdLength,
            "Invalid ID length",
        ));
    }

    let valid = id
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || matches!(char, '-' | '_'));

    if valid {
        Ok(())
    } else {
        Err(AppError::new(
            PathErrorCode::InvalidIdFormat,
            "Invalid ID. Use letters, numbers, hyphen, or underscore.",
        ))
    }
}
