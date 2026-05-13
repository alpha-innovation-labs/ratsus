use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::session_order_preferences::SessionOrderPreferences;

/// Builds persisted ordering preferences from current app state.
pub fn current_session_order_preferences(app: &NexusDemo) -> SessionOrderPreferences {
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
    }
}
