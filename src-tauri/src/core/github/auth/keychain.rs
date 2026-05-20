use std::sync::{Mutex, OnceLock};

use crate::core::errors::{AppError, AppResult};

use crate::core::github::constants::{KEYCHAIN_SERVICE, KEYCHAIN_USER};
use crate::core::github::types::GithubErrorCode;

use super::session::cancel_github_login;

static TOKEN_CACHE: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn token_cache() -> &'static Mutex<Option<String>> {
    TOKEN_CACHE.get_or_init(|| Mutex::new(None))
}

fn cache_token(token: Option<String>) -> AppResult<()> {
    let mut cache = token_cache()
        .lock()
        .map_err(|_error| AppError::unknown("GitHub token cache lock failed"))?;
    *cache = token;
    Ok(())
}

fn cached_token() -> AppResult<Option<String>> {
    token_cache()
        .lock()
        .map(|cache| cache.clone())
        .map_err(|_error| AppError::unknown("GitHub token cache lock failed"))
}

fn token_entry() -> AppResult<keyring::Entry> {
    keyring::Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_USER)
        .map_err(|_error| AppError::new(GithubErrorCode::KeychainFailed, "Keychain access failed"))
}

pub fn save_token(token: &str) -> AppResult<()> {
    token_entry()?.set_password(token).map_err(|_error| {
        AppError::new(GithubErrorCode::KeychainFailed, "Keychain access failed")
    })?;
    cache_token(Some(token.to_string()))
}

pub fn read_token() -> AppResult<Option<String>> {
    if let Some(token) = cached_token()? {
        return Ok(Some(token));
    }

    match token_entry()?.get_password() {
        Ok(token) => {
            cache_token(Some(token.clone()))?;
            Ok(Some(token))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(_error) => Err(AppError::new(
            GithubErrorCode::KeychainFailed,
            "Keychain access failed",
        )),
    }
}

pub fn delete_token() -> AppResult<()> {
    cancel_github_login();
    match token_entry()?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => cache_token(None),
        Err(_error) => Err(AppError::new(
            GithubErrorCode::KeychainFailed,
            "Keychain access failed",
        )),
    }
}
