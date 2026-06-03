use ratatui::style::{Color, Style};

/// Returns the original Ratkit selected file-tree text style.
pub fn selected_file_entry_style(is_dir: bool, dir_style: Style, file_style: Style) -> Style {
    let selected_background = if is_dir {
        dir_style.fg.unwrap_or(Color::Blue)
    } else {
        file_style.fg.unwrap_or(Color::White)
    };
    Style::default().fg(Color::Black).bg(selected_background)
}
