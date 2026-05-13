use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::nexus_demo_state::NexusDemo;
use crate::expo::expo_card_area::ExpoCardArea;
use crate::expo::expo_card_models::expo_card_models;
use crate::expo::expo_card_visible_area::expo_card_visible_area;
use crate::expo::layout_expo_masonry::layout_expo_masonry;
use crate::expo::render_conversation_card::render_conversation_card;

/// Renders the visible window of all Expo conversation cards in masonry columns.
pub fn render_expo_cards(app: &mut NexusDemo, frame: &mut Frame, area: Rect) {
    let cards = expo_card_models(app);
    let (items, content_height) = layout_expo_masonry(area, cards, app.expo_card_width);
    app.expo_scroll = app
        .expo_scroll
        .min(content_height.saturating_sub(usize::from(area.height)));
    for item in items {
        let Some(card_area) = expo_card_visible_area(item.area, area, app.expo_scroll) else {
            continue;
        };
        render_conversation_card(frame, card_area, &item.title, &item.preview);
        app.expo_card_areas.push(ExpoCardArea {
            session_index: item.session_index,
            area: card_area,
        });
    }
}
