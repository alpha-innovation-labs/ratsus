use crate::left_panel::session_order_preferences::SessionOrderPreferences;
use crate::left_panel::session_order_preferences_path::session_order_preferences_path;

/// Loads saved left-panel ordering preferences from disk.
pub fn load_session_order_preferences() -> SessionOrderPreferences {
    let Some(path) = session_order_preferences_path() else {
        return SessionOrderPreferences::default();
    };
    let Ok(content) = std::fs::read_to_string(path) else {
        return SessionOrderPreferences::default();
    };
    serde_json::from_str(&content).unwrap_or_default()
}
