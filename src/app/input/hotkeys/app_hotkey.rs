use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Semantic top-level shortcut resolved from the Ratkit hotkey registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppHotkey {
    /// Move the active chat selection by a signed offset.
    CycleChat(isize),
    /// Open the conversation picker modal.
    OpenConversationPicker,
    /// Open Expo for the focused conversation.
    OpenFocusedConversationExpo,
    /// Open the split-placement picker for the active terminal pane.
    PlaceConversationInActiveSplit,
    /// Split the active terminal pane in one direction.
    SplitTerminal(TerminalSplitDirection),
    /// Start a new harness-backed chat session.
    StartChat,
    /// Start a new normal terminal session.
    StartTerminal,
    /// Toggle left-pane visibility.
    ToggleLeftPane,
    /// Toggle keyboard focus between app panes.
    ToggleFocusedPane,
    /// Quit the application.
    Quit,
}
