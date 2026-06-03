use serde::{Deserialize, Serialize};

/// Session-surface mode shown in the shared left pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeftPaneMode {
    Sessions,
    Plans,
    Files,
}

impl Default for LeftPaneMode {
    /// Defaults the left pane to the existing session list.
    fn default() -> Self {
        Self::Sessions
    }
}
