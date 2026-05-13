use ratkit::primitives::menu_bar::{MenuBar, MenuItem};

use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::menu_bar::active_main_pane_tab_menu_index::active_main_pane_tab_menu_index;

/// Builds the top menu bar with the current main pane tab selected.
pub fn app_menu_bar(selected_tab: MainPaneTab) -> MenuBar {
    MenuBar::new(vec![MenuItem::new("Chat", 0), MenuItem::new("Files", 1)])
        .with_selected(active_main_pane_tab_menu_index(selected_tab))
}

#[cfg(test)]
mod tests {
    use super::app_menu_bar;
    use crate::main_pane::main_pane_tab::MainPaneTab;

    /// Verifies that the menu exposes Chat and Files top-level sections.
    #[test]
    fn creates_chat_and_files_menu_items() {
        let menu_bar = app_menu_bar(MainPaneTab::Chat);

        assert_eq!(menu_bar.items.len(), 2);
        assert_eq!(menu_bar.items[0].name, "Chat");
        assert_eq!(menu_bar.items[1].name, "Files");
    }

    /// Verifies that Chat is selected when the Chat tab is active.
    #[test]
    fn selects_chat_tab_item() {
        let menu_bar = app_menu_bar(MainPaneTab::Chat);

        assert_eq!(menu_bar.selected(), Some(0));
    }

    /// Verifies that Files is selected when the Files tab is active.
    #[test]
    fn selects_files_tab_item() {
        let menu_bar = app_menu_bar(MainPaneTab::Files);

        assert_eq!(menu_bar.selected(), Some(1));
    }
}
