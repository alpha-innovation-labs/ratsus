use std::path::PathBuf;

use crate::nexus_sessions::apply_session_refresh::apply_session_refresh;
use crate::nexus_sessions::new_nexus_chat_session::new_nexus_chat_session;
use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::session_terminal::SessionTerminal;

/// Verifies refreshed metadata updates a session by stable id.
#[test]
fn updates_existing_session_title_by_id() {
    let mut entries = vec![SessionTerminal::dormant(NexusSession::new(
        "old-date",
        "Old title",
        "session-1",
        "/tmp/project",
    ))];

    let changed = apply_session_refresh(
        &mut entries,
        vec![NexusSession::new(
            "new-date",
            "Current title",
            "session-1",
            "/tmp/project",
        )],
    );

    assert!(changed);
    assert_eq!(entries[0].session.title, "Current title");
    assert_eq!(entries[0].session.date, "new-date");
}

/// Verifies a locally spawned new chat adopts its real Nexus id and title.
#[test]
fn replaces_new_chat_placeholder_by_working_dir() {
    let working_dir = PathBuf::from("/tmp/project");
    let mut entries = vec![SessionTerminal::dormant(new_nexus_chat_session(
        &working_dir,
    ))];

    let changed = apply_session_refresh(
        &mut entries,
        vec![NexusSession::new(
            "new-date",
            "Generated title",
            "real-session-id",
            &working_dir,
        )],
    );

    assert!(changed);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].session.id, "real-session-id");
    assert_eq!(entries[0].session.title, "Generated title");
}

/// Verifies registry running status is applied to matching sessions.
#[test]
fn updates_running_status_by_id() {
    let mut entries = vec![SessionTerminal::dormant(NexusSession::new(
        "date-1",
        "Existing",
        "session-1",
        "/tmp/project",
    ))];

    let changed = apply_session_refresh(
        &mut entries,
        vec![NexusSession::new("date-2", "Existing", "session-1", "/tmp/project")
            .with_running(true)],
    );

    assert!(changed);
    assert!(entries[0].session.is_running);
}

/// Verifies sessions missing from a registry refresh are marked inactive.
#[test]
fn clears_running_status_when_session_leaves_registry() {
    let mut entries = vec![SessionTerminal::dormant(
        NexusSession::new("date-1", "Existing", "session-1", "/tmp/project").with_running(true),
    )];

    let changed = apply_session_refresh(&mut entries, Vec::new());

    assert!(changed);
    assert!(!entries[0].session.is_running);
}

/// Verifies unseen refreshed sessions are appended without disturbing existing indexes.
#[test]
fn appends_unseen_refreshed_session() {
    let mut entries = vec![SessionTerminal::dormant(NexusSession::new(
        "date-1",
        "Existing",
        "session-1",
        "/tmp/project",
    ))];

    let changed = apply_session_refresh(
        &mut entries,
        vec![NexusSession::new(
            "date-2",
            "New session",
            "session-2",
            "/tmp/project",
        )],
    );

    assert!(changed);
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].session.id, "session-1");
    assert_eq!(entries[1].session.id, "session-2");
}
