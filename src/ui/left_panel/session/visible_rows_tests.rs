use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::session::normal_terminal_session_info::normal_terminal_session_info;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::default_split_pane_session_group_state::default_split_pane_session_group_state;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::visible_rows::visible_session_rows_with_folders;

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

/// Verifies a folder row is followed by every session in that folder, with no FolderMore overflow.
#[test]
fn folder_rows_emit_all_sessions_without_more_row() {
    let entries = (0..20)
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
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows.len(), 21);
    assert!(matches!(
        rows[0],
        SessionListRow::Folder {
            current_session_count: 20,
            total_session_count: 20,
            ..
        }
    ));
    for (offset, row) in rows.iter().enumerate().skip(1) {
        assert_eq!(*row, SessionListRow::Session { index: offset - 1 });
    }
    assert!(!rows
        .iter()
}

/// Verifies a folder preserves the session-terminals order for its session rows.
#[test]
fn folder_rows_preserve_session_terminals_order_within_a_folder() {
    let entries = vec![
        session_entry(
            "2026-05-12T10:00:00Z",
            "First",
            "first",
            "/tmp/project",
        ),
        session_entry(
            "2026-05-12T11:00:00Z",
            "Second",
            "second",
            "/tmp/project",
        ),
        session_entry(
            "2026-05-12T12:00:00Z",
            "Third",
            "third",
            "/tmp/project",
        ),
    ];

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert!(matches!(rows[0], SessionListRow::Folder { .. }));
    assert_eq!(rows[1], SessionListRow::Session { index: 0 });
    assert_eq!(rows[2], SessionListRow::Session { index: 1 });
    assert_eq!(rows[3], SessionListRow::Session { index: 2 });
}

/// Verifies a new unpositioned session in folder B lands at the top of folder B.
#[test]
fn unpositioned_session_lands_at_top_of_its_folder() {
    let entries = vec![
        session_entry(
            "2026-05-12T10:00:00Z",
            "Alpha Older",
            "alpha-older",
            "/workspace/alpha",
        ),
        session_entry(
            "2026-05-12T11:00:00Z",
            "Beta Older",
            "beta-older",
            "/workspace/beta",
        ),
        session_entry(
            "2026-05-12T12:00:00Z",
            "Beta New",
            "beta-new",
            "/workspace/beta",
        ),
    ];

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[
            PathBuf::from("/workspace/alpha"),
            PathBuf::from("/workspace/beta"),
        ],
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert!(matches!(
        rows[0],
        SessionListRow::Folder { path, .. } if path == &PathBuf::from("/workspace/alpha")
    ));
    assert_eq!(rows[1], SessionListRow::Session { index: 0 });
    assert!(matches!(
        rows[2],
        SessionListRow::Folder { path, .. } if path == &PathBuf::from("/workspace/beta")
    ));
    assert_eq!(rows[3], SessionListRow::Session { index: 2 });
    assert_eq!(rows[4], SessionListRow::Session { index: 1 });
}

/// Verifies a running session partitions into its own folder based on working dir.
#[test]
fn running_session_lands_in_its_own_folder() {
    let mut entries = vec![
        session_entry(
            "2026-05-12T10:00:00Z",
            "Alpha Chat",
            "alpha-chat",
            "/workspace/alpha",
        ),
        session_entry(
            "2026-05-12T11:00:00Z",
            "Beta Chat",
            "beta-chat",
            "/workspace/beta",
        ),
    ];
    entries.push(running_session_entry(
        "Beta Running",
        "beta-running",
        "/workspace/beta",
    ));

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[
            PathBuf::from("/workspace/alpha"),
            PathBuf::from("/workspace/beta"),
        ],
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    let beta_folder_index = rows
        .iter()
        .position(|row| matches!(
            row,
            SessionListRow::Folder { path, .. } if path == &PathBuf::from("/workspace/beta")
        ))
        .expect("beta folder row");
    assert_eq!(
        rows[beta_folder_index + 1],
        SessionListRow::Session { index: 1 }
    );
    assert_eq!(
        rows[beta_folder_index + 2],
        SessionListRow::Session { index: 2 }
    );
    let beta_session_rows: Vec<&SessionListRow> = rows
        .iter()
        .filter(|row| {
            matches!(row, SessionListRow::Session { index } if *index == 2)
        })
        .collect();
    assert_eq!(beta_session_rows.len(), 1);
}

/// Verifies a collapsed folder only emits its folder row, hiding all session rows.
#[test]
fn collapsed_folder_emits_no_session_rows() {
    let entries = vec![
        session_entry(
            "2026-05-12T10:00:00Z",
            "One",
            "one",
            "/tmp/project",
        ),
        session_entry(
            "2026-05-12T11:00:00Z",
            "Two",
            "two",
            "/tmp/project",
        ),
    ];

    let mut collapsed = BTreeSet::new();
    collapsed.insert(PathBuf::from("/tmp/project"));
    let rows = visible_session_rows_with_folders(
        &entries,
        &collapsed,
        &[PathBuf::from("/tmp/project")],
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows.len(), 1);
    assert!(matches!(
        rows[0],
        SessionListRow::Folder { path, .. } if path == &PathBuf::from("/tmp/project")
    ));
}

/// Verifies a folder that is not in folder_order emits no rows.
#[test]
fn hidden_folder_emits_nothing() {
    let entries = vec![session_entry(
        "2026-05-12T10:00:00Z",
        "Hidden",
        "hidden",
        "/tmp/hidden",
    )];

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert!(rows.is_empty());
}

/// Verifies terminals beyond the previous display limit appear in their manual order.
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

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        Some(10),
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert!(matches!(rows[0], SessionListRow::Folder { .. }));
    for (offset, row) in rows.iter().enumerate().skip(1) {
        assert_eq!(*row, SessionListRow::Session { index: offset - 1 });
    }
    assert!(!rows
        .iter()
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

    let rows = visible_session_rows_with_folders(
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
    assert_eq!(folder_rows, 1);
    assert_eq!(rows[1], SessionListRow::Session { index: 0 });
    assert_eq!(rows[2], SessionListRow::Session { index: 1 });
}

/// Verifies chats beyond the previous display limit are visible without pinning.
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

    let rows = visible_session_rows_with_folders(
        &entries,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        Some(20),
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows.len(), 34);
    assert!(rows.contains(&SessionListRow::Session { index: 20 }));
    assert!(!rows
        .iter()
}
