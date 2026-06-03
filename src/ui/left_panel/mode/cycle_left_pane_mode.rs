use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

/// Advances the left pane through sessions, plans, and files while showing its matching preview.
pub fn cycle_left_pane_mode(app: &mut AppState) {
    app.left_pane_mode = next_left_pane_mode(app.left_pane_mode);
    app.active_main_pane_tab = MainPaneTab::Chat;
    app.left_pane_visible = true;
    app.focused_pane = FocusedPane::Left;
}

/// Returns the next left-pane mode in display order.
fn next_left_pane_mode(mode: LeftPaneMode) -> LeftPaneMode {
    match mode {
        LeftPaneMode::Sessions => LeftPaneMode::Plans,
        LeftPaneMode::Plans => LeftPaneMode::Files,
        LeftPaneMode::Files => LeftPaneMode::Sessions,
    }
}

#[cfg(test)]
mod tests {
    use super::cycle_left_pane_mode;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
    use crate::ui::layout::focus::focused_pane::FocusedPane;
    use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

    /// Verifies cycling follows sessions, plans, files, then sessions again.
    #[test]
    fn cycles_modes_in_display_order() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");

        cycle_left_pane_mode(&mut app);
        assert_eq!(app.left_pane_mode, LeftPaneMode::Plans);
        cycle_left_pane_mode(&mut app);
        assert_eq!(app.left_pane_mode, LeftPaneMode::Files);
        cycle_left_pane_mode(&mut app);
        assert_eq!(app.left_pane_mode, LeftPaneMode::Sessions);
    }

    /// Verifies cycling shows the shared left pane and matching main preview surface.
    #[test]
    fn opens_left_pane_and_chat_surface() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.left_pane_visible = false;
        app.active_main_pane_tab = MainPaneTab::Expo;
        app.focused_pane = FocusedPane::Terminal;

        cycle_left_pane_mode(&mut app);

        assert!(app.left_pane_visible);
        assert_eq!(app.active_main_pane_tab, MainPaneTab::Chat);
        assert_eq!(app.focused_pane, FocusedPane::Left);
    }
}
