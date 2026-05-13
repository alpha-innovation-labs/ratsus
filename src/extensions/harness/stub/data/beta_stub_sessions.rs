use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the beta stub folder.
pub fn beta_stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-13T11:00:00Z",
            "Write docs",
            "stub-3",
            "/tmp/ratsus-beta",
            true,
        ),
        stub_session(
            "2026-05-14T09:00:00Z",
            "Map backend adapter",
            "stub-14",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T10:00:00Z",
            "Check terminal resize",
            "stub-15",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T11:00:00Z",
            "Plan picker grouping",
            "stub-16",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T12:00:00Z",
            "Review dormant restore",
            "stub-17",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T13:00:00Z",
            "Write refresh tests",
            "stub-18",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T14:00:00Z",
            "Debug mouse handling",
            "stub-19",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T15:00:00Z",
            "Tune status labels",
            "stub-20",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T16:00:00Z",
            "Review modal borders",
            "stub-21",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T17:00:00Z",
            "Draft release notes",
            "stub-22",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-14T18:00:00Z",
            "Check app lifecycle",
            "stub-23",
            "/tmp/ratsus-beta",
            false,
        ),
    ]
}
