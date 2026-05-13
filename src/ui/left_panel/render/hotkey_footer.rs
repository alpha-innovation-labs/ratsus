use ratatui::style::Color;
use ratkit::widgets::{HotkeyFooter, HotkeyItem};

use crate::ui::left_panel::footer_item::LeftPaneFooterItem;

/// Builds the shared left-pane footer widget from active content items and status.
pub fn left_panel_hotkey_footer(
    items: Vec<LeftPaneFooterItem>,
    status: Option<String>,
) -> HotkeyFooter {
    HotkeyFooter::new(hotkey_items_for_left_panel(items, status))
        .key_color(Color::Cyan)
        .description_color(Color::DarkGray)
        .background_color(Color::Black)
}

/// Converts left-pane footer items and status into Ratkit hotkey footer items.
fn hotkey_items_for_left_panel(
    items: Vec<LeftPaneFooterItem>,
    status: Option<String>,
) -> Vec<HotkeyItem> {
    let mut hotkey_items = items
        .into_iter()
        .map(|item| HotkeyItem::new(item.key, item.description))
        .collect::<Vec<_>>();
    if let Some(status) = status {
        hotkey_items.push(HotkeyItem::new("Selected:", status));
    }
    hotkey_items
}

#[cfg(test)]
mod tests {
    use crate::ui::left_panel::footer_item::LeftPaneFooterItem;
    use crate::ui::left_panel::render::hotkey_footer::left_panel_hotkey_footer;

    /// Verifies the shared renderer preserves active content footer entries.
    #[test]
    fn builds_footer_from_active_content_items() {
        let footer = left_panel_hotkey_footer(vec![LeftPaneFooterItem::new("j/k", "move")], None);

        assert_eq!(footer.items.len(), 1);
        assert_eq!(footer.items[0].key, "j/k");
        assert_eq!(footer.items[0].description, "move");
    }

    /// Verifies the shared renderer can append active content status text.
    #[test]
    fn builds_footer_with_status_text() {
        let footer = left_panel_hotkey_footer(
            vec![LeftPaneFooterItem::new("j/k", "move")],
            Some("/tmp/example".to_string()),
        );

        assert_eq!(footer.items.len(), 2);
        assert_eq!(footer.items[1].key, "Selected:");
        assert_eq!(footer.items[1].description, "/tmp/example");
    }
}
