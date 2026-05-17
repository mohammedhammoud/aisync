use std::sync::atomic::{AtomicU64, Ordering};

static LOGIN_SESSION_ID: AtomicU64 = AtomicU64::new(0);

pub fn next_login_session() -> u64 {
    LOGIN_SESSION_ID.fetch_add(1, Ordering::SeqCst) + 1
}

pub fn cancel_github_login() {
    LOGIN_SESSION_ID.fetch_add(1, Ordering::SeqCst);
}

pub fn is_active_login_session(session_id: u64) -> bool {
    LOGIN_SESSION_ID.load(Ordering::SeqCst) == session_id
}
