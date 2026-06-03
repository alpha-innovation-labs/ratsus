use ratatui::style::Color;

use crate::extensions::file_viewer::tree::render::parse_hex_color::parse_hex_color;

/// Returns the directory icon color used by the original Ratkit file tree.
pub fn yazi_dir_icon_color() -> Color {
    parse_hex_color("#03a9f4").unwrap_or(Color::Blue)
}
