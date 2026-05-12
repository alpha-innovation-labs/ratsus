use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::current_session_order_preferences::current_session_order_preferences;
use crate::left_panel::save_session_order_preferences::save_session_order_preferences;

/// Persists the current left-panel ordering preferences and ignores storage failures.
pub fn persist_session_order_preferences(app: &NexusDemo) {
    let preferences = current_session_order_preferences(app);
    let _ = save_session_order_preferences(&preferences);
}
