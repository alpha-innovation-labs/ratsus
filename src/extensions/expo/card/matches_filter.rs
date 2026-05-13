use crate::extensions::expo::card::model::ExpoCardModel;

/// Returns true when an Expo card matches the current filter text.
pub fn expo_card_matches_filter(card: &ExpoCardModel, filter: &str) -> bool {
    let needle = filter.trim().to_lowercase();
    if needle.is_empty() {
        return true;
    }
    card.title.to_lowercase().contains(&needle)
        || card
            .preview
            .observations
            .iter()
            .any(|observation| observation.to_lowercase().contains(&needle))
}

#[cfg(test)]
mod tests {
    use super::expo_card_matches_filter;
    use crate::extensions::expo::card::model::ExpoCardModel;
    use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

    /// Verifies filtering matches conversation titles.
    #[test]
    fn matches_title() {
        let card = card("Build cache", vec![]);

        assert!(expo_card_matches_filter(&card, "cache"));
    }

    /// Verifies filtering matches observation text.
    #[test]
    fn matches_observation() {
        let card = card("Build", vec!["Observed slow startup"]);

        assert!(expo_card_matches_filter(&card, "startup"));
    }

    /// Builds a card model for filter tests.
    fn card(title: &str, observations: Vec<&str>) -> ExpoCardModel {
        ExpoCardModel {
            session_index: 0,
            title: title.to_string(),
            preview: ConversationObservationPreview {
                has_more: false,
                observations: observations.into_iter().map(str::to_string).collect(),
            },
        }
    }
}
