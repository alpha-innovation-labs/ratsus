use std::path::PathBuf;

use crate::nexus_demo_state::NexusDemo;

/// Toggles whether sessions under a folder are visible.
pub fn toggle_session_folder(app: &mut NexusDemo, folder: PathBuf) {
    if !app.collapsed_folders.remove(&folder) {
        app.collapsed_folders.insert(folder);
    }
}
