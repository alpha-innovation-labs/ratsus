use crate::app::state::app_state::AppState;

const ACTIVE_SCOPE_MARKER: &str = "●";
const INACTIVE_SCOPE_MARKER: &str = "○";

/// Returns the visible picker scope label for the dialog header.
pub fn conversation_picker_scope_label(app: &AppState) -> String {
    format!(
        "{} Workspace  {} All",
        workspace_scope_marker(app),
        all_scope_marker(app)
    )
}

/// Returns the marker for the workspace-scoped picker option.
fn workspace_scope_marker(app: &AppState) -> &'static str {
    if app.conversation_picker.folder_filter.is_some() {
        return ACTIVE_SCOPE_MARKER;
    }
    INACTIVE_SCOPE_MARKER
}

/// Returns the marker for the all-workspaces picker option.
fn all_scope_marker(app: &AppState) -> &'static str {
    if app.conversation_picker.folder_filter.is_none() {
        return ACTIVE_SCOPE_MARKER;
    }
    INACTIVE_SCOPE_MARKER
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::conversation_picker_scope_label;
    use crate::app::test_support::app_fixture::app_fixture;

    /// Workspace scope should show both scope options and mark Workspace active.
    #[test]
    fn marks_workspace_scope_active() -> anyhow::Result<()> {
        let mut app = app_fixture(Vec::new())?;
        app.selected_workspace_path = Some(PathBuf::from("/workspace/alpha"));
        app.conversation_picker.folder_filter = Some(PathBuf::from("/workspace/alpha"));

        assert_eq!(conversation_picker_scope_label(&app), "● Workspace  ○ All");
        Ok(())
    }

    /// All scope should show both scope options and mark All active.
    #[test]
    fn marks_all_scope_active() -> anyhow::Result<()> {
        let mut app = app_fixture(Vec::new())?;
        app.selected_workspace_path = Some(PathBuf::from("/workspace/alpha"));
        app.conversation_picker.folder_filter = None;

        assert_eq!(conversation_picker_scope_label(&app), "○ Workspace  ● All");
        Ok(())
    }
}
