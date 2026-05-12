use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use crate::nexus_sessions::load_nexus_session_registry::load_nexus_session_registry;
use crate::nexus_sessions::session_info::NexusSession;

pub type SessionRefreshResult = Result<Vec<NexusSession>, String>;

/// Starts a background worker that refreshes Nexus registry metadata without blocking the UI.
pub fn spawn_session_refresh_worker(refresh_interval: Duration) -> Receiver<SessionRefreshResult> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || loop {
        let result = load_nexus_session_registry().map_err(|error| error.to_string());
        if sender.send(result).is_err() {
            break;
        }
        thread::sleep(refresh_interval);
    });
    receiver
}
