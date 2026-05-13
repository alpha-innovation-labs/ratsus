use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the gamma stub folder.
pub fn gamma_stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-15T09:00:00Z",
            "Explore gamma project",
            "stub-24",
            "/tmp/ratsus-gamma",
            false,
        ),
        stub_session(
            "2026-05-15T10:00:00Z",
            "Plan gamma cleanup",
            "stub-25",
            "/tmp/ratsus-gamma",
            false,
        ),
        stub_session(
            "2026-05-15T11:00:00Z",
            "Review gamma logs",
            "stub-26",
            "/tmp/ratsus-gamma",
            false,
        ),
        stub_session(
            "2026-05-15T12:00:00Z",
            "Draft gamma docs",
            "stub-27",
            "/tmp/ratsus-gamma",
            false,
        ),
        stub_session(
            "2026-05-15T13:00:00Z",
            "Finalize gamma state",
            "stub-28",
            "/tmp/ratsus-gamma",
            false,
        ),
    ]
}
