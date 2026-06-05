use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::actions::open::open_history_modal;

/// Opens the conversation picker with text filter entry already active.
pub fn open_history_modal_in_filter_mode(app: &mut AppState) {
    open_history_modal(app);
    app.history_modal.is_filtering = true;
}

#[cfg(test)]
mod tests {
    use super::open_history_modal_in_filter_mode;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::history_modal::actions::open::open_history_modal;

    /// Verifies normal picker opens still require slash before text filtering starts.
    #[test]
    fn normal_open_does_not_start_filtering() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/workspace/alpha")])?;

        open_history_modal(&mut app);

        assert!(app.history_modal.is_open);
        assert!(!app.history_modal.is_filtering);
        Ok(())
    }

    /// Verifies session-pane filter opens start directly in text filter mode.
    #[test]
    fn filter_open_starts_filtering() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![dormant_session("Alpha", "a", "/workspace/alpha")])?;

        open_history_modal_in_filter_mode(&mut app);

        assert!(app.history_modal.is_open);
        assert!(app.history_modal.is_filtering);
        Ok(())
    }
}
