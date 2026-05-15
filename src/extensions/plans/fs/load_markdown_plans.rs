use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::fs::is_markdown_plan_path::is_markdown_plan_path;
use crate::extensions::plans::fs::plan_title::plan_title;

/// Loads Markdown plan files from a `plans` directory, creating it when missing.
pub fn load_markdown_plans(plans_dir: &Path) -> Result<Vec<PlanEntry>> {
    fs::create_dir_all(plans_dir)
        .with_context(|| format!("failed to create plans directory {}", plans_dir.display()))?;
    let mut entries = fs::read_dir(plans_dir)
        .with_context(|| format!("failed to read plans directory {}", plans_dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_markdown_plan_path(path))
        .map(|path| PlanEntry::new(path.clone(), plan_title(&path)))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        left.title
            .cmp(&right.title)
            .then(left.path.cmp(&right.path))
    });
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::load_markdown_plans;

    /// Creates the plans directory when it does not exist.
    #[test]
    fn creates_missing_plans_directory() -> anyhow::Result<()> {
        let root = test_root("create");
        let plans_dir = root.join("plans");

        let plans = load_markdown_plans(&plans_dir)?;

        assert!(plans.is_empty());
        assert!(plans_dir.is_dir());
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Keeps Markdown files and ignores all other files.
    #[test]
    fn loads_only_markdown_files() -> anyhow::Result<()> {
        let root = test_root("markdown-only");
        let plans_dir = root.join("plans");
        fs::create_dir_all(&plans_dir)?;
        fs::write(plans_dir.join("alpha.md"), "# Alpha")?;
        fs::write(plans_dir.join("beta.markdown"), "# Beta")?;
        fs::write(plans_dir.join("notes.txt"), "Notes")?;

        let plans = load_markdown_plans(&plans_dir)?;

        assert_eq!(
            plans
                .iter()
                .map(|plan| plan.title.as_str())
                .collect::<Vec<_>>(),
            vec!["alpha", "beta"]
        );
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Builds a unique test root in the system temporary directory.
    fn test_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("ratsus-plans-{label}-{}", uuid::Uuid::new_v4()))
    }
}
