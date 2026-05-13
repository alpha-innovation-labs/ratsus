use ratatui::layout::Rect;

use crate::expo::expo_card_height::expo_card_height;
use crate::expo::expo_card_model::ExpoCardModel;
use crate::expo::expo_masonry_item::ExpoMasonryItem;
use crate::expo::shortest_expo_column_index::shortest_expo_column_index;
use crate::expo::split_expo_columns::split_expo_columns;

/// Lays out Expo cards into shortest-first masonry columns.
pub fn layout_expo_masonry(
    area: Rect,
    cards: Vec<ExpoCardModel>,
    card_width: u16,
) -> (Vec<ExpoMasonryItem>, usize) {
    let columns = split_expo_columns(area, card_width);
    let mut heights = vec![0usize; columns.len()];
    let mut items = Vec::new();
    for card in cards {
        let column_index = shortest_expo_column_index(&heights);
        let column = columns[column_index];
        let height = expo_card_height(&card.preview);
        let y = usize::from(area.y).saturating_add(heights[column_index]);
        let area = Rect::new(
            column.x,
            y.min(usize::from(u16::MAX)) as u16,
            column.width.saturating_sub(1),
            height,
        );
        heights[column_index] = heights[column_index].saturating_add(usize::from(height) + 1);
        items.push(ExpoMasonryItem {
            session_index: card.session_index,
            title: card.title,
            preview: card.preview,
            area,
        });
    }
    (items, heights.into_iter().max().unwrap_or(0))
}
