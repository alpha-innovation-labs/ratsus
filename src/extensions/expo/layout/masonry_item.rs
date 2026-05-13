use ratatui::layout::Rect;

use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

/// Positioned Expo card data ready for rendering and hit testing.
pub struct ExpoMasonryItem {
    pub session_index: usize,
    pub title: String,
    pub preview: ConversationObservationPreview,
    pub area: Rect,
}
