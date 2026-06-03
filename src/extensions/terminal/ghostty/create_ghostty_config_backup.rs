use std::fs;
use std::path::{Path, PathBuf};

/// Creates a one-time backup beside the Ghostty config file when it exists.
pub fn create_ghostty_config_backup(path: &Path) -> anyhow::Result<Option<PathBuf>> {
    if !path.exists() {
        return Ok(None);
    }
    let backup_path = path.with_extension("config.ratsus.bak");
    if backup_path.exists() {
        return Ok(Some(backup_path));
    }
    fs::copy(path, &backup_path)?;
    Ok(Some(backup_path))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::create_ghostty_config_backup;

    /// Verifies backing up an existing config preserves original content.
    #[test]
    fn backs_up_existing_config() -> anyhow::Result<()> {
        let dir = test_dir("backs_up_existing_config")?;
        let config_path = dir.join("config");
        fs::write(&config_path, "theme = Ayu\n")?;

        let backup_path = create_ghostty_config_backup(&config_path)?.expect("backup path");

        assert_eq!(fs::read_to_string(backup_path)?, "theme = Ayu\n");
        let _ = fs::remove_dir_all(dir);
        Ok(())
    }

    /// Verifies missing config files do not create backups.
    #[test]
    fn skips_missing_config_backup() -> anyhow::Result<()> {
        let dir = test_dir("skips_missing_config_backup")?;
        let config_path = dir.join("config");

        assert_eq!(create_ghostty_config_backup(&config_path)?, None);
        let _ = fs::remove_dir_all(dir);
        Ok(())
    }

    /// Creates a test-owned temporary directory.
    fn test_dir(name: &str) -> anyhow::Result<PathBuf> {
        let dir =
            std::env::temp_dir().join(format!("ratsus_ghostty_{name}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}
