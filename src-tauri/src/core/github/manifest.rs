use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::errors::{AppError, AppResult};

use super::constants::{MANIFEST_SCHEMA_VERSION, REMOTE_ROOT};
use super::paths::{remote_path, validate_sync_path};
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

pub fn validate_manifest(content: &str) -> AppResult<()> {
    let manifest: RemoteManifest = serde_json::from_str(content).map_err(AppError::json)?;
    if manifest.schema_version != MANIFEST_SCHEMA_VERSION {
        return Err(AppError::unknown(
            "Unsupported GitHub sync manifest version",
        ));
    }
    for (local_path, item) in manifest.items {
        validate_sync_path(&local_path)?;
        let Some(item_local_path) = item.path.strip_prefix(&format!("{REMOTE_ROOT}/")) else {
            return Err(AppError::unknown("Invalid GitHub sync manifest path"));
        };
        if item_local_path != local_path {
            return Err(AppError::unknown("GitHub sync manifest path mismatch"));
        }
    }
    Ok(())
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
