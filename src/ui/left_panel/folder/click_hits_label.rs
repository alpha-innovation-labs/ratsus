/// Returns true when a folder-row click lands on the folder icon column.
pub fn folder_click_hits_icon(column: u16, list_x: u16) -> bool {
    column == list_x
}

#[cfg(test)]
mod tests {
    use super::folder_click_hits_icon;

    /// Verifies clicks on the folder icon column are accepted.
    #[test]
    fn accepts_folder_icon_column() {
        assert!(folder_click_hits_icon(2, 2));
    }

    /// Verifies clicks after the icon do not toggle the folder.
    #[test]
    fn rejects_columns_after_folder_icon() {
        assert!(!folder_click_hits_icon(3, 2));
    }
}
