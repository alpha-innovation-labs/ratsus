use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;

/// Hides the Expo view and returns the main pane to chat sessions.
pub fn hide_expo(app: &mut AppState) {
    app.active_main_pane_tab = MainPaneTab::Chat;
    app.expo_filtering = false;
}
