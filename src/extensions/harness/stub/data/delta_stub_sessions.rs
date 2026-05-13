use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the delta stub folder.
pub fn delta_stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-16T09:00:00Z",
            "Explore delta project",
            "stub-29",
            "/tmp/ratsus-delta",
            false,
        ),
        stub_session(
            "2026-05-16T10:00:00Z",
            "Plan delta cleanup",
            "stub-30",
            "/tmp/ratsus-delta",
            false,
        ),
        stub_session(
            "2026-05-16T11:00:00Z",
            "Review delta logs",
            "stub-31",
            "/tmp/ratsus-delta",
            false,
        ),
        stub_session(
            "2026-05-16T12:00:00Z",
            "Finalize delta state",
            "stub-32",
            "/tmp/ratsus-delta",
            false,
        ),
    ]
}
