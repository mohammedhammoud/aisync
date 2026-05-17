use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::config::ConfigErrorCode;
use crate::core::github::types::GithubErrorCode;
use crate::core::path_safety::PathErrorCode;
use crate::core::skills::SkillErrorCode;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: AppErrorCode,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(untagged)]
pub enum AppErrorCode {
    Path(PathErrorCode),
    Config(ConfigErrorCode),
    Skill(SkillErrorCode),
    Github(GithubErrorCode),
    System(SystemErrorCode),
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SystemErrorCode {
    Io,
    Json,
    Emit,
    Unknown,
}

impl AppError {
    pub fn new(code: impl Into<AppErrorCode>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }

    pub fn io(_error: std::io::Error) -> Self {
        Self::new(SystemErrorCode::Io, "Filesystem operation failed")
    }

    pub fn json(_error: serde_json::Error) -> Self {
        Self::new(SystemErrorCode::Json, "Data file is invalid")
    }

    pub fn emit(_error: tauri::Error) -> Self {
        Self::new(SystemErrorCode::Emit, "App event delivery failed")
    }

    pub fn unknown(message: impl Into<String>) -> Self {
        Self::new(SystemErrorCode::Unknown, message)
    }
}

impl From<PathErrorCode> for AppErrorCode {
    fn from(code: PathErrorCode) -> Self {
        Self::Path(code)
    }
}

impl From<ConfigErrorCode> for AppErrorCode {
    fn from(code: ConfigErrorCode) -> Self {
        Self::Config(code)
    }
}

impl From<SkillErrorCode> for AppErrorCode {
    fn from(code: SkillErrorCode) -> Self {
        Self::Skill(code)
    }
}

impl From<GithubErrorCode> for AppErrorCode {
    fn from(code: GithubErrorCode) -> Self {
        Self::Github(code)
    }
}

impl From<SystemErrorCode> for AppErrorCode {
    fn from(code: SystemErrorCode) -> Self {
        Self::System(code)
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::io(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::json(error)
    }
}

impl From<tauri::Error> for AppError {
    fn from(error: tauri::Error) -> Self {
        Self::emit(error)
    }
}

impl From<String> for AppError {
    fn from(message: String) -> Self {
        Self::unknown(message)
    }
}
