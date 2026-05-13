use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::expo::expo_column_count::expo_column_count;

/// Splits the Expo viewport into equal masonry columns for a target card width.
pub fn split_expo_columns(area: Rect, card_width: u16) -> Vec<Rect> {
    let column_count = expo_column_count(area.width, card_width);
    let percent = 100 / column_count as u16;
    let mut constraints = vec![Constraint::Percentage(percent); column_count];
    if let Some(last) = constraints.last_mut() {
        *last = Constraint::Min(0);
    }
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area)
        .to_vec()
}
