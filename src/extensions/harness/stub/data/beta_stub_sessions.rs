use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::stub::data::stub_session::stub_session;

/// Builds deterministic fake sessions for the beta stub folder.
pub fn beta_stub_sessions() -> Vec<ChatSession> {
    vec![
        stub_session(
            "2026-05-16T09:00:00Z",
            "Write docs",
            "stub-16",
            "/tmp/ratsus-beta",
            true,
        ),
        stub_session(
            "2026-05-16T10:00:00Z",
            "Map backend adapter",
            "stub-17",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T11:00:00Z",
            "Check terminal resize",
            "stub-18",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T12:00:00Z",
            "Plan picker grouping",
            "stub-19",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T13:00:00Z",
            "Review dormant restore",
            "stub-20",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T14:00:00Z",
            "Write refresh tests",
            "stub-21",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T15:00:00Z",
            "Debug mouse handling",
            "stub-22",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T16:00:00Z",
            "Tune status labels",
            "stub-23",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T17:00:00Z",
            "Review modal borders",
            "stub-24",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T18:00:00Z",
            "Draft release notes",
            "stub-25",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T19:00:00Z",
            "Check app lifecycle",
            "stub-26",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T20:00:00Z",
            "Polish beta fixtures",
            "stub-27",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T21:00:00Z",
            "Validate beta sorting",
            "stub-28",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T22:00:00Z",
            "Review beta filters",
            "stub-29",
            "/tmp/ratsus-beta",
            false,
        ),
        stub_session(
            "2026-05-16T23:00:00Z",
            "Finalize beta demo",
            "stub-30",
            "/tmp/ratsus-beta",
            false,
        ),
    ]
}
