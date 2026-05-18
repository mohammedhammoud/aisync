use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubUser {
    pub login: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubRepo {
    pub name: String,
    pub owner: GithubUser,
    pub default_branch: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubRefObject {
    pub sha: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubRef {
    pub object: GithubRefObject,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubContent {
    pub content: Option<String>,
    pub sha: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubTreeResponse {
    pub tree: Vec<GithubTreeItem>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubTreeItem {
    pub path: String,
    pub r#type: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct CreateRepoRequest<'a> {
    pub name: &'a str,
    pub private: bool,
    pub auto_init: bool,
}
