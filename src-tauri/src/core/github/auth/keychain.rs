use crate::core::errors::{AppError, AppResult};

use crate::core::github::constants::{KEYCHAIN_SERVICE, KEYCHAIN_USER};
use crate::core::github::types::GithubErrorCode;

use super::session::cancel_github_login;

fn token_entry() -> AppResult<keyring::Entry> {
    keyring::Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_USER)
        .map_err(|_error| AppError::new(GithubErrorCode::KeychainFailed, "Keychain access failed"))
}

pub fn save_token(token: &str) -> AppResult<()> {
    token_entry()?
        .set_password(token)
        .map_err(|_error| AppError::new(GithubErrorCode::KeychainFailed, "Keychain access failed"))
}

pub fn read_token() -> AppResult<Option<String>> {
    match token_entry()?.get_password() {
        Ok(token) => Ok(Some(token)),
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
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_error) => Err(AppError::new(
            GithubErrorCode::KeychainFailed,
            "Keychain access failed",
        )),
    }
}
