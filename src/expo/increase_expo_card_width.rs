use crate::app::nexus_demo_state::NexusDemo;
use crate::expo::clamp_expo_card_width::clamp_expo_card_width;
use crate::expo::expo_card_width_limits::expo_card_width_step;
use crate::left_panel::persist_session_order_preferences::persist_session_order_preferences;

/// Increases Expo card target width and persists the zoom setting.
pub fn increase_expo_card_width(app: &mut NexusDemo) {
    app.expo_card_width =
        clamp_expo_card_width(app.expo_card_width.saturating_add(expo_card_width_step()));
    app.expo_scroll = 0;
    persist_session_order_preferences(app);
}
