use ratkit::primitives::menu_bar::{MenuBar, MenuItem};

use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::menu_bar::active_main_pane_tab_menu_index::active_main_pane_tab_menu_index;

/// Builds the Nexus top menu bar with the current main pane tab selected.
pub fn nexus_menu_bar(selected_tab: MainPaneTab) -> MenuBar {
    MenuBar::new(vec![MenuItem::new("Chat", 0)])
        .with_selected(active_main_pane_tab_menu_index(selected_tab))
}

#[cfg(test)]
mod tests {
    use super::nexus_menu_bar;
    use crate::main_pane::main_pane_tab::MainPaneTab;

    /// Verifies that the menu exposes only the Chat top-level section.
    #[test]
    fn creates_only_chat_menu_item() {
        let menu_bar = nexus_menu_bar(MainPaneTab::Chat);

        assert_eq!(menu_bar.items.len(), 1);
        assert_eq!(menu_bar.items[0].name, "Chat");
    }

    /// Verifies that Chat is selected when the Chat tab is active.
    #[test]
    fn selects_chat_tab_item() {
        let menu_bar = nexus_menu_bar(MainPaneTab::Chat);

        assert_eq!(menu_bar.selected(), Some(0));
    }
}
