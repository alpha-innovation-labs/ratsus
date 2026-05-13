use std::sync::{Arc, Mutex};

use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use ratkit::primitives::termtui::Screen;
use ratkit::RedrawSignal;

use crate::terminal::chat_terminal::TerminalCursorState;

/// Deterministic terminal used by stub harness sessions.
pub struct StubTerminal {
    title: String,
    body: String,
    input_log: Arc<Mutex<Vec<String>>>,
    redraw_signal: RedrawSignal,
    exited: bool,
}

impl StubTerminal {
    /// Creates a fake terminal with static display content.
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        let redraw_signal = RedrawSignal::new();
        redraw_signal.request_redraw();
        Self {
            title: title.into(),
            body: body.into(),
            input_log: Arc::new(Mutex::new(Vec::new())),
            redraw_signal,
            exited: false,
        }
    }

    /// Returns whether this fake terminal has been marked exited.
    pub fn has_exited(&mut self) -> bool {
        self.exited
    }

    /// Marks the fake terminal as exited.
    pub fn kill(&mut self) {
        self.exited = true;
        self.redraw_signal.request_redraw();
    }

    /// Stores the requested terminal size for deterministic rendering.
    pub fn resize(&mut self, _rows: u16, _cols: u16) {
        self.redraw_signal.request_redraw();
    }

    /// Renders fake terminal content without spawning a process.
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let mut lines = vec![Line::from(self.body.clone())];
        if let Ok(input_log) = self.input_log.lock() {
            lines.extend(
                input_log
                    .iter()
                    .rev()
                    .take(5)
                    .map(|input| Line::from(input.clone())),
            );
        }
        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(self.title.clone())
                    .borders(Borders::ALL),
            )
            .wrap(Wrap { trim: false });
        frame.render_widget(paragraph, area);
    }

    /// Records input bytes as displayable text for local feedback.
    pub fn write_input(&self, bytes: &[u8]) {
        if let Ok(mut input_log) = self.input_log.lock() {
            input_log.push(format!("input: {}", String::from_utf8_lossy(bytes)));
        }
        self.redraw_signal.request_redraw();
    }

    /// Keeps API parity with PTY terminals.
    pub fn scrollback_up(&self, _rows: usize) {
        self.redraw_signal.request_redraw();
    }

    /// Keeps API parity with PTY terminals.
    pub fn scrollback_down(&self, _rows: usize) {
        self.redraw_signal.request_redraw();
    }

    /// Keeps API parity with PTY terminals.
    pub fn reset_scrollback(&self) {
        self.redraw_signal.request_redraw();
    }

    /// Returns and clears the pending redraw flag.
    pub fn take_needs_redraw(&self) -> bool {
        self.redraw_signal.take_redraw_request()
    }

    /// Fake terminals do not expose parser snapshots yet.
    pub fn screen_snapshot(&self) -> Option<Screen> {
        None
    }

    /// Fake terminals do not render a cursor.
    pub fn cursor_state(&self) -> Option<TerminalCursorState> {
        None
    }
}
