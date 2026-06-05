use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::current_item_count::current_conversation_picker_item_count;
use crate::extensions::history_modal::selection::clamp_selection::clamp_conversation_picker_selection;

/// Toggles the conversation picker between selected-workspace and all-workspaces scopes.
pub fn toggle_conversation_picker_scope(app: &mut AppState) -> bool {
    let Some(workspace) = app.selected_workspace_path.clone() else {
        return false;
    };
    let next_filter = if app.conversation_picker.folder_filter.as_ref() == Some(&workspace) {
        None
    } else {
        Some(workspace)
    };
    let changed = app.conversation_picker.folder_filter != next_filter;
    app.conversation_picker.folder_filter = next_filter;
    let item_count = current_conversation_picker_item_count(app);
    clamp_conversation_picker_selection(&mut app.conversation_picker, item_count);
    changed
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::toggle_conversation_picker_scope;
    use crate::app::test_support::app_fixture::app_fixture;

    /// Verifies scope toggles from workspace filtering to all workspaces.
    #[test]
    fn toggles_workspace_to_all() -> anyhow::Result<()> {
        let mut app = app_fixture(Vec::new())?;
        app.selected_workspace_path = Some(PathBuf::from("/workspace/beta"));
        app.conversation_picker.folder_filter = Some(PathBuf::from("/workspace/beta"));

        assert!(toggle_conversation_picker_scope(&mut app));

        assert_eq!(app.conversation_picker.folder_filter, None);
        Ok(())
    }

    /// Verifies scope toggles from all workspaces back to selected workspace filtering.
    #[test]
    fn toggles_all_to_workspace() -> anyhow::Result<()> {
        let mut app = app_fixture(Vec::new())?;
        app.selected_workspace_path = Some(PathBuf::from("/workspace/beta"));
        app.conversation_picker.folder_filter = None;

        assert!(toggle_conversation_picker_scope(&mut app));

        assert_eq!(
            app.conversation_picker.folder_filter,
            Some(PathBuf::from("/workspace/beta"))
        );
        Ok(())
    }
}
