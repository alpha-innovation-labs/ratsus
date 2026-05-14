use std::fs;
use std::path::{Path, PathBuf};

/// Removes a test-owned temporary workspace when the scenario finishes.
pub struct TemporaryWorkspace {
    path: PathBuf,
}

impl TemporaryWorkspace {
    /// Creates a real temporary workspace without changing the operator HOME.
    pub fn create(test_name: &str) -> anyhow::Result<Self> {
        let path =
            std::env::temp_dir().join(format!("ratsus-e2e-{}-{}", test_name, uuid::Uuid::new_v4()));
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    /// Returns the test-owned workspace path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl AsRef<Path> for TemporaryWorkspace {
    /// Returns the test-owned workspace path for path-based helpers.
    fn as_ref(&self) -> &Path {
        self.path()
    }
}

impl Drop for TemporaryWorkspace {
    /// Removes the test-owned workspace tree.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
