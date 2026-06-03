use ratkit::primitives::menu_bar::{MenuBar, MenuItem};

use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::menu_bar::state::active_tab_menu_index::active_left_pane_mode_menu_index;

/// Builds the top menu bar with the current left-pane mode selected.
pub fn app_menu_bar(selected_mode: LeftPaneMode) -> MenuBar {
    MenuBar::new(vec![
        MenuItem::new("Sessions", 0),
        MenuItem::new("Plans", 1),
        MenuItem::new("Files", 2),
    ])
    .with_selected(active_left_pane_mode_menu_index(selected_mode))
}

#[cfg(test)]
mod tests {
    use super::app_menu_bar;
    use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

    /// Verifies that the menu exposes the left-pane sections.
    #[test]
    fn creates_left_pane_menu_items() {
        let menu_bar = app_menu_bar(LeftPaneMode::Sessions);

        assert_eq!(menu_bar.items.len(), 3);
        assert_eq!(menu_bar.items[0].name, "Sessions");
        assert_eq!(menu_bar.items[1].name, "Plans");
        assert_eq!(menu_bar.items[2].name, "Files");
    }

    /// Verifies that Sessions is selected when Sessions mode is active.
    #[test]
    fn selects_sessions_item() {
        let menu_bar = app_menu_bar(LeftPaneMode::Sessions);

        assert_eq!(menu_bar.selected(), Some(0));
    }

    /// Verifies that Plans is selected when Plans mode is active.
    #[test]
    fn selects_plans_item() {
        let menu_bar = app_menu_bar(LeftPaneMode::Plans);

        assert_eq!(menu_bar.selected(), Some(1));
    }

    /// Verifies that Files is selected when Files mode is active.
    #[test]
    fn selects_files_item() {
        let menu_bar = app_menu_bar(LeftPaneMode::Files);

        assert_eq!(menu_bar.selected(), Some(2));
    }
}
