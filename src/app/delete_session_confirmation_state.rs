/// Pending delete confirmation modal state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeleteSessionConfirmationState {
    pub session_index: Option<usize>,
    pub session_id: Option<String>,
    pub session_title: Option<String>,
    pub session_ids: Vec<String>,
    pub session_titles: Vec<String>,
    pub is_deleting: bool,
    pub preferred_focus_row: usize,
}

impl DeleteSessionConfirmationState {
    /// Opens delete confirmation for one session identity and current index.
    pub fn open(&mut self, session_index: usize, session_id: String, session_title: String) {
        self.open_many(session_index, vec![(session_id, session_title)]);
    }

    /// Opens delete confirmation for one or more session identities.
    pub fn open_many(&mut self, session_index: usize, sessions: Vec<(String, String)>) {
        self.session_index = Some(session_index);
        self.session_id = sessions.first().map(|session| session.0.clone());
        self.session_title = sessions.first().map(|session| session.1.clone());
        self.session_ids = sessions.iter().map(|session| session.0.clone()).collect();
        self.session_titles = sessions.into_iter().map(|session| session.1).collect();
        self.is_deleting = false;
        self.preferred_focus_row = 0;
    }

    /// Marks the confirmation as actively deleting and stores the post-delete focus row.
    pub fn start_deleting(&mut self, preferred_focus_row: usize) {
        self.is_deleting = true;
        self.preferred_focus_row = preferred_focus_row;
    }

    /// Stops the deleting state while keeping the confirmation open.
    pub fn stop_deleting(&mut self) {
        self.is_deleting = false;
    }

    /// Closes any pending delete confirmation.
    pub fn close(&mut self) {
        self.session_index = None;
        self.session_id = None;
        self.session_title = None;
        self.session_ids.clear();
        self.session_titles.clear();
        self.is_deleting = false;
        self.preferred_focus_row = 0;
    }

    /// Returns whether a delete confirmation is currently open.
    pub fn is_open(&self) -> bool {
        !self.session_ids.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::DeleteSessionConfirmationState;

    /// Verifies opening and closing delete confirmation state.
    #[test]
    fn opens_and_closes_confirmation() {
        let mut state = DeleteSessionConfirmationState::default();
        state.open(2, "session-2".to_string(), "Title".to_string());
        assert_eq!(state.session_index, Some(2));
        assert_eq!(state.session_id.as_deref(), Some("session-2"));
        assert_eq!(state.session_title.as_deref(), Some("Title"));
        assert_eq!(state.session_ids, vec!["session-2"]);
        state.close();
        assert!(!state.is_open());
    }

    /// Verifies bulk delete confirmation stores every selected session id.
    #[test]
    fn opens_bulk_confirmation() {
        let mut state = DeleteSessionConfirmationState::default();
        state.open_many(
            1,
            vec![
                ("a".to_string(), "Title A".to_string()),
                ("b".to_string(), "Title B".to_string()),
            ],
        );

        assert_eq!(state.session_ids, vec!["a", "b"]);
        assert_eq!(state.session_titles, vec!["Title A", "Title B"]);
    }
}
