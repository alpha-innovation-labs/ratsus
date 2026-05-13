use ratatui::style::Color;
use ratkit::widgets::{HotkeyFooter, HotkeyItem};

/// Builds the Expo hotkey footer widget.
pub fn expo_hotkey_footer() -> HotkeyFooter {
    HotkeyFooter::new(vec![
        HotkeyItem::new("/", "filter"),
        HotkeyItem::new("h/k", "prev"),
        HotkeyItem::new("j/l", "next"),
        HotkeyItem::new("Enter", "open"),
    ])
    .key_color(Color::Cyan)
    .description_color(Color::DarkGray)
    .background_color(Color::Black)
}
