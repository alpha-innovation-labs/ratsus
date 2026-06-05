use std::path::PathBuf;

use crate::extensions::history_modal::data::mode::ConversationPickerMode;

/// Tracks filter text, filter mode, and highlighted row for the conversation picker modal.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ConversationPickerState {
    pub is_open: bool,
    pub query: String,
    pub is_filtering: bool,
    pub selected_position: usize,
    pub pending_g: bool,
    pub folder_filter: Option<PathBuf>,
    pub mode: ConversationPickerMode,
    pub mouse_down_position: Option<usize>,
    pub mouse_drag_moved: bool,
}

impl ConversationPickerState {
    /// Creates closed conversation picker state with an empty filter.
    pub fn new() -> Self {
        Self::default()
    }
}
