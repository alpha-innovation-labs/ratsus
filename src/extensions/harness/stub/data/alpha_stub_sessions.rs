use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the alpha stub folder.
pub fn alpha_stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-17T09:00:00Z",
            "Plan Rust refactor",
            "stub-1",
            "/tmp/ratsus-alpha",
            true,
        ),
        stub_session(
            "2026-05-17T10:00:00Z",
            "Review terminal UI",
            "stub-2",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T11:00:00Z",
            "Design pane navigation",
            "stub-3",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T12:00:00Z",
            "Audit key behavior",
            "stub-4",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T13:00:00Z",
            "Prototype session filter",
            "stub-5",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T14:00:00Z",
            "Trace refresh worker",
            "stub-6",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T15:00:00Z",
            "Validate split focus",
            "stub-7",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T16:00:00Z",
            "Document list hotkeys",
            "stub-8",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T17:00:00Z",
            "Review deletion flow",
            "stub-9",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T18:00:00Z",
            "Tune render snapshots",
            "stub-10",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T19:00:00Z",
            "Polish picker layout",
            "stub-11",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T20:00:00Z",
            "Review observation cache",
            "stub-12",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T21:00:00Z",
            "Map left panel order",
            "stub-13",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T22:00:00Z",
            "Check folder pinning",
            "stub-14",
            "/tmp/ratsus-alpha",
            false,
        ),
        stub_session(
            "2026-05-17T23:00:00Z",
            "Finalize alpha demo",
            "stub-15",
            "/tmp/ratsus-alpha",
            false,
        ),
    ]
}
