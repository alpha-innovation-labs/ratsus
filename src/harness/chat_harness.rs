use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use ratkit::services::file_watcher::FileWatcher;

use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::expo::observation_preview_request::ObservationPreviewRequest;
use crate::harness::chat_session::ChatSession;
use crate::terminal::session_terminal::SessionTerminal;

/// Defines the backend operations the UI needs from a chat harness.
pub trait ChatHarness: Send + Sync {
    /// Returns the human-readable harness name for UI labels and diagnostics.
    fn display_name(&self) -> &'static str;

    /// Loads the initial catalog of chat sessions for the app.
    fn load_sessions(&self) -> Result<Vec<ChatSession>>;

    /// Refreshes lightweight session metadata such as running status.
    fn refresh_sessions(&self) -> Result<Vec<ChatSession>>;

    /// Spawns a new chat runtime in the requested working directory.
    fn spawn_new_chat(&self, working_dir: &Path, rows: u16, cols: u16) -> Result<SessionTerminal>;

    /// Deletes one backend-owned chat session.
    fn delete_chat_session(&self, session_id: &str) -> Result<()>;

    /// Loads observation previews for the requested sessions.
    fn load_observation_previews(
        &self,
        requests: Vec<ObservationPreviewRequest>,
    ) -> Result<HashMap<String, ConversationObservationPreview>>;

    /// Starts an optional observation watcher for the backend.
    fn start_observation_watcher(&self) -> Option<FileWatcher>;
}
