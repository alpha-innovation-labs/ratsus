use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use ratkit::services::file_watcher::FileWatcher;

use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;
use crate::extensions::expo::observations::preview_request::ObservationPreviewRequest;
use crate::extensions::harness::core::chat_harness::ChatHarness;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::core::chat_session::ChatSessionKind;
use crate::extensions::harness::stub::data::stub_session::stub_session;
use crate::extensions::harness::stub::data::stub_sessions::stub_sessions;
use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// In-memory backend that never touches external commands or backend-owned data paths.
pub struct StubHarness {
    state: Arc<Mutex<StubHarnessState>>,
}

struct StubHarnessState {
    sessions: Vec<ChatSession>,
    next_id: usize,
    refresh_tick: usize,
}

impl StubHarness {
    /// Creates a deterministic stub harness with fake sessions across folders.
    pub fn new() -> Self {
        let sessions = stub_sessions();
        let next_id = sessions.len().saturating_add(1);
        Self {
            state: Arc::new(Mutex::new(StubHarnessState {
                sessions,
                next_id,
                refresh_tick: 0,
            })),
        }
    }
}

impl Default for StubHarness {
    /// Creates the default deterministic stub harness.
    fn default() -> Self {
        Self::new()
    }
}

impl ChatHarness for StubHarness {
    /// Returns the display name for the stub backend.
    fn display_name(&self) -> &'static str {
        "Stub"
    }

    /// Returns deterministic in-memory fake sessions.
    fn load_sessions(&self) -> Result<Vec<ChatSession>> {
        let state = self.state.lock().expect("stub harness state is available");
        Ok(state.sessions.clone())
    }

    /// Toggles fake running state without reading external registry files.
    fn refresh_sessions(&self) -> Result<Vec<ChatSession>> {
        let mut state = self.state.lock().expect("stub harness state is available");
        state.refresh_tick = state.refresh_tick.saturating_add(1);
        let tick = state.refresh_tick;
        for (index, session) in state.sessions.iter_mut().enumerate() {
            session.is_running = (tick + index) % 2 == 0;
        }
        Ok(state.sessions.clone())
    }

    /// Creates a new in-memory fake chat session with a fake terminal.
    fn spawn_new_chat(&self, working_dir: &Path, rows: u16, cols: u16) -> Result<SessionTerminal> {
        let mut state = self.state.lock().expect("stub harness state is available");
        let id = format!("stub-new-{}", state.next_id);
        state.next_id = state.next_id.saturating_add(1);
        let session = stub_session("now", "New stub chat", &id, working_dir.to_path_buf(), true);
        state.sessions.push(session.clone());
        Ok(stub_runtime(session, rows, cols))
    }

    /// Creates a fake terminal for an existing in-memory session.
    fn spawn_existing_chat(
        &self,
        session: &ChatSession,
        rows: u16,
        cols: u16,
    ) -> Result<ChatTerminal> {
        let mut terminal = stub_terminal(session);
        terminal.resize(rows, cols);
        Ok(terminal)
    }

    /// Creates a fake normal terminal without spawning a shell process.
    fn spawn_normal_terminal(
        &self,
        working_dir: &Path,
        rows: u16,
        cols: u16,
    ) -> Option<Result<SessionTerminal>> {
        let mut state = self.state.lock().expect("stub harness state is available");
        let id = format!("stub-terminal-{}", state.next_id);
        state.next_id = state.next_id.saturating_add(1);
        let session = ChatSession::new("now", "Stub terminal", id, working_dir.to_path_buf())
            .with_kind(ChatSessionKind::NormalTerminal);
        let mut terminal = ChatTerminal::stub(
            "Stub terminal",
            format!(
                "Stub terminal in {}. No shell process is running.",
                working_dir.display()
            ),
        );
        terminal.resize(rows, cols);
        Some(Ok(SessionTerminal::with_terminal(session, terminal)))
    }

    /// Disables persisted normal terminal loading in stub mode.
    fn load_normal_terminals_on_startup(&self) -> bool {
        false
    }

    /// Disables normal terminal persistence in stub mode.
    fn persist_normal_terminals(&self) -> bool {
        false
    }

    /// Deletes a fake chat session from memory.
    fn delete_chat_session(&self, session_id: &str) -> Result<()> {
        let mut state = self.state.lock().expect("stub harness state is available");
        state.sessions.retain(|session| session.id != session_id);
        Ok(())
    }

    /// Returns deterministic fake observation previews for requested sessions.
    fn load_observation_previews(
        &self,
        requests: Vec<ObservationPreviewRequest>,
    ) -> Result<HashMap<String, ConversationObservationPreview>> {
        let previews = requests
            .into_iter()
            .map(|request| {
                (
                    request.conversation_id,
                    ConversationObservationPreview {
                        has_more: false,
                        observations: vec![format!("Stub observation for {}", request.title)],
                    },
                )
            })
            .collect();
        Ok(previews)
    }

    /// Disables file watching in stub mode to avoid backend data paths.
    fn start_observation_watcher(&self) -> Option<FileWatcher> {
        None
    }
}

/// Builds a session runtime with a fake terminal implementation.
fn stub_runtime(session: ChatSession, rows: u16, cols: u16) -> SessionTerminal {
    let mut terminal = stub_terminal(&session);
    terminal.resize(rows, cols);
    SessionTerminal::with_terminal(session, terminal)
}

/// Builds a fake terminal for a stub session.
fn stub_terminal(session: &ChatSession) -> ChatTerminal {
    ChatTerminal::stub(
        format!("{} · stub", session.title),
        format!(
            "Stub backend session {} in {}. No external process is running.",
            session.id,
            session.working_dir.display()
        ),
    )
}
