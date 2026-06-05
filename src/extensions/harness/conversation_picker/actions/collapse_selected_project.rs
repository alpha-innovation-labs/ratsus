use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::actions::apply_query_change::apply_history_modal_query_change;
use crate::extensions::history_modal::selection::selected_project_path::selected_history_modal_project_path;
use crate::ui::left_panel::order::persist_preferences::persist_session_order_preferences;

/// Collapses the project folder associated with the selected picker row.
pub fn collapse_selected_conversation_project(app: &mut AppState) {
    let Some(path) = selected_history_modal_project_path(app) else {
        return;
    };
    app.collapsed_folders.insert(path);
    apply_history_modal_query_change(app);
    persist_session_order_preferences(app);
}
