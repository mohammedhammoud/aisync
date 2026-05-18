use std::collections::{BTreeMap, BTreeSet};

use crate::core::errors::AppResult;

use crate::core::github::api::GithubClient;
use crate::core::github::constants::{MANIFEST_FILE, REMOTE_ROOT};
use crate::core::github::manifest::validate_manifest;
use crate::core::github::paths::{remote_path, validate_sync_path};
use crate::core::github::settings::GithubRepoSettings;

pub fn read_remote_files(
    client: &GithubClient,
    settings: &GithubRepoSettings,
    head: &str,
) -> AppResult<BTreeMap<String, String>> {
    let manifest_path = remote_path(MANIFEST_FILE);
    if let Some((content, _sha)) =
        client.read_file(&settings.repo_owner, &settings.repo_name, &manifest_path)?
    {
        validate_manifest(&content)?;
    }

    let paths = client.tree_paths(&settings.repo_owner, &settings.repo_name, head)?;
    let mut remote_paths = BTreeSet::new();
    for path in paths {
        let Some(path) = path
            .strip_prefix(&format!("{REMOTE_ROOT}/"))
            .map(str::to_string)
        else {
            continue;
        };
        if path == MANIFEST_FILE || validate_sync_path(&path).is_err() {
            continue;
        }
        remote_paths.insert(path);
    }
    let mut files = BTreeMap::new();

    for path in remote_paths {
        let full_path = remote_path(&path);
        if let Some((content, _sha)) =
            client.read_file(&settings.repo_owner, &settings.repo_name, &full_path)?
        {
            files.insert(path, content);
        }
    }

    Ok(files)
}
