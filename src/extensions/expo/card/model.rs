use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

/// Prepared Expo card data before masonry layout.
pub struct ExpoCardModel {
    pub session_index: usize,
    pub title: String,
    pub preview: ConversationObservationPreview,
}
