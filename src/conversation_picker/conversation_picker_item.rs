use std::path::PathBuf;

/// Selectable action represented by one picker display row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConversationPickerItemKind {
    Folder { path: PathBuf },
    Session { index: usize },
}

/// Display data for one row in the conversation picker.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationPickerItem {
    pub title: String,
    pub subtitle: String,
    pub is_active: bool,
    pub kind: ConversationPickerItemKind,
}

impl ConversationPickerItem {
    /// Returns whether this picker row represents a parent folder action.
    pub fn is_folder(&self) -> bool {
        matches!(self.kind, ConversationPickerItemKind::Folder { .. })
    }
}
