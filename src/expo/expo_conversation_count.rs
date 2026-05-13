use crate::app::nexus_demo_state::NexusDemo;
use crate::expo::expo_session_indices::expo_session_indices;

/// Returns the number of conversations in the selected Expo folder.
pub fn expo_conversation_count(app: &NexusDemo) -> usize {
    expo_session_indices(app).len()
}
