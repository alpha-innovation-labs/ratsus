use std::io::Write;
use std::sync::{Arc, Mutex};

use ratkit::primitives::termtui::Parser;
use ratkit::RedrawSignal;

use crate::extensions::terminal::process::write_terminal_replies::write_terminal_replies;

/// Processes PTY bytes and replies to terminal protocol requests.
pub fn process_terminal_output(
    parser: &Arc<Mutex<Parser>>,
    writer: &Arc<Mutex<Box<dyn Write + Send>>>,
    redraw_signal: &RedrawSignal,
    bytes: &[u8],
) {
    let mut events = Vec::new();
    if let Ok(mut parser) = parser.lock() {
        parser.screen.process(bytes, &mut events);
        redraw_signal.request_redraw();
    }
    write_terminal_replies(writer, events);
}
