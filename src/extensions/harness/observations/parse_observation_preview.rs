use serde::Deserialize;

use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;

#[derive(Debug, Deserialize)]
struct ObservationStateJson {
    topics: Vec<ObservationTopicJson>,
}

#[derive(Debug, Deserialize)]
struct ObservationTopicJson {
    title: String,
}

/// Parses Nexus observation JSON into topic titles for the Expo preview box.
pub fn parse_observation_preview(
    input: &str,
    _session_title: &str,
) -> serde_json::Result<ConversationObservationPreview> {
    let state: ObservationStateJson = serde_json::from_str(input)?;
    Ok(ConversationObservationPreview {
        has_more: false,
        observations: observation_topic_titles(state.topics),
    })
}

/// Returns observation topic titles while ignoring raw message bullets.
fn observation_topic_titles(topics: Vec<ObservationTopicJson>) -> Vec<String> {
    topics.into_iter().map(|topic| topic.title).collect()
}

#[cfg(test)]
mod tests {
    use super::parse_observation_preview;

    /// Verifies the preview contains observation topic titles, not raw message bullets.
    #[test]
    fn keeps_observation_topic_titles() {
        let input = r#"{"topics":[{"title":"Target","assistantBullets":["one","two"]},{"title":"Other","assistantBullets":["three","four"]}]}"#;

        let preview = parse_observation_preview(input, "Session title").unwrap();

        assert!(!preview.has_more);
        assert_eq!(preview.observations, vec!["Target", "Other"]);
    }

    /// Reproduces Expo preview boxes showing user-message bullets instead of observation topics.
    #[test]
    fn ignores_raw_user_message_bullets() {
        let input = r#"{"topics":[{"title":"Check file watcher availability","userBullets":["Can you check whether file watching exists?"]},{"title":"Add split session keybindings","userBullets":["Please wire Ctrl+] for splits"]}]}"#;

        let preview =
            parse_observation_preview(input, "Can you check whether file watching exists?")
                .unwrap();

        assert_eq!(
            preview.observations,
            vec![
                "Check file watcher availability",
                "Add split session keybindings"
            ]
        );
    }

    /// Verifies consolidated observation JSON metadata is ignored while topics are parsed.
    #[test]
    fn parses_consolidated_observation_json() {
        let input = r#"{"conversationId":"2026-05-14T11-19-44-700Z_abc","cwd":"/tmp/project","summary":"done","topics":[{"index":1,"title":"Target","assistantBullets":["one"]}]}"#;

        let preview = parse_observation_preview(input, "Session title").unwrap();

        assert_eq!(preview.observations, vec!["Target"]);
    }

    /// Verifies topics with empty assistant observation lists still show their title.
    #[test]
    fn handles_empty_observations() {
        let input = r#"{"topics":[{"title":"Target","assistantBullets":[]}]}"#;

        let preview = parse_observation_preview(input, "Session title").unwrap();

        assert!(!preview.has_more);
        assert_eq!(preview.observations, vec!["Target"]);
    }
}
