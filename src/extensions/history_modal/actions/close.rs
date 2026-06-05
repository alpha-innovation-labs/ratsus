use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::mode::HistoryModalMode;

/// Closes the conversation picker without changing the active conversation.
pub fn close_history_modal(app: &mut AppState) {
    app.history_modal.is_open = false;
    app.history_modal.is_filtering = false;
    app.history_modal.pending_g = false;
    app.history_modal.folder_filter = None;
    app.history_modal.mode = HistoryModalMode::Open;
}
