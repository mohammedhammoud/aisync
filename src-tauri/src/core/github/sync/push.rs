use crate::core::constants::{INSTRUCTIONS_FILE, SKILLS_DIR};
use crate::core::errors::AppResult;

use crate::core::github::api::GithubClient;
use crate::core::github::constants::{MANIFEST_FILE, REMOTE_ROOT, SKILL_FILE, SKILL_METADATA_FILE};
use crate::core::github::files::collect_local_files;
use crate::core::github::manifest::manifest;
use crate::core::github::paths::remote_path;
use crate::core::github::settings::GithubRepoSettings;

pub fn push_all(client: &GithubClient, settings: &GithubRepoSettings) -> AppResult<String> {
    let files = collect_local_files()?;
    let mut changed_files = Vec::new();

    for (path, content) in &files {
        let path = remote_path(path);
        let existing = client.read_file(&settings.repo_owner, &settings.repo_name, &path)?;
        if existing
            .as_ref()
            .is_some_and(|(existing_content, _sha)| existing_content == content)
        {
            continue;
        }
        changed_files.push((path, content.clone(), existing.map(|(_, sha)| sha)));
    }

    let mut deleted_files = Vec::new();
    for path in settings.baseline_files.keys() {
        if files.contains_key(path) {
            continue;
        }
        let path = remote_path(path);
        if let Some((_content, sha)) =
            client.read_file(&settings.repo_owner, &settings.repo_name, &path)?
        {
            deleted_files.push((path, sha));
        }
    }

    let manifest_path = remote_path(MANIFEST_FILE);
    let existing_manifest =
        client.read_file(&settings.repo_owner, &settings.repo_name, &manifest_path)?;

    if changed_files.is_empty() && deleted_files.is_empty() && existing_manifest.is_some() {
        return client.branch_head(
            &settings.repo_owner,
            &settings.repo_name,
            &settings.default_branch,
        );
    }

    for (path, sha) in deleted_files {
        client.delete_file(
            &settings.repo_owner,
            &settings.repo_name,
            &path,
            &sha,
            &sync_delete_commit_message(&path),
        )?;
    }

    for (path, content, sha) in changed_files {
        client.write_file(
            &settings.repo_owner,
            &settings.repo_name,
            &path,
            &content,
            sha.as_deref(),
            &sync_commit_message(&path),
        )?;
    }

    client.write_file(
        &settings.repo_owner,
        &settings.repo_name,
        &manifest_path,
        &manifest(&files)?,
        existing_manifest.as_ref().map(|(_, sha)| sha.as_str()),
        "chore(sync): update manifest",
    )?;

    client.branch_head(
        &settings.repo_owner,
        &settings.repo_name,
        &settings.default_branch,
    )
}

fn sync_commit_message(path: &str) -> String {
    if path == remote_path(INSTRUCTIONS_FILE).as_str() {
        return "chore(sync): update instructions".into();
    }

    if let Some(rest) = path.strip_prefix(&format!("{REMOTE_ROOT}/{SKILLS_DIR}/")) {
        if let Some((skill_id, file_name)) = rest.split_once('/') {
            return match file_name {
                SKILL_FILE => format!("chore(sync): update skill {skill_id}"),
                SKILL_METADATA_FILE => format!("chore(sync): update skill metadata {skill_id}"),
                _ => format!("chore(sync): update {rest}"),
            };
        }
    }

    if path == remote_path(MANIFEST_FILE).as_str() {
        return "chore(sync): update manifest".into();
    }

    format!("chore(sync): update {path}")
}

fn sync_delete_commit_message(path: &str) -> String {
    if path == remote_path(INSTRUCTIONS_FILE).as_str() {
        return "chore(sync): delete instructions".into();
    }

    if let Some(rest) = path.strip_prefix(&format!("{REMOTE_ROOT}/{SKILLS_DIR}/")) {
        if let Some((skill_id, file_name)) = rest.split_once('/') {
            return match file_name {
                SKILL_FILE => format!("chore(sync): delete skill {skill_id}"),
                SKILL_METADATA_FILE => format!("chore(sync): delete skill metadata {skill_id}"),
                _ => format!("chore(sync): delete {rest}"),
            };
        }
    }

    format!("chore(sync): delete {path}")
}
