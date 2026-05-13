use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

/// Returns the rendered height for one Expo conversation card.
pub fn expo_card_height(preview: &ConversationObservationPreview) -> u16 {
    let observation_lines = u16::from(preview.has_more) + preview.observations.len() as u16;
    3 + observation_lines.saturating_add(u16::from(observation_lines > 0))
}

#[cfg(test)]
mod tests {
    use super::expo_card_height;
    use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

    /// Verifies observations increase rendered card height below bordered title chrome.
    #[test]
    fn includes_observation_lines() {
        let preview = ConversationObservationPreview {
            has_more: true,
            observations: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        };

        assert_eq!(expo_card_height(&preview), 8);
    }

    /// Verifies title-only cards keep only the title border height.
    #[test]
    fn title_only_height() {
        assert_eq!(
            expo_card_height(&ConversationObservationPreview::default()),
            3
        );
    }
}
