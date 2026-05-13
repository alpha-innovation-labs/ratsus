use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the epsilon stub folder.
pub fn epsilon_stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-13T09:00:00Z",
            "Explore epsilon project",
            "stub-40",
            "/tmp/ratsus-epsilon",
            false,
        ),
        stub_session(
            "2026-05-13T10:00:00Z",
            "Finalize epsilon state",
            "stub-41",
            "/tmp/ratsus-epsilon",
            false,
        ),
    ]
}
