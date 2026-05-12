use std::path::PathBuf;

use crate::left_panel::session_folder_order::session_folder_order;
use crate::terminal::session_terminal::SessionTerminal;

/// Keeps saved folder order entries that exist and appends new folders.
pub fn sync_folder_order(order: &[PathBuf], entries: &[SessionTerminal]) -> Vec<PathBuf> {
    let current = session_folder_order(entries);
    let mut synced = order
        .iter()
        .filter(|folder| current.iter().any(|candidate| candidate == *folder))
        .cloned()
        .collect::<Vec<_>>();
    for folder in current {
        if !synced.iter().any(|candidate| candidate == &folder) {
            synced.push(folder);
        }
    }
    synced
}
