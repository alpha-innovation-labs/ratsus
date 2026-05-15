use std::path::Path;

use anyhow::Result;
use ratatui::{buffer::Buffer, layout::Rect, Frame};
use ratkit::primitives::termtui::{CursorStyle, Screen};

use crate::extensions::terminal::process::pty_terminal::PtyTerminal;
use crate::extensions::terminal::session::stub_terminal::StubTerminal;

/// Cursor data needed to render a terminal cursor in ratatui.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalCursorState {
    pub row: u16,
    pub col: u16,
    pub style: CursorStyle,
}

/// Terminal implementation used by session panes.
pub enum ChatTerminal {
    Pty(PtyTerminal),
    Stub(StubTerminal),
}

impl ChatTerminal {
    /// Spawns a PTY-backed terminal command in the requested working directory.
    pub fn spawn_with_command_in_dir(
        command: &str,
        args: &[&str],
        working_dir: &Path,
        rows: u16,
        cols: u16,
    ) -> Result<Self> {
        Ok(Self::Pty(PtyTerminal::spawn_with_command_in_dir(
            command,
            args,
            working_dir,
            rows,
            cols,
        )?))
    }

    /// Builds a deterministic fake terminal for stub harness sessions.
    pub fn stub(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self::Stub(StubTerminal::new(title, body))
    }

    /// Returns whether the backing terminal has exited.
    pub fn has_exited(&mut self) -> bool {
        match self {
            Self::Pty(terminal) => terminal.has_exited(),
            Self::Stub(terminal) => terminal.has_exited(),
        }
    }

    /// Terminates the backing terminal process when one exists.
    pub fn kill(&mut self) {
        match self {
            Self::Pty(terminal) => terminal.kill(),
            Self::Stub(terminal) => terminal.kill(),
        }
    }

    /// Resizes the terminal implementation.
    pub fn resize(&mut self, rows: u16, cols: u16) {
        match self {
            Self::Pty(terminal) => terminal.resize(rows, cols),
            Self::Stub(terminal) => terminal.resize(rows, cols),
        }
    }

    /// Renders the terminal into the requested frame area.
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        match self {
            Self::Pty(terminal) => terminal.render(frame, area),
            Self::Stub(terminal) => terminal.render(frame, area),
        }
    }

    /// Renders the terminal into a target buffer when supported.
    pub fn render_to_buffer(&self, area: Rect, buffer: &mut Buffer) -> bool {
        match self {
            Self::Pty(terminal) => {
                terminal.render_to_buffer(area, buffer);
                true
            }
            Self::Stub(_) => false,
        }
    }

    /// Writes encoded user input to the terminal implementation.
    pub fn write_input(&self, bytes: &[u8]) {
        match self {
            Self::Pty(terminal) => terminal.write_input(bytes),
            Self::Stub(terminal) => terminal.write_input(bytes),
        }
    }

    /// Scrolls terminal history toward older output.
    pub fn scrollback_up(&self, rows: usize) {
        match self {
            Self::Pty(terminal) => terminal.scrollback_up(rows),
            Self::Stub(terminal) => terminal.scrollback_up(rows),
        }
    }

    /// Scrolls terminal history toward newer output.
    pub fn scrollback_down(&self, rows: usize) {
        match self {
            Self::Pty(terminal) => terminal.scrollback_down(rows),
            Self::Stub(terminal) => terminal.scrollback_down(rows),
        }
    }

    /// Returns the terminal to the live scroll position.
    pub fn reset_scrollback(&self) {
        match self {
            Self::Pty(terminal) => terminal.reset_scrollback(),
            Self::Stub(terminal) => terminal.reset_scrollback(),
        }
    }

    /// Returns and clears the pending redraw flag.
    pub fn take_needs_redraw(&self) -> bool {
        match self {
            Self::Pty(terminal) => terminal.take_needs_redraw(),
            Self::Stub(terminal) => terminal.take_needs_redraw(),
        }
    }

    /// Returns a copy-mode snapshot when the implementation supports it.
    pub fn screen_snapshot(&self) -> Option<Screen> {
        match self {
            Self::Pty(terminal) => terminal.screen_snapshot(),
            Self::Stub(terminal) => terminal.screen_snapshot(),
        }
    }

    /// Returns cursor state when visible.
    pub fn cursor_state(&self) -> Option<TerminalCursorState> {
        match self {
            Self::Pty(terminal) => terminal.cursor_state(),
            Self::Stub(terminal) => terminal.cursor_state(),
        }
    }
}
