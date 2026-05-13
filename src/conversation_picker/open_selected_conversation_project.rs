use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::apply_conversation_picker_query_change::apply_conversation_picker_query_change;
use crate::conversation_picker::selected_conversation_picker_project_path::selected_conversation_picker_project_path;
use crate::left_panel::persist_session_order_preferences::persist_session_order_preferences;

/// Opens the project folder associated with the selected picker row.
pub fn open_selected_conversation_project(app: &mut NexusDemo) {
    let Some(path) = selected_conversation_picker_project_path(app) else {
        return;
    };
    app.collapsed_folders.remove(&path);
    apply_conversation_picker_query_change(app);
    persist_session_order_preferences(app);
}
