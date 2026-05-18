use base64::prelude::*;
use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;
mod types;

use serde::Deserialize;
use serde_json::json;
use std::time::Duration;

use crate::core::errors::{AppError, AppResult};

use super::constants::{GITHUB_ACCEPT_HEADER, GITHUB_API_BASE_URL, GITHUB_API_VERSION, USER_AGENT};
use super::types::GithubErrorCode;

use types::{CreateRepoRequest, GithubContent, GithubRef, GithubTreeResponse};
pub use types::{GithubRepo, GithubUser};

pub struct GithubClient {
    client: Client,
    token: String,
}

impl GithubClient {
    pub fn new(token: String) -> AppResult<Self> {
        Ok(Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|error| AppError::new(GithubErrorCode::Network, error.to_string()))?,
            token,
        })
    }

    fn api_url(path: &str) -> String {
        format!("{GITHUB_API_BASE_URL}{path}")
    }

    fn encode_segment(segment: &str) -> String {
        let mut encoded = String::new();
        for byte in segment.bytes() {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
                encoded.push(byte as char);
            } else {
                encoded.push_str(&format!("%{byte:02X}"));
            }
        }
        encoded
    }

    fn encode_path(path: &str) -> String {
        path.split('/')
            .map(Self::encode_segment)
            .collect::<Vec<_>>()
            .join("/")
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
                AppError::new(GithubErrorCode::AuthFailed, "GitHub authentication failed")
            }
            Some(_) if error.is_status() => {
                AppError::new(GithubErrorCode::Network, "GitHub request failed")
            }
            _ => AppError::new(GithubErrorCode::Network, "Could not reach GitHub"),
        }
    }

    fn parse<T: for<'de> Deserialize<'de>>(response: Response) -> AppResult<T> {
        response
            .error_for_status()
            .map_err(Self::github_error)?
            .json::<T>()
            .map_err(|_error| AppError::new(GithubErrorCode::Network, "Invalid GitHub response"))
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
            .authenticated(self.client.get(Self::api_url(&format!(
                "/repos/{}/{}",
                Self::encode_segment(owner),
                Self::encode_segment(repo)
            ))))
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
        let reference: GithubRef = self.get(&format!(
            "/repos/{}/{}/git/ref/heads/{}",
            Self::encode_segment(owner),
            Self::encode_segment(repo),
            Self::encode_path(branch)
        ))?;
        Ok(reference.object.sha)
    }

    pub fn tree_paths(&self, owner: &str, repo: &str, commit_sha: &str) -> AppResult<Vec<String>> {
        let tree: GithubTreeResponse = self.get(&format!(
            "/repos/{}/{}/git/trees/{}?recursive=1",
            Self::encode_segment(owner),
            Self::encode_segment(repo),
            Self::encode_segment(commit_sha)
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
                "/repos/{}/{}/contents/{}",
                Self::encode_segment(owner),
                Self::encode_segment(repo),
                Self::encode_path(path)
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
            .map_err(|_error| {
                AppError::new(GithubErrorCode::Network, "Invalid GitHub file content")
            })?;
        let decoded = String::from_utf8(decoded).map_err(|_error| {
            AppError::new(GithubErrorCode::Network, "Invalid GitHub file content")
        })?;
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

        let _: serde_json::Value = self.put(
            &format!(
                "/repos/{}/{}/contents/{}",
                Self::encode_segment(owner),
                Self::encode_segment(repo),
                Self::encode_path(path)
            ),
            body,
        )?;
        Ok(())
    }
}
