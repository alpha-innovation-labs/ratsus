use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::actions::apply_query_change::apply_history_modal_query_change;
use crate::extensions::history_modal::actions::close::close_history_modal;

/// Handles staged Escape behavior for the conversation picker.
pub fn handle_history_modal_escape(app: &mut AppState) {
    if app.history_modal.is_filtering {
        app.history_modal.is_filtering = false;
        return;
    }
    if !app.history_modal.query.is_empty() {
        app.history_modal.query.clear();
        apply_history_modal_query_change(app);
        return;
    }
    close_history_modal(app);
}

#[cfg(test)]
mod tests {
    use super::handle_history_modal_escape;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;

    /// First Escape exits filter-entry mode while preserving query results.
    #[test]
    fn first_escape_keeps_filter_query() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.history_modal.is_open = true;
        app.history_modal.is_filtering = true;
        app.history_modal.query = "alpha".to_string();

        handle_history_modal_escape(&mut app);

        assert!(app.history_modal.is_open);
        assert!(!app.history_modal.is_filtering);
        assert_eq!(app.history_modal.query, "alpha");
        Ok(())
    }

    /// Second Escape clears a preserved query without closing the modal.
    #[test]
    fn second_escape_clears_filter_query() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.history_modal.is_open = true;
        app.history_modal.query = "alpha".to_string();

        handle_history_modal_escape(&mut app);

        assert!(app.history_modal.is_open);
        assert!(app.history_modal.query.is_empty());
        Ok(())
    }

    /// Escape closes the modal only when no filter is active or preserved.
    #[test]
    fn closes_when_no_filter_query_exists() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/tmp/project")])?;
        app.history_modal.is_open = true;

        handle_history_modal_escape(&mut app);

        assert!(!app.history_modal.is_open);
        Ok(())
    }
}
