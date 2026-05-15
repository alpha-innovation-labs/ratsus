use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread;

use crate::app::deletion::delete_sessions_result::DeleteSessionsResult;
use crate::extensions::harness::core::chat_harness::ChatHarness;

/// Spawns serialized deletion of chat sessions on a background thread.
pub fn spawn_delete_sessions_worker(
    chat_harness: Arc<dyn ChatHarness>,
    chat_session_ids: Vec<String>,
) -> Receiver<DeleteSessionsResult> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let result = delete_chat_sessions_serially(chat_harness, chat_session_ids);
        let _ = sender.send(result);
    });
    receiver
}

/// Deletes chat sessions one at a time and returns all outcomes.
fn delete_chat_sessions_serially(
    chat_harness: Arc<dyn ChatHarness>,
    chat_session_ids: Vec<String>,
) -> DeleteSessionsResult {
    let mut result = DeleteSessionsResult::default();
    for session_id in chat_session_ids {
        collect_delete_result(
            delete_one_chat_session(chat_harness.as_ref(), session_id),
            &mut result,
        );
    }
    result
}

/// Deletes one chat session through the active harness.
fn delete_one_chat_session(
    chat_harness: &dyn ChatHarness,
    session_id: String,
) -> Result<String, (String, String)> {
    match chat_harness.delete_chat_session(&session_id) {
        Ok(()) => Ok(session_id),
        Err(error) => {
            let message = format!("Failed to delete session {session_id}: {error}");
            Err((session_id, message))
        }
    }
}

/// Adds one delete result into the aggregate result.
fn collect_delete_result(
    delete_result: Result<String, (String, String)>,
    result: &mut DeleteSessionsResult,
) {
    match delete_result {
        Ok(session_id) => result.deleted_chat_ids.push(session_id),
        Err((session_id, error)) => {
            result.failed_chat_ids.push(session_id);
            result.errors.push(error);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    use anyhow::Result;
    use ratkit::services::file_watcher::FileWatcher;

    use super::spawn_delete_sessions_worker;
    use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;
    use crate::extensions::expo::observations::preview_request::ObservationPreviewRequest;
    use crate::extensions::harness::core::chat_harness::ChatHarness;
    use crate::extensions::harness::core::chat_session::ChatSession;
    use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
    use crate::extensions::terminal::session::session_terminal::SessionTerminal;

    struct ConcurrencyTrackingHarness {
        active_calls: AtomicUsize,
        concurrent_call_seen: AtomicBool,
    }

    impl ConcurrencyTrackingHarness {
        /// Creates a harness that records overlapping delete calls.
        fn new() -> Self {
            Self {
                active_calls: AtomicUsize::new(0),
                concurrent_call_seen: AtomicBool::new(false),
            }
        }
    }

    impl ChatHarness for ConcurrencyTrackingHarness {
        /// Returns a test backend display name.
        fn display_name(&self) -> &'static str {
            "Concurrency test"
        }

        /// Unused by delete worker tests.
        fn load_sessions(&self) -> Result<Vec<ChatSession>> {
            unimplemented!("delete worker test does not load sessions")
        }

        /// Unused by delete worker tests.
        fn refresh_sessions(&self) -> Result<Vec<ChatSession>> {
            unimplemented!("delete worker test does not refresh sessions")
        }

        /// Unused by delete worker tests.
        fn spawn_new_chat(
            &self,
            _working_dir: &Path,
            _rows: u16,
            _cols: u16,
        ) -> Result<SessionTerminal> {
            unimplemented!("delete worker test does not spawn chats")
        }

        /// Unused by delete worker tests.
        fn spawn_existing_chat(
            &self,
            _session: &ChatSession,
            _rows: u16,
            _cols: u16,
        ) -> Result<ChatTerminal> {
            unimplemented!("delete worker test does not spawn existing chats")
        }

        /// Records whether delete calls overlap.
        fn delete_chat_session(&self, _session_id: &str) -> Result<()> {
            if self.active_calls.fetch_add(1, Ordering::SeqCst) > 0 {
                self.concurrent_call_seen.store(true, Ordering::SeqCst);
            }
            thread::sleep(Duration::from_millis(25));
            self.active_calls.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        }

        /// Unused by delete worker tests.
        fn load_observation_previews(
            &self,
            _requests: Vec<ObservationPreviewRequest>,
        ) -> Result<HashMap<String, ConversationObservationPreview>> {
            unimplemented!("delete worker test does not load observations")
        }

        /// Unused by delete worker tests.
        fn start_observation_watcher(&self) -> Option<FileWatcher> {
            None
        }
    }

    /// Verifies bulk deletion serializes backend delete operations.
    #[test]
    fn deletes_chat_sessions_without_overlapping_harness_calls() {
        let harness = Arc::new(ConcurrencyTrackingHarness::new());
        let receiver = spawn_delete_sessions_worker(
            harness.clone(),
            vec!["one".to_string(), "two".to_string(), "three".to_string()],
        );

        let result = receiver
            .recv_timeout(Duration::from_secs(1))
            .expect("delete worker returns a result");

        assert_eq!(result.deleted_chat_ids.len(), 3);
        assert!(!harness.concurrent_call_seen.load(Ordering::SeqCst));
    }
}
