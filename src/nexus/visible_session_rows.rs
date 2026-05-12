use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::session_folder_order::session_folder_order;
use crate::session_list_row::SessionListRow;
use crate::session_terminal::SessionTerminal;

/// Builds the visible folder/session tree rows for the left pane.
pub fn visible_session_rows(
    session_terminals: &[SessionTerminal],
    collapsed_folders: &BTreeSet<PathBuf>,
) -> Vec<SessionListRow> {
    let mut rows = Vec::new();
    for folder in session_folder_order(session_terminals) {
        let indexes = session_indexes_for_folder(session_terminals, &folder);
        rows.push(SessionListRow::Folder {
            path: folder.clone(),
            session_count: indexes.len(),
        });
        if collapsed_folders.contains(&folder) {
            continue;
        }
        rows.extend(
            indexes
                .into_iter()
                .map(|index| SessionListRow::Session { index }),
        );
    }
    rows
}

/// Returns flat session indexes that belong to a folder.
fn session_indexes_for_folder(
    session_terminals: &[SessionTerminal],
    folder: &PathBuf,
) -> Vec<usize> {
    session_terminals
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| (entry.session.working_dir == *folder).then_some(index))
        .collect()
}
