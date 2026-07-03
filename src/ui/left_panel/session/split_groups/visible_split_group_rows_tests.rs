use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroup;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::visible_rows::visible_session_rows_with_folders;

/// Verifies one unsplit session still renders as a flat left-panel session row.
#[test]
fn unsplit_session_stays_flat() {
    let sessions = vec![session_entry("Alpha", "a")];

    let rows = visible_session_rows_with_folders(
        &sessions,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        None,
        &empty_groups(),
        &BTreeMap::new(),
    );

    assert_eq!(rows[0], SessionListRow::Session { index: 0 });
}

/// Verifies split pane sessions render under a shared group parent row.
#[test]
fn split_sessions_render_under_group_parent() {
    let sessions = vec![session_entry("Alpha", "a"), session_entry("Beta", "b")];
    let groups = group_state(vec![1, 2]);
    let bundles = BTreeMap::from([(1, vec!["a".to_string()]), (2, vec!["b".to_string()])]);

    let rows = visible_session_rows_with_folders(
        &sessions,
        &BTreeSet::new(),
        &[PathBuf::from("/tmp/project")],
        None,
        &groups,
        &bundles,
    );

    assert_eq!(
        rows[0],
        SessionListRow::SplitGroup {
            group_id: 1,
            name: "Group 1".to_string(),
            child_count: 2,
        }
    );
    assert_eq!(
        rows[1],
        SessionListRow::SplitGroupChild {
            group_id: 1,
            pane_id: 1,
            index: 0,
            is_last: false,
        }
    );
    assert_eq!(
        rows[2],
        SessionListRow::SplitGroupChild {
            group_id: 1,
            pane_id: 2,
            index: 1,
            is_last: true,
        }
    );
}

/// Builds a dormant test session in the shared project folder.
fn session_entry(title: &str, id: &str) -> SessionTerminal {
    SessionTerminal::dormant(ChatSession::new(
        "2026-05-12T10:00:00Z",
        title,
        id,
        "/tmp/project",
    ))
}

/// Builds empty split-group state for flat row tests.
fn empty_groups() -> SplitPaneSessionGroupState {
    SplitPaneSessionGroupState {
        groups: BTreeMap::new(),
        next_id: 1,
    }
}

/// Builds one split group with the requested panes.
fn group_state(panes: Vec<u32>) -> SplitPaneSessionGroupState {
    SplitPaneSessionGroupState {
        groups: BTreeMap::from([(
            1,
            SplitPaneSessionGroup {
                id: 1,
                name: "Group 1".to_string(),
                panes,
            },
        )]),
        next_id: 2,
    }
}
