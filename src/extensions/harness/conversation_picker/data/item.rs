use std::path::PathBuf;

/// Selectable action represented by one picker display row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryModalItemKind {
    Folder {
        path: PathBuf,
        current_session_count: usize,
        total_session_count: usize,
        is_collapsed: bool,
        has_running_session: bool,
    },
    Session {
        index: usize,
        age: String,
        icon: String,
        is_running: bool,
    },
}

/// Display and activation data for one row in the conversation picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryModalItem {
    pub title: String,
    pub is_active: bool,
    pub is_toggled: bool,
    pub kind: HistoryModalItemKind,
}

impl HistoryModalItem {
    /// Returns whether this picker row represents a parent folder action.
    pub fn is_folder(&self) -> bool {
        matches!(self.kind, HistoryModalItemKind::Folder { .. })
    }
}
