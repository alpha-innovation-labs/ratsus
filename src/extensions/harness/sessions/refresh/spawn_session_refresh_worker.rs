use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;

use anyhow::Result;

use crate::extensions::harness::core::chat_harness::ChatHarness;
use crate::extensions::harness::core::chat_session::ChatSession;

/// Spawns a background worker that refreshes session metadata off the UI thread.
pub fn spawn_session_refresh_worker(
    chat_harness: Arc<dyn ChatHarness>,
) -> Receiver<Result<Vec<ChatSession>>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let _ = sender.send(chat_harness.refresh_sessions());
    });
    receiver
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;
    use std::sync::Arc;
    use std::thread;
    use std::time::{Duration, Instant};

    use anyhow::{bail, Result};
    use ratkit::services::file_watcher::FileWatcher;

    use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;
    use crate::extensions::expo::observations::preview_request::ObservationPreviewRequest;
    use crate::extensions::harness::core::chat_harness::ChatHarness;
    use crate::extensions::harness::core::chat_session::ChatSession;
    use crate::extensions::harness::sessions::refresh::spawn_session_refresh_worker::spawn_session_refresh_worker;
    use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
    use crate::extensions::terminal::session::session_terminal::SessionTerminal;

    struct SlowRefreshHarness;

    impl ChatHarness for SlowRefreshHarness {
        /// Returns the test harness name.
        fn display_name(&self) -> &'static str {
            "slow"
        }

        /// Loads no startup sessions for this worker test.
        fn load_sessions(&self) -> Result<Vec<ChatSession>> {
            Ok(Vec::new())
        }

        /// Simulates a slow backend refresh that must not block the caller.
        fn refresh_sessions(&self) -> Result<Vec<ChatSession>> {
            thread::sleep(Duration::from_millis(120));
            Ok(vec![ChatSession::new("now", "ready", "id", "/tmp")])
        }

        /// This test harness does not start a watcher.
        fn start_session_watcher(&self) -> Option<FileWatcher> {
            None
        }

        /// This test harness never matches watcher paths.
        fn is_session_update_path(&self, _path: &Path) -> bool {
            false
        }

        /// Chat spawning is outside this worker test.
        fn spawn_new_chat(
            &self,
            _working_dir: &Path,
            _rows: u16,
            _cols: u16,
        ) -> Result<SessionTerminal> {
            bail!("not used")
        }

        /// Chat resuming is outside this worker test.
        fn spawn_existing_chat(
            &self,
            _session: &ChatSession,
            _rows: u16,
            _cols: u16,
        ) -> Result<ChatTerminal> {
            bail!("not used")
        }

        /// Deletion is outside this worker test.
        fn delete_chat_session(&self, _session_id: &str) -> Result<()> {
            bail!("not used")
        }

        /// Observation loading is outside this worker test.
        fn load_observation_previews(
            &self,
            _requests: Vec<ObservationPreviewRequest>,
        ) -> Result<HashMap<String, ConversationObservationPreview>> {
            Ok(HashMap::new())
        }

        /// This test harness does not start an observation watcher.
        fn start_observation_watcher(&self) -> Option<FileWatcher> {
            None
        }
    }

    /// Verifies slow session refreshes run off-thread instead of blocking tick handling.
    #[test]
    fn returns_before_slow_refresh_finishes() -> Result<()> {
        let started = Instant::now();
        let receiver = spawn_session_refresh_worker(Arc::new(SlowRefreshHarness));

        assert!(started.elapsed() < Duration::from_millis(50));
        assert!(receiver.try_recv().is_err());
        let refreshed = receiver.recv_timeout(Duration::from_secs(1))??;
        assert_eq!(refreshed[0].title, "ready");
        Ok(())
    }
}
