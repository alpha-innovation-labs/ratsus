use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crate::app::deletion::delete_sessions_result::DeleteSessionsResult;
use crate::extensions::harness::core::chat_harness::ChatHarness;

/// Spawns parallel deletion of chat sessions on a background thread.
pub fn spawn_delete_sessions_worker(
    chat_harness: Arc<dyn ChatHarness>,
    chat_session_ids: Vec<String>,
) -> Receiver<DeleteSessionsResult> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let result = delete_chat_sessions_in_parallel(chat_harness, chat_session_ids);
        let _ = sender.send(result);
    });
    receiver
}

/// Deletes chat sessions concurrently and returns all outcomes.
fn delete_chat_sessions_in_parallel(
    chat_harness: Arc<dyn ChatHarness>,
    chat_session_ids: Vec<String>,
) -> DeleteSessionsResult {
    let handles = chat_session_ids
        .into_iter()
        .map(|session_id| spawn_one_delete(chat_harness.clone(), session_id))
        .collect::<Vec<_>>();
    collect_delete_results(handles)
}

/// Spawns one harness delete command for one chat session.
fn spawn_one_delete(
    chat_harness: Arc<dyn ChatHarness>,
    session_id: String,
) -> JoinHandle<Result<String, (String, String)>> {
    thread::spawn(
        move || match chat_harness.delete_chat_session(&session_id) {
            Ok(()) => Ok(session_id),
            Err(error) => {
                let message = format!("Failed to delete session {session_id}: {error}");
                Err((session_id, message))
            }
        },
    )
}

/// Collects all parallel delete results.
fn collect_delete_results(
    handles: Vec<JoinHandle<Result<String, (String, String)>>>,
) -> DeleteSessionsResult {
    let mut result = DeleteSessionsResult::default();
    for handle in handles {
        collect_delete_result(handle, &mut result);
    }
    result
}

/// Adds one joined delete result into the aggregate result.
fn collect_delete_result(
    handle: JoinHandle<Result<String, (String, String)>>,
    result: &mut DeleteSessionsResult,
) {
    match handle.join() {
        Ok(Ok(session_id)) => result.deleted_chat_ids.push(session_id),
        Ok(Err((session_id, error))) => {
            result.failed_chat_ids.push(session_id);
            result.errors.push(error);
        }
        Err(_) => result.errors.push("Delete worker panicked".to_string()),
    }
}
