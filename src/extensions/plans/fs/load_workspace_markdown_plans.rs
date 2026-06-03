use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::fs::is_markdown_plan_path::is_markdown_plan_path;

/// Loads Markdown plans from each workspace folder's `plans` directory.
pub fn load_workspace_markdown_plans(workspace_folders: &[PathBuf]) -> Result<Vec<PlanEntry>> {
    let mut plans = Vec::new();
    for workspace_folder in workspace_folders {
        let plans_dir = workspace_folder.join("plans");
        fs::create_dir_all(&plans_dir)
            .with_context(|| format!("failed to create plans directory {}", plans_dir.display()))?;
        collect_workspace_plans(workspace_folder, &plans_dir, &plans_dir, &mut plans)?;
    }
    plans.sort_by(|left, right| {
        left.folder
            .cmp(&right.folder)
            .then(left.title.cmp(&right.title))
            .then(left.path.cmp(&right.path))
    });
    Ok(plans)
}

/// Recursively collects Markdown files under one workspace `plans` directory.
fn collect_workspace_plans(
    workspace_folder: &Path,
    plans_dir: &Path,
    current_dir: &Path,
    plans: &mut Vec<PlanEntry>,
) -> Result<()> {
    for entry in fs::read_dir(current_dir)
        .with_context(|| format!("failed to read plans directory {}", current_dir.display()))?
    {
        let path = entry?.path();
        if path.is_dir() {
            collect_workspace_plans(workspace_folder, plans_dir, &path, plans)?;
            continue;
        }
        if path.is_file() && is_markdown_plan_path(&path) {
            plans.push(PlanEntry::new(
                path.clone(),
                workspace_folder.to_path_buf(),
                workspace_plan_title(plans_dir, &path),
            ));
        }
    }
    Ok(())
}

/// Returns the path shown below a workspace folder for one Markdown plan.
fn workspace_plan_title(plans_dir: &Path, path: &Path) -> String {
    path.strip_prefix(plans_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::load_workspace_markdown_plans;

    /// Loads plans from each workspace folder's own `plans` directory.
    #[test]
    fn loads_plans_grouped_by_workspace_folder() -> anyhow::Result<()> {
        let root = test_root("workspace-plans");
        let folder_a = root.join("folder-a");
        let folder_b = root.join("folder-b");
        fs::create_dir_all(folder_a.join("plans"))?;
        fs::create_dir_all(folder_b.join("plans"))?;
        fs::write(folder_a.join("plans/alpha.md"), "# Alpha")?;
        fs::write(folder_b.join("plans/beta.md"), "# Beta")?;

        let plans = load_workspace_markdown_plans(&[folder_a.clone(), folder_b.clone()])?;

        assert_eq!(plans.len(), 2);
        assert_eq!(plans[0].folder, folder_a);
        assert_eq!(plans[0].title, "alpha.md");
        assert_eq!(plans[1].folder, folder_b);
        assert_eq!(plans[1].title, "beta.md");
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Builds a unique test root in the system temporary directory.
    fn test_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("ratsus-{label}-{}", uuid::Uuid::new_v4()))
    }
}
