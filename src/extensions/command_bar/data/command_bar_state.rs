/// Tracks command bar modal state, filter text, and highlighted row.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CommandBarState {
    pub is_open: bool,
    pub query: String,
    pub is_filtering: bool,
    pub selected_position: usize,
    pub pending_g: bool,
}

impl CommandBarState {
    /// Creates closed command bar state with an empty filter.
    pub fn new() -> Self {
        Self::default()
    }
}
