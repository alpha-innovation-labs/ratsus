use crate::extensions::harness::core::chat_session::ChatSession;

/// Merges runtime chat status metadata without clobbering stable session listing fields.
pub fn merge_status_session_metadata(existing: &ChatSession, status: ChatSession) -> ChatSession {
    let mut merged = existing.clone();
    merged.is_running = status.is_running;
    if status_title_is_specific(&status.title) {
        merged.title = status.title;
    }
    merged
}

/// Returns true when a chat status title should replace the stable listing title.
fn status_title_is_specific(title: &str) -> bool {
    !title.trim().is_empty() && title != "New Session"
}

#[cfg(test)]
mod tests {
    use super::merge_status_session_metadata;
    use crate::extensions::harness::core::chat_session::ChatSession;

    /// Verifies placeholder chat status titles do not replace stable session titles.
    #[test]
    fn keeps_existing_title_when_status_title_is_placeholder() {
        let existing = ChatSession::new("old", "Real title", "id", "/tmp/project");
        let status = ChatSession::new("new", "New Session", "id", "/tmp/project");

        let merged = merge_status_session_metadata(&existing, status);

        assert_eq!(merged.title, "Real title");
    }

    /// Verifies running status still comes from chat status metadata.
    #[test]
    fn applies_chat_status_running_status() {
        let existing = ChatSession::new("old", "Real title", "id", "/tmp/project");
        let status =
            ChatSession::new("new", "New Session", "id", "/tmp/project").with_running(true);

        let merged = merge_status_session_metadata(&existing, status);

        assert!(merged.is_running);
    }
}
