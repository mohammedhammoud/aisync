use serde::Deserialize;
use std::thread;
use std::time::{Duration, Instant};

use tauri::AppHandle;

use crate::core::errors::{AppError, AppResult};

use crate::core::github::constants::{
    GITHUB_ACCESS_TOKEN_URL, GITHUB_CLIENT_ID, GITHUB_DEVICE_CODE_URL, GITHUB_OAUTH_GRANT_TYPE,
    GITHUB_OAUTH_SCOPE,
};
use crate::core::github::events::{
    emit_login_finished, emit_sync_failed, emit_sync_finished, emit_sync_started,
};
use crate::core::github::sync::setup_github_sync_blocking;
use crate::core::github::types::{GithubErrorCode, GithubLoginStart};

use super::http::{client, github_network_error};
use super::keychain::save_token;
use super::session::{is_active_login_session, next_login_session};

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u32,
    interval: u32,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

enum TokenPollResult {
    Pending,
    SlowDown,
    Token(String),
}

fn poll_for_token(device_code: &str) -> AppResult<TokenPollResult> {
    let response = client()?
        .post(GITHUB_ACCESS_TOKEN_URL)
        .header("Accept", "application/json")
        .form(&[
            ("client_id", GITHUB_CLIENT_ID),
            ("device_code", device_code),
            ("grant_type", GITHUB_OAUTH_GRANT_TYPE),
        ])
        .send()
        .map_err(github_network_error)?
        .error_for_status()
        .map_err(github_network_error)?
        .json::<AccessTokenResponse>()
        .map_err(github_network_error)?;

    match response.error.as_deref() {
        Some("authorization_pending") => return Ok(TokenPollResult::Pending),
        Some("slow_down") => return Ok(TokenPollResult::SlowDown),
        _ => {}
    }

    if let Some(error) = response.error {
        return Err(AppError::new(
            GithubErrorCode::AuthFailed,
            response.error_description.unwrap_or(error),
        ));
    }

    response
        .access_token
        .map(TokenPollResult::Token)
        .ok_or_else(|| {
            AppError::new(
                GithubErrorCode::AuthFailed,
                "GitHub did not return an access token",
            )
        })
}

fn run_github_login(app: AppHandle, session_id: u64, login: GithubLoginStart) {
    let expires_at = Instant::now() + Duration::from_secs(u64::from(login.expires_in));
    let mut interval = Duration::from_secs(u64::from(login.interval));

    while is_active_login_session(session_id) && Instant::now() < expires_at {
        thread::sleep(interval);
        if !is_active_login_session(session_id) || Instant::now() >= expires_at {
            break;
        }

        match poll_for_token(&login.device_code) {
            Ok(TokenPollResult::Token(token)) => {
                if !is_active_login_session(session_id) {
                    return;
                }
                if let Err(error) = save_token(&token) {
                    emit_login_finished(&app, Some(error.message));
                    return;
                }

                emit_login_finished(&app, None);
                emit_sync_started(&app, true);
                let result = setup_github_sync_blocking();
                match &result {
                    Ok(_status) => emit_sync_finished(&app, None, true),
                    Err(error) => emit_sync_failed(&app, error.message.clone(), true),
                }
                return;
            }
            Ok(TokenPollResult::Pending) => {}
            Ok(TokenPollResult::SlowDown) => {
                interval += Duration::from_secs(5);
            }
            Err(error) => {
                emit_login_finished(&app, Some(error.message));
                return;
            }
        }
    }

    if is_active_login_session(session_id) {
        emit_login_finished(&app, Some("GitHub authorization expired.".to_string()));
    }
}

pub fn start_github_login(app: AppHandle) -> AppResult<GithubLoginStart> {
    let response = client()?
        .post(GITHUB_DEVICE_CODE_URL)
        .header("Accept", "application/json")
        .form(&[
            ("client_id", GITHUB_CLIENT_ID),
            ("scope", GITHUB_OAUTH_SCOPE),
        ])
        .send()
        .map_err(github_network_error)?
        .error_for_status()
        .map_err(github_network_error)?
        .json::<DeviceCodeResponse>()
        .map_err(github_network_error)?;

    let login = GithubLoginStart {
        device_code: response.device_code,
        user_code: response.user_code,
        verification_uri: response.verification_uri,
        expires_in: response.expires_in,
        interval: response.interval,
    };
    let session_id = next_login_session();
    let worker_login = login.clone();
    thread::spawn(move || run_github_login(app, session_id, worker_login));

    Ok(login)
}
