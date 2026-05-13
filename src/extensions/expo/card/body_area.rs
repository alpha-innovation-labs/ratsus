use ratatui::layout::Rect;

use crate::extensions::expo::layout::split_view_area::split_expo_view_area;

/// Returns the scrollable Expo cards body area for a full main-pane content area.
pub fn expo_cards_body_area(area: Rect) -> Rect {
    split_expo_view_area(area).0
}
