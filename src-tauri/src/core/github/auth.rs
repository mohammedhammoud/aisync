use reqwest::blocking::Client;
use serde::Deserialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use tauri::AppHandle;

use crate::core::errors::{AppError, AppResult};

use super::constants::{
    GITHUB_ACCESS_TOKEN_URL, GITHUB_CLIENT_ID, GITHUB_DEVICE_CODE_URL, GITHUB_OAUTH_GRANT_TYPE,
    GITHUB_OAUTH_SCOPE, KEYCHAIN_SERVICE, KEYCHAIN_USER, USER_AGENT,
};
use super::events::{emit_login_finished, emit_sync_failed, emit_sync_finished, emit_sync_started};
use super::settings::{clear_github_repo, read_local_settings};
use super::sync::{has_local_changes, setup_github_sync_blocking};
use super::types::{GithubErrorCode, GithubLoginStart, GithubSyncStatus};

static TOKEN_FALLBACK: Mutex<Option<String>> = Mutex::new(None);
static LOGIN_SESSION_ID: AtomicU64 = AtomicU64::new(0);

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

fn github_network_error(error: reqwest::Error) -> AppError {
    AppError::new(GithubErrorCode::Network, error.to_string())
}

fn client() -> AppResult<Client> {
    Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(github_network_error)
}

fn token_entry() -> AppResult<keyring::Entry> {
    keyring::Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_USER)
        .map_err(|error| AppError::new(GithubErrorCode::KeychainFailed, error.to_string()))
}

pub fn save_token(token: &str) -> AppResult<()> {
    token_entry()?
        .set_password(token)
        .map_err(|error| AppError::new(GithubErrorCode::KeychainFailed, error.to_string()))?;
    *TOKEN_FALLBACK
        .lock()
        .map_err(|error| AppError::unknown(error.to_string()))? = Some(token.to_string());
    Ok(())
}

pub fn read_token() -> AppResult<Option<String>> {
    match token_entry()?.get_password() {
        Ok(token) => Ok(Some(token)),
        Err(keyring::Error::NoEntry) => TOKEN_FALLBACK
            .lock()
            .map(|token| token.clone())
            .map_err(|error| AppError::unknown(error.to_string())),
        Err(error) => Err(AppError::new(
            GithubErrorCode::KeychainFailed,
            error.to_string(),
        )),
    }
}

pub fn delete_token() -> AppResult<()> {
    cancel_github_login();
    *TOKEN_FALLBACK
        .lock()
        .map_err(|error| AppError::unknown(error.to_string()))? = None;
    match token_entry()?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(error) => Err(AppError::new(
            GithubErrorCode::KeychainFailed,
            error.to_string(),
        )),
    }
}

fn cancel_github_login() {
    LOGIN_SESSION_ID.fetch_add(1, Ordering::SeqCst);
}

fn is_active_login_session(session_id: u64) -> bool {
    LOGIN_SESSION_ID.load(Ordering::SeqCst) == session_id
}

fn poll_for_token(device_code: &str) -> AppResult<Option<String>> {
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

    if matches!(
        response.error.as_deref(),
        Some("authorization_pending" | "slow_down")
    ) {
        return Ok(None);
    }

    if let Some(error) = response.error {
        return Err(AppError::new(
            GithubErrorCode::AuthFailed,
            response.error_description.unwrap_or(error),
        ));
    }

    response.access_token.map(Some).ok_or_else(|| {
        AppError::new(
            GithubErrorCode::AuthFailed,
            "GitHub did not return an access token",
        )
    })
}

fn run_github_login(app: AppHandle, session_id: u64, login: GithubLoginStart) {
    let expires_at = Instant::now() + Duration::from_secs(u64::from(login.expires_in));
    let interval = Duration::from_secs(u64::from(login.interval));

    while is_active_login_session(session_id) && Instant::now() < expires_at {
        thread::sleep(interval);
        if !is_active_login_session(session_id) || Instant::now() >= expires_at {
            break;
        }

        match poll_for_token(&login.device_code) {
            Ok(Some(token)) => {
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
            Ok(None) => {}
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

#[tauri::command]
#[specta::specta]
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
    let session_id = LOGIN_SESSION_ID.fetch_add(1, Ordering::SeqCst) + 1;
    let worker_login = login.clone();
    thread::spawn(move || run_github_login(app, session_id, worker_login));

    Ok(login)
}

#[tauri::command]
#[specta::specta]
pub fn logout_github() -> AppResult<()> {
    delete_token()?;
    clear_github_repo()
}

#[tauri::command]
#[specta::specta]
pub fn get_github_sync_status() -> AppResult<GithubSyncStatus> {
    let settings = read_local_settings()?;
    let has_token = read_token()?.is_some();
    let github = settings.github;

    let has_local_changes = github.as_ref().is_some_and(has_local_changes);

    Ok(GithubSyncStatus {
        connected: has_token && github.is_some(),
        repo_owner: github.as_ref().map(|item| item.repo_owner.clone()),
        repo_name: github.as_ref().map(|item| item.repo_name.clone()),
        default_branch: github.as_ref().map(|item| item.default_branch.clone()),
        last_synced_commit_sha: github
            .as_ref()
            .and_then(|item| item.last_synced_commit_sha.clone()),
        last_synced_at: github.as_ref().and_then(|item| item.last_synced_at.clone()),
        has_token,
        has_local_changes,
    })
}
