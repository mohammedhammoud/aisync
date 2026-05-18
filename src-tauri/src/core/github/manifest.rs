use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::errors::{AppError, AppResult};

use super::constants::MANIFEST_SCHEMA_VERSION;
use super::paths::remote_path;
use super::settings::get_or_create_device_id;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RemoteManifest {
    schema_version: u8,
    updated_at: String,
    updated_by: String,
    items: BTreeMap<String, RemoteManifestItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct RemoteManifestItem {
    path: String,
    updated_at: String,
    updated_by: String,
}

pub fn manifest(files: &BTreeMap<String, String>) -> AppResult<String> {
    let now = chrono::Utc::now().to_rfc3339();
    let device_id = get_or_create_device_id()?;
    let items = files
        .keys()
        .map(|path| {
            (
                path.clone(),
                RemoteManifestItem {
                    path: remote_path(path),
                    updated_at: now.clone(),
                    updated_by: device_id.clone(),
                },
            )
        })
        .collect();

    serde_json::to_string_pretty(&RemoteManifest {
        schema_version: MANIFEST_SCHEMA_VERSION,
        updated_at: now,
        updated_by: device_id,
        items,
    })
    .map(|content| format!("{content}\n"))
    .map_err(AppError::json)
}
