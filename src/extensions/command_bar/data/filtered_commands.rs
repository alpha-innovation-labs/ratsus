use crate::extensions::command_bar::command::command_id::CommandBarCommandId;
use crate::extensions::command_bar::data::command_item::CommandBarItem;

/// Returns command bar rows matching the current query.
pub fn filtered_command_bar_items(query: &str) -> Vec<CommandBarItem> {
    command_bar_items()
        .into_iter()
        .filter(|item| command_matches_query(item, query))
        .collect()
}

/// Returns all command bar rows in display order.
pub fn command_bar_items() -> Vec<CommandBarItem> {
    vec![
        CommandBarItem::new(
            CommandBarCommandId::OpenConversationHistory,
            "Chat history",
            "Ctrl+H",
        ),
        CommandBarItem::new(
            CommandBarCommandId::CycleNextSession,
            "Next session",
            "Ctrl+Tab",
        ),
        CommandBarItem::new(
            CommandBarCommandId::CyclePreviousSession,
            "Previous session",
            "Ctrl+Shift+Tab",
        ),
        CommandBarItem::new(
            CommandBarCommandId::OpenFocusedConversationExpo,
            "Toggle Expo",
            "Ctrl+E",
        ),
        CommandBarItem::new(
            CommandBarCommandId::SplitVertical,
            "Split vertical",
            "Ctrl+]",
        ),
        CommandBarItem::new(
            CommandBarCommandId::SplitHorizontal,
            "Split horizontal",
            "Ctrl+[",
        ),
        CommandBarItem::new(
            CommandBarCommandId::PlaceVertical,
            "Place in vertical split",
            "Ctrl+Shift+]",
        ),
        CommandBarItem::new(
            CommandBarCommandId::PlaceHorizontal,
            "Place in horizontal split",
            "Ctrl+Shift+[",
        ),
        workspace_command_bar_item(0),
        workspace_command_bar_item(1),
        workspace_command_bar_item(2),
        workspace_command_bar_item(3),
        workspace_command_bar_item(4),
        workspace_command_bar_item(5),
        workspace_command_bar_item(6),
        workspace_command_bar_item(7),
        workspace_command_bar_item(8),
        CommandBarItem::new(CommandBarCommandId::StartChat, "New chat", "Ctrl+N"),
        CommandBarItem::new(CommandBarCommandId::StartTerminal, "New terminal", "Ctrl+T"),
        CommandBarItem::new(
            CommandBarCommandId::ToggleLeftPane,
            "Toggle left pane",
            "Ctrl+L",
        ),
        CommandBarItem::new(
            CommandBarCommandId::ToggleFocusedPane,
            "Toggle focus",
            "Ctrl+X",
        ),
        CommandBarItem::new(
            CommandBarCommandId::ToggleWorkspaceView,
            "Toggle workspace view",
            "",
        ),
        CommandBarItem::new(CommandBarCommandId::Quit, "Quit", "Ctrl+Q"),
    ]
}

/// Builds one workspace-selection command row for a zero-based workspace index.
fn workspace_command_bar_item(index: usize) -> CommandBarItem {
    let title = match index {
        0 => "Select workspace 1",
        1 => "Select workspace 2",
        2 => "Select workspace 3",
        3 => "Select workspace 4",
        4 => "Select workspace 5",
        5 => "Select workspace 6",
        6 => "Select workspace 7",
        7 => "Select workspace 8",
        8 => "Select workspace 9",
        _ => "Select workspace",
    };
    let hotkey = match index {
        0 => "Ctrl+1",
        1 => "Ctrl+2",
        2 => "Ctrl+3",
        3 => "Ctrl+4",
        4 => "Ctrl+5",
        5 => "Ctrl+6",
        6 => "Ctrl+7",
        7 => "Ctrl+8",
        8 => "Ctrl+9",
        _ => "Ctrl+?",
    };
    CommandBarItem::new(CommandBarCommandId::SelectWorkspace(index), title, hotkey)
}

/// Returns whether a command row matches the normalized query.
fn command_matches_query(item: &CommandBarItem, query: &str) -> bool {
    let normalized = query.trim().to_lowercase();
    if normalized.is_empty() {
        return true;
    }
    item.title.to_lowercase().contains(&normalized)
        || item.hotkey.to_lowercase().contains(&normalized)
}

#[cfg(test)]
mod tests {
    use super::{command_bar_items, filtered_command_bar_items};
    use crate::extensions::command_bar::command::command_id::CommandBarCommandId;

    /// Verifies the command bar exposes global commands and their display hotkeys.
    #[test]
    fn exposes_history_and_split_commands() {
        let items = command_bar_items();

        assert!(items.iter().any(
            |item| item.id == CommandBarCommandId::OpenConversationHistory
                && item.hotkey == "Ctrl+H"
        ));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::CycleNextSession
                && item.title == "Next session"
                && item.hotkey == "Ctrl+Tab"));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::CyclePreviousSession
                && item.title == "Previous session"
                && item.hotkey == "Ctrl+Shift+Tab"));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::SplitVertical
                && item.title == "Split vertical"));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::ToggleWorkspaceView
                && item.title == "Toggle workspace view"
                && item.hotkey.is_empty()));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::SelectWorkspace(0)
                && item.title == "Select workspace 1"
                && item.hotkey == "Ctrl+1"));
    }

    /// Verifies command filtering matches command titles.
    #[test]
    fn filters_commands_by_title() {
        let items = filtered_command_bar_items("history");

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, CommandBarCommandId::OpenConversationHistory);
    }
}
