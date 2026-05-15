use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

use ratkit::primitives::resizable_grid::PaneId;

use crate::extensions::terminal::session::is_normal_terminal_session::is_normal_terminal_session;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::split_groups::append_visible_folder_session_rows::append_visible_folder_session_rows;
use crate::ui::left_panel::session::split_groups::build_grouped_session_lookup::build_grouped_session_lookup;

const RECENT_SESSION_LIMIT: usize = 10;

/// Builds the visible folder/session tree rows for the left pane.
pub fn visible_session_rows(
    session_terminals: &[SessionTerminal],
    collapsed_folders: &BTreeSet<PathBuf>,
    folder_order: &[PathBuf],
    _pinned_session_index: Option<usize>,
    split_groups: &SplitPaneSessionGroupState,
    pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
) -> Vec<SessionListRow> {
    let sessions_by_folder = session_indexes_by_folder(session_terminals);
    let grouped_lookup =
        build_grouped_session_lookup(session_terminals, split_groups, pane_session_bundles);
    let split_group_names = split_group_names(split_groups);
    let mut rows = Vec::new();
    let mut rendered_folders = BTreeSet::new();
    for folder in folder_order {
        if !rendered_folders.insert(folder.clone()) {
            continue;
        }
        let Some(indexes) = sessions_by_folder.get(folder) else {
            continue;
        };
        if collapsed_folders.contains(folder) {
            continue;
        }
        append_visible_folder_session_rows(
            &mut rows,
            session_terminals,
            folder,
            indexes,
            &grouped_lookup,
            &split_group_names,
        );
    }
    rows
}

/// Builds legacy visible folder/session tree rows with explicit folder parents.
pub fn visible_session_rows_with_folders(
    session_terminals: &[SessionTerminal],
    collapsed_folders: &BTreeSet<PathBuf>,
    folder_order: &[PathBuf],
    pinned_session_index: Option<usize>,
    split_groups: &SplitPaneSessionGroupState,
    pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
) -> Vec<SessionListRow> {
    let sessions_by_folder = session_indexes_by_folder(session_terminals);
    let grouped_lookup =
        build_grouped_session_lookup(session_terminals, split_groups, pane_session_bundles);
    let split_group_names = split_group_names(split_groups);
    let mut rows = Vec::new();
    let mut rendered_folders = BTreeSet::new();
    for folder in folder_order {
        if !rendered_folders.insert(folder.clone()) {
            continue;
        }
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
        append_visible_folder_session_rows(
            &mut rows,
            session_terminals,
            folder,
            &visible_indexes,
            &grouped_lookup,
            &split_group_names,
        );
        if indexes.len() > visible_indexes.len() {
            rows.push(SessionListRow::FolderMore {
                path: folder.clone(),
            });
        }
    }
    rows
}

/// Returns legacy visible session indexes, preserving required pinned sessions.
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
    push_running_sessions(session_terminals, indexes, &mut visible);
    visible.sort_by_key(|index| indexes.iter().position(|candidate| candidate == index));
    visible
}

/// Adds a terminal immediately after the visible limit so new shells stay visible.
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
        push_unique_index(visible, extra_index);
    }
}

/// Adds the focused session when it belongs to this folder but is outside the visible limit.
fn push_pinned_session(
    indexes: &[usize],
    pinned_session_index: Option<usize>,
    visible: &mut Vec<usize>,
) {
    let Some(pinned_index) = pinned_session_index else {
        return;
    };
    if indexes.contains(&pinned_index) {
        push_unique_index(visible, pinned_index);
    }
}

/// Adds every running session so live work is never hidden behind the more row.
fn push_running_sessions(
    session_terminals: &[SessionTerminal],
    indexes: &[usize],
    visible: &mut Vec<usize>,
) {
    for index in indexes {
        if session_terminals
            .get(*index)
            .is_some_and(|entry| entry.session.is_running)
        {
            push_unique_index(visible, *index);
        }
    }
}

/// Adds one index only when it is not already visible.
fn push_unique_index(visible: &mut Vec<usize>, index: usize) {
    if !visible.contains(&index) {
        visible.push(index);
    }
}

/// Returns current split group names by id for visible row projection.
fn split_group_names(split_groups: &SplitPaneSessionGroupState) -> BTreeMap<u64, String> {
    split_groups
        .groups
        .iter()
        .map(|(group_id, group)| (*group_id, group.name.clone()))
        .collect()
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
