use serde::{Deserialize, Serialize};
use specta::Type;

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(
    tag = "kind",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum LinkStatus {
    Skill {
        config_name: String,
        skill_id: String,
        state: LinkState,
        target_path: String,
    },
    Instructions {
        config_name: String,
        state: LinkState,
        target_path: String,
    },
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(
    tag = "kind",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum ForceLinkTarget {
    Skill {
        config_name: String,
        skill_id: String,
        target_path: String,
    },
    Instructions {
        config_name: String,
        target_path: String,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LinkState {
    Blocked,
    Missing,
}
