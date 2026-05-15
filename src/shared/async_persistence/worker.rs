use std::collections::HashMap;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

use crate::shared::async_persistence::save_job::SaveJob;

const SAVE_DEBOUNCE: Duration = Duration::from_millis(75);

static SAVE_SENDER: OnceLock<Sender<SaveJob>> = OnceLock::new();

/// Enqueues a debounced background text save and coalesces older saves with the same key.
pub fn enqueue_save(job: SaveJob) {
    let sender = SAVE_SENDER.get_or_init(spawn_save_worker);
    let _ = sender.send(job);
}

/// Starts the shared persistence worker thread.
fn spawn_save_worker() -> Sender<SaveJob> {
    let (sender, receiver) = mpsc::channel::<SaveJob>();
    thread::spawn(move || {
        let mut pending = HashMap::<&'static str, SaveJob>::new();
        while let Ok(job) = receiver.recv() {
            pending.insert(job.key, job);
            loop {
                match receiver.recv_timeout(SAVE_DEBOUNCE) {
                    Ok(job) => {
                        pending.insert(job.key, job);
                    }
                    Err(RecvTimeoutError::Timeout) => {
                        flush_pending(&mut pending);
                        break;
                    }
                    Err(RecvTimeoutError::Disconnected) => {
                        flush_pending(&mut pending);
                        return;
                    }
                }
            }
        }
    });
    sender
}

/// Writes all pending coalesced save jobs to disk.
fn flush_pending(pending: &mut HashMap<&'static str, SaveJob>) {
    for (_, job) in pending.drain() {
        if let Some(parent) = job.path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(job.path, job.content);
    }
}
