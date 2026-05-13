use crate::app::app_state::AppState;
use crate::expo::clamp_expo_card_width::clamp_expo_card_width;
use crate::expo::expo_card_width_limits::expo_card_width_step;
use crate::left_panel::persist_session_order_preferences::persist_session_order_preferences;

/// Decreases Expo card target width and persists the zoom setting.
pub fn decrease_expo_card_width(app: &mut AppState) {
    app.expo_card_width =
        clamp_expo_card_width(app.expo_card_width.saturating_sub(expo_card_width_step()));
    app.expo_scroll = 0;
    persist_session_order_preferences(app);
}
