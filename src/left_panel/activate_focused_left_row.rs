use crate::app::activate_expo_folder::activate_expo_folder;
use crate::app::app_state::AppState;
use crate::conversation_picker::open_folder_conversation_picker::open_folder_conversation_picker;
use crate::left_panel::focused_left_row::focused_left_row;
use crate::left_panel::session_list_row::SessionListRow;

/// Activates the focused left-panel row as Expo, more-history picker, or session selection.
pub fn activate_focused_left_row(app: &mut AppState) {
    let Some(row) = focused_left_row(app) else {
        return;
    };
    match row {
        SessionListRow::Folder { path, .. } => activate_expo_folder(app, path),
        SessionListRow::FolderMore { path } => open_folder_conversation_picker(app, path),
        SessionListRow::Session { index } => {
            app.focused_index = index;
            app.activate_focused_session();
        }
    }
}
