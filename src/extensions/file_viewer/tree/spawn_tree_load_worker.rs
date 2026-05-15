use std::io;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use ratkit::widgets::file_system_tree::FileSystemTree;

use crate::extensions::file_viewer::tree::tree_load_result::TreeLoadResult;

/// Loads the visible root file tree on a background thread.
pub fn spawn_tree_load_worker(root: PathBuf) -> Receiver<TreeLoadResult> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let tree = FileSystemTree::new(root.clone())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let _ = sender.send(TreeLoadResult { root, tree });
    });
    receiver
}
