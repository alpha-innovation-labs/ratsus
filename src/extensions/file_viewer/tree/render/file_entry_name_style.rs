use ratatui::style::Style;

/// Returns the original Ratkit file-tree name style for one unselected entry.
pub fn file_entry_name_style(is_dir: bool, dir_style: Style, file_style: Style) -> Style {
    if is_dir {
        dir_style
    } else {
        file_style
    }
}
