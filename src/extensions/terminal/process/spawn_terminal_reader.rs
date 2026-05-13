use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

use ratkit::primitives::termtui::Parser;
use ratkit::RedrawSignal;

use crate::extensions::terminal::process::process_terminal_output::process_terminal_output;

/// Spawns the background reader that feeds PTY bytes into the VT parser.
pub fn spawn_terminal_reader(
    parser: &Arc<Mutex<Parser>>,
    writer: &Arc<Mutex<Box<dyn Write + Send>>>,
    redraw_signal: &RedrawSignal,
    mut reader: Box<dyn Read + Send>,
) {
    let parser_clone = Arc::clone(parser);
    let writer_clone = Arc::clone(writer);
    let redraw_signal_clone = redraw_signal.clone();
    thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => process_terminal_output(
                    &parser_clone,
                    &writer_clone,
                    &redraw_signal_clone,
                    &buf[..n],
                ),
                Err(_) => break,
            }
        }
    });
}
