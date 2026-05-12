use std::io;
use std::path::PathBuf;

use ratkit::widgets::file_system_tree::{FileSystemTree, FileSystemTreeState};

/// File-system tree state copied from the Ratkit file system tree demo.
pub struct FileSystemTreeView {
    pub tree: FileSystemTree<'static>,
    pub state: FileSystemTreeState,
    pub last_selection: String,
}

impl FileSystemTreeView {
    /// Builds the file-system tree from the current working directory.
    pub fn new() -> io::Result<Self> {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let tree =
            FileSystemTree::new(root).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let mut state = FileSystemTreeState::new();
        state.select(vec![0]);

        Ok(Self {
            tree,
            state,
            last_selection: "No selection".to_string(),
        })
    }
}
