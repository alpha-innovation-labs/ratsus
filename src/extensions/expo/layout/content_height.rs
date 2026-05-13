use ratatui::layout::Rect;

use crate::app::state::app_state::AppState;
use crate::extensions::expo::card::models::expo_card_models;
use crate::extensions::expo::layout::masonry::layout_expo_masonry;

/// Returns the total scrollable height for the current Expo masonry content.
pub fn expo_content_height(app: &AppState, area: Rect) -> usize {
    let (_, content_height) = layout_expo_masonry(area, expo_card_models(app), app.expo_card_width);
    content_height
}
