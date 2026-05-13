use crate::app::state::app_state::AppState;
use crate::extensions::expo::card::clamp_width::clamp_expo_card_width;
use crate::extensions::expo::card::width_limits::expo_card_width_step;
use crate::ui::left_panel::order::persist_preferences::persist_session_order_preferences;

/// Increases Expo card target width and persists the zoom setting.
pub fn increase_expo_card_width(app: &mut AppState) {
    app.expo_card_width =
        clamp_expo_card_width(app.expo_card_width.saturating_add(expo_card_width_step()));
    app.expo_scroll = 0;
    persist_session_order_preferences(app);
}
