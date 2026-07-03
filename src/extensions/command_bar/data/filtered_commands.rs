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
            CommandBarCommandId::CycleLeftPaneMode,
            "Cycle sessions, plans, files",
            "Ctrl+`",
        ),
        CommandBarItem::new(
            CommandBarCommandId::OpenFocusedConversationExpo,
            "Toggle Expo",
            "Ctrl+E",
        ),
        CommandBarItem::new(
            CommandBarCommandId::SplitVertical,
            "Split vertical",
            "Alt+N",
        ),
        CommandBarItem::new(
            CommandBarCommandId::SplitHorizontal,
            "Split horizontal",
            "Alt+Shift+N",
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
        CommandBarItem::new(
            CommandBarCommandId::SetupGhosttyConfig,
            "Setup Ghostty hotkeys",
            "",
        ),
        CommandBarItem::new(CommandBarCommandId::StartChat, "New chat", "Cmd+N"),
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
        CommandBarItem::new(CommandBarCommandId::Quit, "Quit", "Ctrl+Q"),
        CommandBarItem::new(CommandBarCommandId::DeleteSession, "Delete session", "d"),
    ]
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
            .any(|item| item.id == CommandBarCommandId::CycleLeftPaneMode
                && item.title == "Cycle sessions, plans, files"
                && item.hotkey == "Ctrl+`"));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::SplitVertical
                && item.title == "Split vertical"
                && item.hotkey == "Alt+N"));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::SplitHorizontal
                && item.title == "Split horizontal"
                && item.hotkey == "Alt+Shift+N"));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::StartChat
                && item.title == "New chat"
                && item.hotkey == "Cmd+N"));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::SetupGhosttyConfig
                && item.title == "Setup Ghostty hotkeys"
                && item.hotkey.is_empty()));
        assert!(items
            .iter()
            .any(|item| item.id == CommandBarCommandId::DeleteSession
                && item.title == "Delete session"
                && item.hotkey == "d"));
    }

    /// Verifies command filtering matches command titles.
    #[test]
    fn filters_commands_by_title() {
        let items = filtered_command_bar_items("history");

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, CommandBarCommandId::OpenConversationHistory);
    }

    /// Verifies the delete session command is reachable by title filter.
    #[test]
    fn filters_commands_by_delete_keyword() {
        let items = filtered_command_bar_items("delete");

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, CommandBarCommandId::DeleteSession);
    }
}
