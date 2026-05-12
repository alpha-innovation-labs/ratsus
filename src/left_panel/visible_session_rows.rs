use std::collections::BTreeSet;
use std::path::PathBuf;

use std::cmp::Reverse;

use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::session_modified_timestamp::session_modified_timestamp;
use crate::terminal::session_terminal::SessionTerminal;

const RECENT_SESSION_LIMIT: usize = 10;

/// Builds the visible folder/session tree rows for the left pane.
pub fn visible_session_rows(
    session_terminals: &[SessionTerminal],
    collapsed_folders: &BTreeSet<PathBuf>,
    folder_order: &[PathBuf],
) -> Vec<SessionListRow> {
    let mut rows = Vec::new();
    for folder in folder_order {
        let indexes =
            limited_session_indexes(session_indexes_for_folder(session_terminals, folder));
        let total_session_count = total_session_count_for_folder(session_terminals, folder);
        if indexes.is_empty() {
            continue;
        }
        rows.push(SessionListRow::Folder {
            path: folder.clone(),
            current_session_count: indexes.len(),
            total_session_count,
        });
        if collapsed_folders.contains(folder) {
            continue;
        }
        let recent_session_count = indexes.len();
        rows.extend(
            indexes
                .into_iter()
                .map(|index| SessionListRow::Session { index }),
        );
        if total_session_count > recent_session_count {
            rows.push(SessionListRow::FolderMore {
                path: folder.clone(),
            });
        }
    }
    rows
}

/// Returns at most the number of recent sessions shown directly in one folder.
fn limited_session_indexes(indexes: Vec<usize>) -> Vec<usize> {
    indexes.into_iter().take(RECENT_SESSION_LIMIT).collect()
}

/// Returns the total number of sessions that belong to a folder.
fn total_session_count_for_folder(
    session_terminals: &[SessionTerminal],
    folder: &PathBuf,
) -> usize {
    session_terminals
        .iter()
        .filter(|entry| entry.session.working_dir == *folder)
        .count()
}

/// Returns folder session indexes sorted from newest to oldest activity.
fn session_indexes_for_folder(
    session_terminals: &[SessionTerminal],
    folder: &PathBuf,
) -> Vec<usize> {
    let mut indexes = session_terminals
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| (entry.session.working_dir == *folder).then_some(index))
        .collect::<Vec<_>>();
    indexes.sort_by_key(|index| {
        Reverse(session_modified_timestamp(
            &session_terminals[*index].session,
        ))
    });
    indexes
}
