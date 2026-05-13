use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use ratkit::services::file_watcher::FileWatcher;

use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::expo::observation_preview_request::ObservationPreviewRequest;
use crate::harness::chat_harness::ChatHarness;
use crate::harness::chat_session::ChatSession;
use crate::harnesses::nexus::apply_session_refresh::apply_session_refresh;
use crate::harnesses::nexus::delete_nexus_session::delete_nexus_session;
use crate::harnesses::nexus::load_chat_sessions::load_chat_sessions;
use crate::harnesses::nexus::load_nexus_session_registry::load_nexus_session_registry;
use crate::harnesses::nexus::observations::load_observation_previews::load_observation_previews;
use crate::harnesses::nexus::observations::path_is_observation_state::path_is_observation_state;
use crate::harnesses::nexus::observations::start_observation_watcher::start_observation_watcher;
use crate::harnesses::nexus::spawn_new_nexus_session_terminal::spawn_new_nexus_session_terminal;
use crate::terminal::chat_terminal::ChatTerminal;
use crate::terminal::session_terminal::SessionTerminal;

/// Harness implementation that delegates to the real Nexus CLI and data files.
pub struct NexusHarness;

impl ChatHarness for NexusHarness {
    /// Returns the display name for the real Nexus backend.
    fn display_name(&self) -> &'static str {
        "Nexus"
    }

    /// Loads stable Nexus sessions through the Nexus CLI.
    fn load_sessions(&self) -> Result<Vec<ChatSession>> {
        Ok(load_chat_sessions()?)
    }

    /// Loads Nexus registry metadata used for periodic refreshes.
    fn refresh_sessions(&self) -> Result<Vec<ChatSession>> {
        Ok(load_nexus_session_registry()?)
    }

    /// Spawns a fresh Nexus CLI chat in a PTY.
    fn spawn_new_chat(&self, working_dir: &Path, rows: u16, cols: u16) -> Result<SessionTerminal> {
        spawn_new_nexus_session_terminal(working_dir, rows, cols)
    }

    /// Spawns an existing Nexus chat session through `nexus --resume`.
    fn spawn_existing_chat(
        &self,
        session: &ChatSession,
        rows: u16,
        cols: u16,
    ) -> Result<ChatTerminal> {
        ChatTerminal::spawn_with_command_in_dir(
            "nexus",
            &["--resume", session.id.as_str()],
            &session.working_dir,
            rows.max(1),
            cols.max(1),
        )
    }

    /// Deletes a Nexus chat session through the Nexus CLI.
    fn delete_chat_session(&self, session_id: &str) -> Result<()> {
        Ok(delete_nexus_session(session_id)?)
    }

    /// Applies Nexus registry refresh semantics.
    fn merge_session_refresh(
        &self,
        session_terminals: &mut [SessionTerminal],
        refreshed_sessions: Vec<ChatSession>,
    ) -> bool {
        apply_session_refresh(session_terminals, refreshed_sessions)
    }

    /// Loads Nexus observation previews from Nexus observation state files.
    fn load_observation_previews(
        &self,
        requests: Vec<ObservationPreviewRequest>,
    ) -> Result<HashMap<String, ConversationObservationPreview>> {
        Ok(load_observation_previews(requests)?)
    }

    /// Starts the Nexus observation file watcher.
    fn start_observation_watcher(&self) -> Option<FileWatcher> {
        start_observation_watcher()
    }

    /// Detects Nexus observation state files from watcher changes.
    fn is_observation_state_path(&self, path: &Path) -> bool {
        path_is_observation_state(path)
    }
}
