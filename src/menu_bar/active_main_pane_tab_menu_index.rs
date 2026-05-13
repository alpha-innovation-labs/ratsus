use crate::main_pane::main_pane_tab::MainPaneTab;

/// Returns the menu item index for the active main pane tab.
pub fn active_main_pane_tab_menu_index(tab: MainPaneTab) -> usize {
    match tab {
        MainPaneTab::Chat => 0,
        MainPaneTab::Files => 1,
        MainPaneTab::Diff => 2,
        MainPaneTab::Expo => 3,
    }
}

#[cfg(test)]
mod tests {
    use super::active_main_pane_tab_menu_index;
    use crate::main_pane::main_pane_tab::MainPaneTab;

    /// Verifies that each main pane tab maps to its menu item position.
    #[test]
    fn maps_tabs_to_menu_indexes() {
        assert_eq!(active_main_pane_tab_menu_index(MainPaneTab::Chat), 0);
        assert_eq!(active_main_pane_tab_menu_index(MainPaneTab::Files), 1);
        assert_eq!(active_main_pane_tab_menu_index(MainPaneTab::Diff), 2);
        assert_eq!(active_main_pane_tab_menu_index(MainPaneTab::Expo), 3);
    }
}
