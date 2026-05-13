use crate::app::state::app_state::AppState;
use crate::extensions::expo::filter::folder_conversation_indices::folder_conversation_indices;

/// Returns all session indexes for the currently selected Expo folder.
pub fn expo_session_indices(app: &AppState) -> Vec<usize> {
    app.selected_expo_folder
        .as_deref()
        .map(|folder| folder_conversation_indices(&app.session_terminals, folder))
        .unwrap_or_default()
}
