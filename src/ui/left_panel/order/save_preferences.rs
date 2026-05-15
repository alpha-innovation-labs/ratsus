use std::io;

use crate::shared::async_persistence::save_job::SaveJob;
use crate::shared::async_persistence::worker::enqueue_save;
use crate::ui::left_panel::order::preferences::SessionOrderPreferences;
use crate::ui::left_panel::order::preferences_path::session_order_preferences_path;

/// Saves left-panel ordering preferences to disk.
pub fn save_session_order_preferences(preferences: &SessionOrderPreferences) -> io::Result<()> {
    let Some(path) = session_order_preferences_path() else {
        return Ok(());
    };
    let content = serde_json::to_string_pretty(preferences)?;
    enqueue_save(SaveJob {
        key: "session-order-preferences",
        path,
        content,
    });
    Ok(())
}
