use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::ui::left_panel::order::persist_preferences::persist_session_order_preferences;

/// Toggles whether sessions under a folder are visible.
pub fn toggle_session_folder(app: &mut AppState, folder: PathBuf) {
    if !app.collapsed_folders.remove(&folder) {
        app.collapsed_folders.insert(folder);
    }
    app.keep_focused_row_visible();
    persist_session_order_preferences(app);
}
