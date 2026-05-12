use std::path::PathBuf;

/// Tracks filter text and highlighted row for the conversation picker modal.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ConversationPickerState {
    pub is_open: bool,
    pub query: String,
    pub selected_position: usize,
    pub folder_filter: Option<PathBuf>,
}

impl ConversationPickerState {
    /// Creates closed conversation picker state with an empty filter.
    pub fn new() -> Self {
        Self::default()
    }
}
