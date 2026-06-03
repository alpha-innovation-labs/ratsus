use std::fs;
use std::path::{Path, PathBuf};

use crate::extensions::terminal::ghostty::apply_required_keybinds_to_config::apply_required_keybinds_to_config;
use crate::extensions::terminal::ghostty::config_path::ghostty_config_path;
use crate::extensions::terminal::ghostty::create_ghostty_config_backup::create_ghostty_config_backup;

/// Outcome of setting up Ghostty config for Ratsus shortcuts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GhosttySetupResult {
    pub config_path: PathBuf,
    pub backup_path: Option<PathBuf>,
    pub changed: bool,
}

/// Updates the user's Ghostty config so Ratsus can receive required shortcuts.
pub fn setup_ghostty_config() -> anyhow::Result<GhosttySetupResult> {
    setup_ghostty_config_at_path(&ghostty_config_path())
}

/// Updates a specific Ghostty config path with Ratsus shortcut overrides.
pub fn setup_ghostty_config_at_path(path: &Path) -> anyhow::Result<GhosttySetupResult> {
    let existing = read_existing_config(path)?;
    let updated = apply_required_keybinds_to_config(&existing);
    let changed = existing != updated;
    let backup_path = if changed {
        create_ghostty_config_backup(path)?
    } else {
        None
    };
    if changed {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, updated)?;
    }
    Ok(GhosttySetupResult {
        config_path: path.to_path_buf(),
        backup_path,
        changed,
    })
}

/// Reads an existing config file, or returns empty content when missing.
fn read_existing_config(path: &Path) -> anyhow::Result<String> {
    if !path.exists() {
        return Ok(String::new());
    }
    Ok(fs::read_to_string(path)?)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::setup_ghostty_config_at_path;

    /// Verifies setup writes a config file with the managed keybind block.
    #[test]
    fn writes_required_keybinds_to_config() -> anyhow::Result<()> {
        let dir = test_dir("writes_required_keybinds_to_config")?;
        let config_path = dir.join("config");

        let result = setup_ghostty_config_at_path(&config_path)?;
        let content = fs::read_to_string(&config_path)?;

        assert!(result.changed);
        assert!(content.contains("keybind = ctrl+tab=csi:9;5u"));
        assert!(content.contains("keybind = super+digit_9=unbind"));
        let _ = fs::remove_dir_all(dir);
        Ok(())
    }

    /// Verifies setup creates a backup before changing an existing config file.
    #[test]
    fn creates_backup_for_existing_config() -> anyhow::Result<()> {
        let dir = test_dir("creates_backup_for_existing_config")?;
        let config_path = dir.join("config");
        fs::write(&config_path, "theme = Ayu\n")?;

        let result = setup_ghostty_config_at_path(&config_path)?;

        let backup_path = result.backup_path.expect("backup path");
        assert_eq!(fs::read_to_string(backup_path)?, "theme = Ayu\n");
        let _ = fs::remove_dir_all(dir);
        Ok(())
    }

    /// Verifies setup is idempotent after the first write.
    #[test]
    fn does_not_rewrite_unchanged_config() -> anyhow::Result<()> {
        let dir = test_dir("does_not_rewrite_unchanged_config")?;
        let config_path = dir.join("config");
        setup_ghostty_config_at_path(&config_path)?;

        let result = setup_ghostty_config_at_path(&config_path)?;

        assert!(!result.changed);
        assert_eq!(result.backup_path, None);
        let _ = fs::remove_dir_all(dir);
        Ok(())
    }

    /// Creates a test-owned temporary directory.
    fn test_dir(name: &str) -> anyhow::Result<PathBuf> {
        let dir = std::env::temp_dir().join(format!(
            "ratsus_ghostty_setup_{name}_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}
