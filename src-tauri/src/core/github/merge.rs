use std::collections::{BTreeMap, BTreeSet};

use crate::core::errors::AppResult;
use crate::core::path_safety::app_root;
use crate::core::sync::run_sync;

use super::constants::CONFLICTS_DIR;
use super::files::{
    rebuild_skill_index_from_disk, remove_local_file, write_conflict_file, write_local_file,
};
use super::settings::GithubRepoSettings;
use super::types::SyncConflict;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MergeOutcome {
    pub conflicts: Vec<SyncConflict>,
    pub changed_local_files: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MergeAction {
    KeepLocal { path: String },
    WriteRemote { path: String, content: String },
    RemoveLocal { path: String },
    Conflict(SyncConflict),
}

pub fn plan_merge(
    baseline_files: &BTreeMap<String, String>,
    local_files: &BTreeMap<String, String>,
    remote_files: &BTreeMap<String, String>,
) -> Vec<MergeAction> {
    let mut actions = Vec::new();
    let mut paths = BTreeSet::new();
    paths.extend(baseline_files.keys().cloned());
    paths.extend(local_files.keys().cloned());
    paths.extend(remote_files.keys().cloned());

    for path in paths {
        let base = baseline_files.get(&path);
        let local = local_files.get(&path);
        let remote = remote_files.get(&path);

        if local == remote {
            actions.push(MergeAction::KeepLocal { path });
            continue;
        }

        if base == local {
            match remote {
                Some(content) => actions.push(MergeAction::WriteRemote {
                    path,
                    content: content.clone(),
                }),
                None => actions.push(MergeAction::RemoveLocal { path }),
            }
            continue;
        }

        if base == remote {
            actions.push(MergeAction::KeepLocal { path });
            continue;
        }

        actions.push(MergeAction::Conflict(SyncConflict {
            path,
            message: "Local and remote changes conflict".into(),
            local_content: local.cloned(),
            remote_content: remote.cloned(),
        }));
    }

    actions
}

pub fn merge_remote_local(
    settings: &GithubRepoSettings,
    local_files: &BTreeMap<String, String>,
    remote_files: &BTreeMap<String, String>,
) -> AppResult<MergeOutcome> {
    let actions = plan_merge(&settings.baseline_files, local_files, remote_files);
    let conflicts = actions
        .iter()
        .filter_map(|action| match action {
            MergeAction::Conflict(conflict) => Some(conflict.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    if !conflicts.is_empty() {
        let conflict_root = app_root()
            .join(CONFLICTS_DIR)
            .join(chrono::Utc::now().format("%Y%m%d%H%M%S%f").to_string());

        for conflict in &conflicts {
            write_conflict_file(
                &conflict_root,
                &conflict.path,
                "local",
                conflict.local_content.as_ref(),
            )?;
            write_conflict_file(
                &conflict_root,
                &conflict.path,
                "remote",
                conflict.remote_content.as_ref(),
            )?;
        }
        return Ok(MergeOutcome {
            conflicts,
            changed_local_files: false,
        });
    }

    let mut changed_local_files = false;
    for action in actions {
        match action {
            MergeAction::WriteRemote { path, content } => {
                write_local_file(&path, &content)?;
                changed_local_files = true;
            }
            MergeAction::RemoveLocal { path } => {
                remove_local_file(&path)?;
                changed_local_files = true;
            }
            MergeAction::KeepLocal { .. } | MergeAction::Conflict(_) => {}
        }
    }

    if changed_local_files {
        rebuild_skill_index_from_disk()?;
        run_sync()?;
    }

    Ok(MergeOutcome {
        conflicts: Vec::new(),
        changed_local_files,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{plan_merge, MergeAction};

    fn files(items: &[(&str, &str)]) -> BTreeMap<String, String> {
        items
            .iter()
            .map(|(path, content)| ((*path).to_string(), (*content).to_string()))
            .collect()
    }

    #[test]
    fn keeps_local_when_remote_matches_baseline() {
        let actions = plan_merge(
            &files(&[("instructions.md", "base")]),
            &files(&[("instructions.md", "local")]),
            &files(&[("instructions.md", "base")]),
        );

        assert_eq!(
            actions,
            vec![MergeAction::KeepLocal {
                path: "instructions.md".into()
            }]
        );
    }

    #[test]
    fn writes_remote_when_local_matches_baseline() {
        let actions = plan_merge(
            &files(&[("instructions.md", "base")]),
            &files(&[("instructions.md", "base")]),
            &files(&[("instructions.md", "remote")]),
        );

        assert_eq!(
            actions,
            vec![MergeAction::WriteRemote {
                path: "instructions.md".into(),
                content: "remote".into()
            }]
        );
    }

    #[test]
    fn removes_local_when_remote_deleted_and_local_matches_baseline() {
        let actions = plan_merge(
            &files(&[("skills/audit/SKILL.md", "base")]),
            &files(&[("skills/audit/SKILL.md", "base")]),
            &BTreeMap::new(),
        );

        assert_eq!(
            actions,
            vec![MergeAction::RemoveLocal {
                path: "skills/audit/SKILL.md".into()
            }]
        );
    }

    #[test]
    fn conflicts_when_local_and_remote_changed_differently() {
        let actions = plan_merge(
            &files(&[("instructions.md", "base")]),
            &files(&[("instructions.md", "local")]),
            &files(&[("instructions.md", "remote")]),
        );

        let [MergeAction::Conflict(conflict)] = actions.as_slice() else {
            panic!("expected conflict");
        };
        assert_eq!(conflict.path, "instructions.md");
        assert_eq!(conflict.local_content.as_deref(), Some("local"));
        assert_eq!(conflict.remote_content.as_deref(), Some("remote"));
    }

    #[test]
    fn keeps_when_local_and_remote_match() {
        let actions = plan_merge(
            &files(&[("instructions.md", "base")]),
            &files(&[("instructions.md", "same")]),
            &files(&[("instructions.md", "same")]),
        );

        assert_eq!(
            actions,
            vec![MergeAction::KeepLocal {
                path: "instructions.md".into()
            }]
        );
    }

    #[test]
    fn writes_remote_only_new_file() {
        let actions = plan_merge(
            &BTreeMap::new(),
            &BTreeMap::new(),
            &files(&[("skills/audit/SKILL.md", "remote")]),
        );

        assert_eq!(
            actions,
            vec![MergeAction::WriteRemote {
                path: "skills/audit/SKILL.md".into(),
                content: "remote".into()
            }]
        );
    }

    #[test]
    fn keeps_local_only_new_file() {
        let actions = plan_merge(
            &BTreeMap::new(),
            &files(&[("skills/audit/SKILL.md", "local")]),
            &BTreeMap::new(),
        );

        assert_eq!(
            actions,
            vec![MergeAction::KeepLocal {
                path: "skills/audit/SKILL.md".into()
            }]
        );
    }

    #[test]
    fn conflicts_when_local_deleted_and_remote_changed() {
        let actions = plan_merge(
            &files(&[("instructions.md", "base")]),
            &BTreeMap::new(),
            &files(&[("instructions.md", "remote")]),
        );

        let [MergeAction::Conflict(conflict)] = actions.as_slice() else {
            panic!("expected conflict");
        };
        assert_eq!(conflict.path, "instructions.md");
        assert_eq!(conflict.local_content, None);
        assert_eq!(conflict.remote_content.as_deref(), Some("remote"));
    }
}
