use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::actions::apply_query_change::apply_conversation_picker_query_change;
use crate::extensions::harness::conversation_picker::selection::selected_project_path::selected_conversation_picker_project_path;
use crate::ui::left_panel::order::persist_preferences::persist_session_order_preferences;

/// Collapses the project folder associated with the selected picker row.
pub fn collapse_selected_conversation_project(app: &mut AppState) {
    let Some(path) = selected_conversation_picker_project_path(app) else {
        return;
    };
    app.collapsed_folders.insert(path);
    apply_conversation_picker_query_change(app);
    persist_session_order_preferences(app);
}
