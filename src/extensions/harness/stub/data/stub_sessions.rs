use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds the deterministic initial fake session catalog.
pub fn stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-13T09:00:00Z",
            "Plan Rust refactor",
            "stub-1",
            "/tmp/ratsus-alpha",
            true,
        ),
        stub_session(
            "2026-05-13T10:00:00Z",
            "Review terminal UI",
            "stub-2",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T11:00:00Z",
            "Write docs",
            "stub-3",
            "/tmp/ratsus-beta",
            false,
        ),
    ]
}
