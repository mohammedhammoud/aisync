use base64::prelude::*;
use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::errors::{AppError, AppResult};

use super::constants::{GITHUB_ACCEPT_HEADER, GITHUB_API_BASE_URL, GITHUB_API_VERSION, USER_AGENT};
use super::types::GithubErrorCode;

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
struct CreateRepoRequest<'a> {
    name: &'a str,
    private: bool,
    auto_init: bool,
}

pub struct GithubClient {
    client: Client,
    token: String,
}

impl GithubClient {
    pub fn new(token: String) -> AppResult<Self> {
        Ok(Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .map_err(|error| AppError::new(GithubErrorCode::Network, error.to_string()))?,
            token,
        })
    }

    fn api_url(path: &str) -> String {
        format!("{GITHUB_API_BASE_URL}{path}")
    }

    fn authenticated(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        request
            .bearer_auth(&self.token)
            .header("Accept", GITHUB_ACCEPT_HEADER)
            .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
    }

    fn github_error(error: reqwest::Error) -> AppError {
        match error.status() {
            Some(StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) => {
                AppError::new(GithubErrorCode::AuthFailed, error.to_string())
            }
            Some(_) if error.is_status() => {
                AppError::new(GithubErrorCode::Network, error.to_string())
            }
            _ => AppError::new(GithubErrorCode::Network, error.to_string()),
        }
    }

    fn parse<T: for<'de> Deserialize<'de>>(response: Response) -> AppResult<T> {
        response
            .error_for_status()
            .map_err(Self::github_error)?
            .json::<T>()
            .map_err(|error| AppError::new(GithubErrorCode::Network, error.to_string()))
    }

    fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> AppResult<T> {
        let response = self
            .authenticated(self.client.get(Self::api_url(path)))
            .send()
            .map_err(Self::github_error)?;
        Self::parse(response)
    }

    fn put<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> AppResult<T> {
        let response = self
            .authenticated(self.client.put(Self::api_url(path)))
            .json(&body)
            .send()
            .map_err(Self::github_error)?;
        Self::parse(response)
    }

    pub fn current_user(&self) -> AppResult<GithubUser> {
        self.get("/user")
    }

    pub fn get_repo(&self, owner: &str, repo: &str) -> AppResult<Option<GithubRepo>> {
        let response = self
            .authenticated(
                self.client
                    .get(Self::api_url(&format!("/repos/{owner}/{repo}"))),
            )
            .send()
            .map_err(Self::github_error)?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        Self::parse(response).map(Some)
    }

    pub fn create_private_repo(&self, name: &str) -> AppResult<GithubRepo> {
        let response = self
            .authenticated(self.client.post(Self::api_url("/user/repos")))
            .json(&CreateRepoRequest {
                name,
                private: true,
                auto_init: true,
            })
            .send()
            .map_err(Self::github_error)?;
        Self::parse(response)
    }

    pub fn branch_head(&self, owner: &str, repo: &str, branch: &str) -> AppResult<String> {
        let reference: GithubRef =
            self.get(&format!("/repos/{owner}/{repo}/git/ref/heads/{branch}"))?;
        Ok(reference.object.sha)
    }

    pub fn tree_paths(&self, owner: &str, repo: &str, commit_sha: &str) -> AppResult<Vec<String>> {
        let tree: GithubTreeResponse = self.get(&format!(
            "/repos/{owner}/{repo}/git/trees/{commit_sha}?recursive=1"
        ))?;
        Ok(tree
            .tree
            .into_iter()
            .filter(|item| item.r#type == "blob")
            .map(|item| item.path)
            .collect())
    }

    pub fn read_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> AppResult<Option<(String, String)>> {
        let response = self
            .authenticated(self.client.get(Self::api_url(&format!(
                "/repos/{owner}/{repo}/contents/{path}"
            ))))
            .send()
            .map_err(Self::github_error)?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        let content: GithubContent = Self::parse(response)?;
        let encoded = content.content.unwrap_or_default().replace('\n', "");
        let decoded = BASE64_STANDARD
            .decode(encoded.as_bytes())
            .map_err(|error| AppError::new(GithubErrorCode::Network, error.to_string()))?;
        let decoded = String::from_utf8(decoded)
            .map_err(|error| AppError::new(GithubErrorCode::Network, error.to_string()))?;
        Ok(Some((decoded, content.sha)))
    }

    pub fn write_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        content: &str,
        sha: Option<&str>,
        message: &str,
    ) -> AppResult<()> {
        let mut body = json!({
            "message": message,
            "content": BASE64_STANDARD.encode(content.as_bytes())
        });
        if let Some(sha) = sha {
            body["sha"] = json!(sha);
        }

        let _: serde_json::Value =
            self.put(&format!("/repos/{owner}/{repo}/contents/{path}"), body)?;
        Ok(())
    }
}
