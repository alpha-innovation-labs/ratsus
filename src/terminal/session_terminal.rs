use std::sync::Arc;

use anyhow::{bail, Result};

use crate::copy_mode::terminal_copy_selection::TerminalCopySelection;
use crate::harness::chat_harness::ChatHarness;
use crate::harness::chat_session::ChatSession;
use crate::terminal::chat_terminal::ChatTerminal;
use crate::terminal::default_shell_command::default_shell_command;
use crate::terminal::is_normal_terminal_session::is_normal_terminal_session;

/// A chat or shell session paired with its lazily spawned terminal process.
pub struct SessionTerminal {
    pub session: ChatSession,
    pub terminal: Option<ChatTerminal>,
    pub copy_selection: TerminalCopySelection,
    pub chat_harness: Option<Arc<dyn ChatHarness>>,
}

impl SessionTerminal {
    /// Creates session state without starting the backing terminal.
    pub fn dormant(session: ChatSession) -> Self {
        Self {
            session,
            terminal: None,
            copy_selection: TerminalCopySelection::default(),
            chat_harness: None,
        }
    }

    /// Creates dormant chat session state attached to a backend harness.
    pub fn dormant_with_harness(session: ChatSession, chat_harness: Arc<dyn ChatHarness>) -> Self {
        Self {
            session,
            terminal: None,
            copy_selection: TerminalCopySelection::default(),
            chat_harness: Some(chat_harness),
        }
    }

    /// Creates session state with an already-started or fake terminal.
    pub fn with_terminal(session: ChatSession, terminal: ChatTerminal) -> Self {
        Self {
            session,
            terminal: Some(terminal),
            copy_selection: TerminalCopySelection::default(),
            chat_harness: None,
        }
    }

    /// Starts the backing terminal when it has not been started yet.
    pub fn ensure_terminal(&mut self, rows: u16, cols: u16) -> Result<&mut ChatTerminal> {
        if self.terminal.is_none() {
            self.terminal = Some(spawn_terminal_for_session(
                &self.session,
                self.chat_harness.as_ref(),
                rows,
                cols,
            )?);
        }
        Ok(self.terminal.as_mut().expect("terminal is initialized"))
    }
}

/// Spawns the correct terminal for a chat or normal terminal session.
fn spawn_terminal_for_session(
    session: &ChatSession,
    chat_harness: Option<&Arc<dyn ChatHarness>>,
    rows: u16,
    cols: u16,
) -> Result<ChatTerminal> {
    let working_dir = session.working_dir.clone();
    if is_normal_terminal_session(session) {
        let shell = default_shell_command();
        return ChatTerminal::spawn_with_command_in_dir(
            &shell,
            &[],
            &working_dir,
            rows.max(1),
            cols.max(1),
        );
    }
    if let Some(chat_harness) = chat_harness {
        return chat_harness.spawn_existing_chat(session, rows.max(1), cols.max(1));
    }
    bail!(
        "cannot spawn chat session {} without an attached chat harness",
        session.id
    )
}
