use ratatui::style::Color;

/// Returns the same Yazi-style blue used by Ratkit's file-system folder icon.
pub fn folder_blue_color() -> Color {
    Color::Rgb(3, 169, 244)
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::folder_blue_color;

    /// Verifies the folder blue matches Ratkit file-system tree folder blue.
    #[test]
    fn matches_file_system_tree_folder_blue() {
        assert_eq!(folder_blue_color(), Color::Rgb(3, 169, 244));
    }
}
