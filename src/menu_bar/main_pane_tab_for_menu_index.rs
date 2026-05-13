use crate::main_pane::main_pane_tab::MainPaneTab;

/// Returns the main pane tab selected by a menu item index.
pub fn main_pane_tab_for_menu_index(index: usize) -> Option<MainPaneTab> {
    match index {
        0 => Some(MainPaneTab::Chat),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::main_pane_tab_for_menu_index;
    use crate::main_pane::main_pane_tab::MainPaneTab;

    /// Verifies that the visible menu item position maps to the Chat tab.
    #[test]
    fn maps_chat_menu_index_to_tab() {
        assert_eq!(main_pane_tab_for_menu_index(0), Some(MainPaneTab::Chat));
    }

    /// Verifies that removed menu item positions are ignored safely.
    #[test]
    fn ignores_removed_menu_indexes() {
        assert_eq!(main_pane_tab_for_menu_index(1), None);
        assert_eq!(main_pane_tab_for_menu_index(2), None);
        assert_eq!(main_pane_tab_for_menu_index(3), None);
    }
}
