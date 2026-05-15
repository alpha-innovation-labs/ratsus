use crate::app::state::app_state::AppState;

/// Returns the visible picker scope label for the dialog header.
pub fn conversation_picker_scope_label(app: &AppState) -> &'static str {
    if app.conversation_picker.folder_filter.is_none() {
        return "All";
    }
    if app.conversation_picker.folder_filter.as_deref() == app.selected_workspace_path.as_deref() {
        return "Workspace";
    }
    "Folder"
}
