use crate::extensions::file_viewer::tabs::tab::MainPaneTab;

/// Returns the main pane tab selected by a menu item index.
pub fn main_pane_tab_for_menu_index(index: usize) -> Option<MainPaneTab> {
    match index {
        0 => Some(MainPaneTab::Chat),
        1 => Some(MainPaneTab::Files),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::main_pane_tab_for_menu_index;
    use crate::extensions::file_viewer::tabs::tab::MainPaneTab;

    /// Verifies that visible menu item positions map to their tabs.
    #[test]
    fn maps_menu_indexes_to_tabs() {
        assert_eq!(main_pane_tab_for_menu_index(0), Some(MainPaneTab::Chat));
        assert_eq!(main_pane_tab_for_menu_index(1), Some(MainPaneTab::Files));
    }

    /// Verifies that removed menu item positions are ignored safely.
    #[test]
    fn ignores_removed_menu_indexes() {
        assert_eq!(main_pane_tab_for_menu_index(2), None);
        assert_eq!(main_pane_tab_for_menu_index(3), None);
    }
}
