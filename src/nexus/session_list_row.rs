use std::path::PathBuf;

/// One visible row in the left Nexus session tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionListRow {
    Folder { path: PathBuf, session_count: usize },
    Session { index: usize },
}

impl SessionListRow {
    /// Returns the session index represented by this row, when this is a session row.
    pub fn session_index(&self) -> Option<usize> {
        match self {
            Self::Session { index } => Some(*index),
            Self::Folder { .. } => None,
        }
    }
}
