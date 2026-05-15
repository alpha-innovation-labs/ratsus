use std::path::PathBuf;

use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroupId;

/// One visible row in the left chat session tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionListRow {
    Folder {
        path: PathBuf,
        current_session_count: usize,
        total_session_count: usize,
    },
    SplitGroup {
        group_id: SplitPaneSessionGroupId,
        name: String,
        child_count: usize,
    },
    SplitGroupChild {
        group_id: SplitPaneSessionGroupId,
        pane_id: PaneId,
        index: usize,
        is_last: bool,
    },
    Session {
        index: usize,
    },
    FolderMore {
        path: PathBuf,
    },
}

impl SessionListRow {
    /// Returns the session index represented by this row, when this is a session row.
    pub fn session_index(&self) -> Option<usize> {
        match self {
            Self::Session { index } | Self::SplitGroupChild { index, .. } => Some(*index),
            Self::Folder { .. } | Self::FolderMore { .. } | Self::SplitGroup { .. } => None,
        }
    }
}
