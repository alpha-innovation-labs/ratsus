/// Session-surface mode shown in the shared left pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeftPaneMode {
    Sessions,
    Plans,
}

impl Default for LeftPaneMode {
    /// Defaults the left pane to the existing session list.
    fn default() -> Self {
        Self::Sessions
    }
}
