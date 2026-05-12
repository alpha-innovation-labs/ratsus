use std::io::Write;
use std::sync::{Arc, Mutex};

use ratkit::primitives::termtui::VtEvent;

/// Writes terminal protocol replies back to the PTY.
pub fn write_terminal_replies(writer: &Arc<Mutex<Box<dyn Write + Send>>>, events: Vec<VtEvent>) {
    if events.is_empty() {
        return;
    }
    if let Ok(mut writer) = writer.lock() {
        for event in events {
            if let VtEvent::Reply(reply) = event {
                let _ = writer.write_all(reply.as_bytes());
                let _ = writer.flush();
            }
        }
    }
}
