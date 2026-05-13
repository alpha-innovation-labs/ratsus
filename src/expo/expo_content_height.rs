use ratatui::layout::Rect;

use crate::app::app_state::AppState;
use crate::expo::expo_card_models::expo_card_models;
use crate::expo::layout_expo_masonry::layout_expo_masonry;

/// Returns the total scrollable height for the current Expo masonry content.
pub fn expo_content_height(app: &AppState, area: Rect) -> usize {
    let (_, content_height) = layout_expo_masonry(area, expo_card_models(app), app.expo_card_width);
    content_height
}
