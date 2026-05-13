use crate::expo::conversation_observation_preview::ConversationObservationPreview;

/// Prepared Expo card data before masonry layout.
pub struct ExpoCardModel {
    pub session_index: usize,
    pub title: String,
    pub preview: ConversationObservationPreview,
}
