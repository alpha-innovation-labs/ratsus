use crate::extensions::terminal::ghostty::required_keybind_lines::required_ghostty_keybind_lines;

const MANAGED_BLOCK_BEGIN: &str = "# BEGIN RATSUS MANAGED HOTKEYS";
const MANAGED_BLOCK_END: &str = "# END RATSUS MANAGED HOTKEYS";

/// Returns the full Ghostty config block managed by Ratsus.
pub fn managed_ghostty_config_block() -> String {
    let mut lines = vec![
        MANAGED_BLOCK_BEGIN.to_string(),
        "# Lets Ratsus receive terminal shortcuts that Ghostty binds by default.".to_string(),
    ];
    lines.extend(required_ghostty_keybind_lines());
    lines.push(MANAGED_BLOCK_END.to_string());
    lines.join("\n")
}

/// Removes the existing Ratsus-managed Ghostty block from config content.
pub fn remove_managed_ghostty_config_block(content: &str) -> String {
    let mut output = Vec::new();
    let mut inside_managed_block = false;
    for line in content.lines() {
        if line.trim() == MANAGED_BLOCK_BEGIN {
            inside_managed_block = true;
            continue;
        }
        if line.trim() == MANAGED_BLOCK_END {
            inside_managed_block = false;
            continue;
        }
        if !inside_managed_block {
            output.push(line);
        }
    }
    output.join("\n").trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::{managed_ghostty_config_block, remove_managed_ghostty_config_block};

    /// Verifies the managed block includes required shortcut overrides.
    #[test]
    fn managed_block_contains_required_overrides() {
        let block = managed_ghostty_config_block();

        assert!(block.contains("keybind = ctrl+tab=csi:9;5u"));
        assert!(block.contains("keybind = ctrl+grave_accent=csi:96;5u"));
        assert!(block.contains("keybind = super+digit_1=unbind"));
    }

    /// Verifies an old managed block is removed while user config remains.
    #[test]
    fn removes_existing_managed_block() {
        let content = "theme = dark\n# BEGIN RATSUS MANAGED HOTKEYS\nold\n# END RATSUS MANAGED HOTKEYS\nfont-size = 18";

        let updated = remove_managed_ghostty_config_block(content);

        assert_eq!(updated, "theme = dark\nfont-size = 18");
    }
}
