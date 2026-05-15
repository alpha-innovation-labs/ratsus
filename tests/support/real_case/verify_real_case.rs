use std::path::Path;

use anyhow::ensure;
use ratsus::app::state::app_state::AppState;

/// Verifies each scenario used the real Nexus backend and real filesystem boundaries.
pub fn verify_real_case(
    app: &AppState,
    domain: &str,
    name: &str,
    workspace: impl AsRef<Path>,
) -> anyhow::Result<()> {
    ensure!(
        app.chat_harness.display_name() == "Nexus",
        "E2E tests must run against the real Nexus harness"
    );
    ensure!(
        !app.session_terminals.is_empty(),
        "real Nexus catalog must not be empty"
    );
    ensure!(
        workspace.as_ref().exists(),
        "test-owned workspace must exist while the scenario runs"
    );
    if domain.contains("delete") || name.contains("delete") {
        ensure!(
            app.delete_confirmation.is_open(),
            "delete dialog must be open through real app state"
        );
    }
    if name == "refresh_nexus_registry_watcher" {
        ensure!(
            app.session_watcher.is_some(),
            "Nexus session registry watcher must be installed"
        );
    }
    if name == "picker_workspace_scope_toggle" {
        ensure!(
            app.conversation_picker.folder_filter.as_deref()
                == app.selected_workspace_path.as_deref(),
            "conversation picker must be scoped to the selected workspace"
        );
    } else if domain.contains("workspace") || name.contains("workspace") {
        ensure!(
            app.selected_workspace_path.is_some(),
            "workspace pane must have a selected folder"
        );
    }
    Ok(())
}
