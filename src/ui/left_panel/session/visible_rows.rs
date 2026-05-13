use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use crate::extensions::terminal::session::is_normal_terminal_session::is_normal_terminal_session;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::session::list_row::SessionListRow;

const RECENT_SESSION_LIMIT: usize = 10;

/// Builds the visible folder/session tree rows for the left pane.
pub fn visible_session_rows(
    session_terminals: &[SessionTerminal],
    collapsed_folders: &BTreeSet<PathBuf>,
    folder_order: &[PathBuf],
    pinned_session_index: Option<usize>,
) -> Vec<SessionListRow> {
    let sessions_by_folder = session_indexes_by_folder(session_terminals);
    let mut rows = Vec::new();
    for folder in folder_order {
        let Some(indexes) = sessions_by_folder.get(folder) else {
            continue;
        };
        let visible_indexes =
            visible_folder_session_indexes(session_terminals, indexes, pinned_session_index);
        rows.push(SessionListRow::Folder {
            path: folder.clone(),
            current_session_count: visible_indexes.len(),
            total_session_count: indexes.len(),
        });
        if collapsed_folders.contains(folder) {
            continue;
        }
        rows.extend(
            visible_indexes
                .iter()
                .map(|index| SessionListRow::Session { index: *index }),
        );
        if indexes.len() > visible_indexes.len() {
            rows.push(SessionListRow::FolderMore {
                path: folder.clone(),
            });
        }
    }
    rows
}

/// Returns visible session indexes, always including the pinned active session when present.
fn visible_folder_session_indexes(
    session_terminals: &[SessionTerminal],
    indexes: &[usize],
    pinned_session_index: Option<usize>,
) -> Vec<usize> {
    let mut visible = indexes
        .iter()
        .take(RECENT_SESSION_LIMIT)
        .copied()
        .collect::<Vec<_>>();
    push_normal_terminal_after_limit(session_terminals, indexes, &mut visible);
    push_pinned_session(indexes, pinned_session_index, &mut visible);
    visible.sort_by_key(|index| indexes.iter().position(|candidate| candidate == index));
    visible
}

/// Adds a terminal immediately after the visible limit so Ctrl+T remains visible there.
fn push_normal_terminal_after_limit(
    session_terminals: &[SessionTerminal],
    indexes: &[usize],
    visible: &mut Vec<usize>,
) {
    let Some(extra_index) = indexes.get(RECENT_SESSION_LIMIT).copied() else {
        return;
    };
    if session_terminals
        .get(extra_index)
        .is_some_and(|entry| is_normal_terminal_session(&entry.session))
    {
        push_unique(visible, extra_index);
    }
}

/// Adds the active session when it belongs to this folder but is outside the visible limit.
fn push_pinned_session(
    indexes: &[usize],
    pinned_session_index: Option<usize>,
    visible: &mut Vec<usize>,
) {
    let Some(pinned_index) = pinned_session_index else {
        return;
    };
    if indexes.contains(&pinned_index) {
        push_unique(visible, pinned_index);
    }
}

/// Pushes one index only when it is not already visible.
fn push_unique(visible: &mut Vec<usize>, index: usize) {
    if !visible.contains(&index) {
        visible.push(index);
    }
}

/// Groups session indexes by folder while preserving the current session vector order.
fn session_indexes_by_folder(
    session_terminals: &[SessionTerminal],
) -> HashMap<PathBuf, Vec<usize>> {
    let mut sessions_by_folder = HashMap::new();
    for (index, entry) in session_terminals.iter().enumerate() {
        sessions_by_folder
            .entry(entry.session.working_dir.clone())
            .or_insert_with(Vec::new)
            .push(index);
    }
    sessions_by_folder
}
