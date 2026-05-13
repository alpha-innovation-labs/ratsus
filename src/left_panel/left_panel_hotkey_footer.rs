use ratatui::style::Color;
use ratkit::widgets::{HotkeyFooter, HotkeyItem};

/// Builds the left-pane hotkey footer widget.
pub fn left_panel_hotkey_footer() -> HotkeyFooter {
    HotkeyFooter::new(vec![
        HotkeyItem::new("j/k", "move"),
        HotkeyItem::new("h/l", "fold"),
        HotkeyItem::new("gg/G", "edge"),
        HotkeyItem::new("/", "find"),
        HotkeyItem::new("Space", "select"),
        HotkeyItem::new("enter", "open"),
    ])
    .key_color(Color::Cyan)
    .description_color(Color::DarkGray)
    .background_color(Color::Black)
}
