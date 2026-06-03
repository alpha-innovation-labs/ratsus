use serde::{Deserialize, Serialize};

/// Selectable tabs displayed in the main main pane title bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MainPaneTab {
    Chat,
    Files,
    Diff,
    Expo,
}

impl Default for MainPaneTab {
    /// Defaults the main pane to the chat surface.
    fn default() -> Self {
        Self::Chat
    }
}
