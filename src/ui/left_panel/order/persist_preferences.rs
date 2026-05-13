use crate::app::state::app_state::AppState;
use crate::ui::left_panel::order::current_preferences::current_session_order_preferences;
use crate::ui::left_panel::order::save_preferences::save_session_order_preferences;

/// Persists the current left-panel ordering preferences and ignores storage failures.
pub fn persist_session_order_preferences(app: &AppState) {
    let preferences = current_session_order_preferences(app);
    let _ = save_session_order_preferences(&preferences);
}
