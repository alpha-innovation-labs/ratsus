use crate::extensions::terminal::ghostty::managed_config_block::{
    managed_ghostty_config_block, remove_managed_ghostty_config_block,
};

/// Applies the Ratsus-managed Ghostty keybind block to config content.
pub fn apply_required_keybinds_to_config(content: &str) -> String {
    let base = remove_managed_ghostty_config_block(content);
    if base.is_empty() {
        return format!("{}\n", managed_ghostty_config_block());
    }
    format!("{}\n\n{}\n", base, managed_ghostty_config_block())
}

#[cfg(test)]
mod tests {
    use super::apply_required_keybinds_to_config;

    /// Verifies applying config appends the managed override block.
    #[test]
    fn appends_managed_keybind_block() {
        let updated = apply_required_keybinds_to_config("theme = Ayu\n");

        assert!(updated.contains("theme = Ayu"));
        assert!(updated.contains("keybind = ctrl+tab=csi:9;5u"));
        assert!(updated.ends_with('\n'));
    }

    /// Verifies applying config is stable across repeated runs.
    #[test]
    fn remains_idempotent() {
        let first = apply_required_keybinds_to_config("theme = Ayu\n");
        let second = apply_required_keybinds_to_config(&first);

        assert_eq!(first, second);
    }
}
