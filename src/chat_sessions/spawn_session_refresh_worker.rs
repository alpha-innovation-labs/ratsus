use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::harness::chat_harness::ChatHarness;
use crate::harness::chat_session::ChatSession;

pub type SessionRefreshResult = Result<Vec<ChatSession>, String>;

/// Starts a background worker that refreshes harness metadata without blocking the UI.
pub fn spawn_session_refresh_worker(
    chat_harness: Arc<dyn ChatHarness>,
    refresh_interval: Duration,
) -> Receiver<SessionRefreshResult> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || loop {
        let result = chat_harness
            .refresh_sessions()
            .map_err(|error| error.to_string());
        if sender.send(result).is_err() {
            break;
        }
        thread::sleep(refresh_interval);
    });
    receiver
}
