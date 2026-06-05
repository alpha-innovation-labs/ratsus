use crate::app::expo::activate_expo_folder::activate_expo_folder;
use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::actions::open_folder::open_folder_history_modal;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::left_panel::focus::focused_row::focused_left_row;
use crate::ui::left_panel::session::activation::activate_split_group_child::activate_split_group_child;
use crate::ui::left_panel::session::activation::activate_split_group_parent::activate_split_group_parent;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Activates the focused left-panel row as Expo, more-history picker, or session selection.
pub fn activate_focused_left_row(app: &mut AppState) {
    let Some(row) = focused_left_row(app) else {
        return;
    };
    match row {
        SessionListRow::Folder { path, .. } => activate_expo_folder(app, path),
        SessionListRow::FolderMore { path } => open_folder_history_modal(app, path),
        SessionListRow::SplitGroup { group_id, .. } => activate_split_group_parent(app, group_id),
        SessionListRow::SplitGroupChild { pane_id, index, .. } => {
            activate_split_group_child(app, pane_id, index);
        }
        SessionListRow::Session { index } => {
            app.focused_index = index;
            app.activate_focused_session();
            app.focused_pane = FocusedPane::Terminal;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::layout::focus::focused_pane::FocusedPane;
    use crate::ui::left_panel::input::activate_focused_row::activate_focused_left_row;

    /// Verifies opening a conversation from the left pane returns focus to the main pane.
    #[test]
    fn activating_session_row_focuses_main_pane() {
        let mut app = app_fixture(vec![dormant_session("api", "api-id", "/tmp/project")])
            .expect("app fixture");
        app.focused_pane = FocusedPane::Left;
        app.focused_row = 0;

        activate_focused_left_row(&mut app);

        assert_eq!(app.focused_pane, FocusedPane::Terminal);
    }
}
