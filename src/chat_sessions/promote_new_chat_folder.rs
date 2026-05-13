use std::path::{Path, PathBuf};

/// Moves the folder for a newly created chat to the top of the left-panel folder order.
pub fn promote_new_chat_folder(folder_order: &mut Vec<PathBuf>, working_dir: &Path) {
    folder_order.retain(|folder| folder != working_dir);
    folder_order.insert(0, working_dir.to_path_buf());
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::promote_new_chat_folder;

    /// Existing folders move to the top when a new chat is created inside them.
    #[test]
    fn moves_existing_folder_to_top() {
        let mut order = vec![PathBuf::from("/tmp/a"), PathBuf::from("/tmp/b")];

        promote_new_chat_folder(&mut order, Path::new("/tmp/b"));

        assert_eq!(
            order,
            vec![PathBuf::from("/tmp/b"), PathBuf::from("/tmp/a")]
        );
    }

    /// New folders are inserted at the top.
    #[test]
    fn inserts_new_folder_at_top() {
        let mut order = vec![PathBuf::from("/tmp/a")];

        promote_new_chat_folder(&mut order, Path::new("/tmp/new"));

        assert_eq!(order[0], PathBuf::from("/tmp/new"));
    }
}
