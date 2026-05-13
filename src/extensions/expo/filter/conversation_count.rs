use crate::app::state::app_state::AppState;
use crate::extensions::expo::filter::session_indices::expo_session_indices;

/// Returns the number of conversations in the selected Expo folder.
pub fn expo_conversation_count(app: &AppState) -> usize {
    expo_session_indices(app).len()
}
