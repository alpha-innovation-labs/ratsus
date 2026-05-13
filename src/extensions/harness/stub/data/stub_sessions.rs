use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::alpha_stub_sessions::alpha_stub_sessions;
use crate::extensions::harness::stub::data::beta_stub_sessions::beta_stub_sessions;
use crate::extensions::harness::stub::data::delta_stub_sessions::delta_stub_sessions;
use crate::extensions::harness::stub::data::epsilon_stub_sessions::epsilon_stub_sessions;
use crate::extensions::harness::stub::data::gamma_stub_sessions::gamma_stub_sessions;

/// Builds the deterministic initial fake session catalog.
pub fn stub_sessions() -> Vec<ChatSession> {
    let mut sessions = Vec::new();
    sessions.extend(alpha_stub_sessions());
    sessions.extend(beta_stub_sessions());
    sessions.extend(gamma_stub_sessions());
    sessions.extend(delta_stub_sessions());
    sessions.extend(epsilon_stub_sessions());
    sessions
}
