use std::fs;
use std::io;

use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::harnesses::nexus::observations::observation_state_path::observation_state_path;
use crate::harnesses::nexus::observations::parse_observation_preview::parse_observation_preview;

/// Loads the latest observation preview for a Nexus conversation id and title.
pub fn load_observation_preview(
    conversation_id: &str,
    session_title: &str,
) -> io::Result<ConversationObservationPreview> {
    let input = fs::read_to_string(observation_state_path(conversation_id)?)?;
    parse_observation_preview(&input, session_title)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}
