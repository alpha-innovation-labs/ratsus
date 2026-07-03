use ratsus::app::state::app_state::AppState;
use ratsus::extensions::file_viewer::tabs::tab::MainPaneTab;
use ratsus::ui::layout::focus::focused_pane::FocusedPane;

use crate::support::real_case::open_delete_dialog::open_delete_dialog;
use crate::support::real_case::prepare_files::prepare_files;
use crate::support::real_case::prepare_grid::prepare_grid;
use crate::support::temporary_workspace::TemporaryWorkspace;

/// Applies real user-facing app state needed by a scenario before rendering.
pub fn prepare_real_case(
    app: &mut AppState,
    domain: &str,
    name: &str,
    workspace: &TemporaryWorkspace,
) -> anyhow::Result<()> {
    if domain.contains("file") || name.starts_with("files_") || name.starts_with("diff_") {
        prepare_files(app, workspace.path())?;
    }
    if domain.contains("expo") || name.starts_with("expo_") || name.contains("observation") {
        app.active_main_pane_tab = MainPaneTab::Expo;
        app.selected_expo_folder = app
            .session_terminals
            .first()
            .map(|entry| entry.session.working_dir.clone());
    }
    if name.contains("picker") {
        app.history_modal.is_open = true;
        app.history_modal.query = "plan".to_string();
    }
    if domain.contains("delete") || name.contains("delete") {
        open_delete_dialog(app)?;
    }
    if domain.contains("notification") || name.contains("toast") || name.contains("failure") {
        ratsus::ui::notifications::toast::show_failed_to_delete_session::show_failed_to_delete_session_toast(
            &mut app.toast_manager,
            &anyhow::anyhow!("real e2e failure notification"),
        );
    }
    if domain.contains("grid") || name.contains("split") || name.contains("bundle") {
        prepare_grid(app)?;
    }
    if domain.contains("layout") || name.contains("focus") {
        app.focused_pane = FocusedPane::Left;
    }
    if name.contains("small_terminal") {
        app.left_pane_visible = true;
    }
    if name.contains("terminal_open_existing") || name.contains("terminal_resume_owned_nexus") {
        app.session_terminals[0].ensure_terminal(8, 40)?;
    }
    Ok(())
}
