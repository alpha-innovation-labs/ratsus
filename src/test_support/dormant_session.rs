use crate::harness::chat_session::ChatSession;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds a dormant chat session fixture.
pub fn dormant_session(title: &str, id: &str, working_dir: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new("now", title, id, working_dir))
}
