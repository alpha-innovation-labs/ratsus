use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

/// Returns the left-pane mode selected by a menu item index.
pub fn left_pane_mode_for_menu_index(index: usize) -> Option<LeftPaneMode> {
    match index {
        0 => Some(LeftPaneMode::Sessions),
        1 => Some(LeftPaneMode::Plans),
        2 => Some(LeftPaneMode::Files),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::left_pane_mode_for_menu_index;
    use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

    /// Verifies that visible menu item positions map to left-pane modes.
    #[test]
    fn maps_menu_indexes_to_modes() {
        assert_eq!(
            left_pane_mode_for_menu_index(0),
            Some(LeftPaneMode::Sessions)
        );
        assert_eq!(left_pane_mode_for_menu_index(1), Some(LeftPaneMode::Plans));
        assert_eq!(left_pane_mode_for_menu_index(2), Some(LeftPaneMode::Files));
    }

    /// Verifies that unknown menu item positions are ignored safely.
    #[test]
    fn ignores_unknown_menu_indexes() {
        assert_eq!(left_pane_mode_for_menu_index(3), None);
    }
}
