use std::io;

use crate::ui::left_panel::order::preferences::SessionOrderPreferences;
use crate::ui::left_panel::order::preferences_path::session_order_preferences_path;

/// Saves left-panel ordering preferences to disk.
pub fn save_session_order_preferences(preferences: &SessionOrderPreferences) -> io::Result<()> {
    let Some(path) = session_order_preferences_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(preferences)?;
    std::fs::write(path, content)
}
