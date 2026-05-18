use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use tauri::{AppHandle, Emitter};

use crate::core::errors::AppResult;

use super::auth::get_github_sync_status;
use super::sync::sync_auto_blocking;
use super::types::{GithubSyncEvent, SyncResult};

const AUTO_SYNC_DEBOUNCE: Duration = Duration::from_secs(2);
pub const GITHUB_SYNC_EVENT: &str = "github-sync-state-changed";

static AUTO_SYNC_SENDER: OnceLock<Sender<AutoSyncMessage>> = OnceLock::new();

#[derive(Clone, Copy)]
enum AutoSyncMessage {
    Trigger,
}

pub fn init(app: AppHandle) {
    let (sender, receiver) = mpsc::channel::<AutoSyncMessage>();
    if AUTO_SYNC_SENDER.set(sender).is_err() {
        return;
    }

    let worker_app = app.clone();
    thread::spawn(move || {
        while receiver.recv().is_ok() {
            loop {
                match receiver.recv_timeout(AUTO_SYNC_DEBOUNCE) {
                    Ok(AutoSyncMessage::Trigger) => continue,
                    Err(RecvTimeoutError::Timeout) => break,
                    Err(RecvTimeoutError::Disconnected) => return,
                }
            }
            run_auto_sync(&worker_app);
        }
    });

    emit_current_status(&app);
}

pub fn request_auto_sync() -> AppResult<()> {
    if let Some(sender) = AUTO_SYNC_SENDER.get() {
        sender
            .send(AutoSyncMessage::Trigger)
            .map_err(|error| crate::core::errors::AppError::unknown(error.to_string()))?;
    }
    Ok(())
}

pub fn emit_current_status(app: &AppHandle) {
    match get_github_sync_status() {
        Ok(status) => emit_sync_event(
            app,
            GithubSyncEvent {
                status: Some(status),
                last_result: None,
                is_connecting: false,
                is_syncing: false,
                is_user_initiated_sync: false,
                last_error: None,
            },
        ),
        Err(error) => emit_sync_event(
            app,
            GithubSyncEvent {
                status: None,
                last_result: None,
                is_connecting: false,
                is_syncing: false,
                is_user_initiated_sync: false,
                last_error: Some(error.message),
            },
        ),
    }
}

pub fn emit_sync_started(app: &AppHandle, is_user_initiated_sync: bool) {
    emit_sync_event(
        app,
        GithubSyncEvent {
            status: get_github_sync_status().ok(),
            last_result: None,
            is_connecting: false,
            is_syncing: true,
            is_user_initiated_sync,
            last_error: None,
        },
    );
}

pub fn emit_sync_finished(
    app: &AppHandle,
    last_result: Option<SyncResult>,
    _is_user_initiated_sync: bool,
) {
    emit_sync_event(
        app,
        GithubSyncEvent {
            status: get_github_sync_status().ok(),
            last_result,
            is_connecting: false,
            is_syncing: false,
            is_user_initiated_sync: false,
            last_error: None,
        },
    );
}

pub fn emit_sync_failed(app: &AppHandle, error: String, _is_user_initiated_sync: bool) {
    emit_sync_event(
        app,
        GithubSyncEvent {
            status: get_github_sync_status().ok(),
            last_result: None,
            is_connecting: false,
            is_syncing: false,
            is_user_initiated_sync: false,
            last_error: Some(error),
        },
    );
}

fn run_auto_sync(app: &AppHandle) {
    emit_sync_started(app, false);
    match sync_auto_blocking() {
        Ok(result) => emit_sync_event(
            app,
            GithubSyncEvent {
                status: Some(result.status),
                last_result: result.result,
                is_connecting: false,
                is_syncing: false,
                is_user_initiated_sync: false,
                last_error: None,
            },
        ),
        Err(error) => emit_sync_failed(app, error.message, false),
    }
}

pub fn emit_login_finished(app: &AppHandle, error: Option<String>) {
    emit_sync_event(
        app,
        GithubSyncEvent {
            status: get_github_sync_status().ok(),
            last_result: None,
            is_connecting: false,
            is_syncing: false,
            is_user_initiated_sync: false,
            last_error: error,
        },
    );
}

fn emit_sync_event(app: &AppHandle, event: GithubSyncEvent) {
    let _ = app.emit(GITHUB_SYNC_EVENT, event);
}
