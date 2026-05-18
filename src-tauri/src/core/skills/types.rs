use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::config::SkillMetadata;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
pub enum SkillErrorCode {
    #[serde(rename = "skill_not_found")]
    NotFound,
    #[serde(rename = "skill_already_exists")]
    AlreadyExists,
    #[serde(rename = "skill_id_mismatch")]
    IdMismatch,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillEditorRecord {
    pub metadata: SkillMetadata,
    pub body: String,
    pub frontmatter_lines: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedSkillContent {
    pub body: String,
    pub frontmatter_lines: Vec<String>,
}
