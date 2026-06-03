use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

/// Returns the menu item index for the active left-pane mode.
pub fn active_left_pane_mode_menu_index(mode: LeftPaneMode) -> usize {
    match mode {
        LeftPaneMode::Sessions => 0,
        LeftPaneMode::Plans => 1,
        LeftPaneMode::Files => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::active_left_pane_mode_menu_index;
    use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

    /// Verifies that each left-pane mode maps to its menu item position.
    #[test]
    fn maps_modes_to_menu_indexes() {
        assert_eq!(active_left_pane_mode_menu_index(LeftPaneMode::Sessions), 0);
        assert_eq!(active_left_pane_mode_menu_index(LeftPaneMode::Plans), 1);
        assert_eq!(active_left_pane_mode_menu_index(LeftPaneMode::Files), 2);
    }
}
