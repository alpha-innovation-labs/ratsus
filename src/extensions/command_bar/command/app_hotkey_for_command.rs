use crate::app::input::hotkeys::app_hotkey::AppHotkey;
use crate::extensions::command_bar::command::command_id::CommandBarCommandId;
use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Maps a command bar command identifier to the existing app hotkey action.
pub fn app_hotkey_for_command(command_id: CommandBarCommandId) -> AppHotkey {
    match command_id {
        CommandBarCommandId::OpenConversationHistory => AppHotkey::OpenHistoryModal,
        CommandBarCommandId::CycleNextSession => AppHotkey::CycleSession(1),
        CommandBarCommandId::CyclePreviousSession => AppHotkey::CycleSession(-1),
        CommandBarCommandId::CycleLeftPaneMode => AppHotkey::CycleLeftPaneMode,
        CommandBarCommandId::OpenFocusedConversationExpo => AppHotkey::OpenFocusedConversationExpo,
        CommandBarCommandId::SplitHorizontal => {
            AppHotkey::SplitTerminal(TerminalSplitDirection::Bottom)
        }
        CommandBarCommandId::SplitVertical => {
            AppHotkey::SplitTerminal(TerminalSplitDirection::Right)
        }
        CommandBarCommandId::PlaceHorizontal => {
            AppHotkey::PlaceConversationInActiveSplit(TerminalSplitDirection::Bottom)
        }
        CommandBarCommandId::PlaceVertical => {
            AppHotkey::PlaceConversationInActiveSplit(TerminalSplitDirection::Right)
        }
        CommandBarCommandId::SetupGhosttyConfig => AppHotkey::SetupGhosttyConfig,
        CommandBarCommandId::StartChat => AppHotkey::StartChat,
        CommandBarCommandId::StartTerminal => AppHotkey::StartTerminal,
        CommandBarCommandId::ToggleLeftPane => AppHotkey::ToggleLeftPane,
        CommandBarCommandId::ToggleFocusedPane => AppHotkey::ToggleFocusedPane,
        CommandBarCommandId::DeleteSession => AppHotkey::DeleteFocusedSession,
        CommandBarCommandId::Quit => AppHotkey::Quit,
    }
}
