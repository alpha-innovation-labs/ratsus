/// Identifies one session whose observation preview should be loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationPreviewRequest {
    pub conversation_id: String,
    pub title: String,
}

impl ObservationPreviewRequest {
    /// Creates an observation preview request from session id and title.
    pub fn new(conversation_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            conversation_id: conversation_id.into(),
            title: title.into(),
        }
    }
}
