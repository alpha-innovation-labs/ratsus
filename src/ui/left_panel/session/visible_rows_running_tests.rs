use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::default_split_pane_session_group_state::default_split_pane_session_group_state;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::visible_rows::visible_session_rows;
use crate::ui::left_panel::session::visible_rows_cache::VisibleSessionRowsCache;

/// Builds a dormant chat session for visible-row running-state tests.
fn session_entry(index: usize, is_running: bool) -> SessionTerminal {
    SessionTerminal::dormant(
        ChatSession::new(
            "2026-05-12T10:00:00Z",
            format!("Session {index}"),
            format!("id-{index}"),
            "/tmp/project",
        )
        .with_running(is_running),
    )
}

/// Builds a stable folder order for the test project.
fn folder_order() -> Vec<PathBuf> {
    vec![PathBuf::from("/tmp/project")]
}

/// Verifies running chats remain visible because the session pane no longer has a recent limit.
#[test]
fn shows_running_chat_beyond_old_visible_limit() {
    let entries = (0..33)
        .map(|index| session_entry(index, index == 20))
        .collect::<Vec<_>>();

    let rows = visible_session_rows(
        &entries,
        &BTreeSet::new(),
        &folder_order(),
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert_eq!(rows.len(), 33);
    assert!(rows.contains(&SessionListRow::Session { index: 20 }));
    assert!(!rows
        .iter()
        .any(|row| matches!(row, SessionListRow::FolderMore { .. })));
}

/// Verifies visible-row caching still invalidates when a chat starts running for rendered status changes.
#[test]
fn cache_rebuilds_when_chat_starts_running() {
    let mut entries = (0..33)
        .map(|index| session_entry(index, false))
        .collect::<Vec<_>>();
    let mut cache = VisibleSessionRowsCache::new();
    let folder_order = folder_order();

    let _ = cache.rows(
        &entries,
        &BTreeSet::new(),
        &folder_order,
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );
    let first_rebuild_count = cache.rebuild_count();

    entries[20].session.is_running = true;
    let updated_rows = cache.rows(
        &entries,
        &BTreeSet::new(),
        &folder_order,
        None,
        &default_split_pane_session_group_state(),
        &BTreeMap::new(),
    );

    assert!(updated_rows.contains(&SessionListRow::Session { index: 20 }));
    assert_eq!(cache.rebuild_count(), first_rebuild_count + 1);
}
