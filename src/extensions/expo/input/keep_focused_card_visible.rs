use crate::app::state::app_state::AppState;
use crate::extensions::expo::card::body_area::expo_cards_body_area;
use crate::extensions::expo::card::models::expo_card_models;
use crate::extensions::expo::layout::masonry::layout_expo_masonry;

/// Adjusts Expo scroll so the focused conversation card remains visible.
pub fn keep_focused_expo_card_visible(app: &mut AppState) {
    let body = expo_cards_body_area(app.last_terminal_area);
    let (items, _) = layout_expo_masonry(body, expo_card_models(app), app.expo_card_width);
    let Some(item) = items
        .iter()
        .find(|item| item.session_index == app.focused_index)
    else {
        return;
    };
    let viewport_top = usize::from(body.y).saturating_add(app.expo_scroll);
    let viewport_bottom = viewport_top.saturating_add(usize::from(body.height));
    let card_top = usize::from(item.area.y);
    let card_bottom = card_top.saturating_add(usize::from(item.area.height));
    if card_top < viewport_top {
        app.expo_scroll = card_top.saturating_sub(usize::from(body.y));
    } else if card_bottom > viewport_bottom {
        app.expo_scroll = card_bottom
            .saturating_sub(usize::from(body.y))
            .saturating_sub(usize::from(body.height));
    }
}
