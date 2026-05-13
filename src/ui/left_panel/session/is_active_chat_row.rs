use crate::extensions::file_viewer::tabs::tab::MainPaneTab;

/// Returns true when a session row should use the active chat highlight.
pub fn session_is_active_chat_row(tab: MainPaneTab, index: usize, active_index: usize) -> bool {
    tab == MainPaneTab::Chat && index == active_index
}

#[cfg(test)]
mod tests {
    use super::session_is_active_chat_row;
    use crate::extensions::file_viewer::tabs::tab::MainPaneTab;

    /// Verifies active chat highlighting is disabled while Expo is visible.
    #[test]
    fn disables_active_chat_highlight_for_expo() {
        assert!(!session_is_active_chat_row(MainPaneTab::Expo, 0, 0));
    }

    /// Verifies active chat highlighting remains enabled for the Chat tab.
    #[test]
    fn enables_active_chat_highlight_for_chat() {
        assert!(session_is_active_chat_row(MainPaneTab::Chat, 0, 0));
    }
}
