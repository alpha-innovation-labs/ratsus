use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::fs::is_markdown_plan_path::is_markdown_plan_path;
use crate::extensions::plans::fs::plan_title::plan_title;

/// Loads Markdown plan files from a `plans` directory.
pub fn load_markdown_plans(plans_dir: &Path) -> Result<Vec<PlanEntry>> {
    let mut entries = Vec::new();
    collect_markdown_plans(plans_dir, plans_dir, &mut entries)?;
    entries.sort_by(|left, right| {
        left.folder
            .cmp(&right.folder)
            .then(left.title.cmp(&right.title))
            .then(left.path.cmp(&right.path))
    });
    Ok(entries)
}

/// Recursively collects Markdown plans and groups them by containing directory.
fn collect_markdown_plans(root: &Path, dir: &Path, entries: &mut Vec<PlanEntry>) -> Result<()> {
    for entry in fs::read_dir(dir)
        .with_context(|| format!("failed to read plans directory {}", dir.display()))?
    {
        let path = entry?.path();
        if path.is_dir() {
            collect_markdown_plans(root, &path, entries)?;
            continue;
        }
        if !path.is_file() || !is_markdown_plan_path(&path) {
            continue;
        }
        let folder = path
            .parent()
            .unwrap_or(root)
            .strip_prefix(root)
            .unwrap_or_else(|_| path.parent().unwrap_or(root))
            .to_path_buf();
        let folder = if folder.as_os_str().is_empty() {
            root.to_path_buf()
        } else {
            root.join(folder)
        };
        entries.push(PlanEntry::new(path.clone(), folder, plan_title(&path)));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::load_markdown_plans;

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

    /// Keeps nested plan folder information for grouped left-pane rendering.
    #[test]
    fn loads_nested_markdown_files_with_folders() -> anyhow::Result<()> {
        let root = test_root("nested");
        let plans_dir = root.join("plans");
        let nested_dir = plans_dir.join("feature");
        fs::create_dir_all(&nested_dir)?;
        fs::write(nested_dir.join("alpha.md"), "# Alpha")?;

        let plans = load_markdown_plans(&plans_dir)?;

        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].folder, nested_dir);
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Builds a unique test root in the system temporary directory.
    fn test_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("ratsus-plans-{label}-{}", uuid::Uuid::new_v4()))
    }
}
