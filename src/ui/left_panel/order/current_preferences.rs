use crate::app::state::app_state::AppState;
use crate::ui::layout::resizable_grid::shell_split_percent::shell_split_percent;
use crate::ui::layout::resizable_grid::workspace_split_percent::workspace_split_percent;
use crate::ui::left_panel::order::preferences::SessionOrderPreferences;

/// Builds persisted ordering preferences from current app state.
pub fn current_session_order_preferences(app: &AppState) -> SessionOrderPreferences {
    SessionOrderPreferences {
        active_session_id: app
            .session_terminals
            .get(app.active_index)
            .map(|entry| entry.session.id.clone()),
        session_ids: app
            .session_terminals
            .iter()
            .map(|entry| entry.session.id.clone())
            .collect(),
        folder_paths: app.folder_order.clone(),
        collapsed_folder_paths: app.collapsed_folders.iter().cloned().collect(),
        expo_card_width: app.expo_card_width,
        shell_split_percent: shell_split_percent(&app.layout),
        workspace_split_percent: workspace_split_percent(&app.layout),
    }
}
