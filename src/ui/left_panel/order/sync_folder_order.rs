use std::path::PathBuf;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::left_panel::order::session_folder_order::session_folder_order;

/// Keeps saved folder order entries that exist and appends new folders.
pub fn sync_folder_order(order: &[PathBuf], entries: &[SessionTerminal]) -> Vec<PathBuf> {
    let current = session_folder_order(entries);
    let mut synced = Vec::new();
    for folder in order {
        if current.iter().any(|candidate| candidate == folder) {
            push_unique_folder(&mut synced, folder.clone());
        }
    }
    for folder in current {
        push_unique_folder(&mut synced, folder);
    }
    synced
}

/// Pushes a folder once while preserving order.
fn push_unique_folder(folders: &mut Vec<PathBuf>, folder: PathBuf) {
    if !folders.iter().any(|candidate| candidate == &folder) {
        folders.push(folder);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::sync_folder_order;
    use crate::app::test_support::dormant_session::dormant_session;

    /// Verifies duplicate saved folders are collapsed during order synchronization.
    #[test]
    fn deduplicates_saved_folder_order() {
        let entries = vec![dormant_session("Alpha", "a", "/tmp/project")];
        let synced = sync_folder_order(
            &[PathBuf::from("/tmp/project"), PathBuf::from("/tmp/project")],
            &entries,
        );

        assert_eq!(synced, vec![PathBuf::from("/tmp/project")]);
    }
}
