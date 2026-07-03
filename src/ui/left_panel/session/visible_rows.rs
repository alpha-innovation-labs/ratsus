use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

use ratkit::primitives::resizable_grid::PaneId;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::split_groups::append_visible_folder_session_rows::append_visible_folder_session_rows;
use crate::ui::left_panel::session::split_groups::build_grouped_session_lookup::build_grouped_session_lookup;

/// Builds the visible folder/session tree rows for the consolidated left pane.
pub fn visible_session_rows_with_folders(
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
        let total = indexes.len();
        rows.push(SessionListRow::Folder {
            path: folder.clone(),
            current_session_count: total,
            total_session_count: total,
        });
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
