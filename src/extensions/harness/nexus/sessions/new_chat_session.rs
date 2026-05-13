use std::path::Path;

use uuid::Uuid;

use crate::extensions::harness::core::chat_session::ChatSession;

/// Creates placeholder metadata for a newly spawned Nexus chat terminal.
pub fn new_nexus_chat_session(working_dir: &Path) -> ChatSession {
    ChatSession::new(
        "now",
        "New Nexus chat",
        format!("new-{}", Uuid::new_v4()),
        working_dir.to_path_buf(),
    )
}
