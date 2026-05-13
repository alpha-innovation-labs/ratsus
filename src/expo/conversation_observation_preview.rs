/// Latest observation bullets shown on an Expo conversation card.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConversationObservationPreview {
    pub has_more: bool,
    pub observations: Vec<String>,
}
