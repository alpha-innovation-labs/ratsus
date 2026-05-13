use crate::harness::chat_session::ChatSession;

/// Merges runtime registry metadata without clobbering stable session listing fields.
pub fn merge_registry_session_metadata(
    existing: &ChatSession,
    registry: ChatSession,
) -> ChatSession {
    let mut merged = existing.clone();
    merged.is_running = registry.is_running;
    if registry_title_is_specific(&registry.title) {
        merged.title = registry.title;
    }
    merged
}

/// Returns true when a registry title should replace the stable listing title.
fn registry_title_is_specific(title: &str) -> bool {
    !title.trim().is_empty() && title != "New Session"
}

#[cfg(test)]
mod tests {
    use super::merge_registry_session_metadata;
    use crate::harness::chat_session::ChatSession;

    /// Verifies placeholder registry titles do not replace stable session titles.
    #[test]
    fn keeps_existing_title_when_registry_title_is_placeholder() {
        let existing = ChatSession::new("old", "Real title", "id", "/tmp/project");
        let registry = ChatSession::new("new", "New Session", "id", "/tmp/project");

        let merged = merge_registry_session_metadata(&existing, registry);

        assert_eq!(merged.title, "Real title");
    }

    /// Verifies running status still comes from registry metadata.
    #[test]
    fn applies_registry_running_status() {
        let existing = ChatSession::new("old", "Real title", "id", "/tmp/project");
        let registry =
            ChatSession::new("new", "New Session", "id", "/tmp/project").with_running(true);

        let merged = merge_registry_session_metadata(&existing, registry);

        assert!(merged.is_running);
    }
}
