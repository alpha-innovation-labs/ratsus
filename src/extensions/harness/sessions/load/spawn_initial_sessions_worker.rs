use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;

use anyhow::Result;

use crate::extensions::harness::core::chat_harness::ChatHarness;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::persistence::load_normal_terminal_sessions::load_normal_terminal_sessions;

/// Spawns a background worker for startup session loading.
pub fn spawn_initial_sessions_worker(
    chat_harness: Arc<dyn ChatHarness>,
) -> Receiver<Result<Vec<ChatSession>>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let result = load_initial_sessions(chat_harness);
        let _ = sender.send(result);
    });
    receiver
}

/// Loads chat and configured normal terminal sessions for startup.
fn load_initial_sessions(chat_harness: Arc<dyn ChatHarness>) -> Result<Vec<ChatSession>> {
    let mut sessions = chat_harness.load_sessions()?;
    if chat_harness.load_normal_terminals_on_startup() {
        sessions.extend(load_normal_terminal_sessions());
    }
    Ok(sessions)
}
