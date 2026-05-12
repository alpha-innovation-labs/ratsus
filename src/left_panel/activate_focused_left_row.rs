use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::open_folder_conversation_picker::open_folder_conversation_picker;
use crate::left_panel::focused_left_row::focused_left_row;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::toggle_session_folder::toggle_session_folder;

/// Activates the focused left-panel row as either a folder toggle or a session selection.
pub fn activate_focused_left_row(app: &mut NexusDemo) {
    let Some(row) = focused_left_row(app) else {
        return;
    };
    match row {
        SessionListRow::Folder { path, .. } => toggle_session_folder(app, path),
        SessionListRow::FolderMore { path } => open_folder_conversation_picker(app, path),
        SessionListRow::Session { index } => {
            app.focused_index = index;
            app.activate_focused_session();
        }
    }
}
