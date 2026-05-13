use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::visible_session_rows::visible_session_rows;
use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::normal_terminal_session_info::normal_terminal_session_info;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds a dormant session entry for left-pane ordering tests.
fn session_entry(date: &str, title: &str, id: &str, working_dir: &str) -> SessionTerminal {
    SessionTerminal::dormant(NexusSession::new(date, title, id, working_dir))
}

/// Verifies visible rows preserve the manual/vector session order within a folder.
#[test]
fn preserves_manual_session_order() {
    let entries = vec![
        session_entry(
            "2026-05-11T10:00:00Z",
            "Manual First",
            "manual-first",
            "/tmp/project",
        ),
        session_entry(
            "2026-05-12T10:00:00Z",
            "Manual Second",
            "manual-second",
            "/tmp/project",
        ),
    ];

    let rows = visible_session_rows(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        None,
    );

    assert_eq!(rows[1], SessionListRow::Session { index: 0 });
    assert_eq!(rows[2], SessionListRow::Session { index: 1 });
}

/// Verifies the sidebar shows the first ten manually ordered sessions and excludes overflow entries.
#[test]
fn limits_folder_sessions_to_ten_manual_ordered_entries() {
    let entries = (0..11)
        .map(|index| {
            session_entry(
                "2026-05-12T10:00:00Z",
                &format!("Session {index}"),
                &format!("id-{index}"),
                "/tmp/project",
            )
        })
        .collect::<Vec<_>>();

    let rows = visible_session_rows(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        None,
    );

    assert_eq!(
        rows[0],
        SessionListRow::Folder {
            path: PathBuf::from("/tmp/project"),
            current_session_count: 10,
            total_session_count: 11,
        }
    );
    for index in 0..10 {
        assert_eq!(rows[index + 1], SessionListRow::Session { index });
    }
    assert_eq!(
        rows[11],
        SessionListRow::FolderMore {
            path: PathBuf::from("/tmp/project"),
        }
    );
}

/// Verifies a terminal inserted after the tenth visible chat still appears below it.
#[test]
fn shows_terminal_immediately_after_visible_limit() {
    let mut entries = (0..10)
        .map(|index| {
            session_entry(
                "2026-05-12T10:00:00Z",
                &format!("Session {index}"),
                &format!("id-{index}"),
                "/tmp/project",
            )
        })
        .collect::<Vec<_>>();
    entries.push(SessionTerminal::dormant(normal_terminal_session_info(
        "/tmp/project".as_ref(),
    )));
    entries.push(session_entry(
        "2026-05-12T10:11:00Z",
        "Overflow",
        "overflow",
        "/tmp/project",
    ));

    let rows = visible_session_rows(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        Some(10),
    );

    assert_eq!(
        rows[0],
        SessionListRow::Folder {
            path: PathBuf::from("/tmp/project"),
            current_session_count: 11,
            total_session_count: 12,
        }
    );
    assert_eq!(rows[10], SessionListRow::Session { index: 9 });
    assert_eq!(rows[11], SessionListRow::Session { index: 10 });
    assert_eq!(
        rows[12],
        SessionListRow::FolderMore {
            path: PathBuf::from("/tmp/project"),
        }
    );
}

/// Verifies the active chat remains visible even when it is beyond the folder display limit.
#[test]
fn shows_pinned_active_chat_beyond_visible_limit() {
    let entries = (0..33)
        .map(|index| {
            session_entry(
                "2026-05-12T10:00:00Z",
                &format!("Session {index}"),
                &format!("id-{index}"),
                "/tmp/project",
            )
        })
        .collect::<Vec<_>>();

    let rows = visible_session_rows(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        Some(20),
    );

    assert_eq!(
        rows[0],
        SessionListRow::Folder {
            path: PathBuf::from("/tmp/project"),
            current_session_count: 11,
            total_session_count: 33,
        }
    );
    assert!(rows.contains(&SessionListRow::Session { index: 20 }));
    assert_eq!(
        rows.last(),
        Some(&SessionListRow::FolderMore {
            path: PathBuf::from("/tmp/project"),
        })
    );
}
