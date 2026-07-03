/// Stable command identifiers displayed and executed by the command bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandBarCommandId {
    OpenConversationHistory,
    CycleNextSession,
    CyclePreviousSession,
    CycleLeftPaneMode,
    OpenFocusedConversationExpo,
    SplitHorizontal,
    SplitVertical,
    PlaceHorizontal,
    PlaceVertical,
    SetupGhosttyConfig,
    StartChat,
    StartTerminal,
    ToggleLeftPane,
    ToggleFocusedPane,
    DeleteSession,
    Quit,
}
