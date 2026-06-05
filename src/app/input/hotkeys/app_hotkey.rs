use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Semantic top-level shortcut resolved from the Ratkit hotkey registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppHotkey {
    /// Move the active session selection by a signed offset.
    CycleSession(isize),
    /// Cycle the shared left pane through sessions, plans, and files.
    CycleLeftPaneMode,
    /// Open the command bar modal.
    OpenCommandBar,
    /// Open the conversation picker modal.
    OpenConversationPicker,
    /// Open Expo for the focused conversation.
    OpenFocusedConversationExpo,
    /// Open the split-placement picker with the requested split direction.
    PlaceConversationInActiveSplit(TerminalSplitDirection),
    /// Split the active terminal pane in one direction.
    SplitTerminal(TerminalSplitDirection),
    /// Select a workspace by visible zero-based index.
    SelectWorkspace(usize),
    /// Configure Ghostty so Ratsus can receive app shortcuts.
    SetupGhosttyConfig,
    /// Start a new harness-backed chat session.
    StartChat,
    /// Start a new normal terminal session.
    StartTerminal,
    /// Toggle left-pane visibility.
    ToggleLeftPane,
    /// Toggle keyboard focus between app panes.
    ToggleFocusedPane,
    /// Toggle between workspace-pane and legacy all-folders left-pane modes.
    ToggleWorkspaceView,
    /// Open delete confirmation for the focused session.
    DeleteFocusedSession,
    /// Quit the application.
    Quit,
}
