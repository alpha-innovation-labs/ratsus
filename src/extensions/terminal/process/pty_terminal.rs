use std::ffi::OsString;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ratatui::{buffer::Buffer, layout::Rect, Frame};
use ratkit::primitives::termtui::{render_screen, Parser, Screen};
use ratkit::RedrawSignal;

use crate::extensions::terminal::process::spawn_terminal_reader::spawn_terminal_reader;
use crate::extensions::terminal::session::chat_terminal::TerminalCursorState;

/// Embedded PTY-backed terminal used by real backends.
pub struct PtyTerminal {
    parser: Arc<Mutex<Parser>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    redraw_signal: RedrawSignal,
    child: Box<dyn Child + Send + Sync>,
}

impl PtyTerminal {
    /// Spawns a command in a specific directory inside a PTY sized for the terminal pane.
    pub fn spawn_with_command_in_dir(
        command: &str,
        args: &[&str],
        working_dir: &Path,
        rows: u16,
        cols: u16,
    ) -> Result<Self> {
        let mut cmd = CommandBuilder::new(command);
        for arg in args {
            cmd.arg(arg);
        }
        Self::spawn_command_builder_in_dir(cmd, working_dir, rows, cols)
    }

    /// Spawns a PTY-backed terminal argv in a specific directory.
    pub fn spawn_argv_in_dir(
        argv: Vec<OsString>,
        working_dir: &Path,
        rows: u16,
        cols: u16,
    ) -> Result<Self> {
        Self::spawn_command_builder_in_dir(CommandBuilder::from_argv(argv), working_dir, rows, cols)
    }

    /// Spawns a prepared command builder in a PTY sized for the terminal pane.
    fn spawn_command_builder_in_dir(
        mut cmd: CommandBuilder,
        working_dir: &Path,
        rows: u16,
        cols: u16,
    ) -> Result<Self> {
        let pty_system = native_pty_system();
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        cmd.env("TERM", "xterm-256color");
        cmd.cwd(working_dir);
        let child = pair.slave.spawn_command(cmd)?;
        let master = pair.master;
        let reader = master.try_clone_reader()?;
        let writer = master.take_writer()?;
        let writer = Arc::new(Mutex::new(writer));
        let parser = Arc::new(Mutex::new(Parser::new(rows, cols, 10000)));
        let redraw_signal = RedrawSignal::new();
        redraw_signal.request_redraw();
        spawn_terminal_reader(&parser, &writer, &redraw_signal, reader);
        Ok(Self {
            parser,
            writer,
            master: Arc::new(Mutex::new(master)),
            redraw_signal,
            child,
        })
    }

    /// Returns whether the backing child process has exited.
    pub fn has_exited(&mut self) -> bool {
        self.child.try_wait().ok().flatten().is_some()
    }

    /// Terminates the backing child process.
    pub fn kill(&mut self) {
        let _ = self.child.kill();
    }

    /// Resizes both the parser and backing PTY.
    pub fn resize(&mut self, rows: u16, cols: u16) {
        if rows == 0 || cols == 0 {
            return;
        }
        if let Ok(mut parser) = self.parser.lock() {
            parser.set_size(rows, cols);
        }
        if let Ok(master) = self.master.lock() {
            let _ = master.resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            });
        }
        self.redraw_signal.request_redraw();
    }

    /// Renders the terminal screen into the requested frame area.
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        self.render_to_buffer(area, frame.buffer_mut());
    }

    /// Renders the terminal screen into a buffer without cloning the screen.
    pub fn render_to_buffer(&self, area: Rect, buffer: &mut Buffer) {
        if let Ok(parser) = self.parser.lock() {
            render_screen(parser.screen(), area, buffer);
        }
    }

    /// Writes encoded input bytes into the PTY.
    pub fn write_input(&self, bytes: &[u8]) {
        if let Ok(mut writer) = self.writer.lock() {
            let _ = writer.write_all(bytes);
            let _ = writer.flush();
        }
    }

    /// Scrolls terminal history toward older output by the requested row count.
    pub fn scrollback_up(&self, rows: usize) {
        if let Ok(mut parser) = self.parser.lock() {
            parser.screen.scroll_screen_up(rows);
        }
        self.redraw_signal.request_redraw();
    }

    /// Scrolls terminal history toward newer output by the requested row count.
    pub fn scrollback_down(&self, rows: usize) {
        if let Ok(mut parser) = self.parser.lock() {
            parser.screen.scroll_screen_down(rows);
        }
        self.redraw_signal.request_redraw();
    }

    /// Returns the terminal view to the live screen at the bottom of history.
    pub fn reset_scrollback(&self) {
        if let Ok(mut parser) = self.parser.lock() {
            parser.set_scrollback(0);
        }
        self.redraw_signal.request_redraw();
    }

    /// Returns the pending redraw flag and clears it.
    pub fn take_needs_redraw(&self) -> bool {
        self.redraw_signal.take_redraw_request()
    }

    /// Returns a clone of the current terminal screen for frozen copy mode.
    pub fn screen_snapshot(&self) -> Option<Screen> {
        let parser = self.parser.lock().ok()?;
        Some(parser.screen().clone())
    }

    /// Returns cursor state when the live screen cursor should be shown.
    pub fn cursor_state(&self) -> Option<TerminalCursorState> {
        let parser = self.parser.lock().ok()?;
        let screen = parser.screen();
        if screen.hide_cursor() || screen.scrollback() > 0 {
            return None;
        }
        let (row, col) = screen.cursor_position();
        Some(TerminalCursorState {
            row,
            col,
            style: screen.cursor_style(),
        })
    }
}
