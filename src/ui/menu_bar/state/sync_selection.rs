use ratkit::primitives::menu_bar::MenuBar;

/// Updates an existing menu bar so only the requested item index is selected.
pub fn sync_app_menu_bar_selection(menu_bar: &mut MenuBar, selected_index: usize) {
    for (index, item) in menu_bar.items.iter_mut().enumerate() {
        item.selected = index == selected_index;
    }
}

#[cfg(test)]
mod tests {
    use ratkit::primitives::menu_bar::{MenuBar, MenuItem};

    use super::sync_app_menu_bar_selection;

    /// Verifies that selection sync clears old items and selects only the requested item.
    #[test]
    fn selects_only_requested_index() {
        let mut menu_bar = MenuBar::new(vec![
            MenuItem::new("Chat", 0),
            MenuItem::new("Files", 1),
            MenuItem::new("Diff", 2),
        ])
        .with_selected(0);

        sync_app_menu_bar_selection(&mut menu_bar, 2);

        assert_eq!(menu_bar.selected(), Some(2));
    }
}
