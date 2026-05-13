use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::apply_conversation_picker_query_change::apply_conversation_picker_query_change;
use crate::conversation_picker::close_conversation_picker::close_conversation_picker;

/// Handles staged Escape behavior for the conversation picker.
pub fn handle_conversation_picker_escape(app: &mut NexusDemo) {
    if app.conversation_picker.is_filtering {
        app.conversation_picker.is_filtering = false;
        return;
    }
    if !app.conversation_picker.query.is_empty() {
        app.conversation_picker.query.clear();
        apply_conversation_picker_query_change(app);
        return;
    }
    close_conversation_picker(app);
}

#[cfg(test)]
mod tests {
    use super::handle_conversation_picker_escape;
    use crate::test_support::dormant_session::dormant_session;
    use crate::test_support::nexus_demo_fixture::nexus_demo_fixture;

    /// First Escape exits filter-entry mode while preserving query results.
    #[test]
    fn first_escape_keeps_filter_query() -> anyhow::Result<()> {
        let mut app = nexus_demo_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.conversation_picker.is_open = true;
        app.conversation_picker.is_filtering = true;
        app.conversation_picker.query = "alpha".to_string();

        handle_conversation_picker_escape(&mut app);

        assert!(app.conversation_picker.is_open);
        assert!(!app.conversation_picker.is_filtering);
        assert_eq!(app.conversation_picker.query, "alpha");
        Ok(())
    }

    /// Second Escape clears a preserved query without closing the modal.
    #[test]
    fn second_escape_clears_filter_query() -> anyhow::Result<()> {
        let mut app = nexus_demo_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.conversation_picker.is_open = true;
        app.conversation_picker.query = "alpha".to_string();

        handle_conversation_picker_escape(&mut app);

        assert!(app.conversation_picker.is_open);
        assert!(app.conversation_picker.query.is_empty());
        Ok(())
    }

    /// Escape closes the modal only when no filter is active or preserved.
    #[test]
    fn closes_when_no_filter_query_exists() -> anyhow::Result<()> {
        let mut app = nexus_demo_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.conversation_picker.is_open = true;

        handle_conversation_picker_escape(&mut app);

        assert!(!app.conversation_picker.is_open);
        Ok(())
    }
}
