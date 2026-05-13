use serde::Deserialize;

use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

#[derive(Debug, Deserialize)]
struct ObservationStateJson {
    topics: Vec<ObservationTopicJson>,
}

#[derive(Debug, Deserialize)]
struct ObservationTopicJson {
    title: String,
    #[serde(rename = "assistantBullets", default)]
    assistant_bullets: Vec<String>,
}

/// Parses Nexus observation state JSON into card bullets for the matching session title.
pub fn parse_observation_preview(
    input: &str,
    session_title: &str,
) -> serde_json::Result<ConversationObservationPreview> {
    let state: ObservationStateJson = serde_json::from_str(input)?;
    Ok(ConversationObservationPreview {
        has_more: false,
        observations: matching_topic_bullets(state.topics, session_title),
    })
}

/// Returns assistant bullets whose topic title matches the current session title.
fn matching_topic_bullets(topics: Vec<ObservationTopicJson>, session_title: &str) -> Vec<String> {
    topics
        .into_iter()
        .filter(|topic| topic.title == session_title)
        .flat_map(|topic| topic.assistant_bullets)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_observation_preview;

    /// Verifies the preview contains every assistant observation from the matching topic.
    #[test]
    fn keeps_matching_topic_observations() {
        let input = r#"{"topics":[{"title":"Target","assistantBullets":["one","two"]},{"title":"Other","assistantBullets":["three","four"]}]}"#;

        let preview = parse_observation_preview(input, "Target").unwrap();

        assert!(!preview.has_more);
        assert_eq!(preview.observations, vec!["one", "two"]);
    }

    /// Reproduces Expo cards showing bullets from unrelated topics in the same state file.
    #[test]
    fn excludes_unrelated_topic_bullets_for_titled_card() {
        let input = r#"{"topics":[{"title":"Check file watcher availability","assistantBullets":["Ratkit includes a FileWatcher service"]},{"title":"Add split session keybindings","assistantBullets":["Preparing to inspect codebase for keybinding implementation"]}]}"#;

        let preview = parse_observation_preview(input, "Add split session keybindings").unwrap();

        assert_eq!(
            preview.observations,
            vec!["Preparing to inspect codebase for keybinding implementation"]
        );
    }

    /// Verifies empty assistant observation lists parse safely.
    #[test]
    fn handles_empty_observations() {
        let input = r#"{"topics":[{"title":"Target","assistantBullets":[]}]}"#;

        let preview = parse_observation_preview(input, "Target").unwrap();

        assert!(!preview.has_more);
        assert!(preview.observations.is_empty());
    }
}
