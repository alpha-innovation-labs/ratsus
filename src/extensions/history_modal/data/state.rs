use std::path::PathBuf;

use crate::extensions::history_modal::data::mode::HistoryModalMode;

/// Backward-compatible alias for the history modal state used by app state.
pub type ConversationPickerState = HistoryModalState;

/// Tracks filter text, filter mode, and highlighted row for the conversation picker modal.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HistoryModalState {
    pub is_open: bool,
    pub query: String,
    pub is_filtering: bool,
    pub selected_position: usize,
    pub pending_g: bool,
    pub folder_filter: Option<PathBuf>,
    pub mode: HistoryModalMode,
    pub mouse_down_position: Option<usize>,
    pub mouse_drag_moved: bool,
}

impl HistoryModalState {
    /// Creates closed conversation picker state with an empty filter.
    pub fn new() -> Self {
        Self::default()
    }
}
