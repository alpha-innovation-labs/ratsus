use std::path::PathBuf;

use crate::terminal::session_terminal::SessionTerminal;

/// Returns session folders in first-seen order.
pub fn session_folder_order(session_terminals: &[SessionTerminal]) -> Vec<PathBuf> {
    let mut folders = Vec::new();
    for entry in session_terminals {
        let folder = entry.session.working_dir.clone();
        if !folders.iter().any(|seen| seen == &folder) {
            folders.push(folder);
        }
    }
    folders
}

#[cfg(test)]
mod tests {
    use super::session_folder_order;

    /// Documents that real ordering depends on terminal entries and is covered through UI row tests.
    #[test]
    fn empty_entries_have_no_folders() {
        assert!(session_folder_order(&[]).is_empty());
    }
}
