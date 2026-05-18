use reqwest::blocking::Client;
use std::time::Duration;

use crate::core::errors::{AppError, AppResult};

use crate::core::github::constants::USER_AGENT;
use crate::core::github::types::GithubErrorCode;

pub fn github_network_error(_error: reqwest::Error) -> AppError {
    AppError::new(GithubErrorCode::Network, "Could not reach GitHub")
}

pub fn client() -> AppResult<Client> {
    Client::builder()
        .user_agent(USER_AGENT)
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(github_network_error)
}
