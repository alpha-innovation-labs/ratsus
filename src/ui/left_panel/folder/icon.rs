/// Returns the file-tree folder icon for the current folder expansion state.
pub fn folder_icon(is_collapsed: bool) -> &'static str {
    if is_collapsed {
        "\u{f114}"
    } else {
        "\u{f115}"
    }
}

#[cfg(test)]
mod tests {
    use super::folder_icon;

    /// Verifies collapsed folders use the closed folder icon from the file view.
    #[test]
    fn collapsed_folder_uses_closed_icon() {
        assert_eq!(folder_icon(true), "\u{f114}");
    }

    /// Verifies expanded folders use the open folder icon from the file view.
    #[test]
    fn expanded_folder_uses_open_icon() {
        assert_eq!(folder_icon(false), "\u{f115}");
    }
}
