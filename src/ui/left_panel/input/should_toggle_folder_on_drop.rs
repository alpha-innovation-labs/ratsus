use std::path::Path;

/// Returns true when a folder drag ended as a click instead of a drag reorder.
pub fn should_toggle_folder_on_drop(source: &Path, target: &Path, moved: bool) -> bool {
    !moved && source == target
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::should_toggle_folder_on_drop;

    /// Verifies mouse down/up on the same folder is treated as a folder click.
    #[test]
    fn toggles_same_folder_without_drag_motion() {
        assert!(should_toggle_folder_on_drop(
            Path::new("/tmp/a"),
            Path::new("/tmp/a"),
            false,
        ));
    }

    /// Verifies a drag motion prevents folder toggle behavior.
    #[test]
    fn does_not_toggle_after_drag_motion() {
        assert!(!should_toggle_folder_on_drop(
            Path::new("/tmp/a"),
            Path::new("/tmp/a"),
            true,
        ));
    }

    /// Verifies dropping on another folder is treated as reorder, not toggle.
    #[test]
    fn does_not_toggle_different_folder() {
        assert!(!should_toggle_folder_on_drop(
            Path::new("/tmp/a"),
            Path::new("/tmp/b"),
            false,
        ));
    }
}
