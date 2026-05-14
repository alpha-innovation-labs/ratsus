use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use ratkit::services::file_watcher::FileWatcher;

use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;
use crate::extensions::expo::observations::preview_request::ObservationPreviewRequest;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Defines the backend operations the UI needs from a chat harness.
pub trait ChatHarness: Send + Sync {
    /// Returns the human-readable harness name for UI labels and diagnostics.
    fn display_name(&self) -> &'static str;

    /// Loads the initial catalog of chat sessions for the app.
    fn load_sessions(&self) -> Result<Vec<ChatSession>>;

    /// Refreshes lightweight session metadata such as running status.
    fn refresh_sessions(&self) -> Result<Vec<ChatSession>>;

    /// Starts an optional session metadata watcher for the backend.
    fn start_session_watcher(&self) -> Option<FileWatcher> {
        None
    }

    /// Returns whether a watched changed path should refresh session metadata.
    fn is_session_update_path(&self, _path: &Path) -> bool {
        false
    }

    /// Spawns a new chat runtime in the requested working directory.
    fn spawn_new_chat(&self, working_dir: &Path, rows: u16, cols: u16) -> Result<SessionTerminal>;

    /// Spawns or creates a terminal for an existing chat session.
    fn spawn_existing_chat(
        &self,
        session: &ChatSession,
        rows: u16,
        cols: u16,
    ) -> Result<ChatTerminal>;

    /// Optionally handles creation of a normal terminal for this harness mode.
    fn spawn_normal_terminal(
        &self,
        _working_dir: &Path,
        _rows: u16,
        _cols: u16,
    ) -> Option<Result<SessionTerminal>> {
        None
    }

    /// Returns whether normal terminal sessions should be loaded at startup for this mode.
    fn load_normal_terminals_on_startup(&self) -> bool {
        true
    }

    /// Returns whether normal terminal sessions should be persisted for this mode.
    fn persist_normal_terminals(&self) -> bool {
        true
    }

    /// Deletes one backend-owned chat session.
    fn delete_chat_session(&self, session_id: &str) -> Result<()>;

    /// Applies refreshed session metadata and returns whether visible state changed.
    fn merge_session_refresh(
        &self,
        session_terminals: &mut [SessionTerminal],
        refreshed_sessions: Vec<ChatSession>,
    ) -> bool {
        merge_sessions_by_id(session_terminals, refreshed_sessions)
    }

    /// Loads observation previews for the requested sessions.
    fn load_observation_previews(
        &self,
        requests: Vec<ObservationPreviewRequest>,
    ) -> Result<HashMap<String, ConversationObservationPreview>>;

    /// Starts an optional observation watcher for the backend.
    fn start_observation_watcher(&self) -> Option<FileWatcher>;

    /// Returns whether a watched changed path should refresh observation previews.
    fn is_observation_state_path(&self, _path: &Path) -> bool {
        false
    }
}

/// Applies refreshed session data by stable id with backend-neutral replacement semantics.
fn merge_sessions_by_id(
    session_terminals: &mut [SessionTerminal],
    refreshed_sessions: Vec<ChatSession>,
) -> bool {
    let mut changed = false;
    for refreshed_session in refreshed_sessions {
        let Some(index) = session_terminals
            .iter()
            .position(|entry| entry.session.id == refreshed_session.id)
        else {
            continue;
        };
        if session_terminals[index].session != refreshed_session {
            session_terminals[index].session = refreshed_session;
            changed = true;
        }
    }
    changed
}
