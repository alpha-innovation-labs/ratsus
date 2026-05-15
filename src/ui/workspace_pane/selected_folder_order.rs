use std::path::PathBuf;

/// Returns the folder order scoped to the selected workspace when present.
pub fn selected_folder_order(folder_order: &[PathBuf], selected: Option<&PathBuf>) -> Vec<PathBuf> {
    let Some(selected) = selected else {
        return folder_order.to_vec();
    };
    folder_order
        .iter()
        .filter(|folder| *folder == selected)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::selected_folder_order;

    /// Verifies selected folder order keeps only the active workspace folder.
    #[test]
    fn keeps_only_selected_workspace_folder() {
        let folders = vec![PathBuf::from("/a"), PathBuf::from("/b")];

        assert_eq!(
            selected_folder_order(&folders, Some(&PathBuf::from("/b"))),
            vec![PathBuf::from("/b")]
        );
    }
}
