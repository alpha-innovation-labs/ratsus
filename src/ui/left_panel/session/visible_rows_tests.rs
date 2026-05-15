use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::session::normal_terminal_session_info::normal_terminal_session_info;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::default_split_pane_session_group_state::default_split_pane_session_group_state;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::visible_rows::{
    visible_session_rows, visible_session_rows_with_folders,
};

/// Builds a dormant session entry for left-pane ordering tests.
fn session_entry(date: &str, title: &str, id: &str, working_dir: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new(date, title, id, working_dir))
}

/// Builds a running session entry for left-pane visibility tests.
fn running_session_entry(title: &str, id: &str, working_dir: &str) -> SessionTerminal {
    SessionTerminal::dormant(
        ChatSession::new("2026-05-12T10:00:00Z", title, id, working_dir).with_running(true),
    )
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
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows[0], SessionListRow::Session { index: 0 });
    assert_eq!(rows[1], SessionListRow::Session { index: 1 });
}

/// Verifies the sidebar shows every session without a folder header or overflow row.
#[test]
fn shows_all_folder_sessions_without_more_row() {
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
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows.len(), 11);
    assert!(!rows
        .iter()
        .any(|row| matches!(row, SessionListRow::Folder { .. })));
    assert!(!rows
        .iter()
        .any(|row| matches!(row, SessionListRow::FolderMore { .. })));
    for (index, row) in rows.iter().enumerate().take(11) {
        assert_eq!(*row, SessionListRow::Session { index });
    }
}

/// Verifies terminals beyond the old display limit appear in their manual order.
#[test]
fn shows_terminal_in_full_manual_order() {
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
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows[9], SessionListRow::Session { index: 9 });
    assert_eq!(rows[10], SessionListRow::Session { index: 10 });
    assert_eq!(rows[11], SessionListRow::Session { index: 11 });
}

/// Verifies legacy folder rows keep the old ten-row overflow behavior.
#[test]
fn legacy_folder_rows_show_more_after_ten_sessions() {
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

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        Some(0),
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert!(matches!(
        rows[0],
        SessionListRow::Folder {
            current_session_count: 10,
            total_session_count: 11,
            ..
        }
    ));
    assert_eq!(rows[10], SessionListRow::Session { index: 9 });
    assert!(matches!(rows[11], SessionListRow::FolderMore { .. }));
}

/// Verifies legacy folder rows retain focused and running sessions beyond the limit.
#[test]
fn legacy_folder_rows_include_focused_and_running_sessions_after_limit() {
    let mut entries = (0..12)
        .map(|index| {
            session_entry(
                "2026-05-12T10:00:00Z",
                &format!("Session {index}"),
                &format!("id-{index}"),
                "/tmp/project",
            )
        })
        .collect::<Vec<_>>();
    entries.push(running_session_entry("Running", "running", "/tmp/project"));

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        Some(11),
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert!(rows.contains(&SessionListRow::Session { index: 11 }));
    assert!(rows.contains(&SessionListRow::Session { index: 12 }));
    assert!(matches!(
        rows.last(),
        Some(SessionListRow::FolderMore { .. })
    ));
}

/// Verifies duplicate folder-order entries render one group of session rows.
#[test]
fn duplicate_folder_order_renders_one_session_group() {
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
        &[PathBuf::from("/tmp/project"), PathBuf::from("/tmp/project")],
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    let folder_rows = rows
        .iter()
        .filter(|row| matches!(row, SessionListRow::Folder { .. }))
        .count();
    assert_eq!(folder_rows, 0);
    assert_eq!(rows[0], SessionListRow::Session { index: 0 });
    assert_eq!(rows[1], SessionListRow::Session { index: 1 });
}

/// Verifies chats beyond the old display limit are visible without pinning.
#[test]
fn shows_chat_beyond_old_visible_limit() {
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
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows.len(), 33);
    assert!(rows.contains(&SessionListRow::Session { index: 20 }));
    assert!(!rows
        .iter()
        .any(|row| matches!(row, SessionListRow::FolderMore { .. })));
}
