use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the epsilon stub folder.
pub fn epsilon_stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-17T09:00:00Z",
            "Explore epsilon project",
            "stub-33",
            "/tmp/ratsus-epsilon",
            false,
        ),
        stub_session(
            "2026-05-17T10:00:00Z",
            "Finalize epsilon state",
            "stub-34",
            "/tmp/ratsus-epsilon",
            false,
        ),
    ]
}
