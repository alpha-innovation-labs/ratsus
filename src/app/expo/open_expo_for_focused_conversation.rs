use crate::app::expo::activate_expo_folder::activate_expo_folder;
use crate::app::state::app_state::AppState;

/// Opens Expo for the working directory of the currently focused conversation.
pub fn open_expo_for_focused_conversation(app: &mut AppState) {
    let Some(folder) = app
        .session_terminals
        .get(app.focused_index)
        .map(|entry| entry.session.working_dir.clone())
    else {
        return;
    };
    activate_expo_folder(app, folder);
}
