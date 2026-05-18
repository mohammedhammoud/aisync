use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GithubErrorCode {
    AuthFailed,
    KeychainFailed,
    Network,
    NotConnected,
    RemotePathInvalid,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GithubLoginStart {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u32,
    pub interval: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GithubSyncStatus {
    pub connected: bool,
    pub repo_owner: Option<String>,
    pub repo_name: Option<String>,
    pub default_branch: Option<String>,
    pub last_synced_commit_sha: Option<String>,
    pub last_synced_at: Option<String>,
    pub has_token: bool,
    pub has_local_changes: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SyncState {
    NotConnected,
    UpToDate,
    Pulled,
    Pushed,
    Conflict,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SyncConflict {
    pub path: String,
    pub message: String,
    pub local_content: Option<String>,
    pub remote_content: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SyncConflictResolution {
    Local,
    Remote,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SyncResult {
    pub status: SyncState,
    pub conflicts: Vec<SyncConflict>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GithubAutoSyncResult {
    pub status: GithubSyncStatus,
    pub result: Option<SyncResult>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Type, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GithubSyncEvent {
    pub status: Option<GithubSyncStatus>,
    pub last_result: Option<SyncResult>,
    pub is_connecting: bool,
    pub is_syncing: bool,
    pub is_user_initiated_sync: bool,
    pub last_error: Option<String>,
}
