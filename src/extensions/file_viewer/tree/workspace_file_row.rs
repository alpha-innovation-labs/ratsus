use std::path::PathBuf;

/// One visible row in the workspace-grouped file tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceFileRow {
    /// Root workspace folder row.
    WorkspaceFolder { path: PathBuf },
    /// File-system entry below a workspace folder.
    Entry {
        path: PathBuf,
        depth: usize,
        is_dir: bool,
    },
}
