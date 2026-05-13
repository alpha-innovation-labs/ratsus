use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the alpha stub folder.
pub fn alpha_stub_sessions() -> Vec<ChatSession> {
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
            "2026-05-13T12:00:00Z",
            "Design pane navigation",
            "stub-4",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T13:00:00Z",
            "Audit key behavior",
            "stub-5",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T14:00:00Z",
            "Prototype session filter",
            "stub-6",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T15:00:00Z",
            "Trace refresh worker",
            "stub-7",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T16:00:00Z",
            "Validate split focus",
            "stub-8",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T17:00:00Z",
            "Document list hotkeys",
            "stub-9",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T18:00:00Z",
            "Review deletion flow",
            "stub-10",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T19:00:00Z",
            "Tune render snapshots",
            "stub-11",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T20:00:00Z",
            "Polish picker layout",
            "stub-12",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-13T21:00:00Z",
            "Review observation cache",
            "stub-13",
            "/tmp/ratsus-alpha",
            false,
        ),
    ]
}
