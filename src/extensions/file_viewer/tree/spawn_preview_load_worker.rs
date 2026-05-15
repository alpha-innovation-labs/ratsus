use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;

use crate::extensions::file_viewer::tree::preview_load_result::PreviewLoadResult;

/// Loads selected file preview content on a background thread.
pub fn spawn_preview_load_worker(path: PathBuf) -> Receiver<PreviewLoadResult> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let content = std::fs::read_to_string(&path);
        let _ = sender.send(PreviewLoadResult { path, content });
    });
    receiver
}
