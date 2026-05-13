use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds a dormant Nexus session fixture.
pub fn dormant_session(title: &str, id: &str, working_dir: &str) -> SessionTerminal {
    SessionTerminal::dormant(NexusSession::new("now", title, id, working_dir))
}
