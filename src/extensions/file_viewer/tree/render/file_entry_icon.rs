use ratatui::style::Color;

use crate::extensions::file_viewer::tree::render::parse_hex_color::parse_hex_color;
use crate::extensions::file_viewer::tree::render::yazi_dir_icon_color::yazi_dir_icon_color;

/// Returns the original Ratkit file-tree icon and icon color for one entry.
pub fn file_entry_icon(
    name: &str,
    is_dir: bool,
    expanded: bool,
    use_dark_theme: bool,
) -> (char, Color) {
    if is_dir {
        return if expanded {
            ('\u{f115}', yazi_dir_icon_color())
        } else {
            ('\u{f114}', yazi_dir_icon_color())
        };
    }

    let theme = if use_dark_theme {
        devicons::Theme::Dark
    } else {
        devicons::Theme::Light
    };
    let icon = devicons::icon_for_file(name, &Some(theme));
    (
        icon.icon,
        parse_hex_color(icon.color).unwrap_or(Color::White),
    )
}
