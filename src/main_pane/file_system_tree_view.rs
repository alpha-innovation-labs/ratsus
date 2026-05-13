use std::io;
use std::path::PathBuf;

use ratatui::layout::Rect;
use ratkit::widgets::file_system_tree::{FileSystemTree, FileSystemTreeState};
use ratkit::widgets::markdown_preview::MarkdownWidget;

use crate::main_pane::file_preview_markdown_for_path::file_preview_markdown_for_path;
use crate::main_pane::markdown_widget_for_content::markdown_widget_for_content;

/// File-system tree state copied from the Ratkit file system tree demo.
pub struct FileSystemTreeView {
    pub tree: FileSystemTree<'static>,
    pub state: FileSystemTreeState,
    pub last_selection: String,
    pub last_tree_area: Rect,
    pub preview: MarkdownWidget<'static>,
}

impl FileSystemTreeView {
    /// Builds the file-system tree from the current working directory.
    pub fn new() -> io::Result<Self> {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::with_root(root)
    }

    /// Builds the file-system tree from an explicit root directory.
    pub fn with_root(root: PathBuf) -> io::Result<Self> {
        let tree = FileSystemTree::new(root.clone())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let mut state = FileSystemTreeState::new();
        state.select(vec![0]);
        let preview = markdown_widget_for_content(file_preview_markdown_for_path(&root, true));

        Ok(Self {
            tree,
            state,
            last_selection: root.display().to_string(),
            last_tree_area: Rect::default(),
            preview,
        })
    }
}
