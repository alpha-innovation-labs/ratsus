use std::fs;

use crate::extensions::plans::data::plan_list_state::PlanListState;

/// Filtering narrows visible plans and activates the first matching plan.
#[test]
fn filtering_updates_visible_and_active_plan() -> anyhow::Result<()> {
    let root = test_root("filter");
    let plans_dir = root.join("plans");
    fs::create_dir_all(&plans_dir)?;
    fs::write(plans_dir.join("alpha.md"), "# Alpha")?;
    fs::write(plans_dir.join("beta.md"), "# Beta")?;
    let mut state = PlanListState::with_root(root.clone())?;

    state.start_filtering();
    state.push_filter_character('b');

    assert_eq!(state.visible_indices().len(), 1);
    assert_eq!(
        state.active_plan().map(|plan| plan.title.as_str()),
        Some("beta")
    );
    let _ = fs::remove_dir_all(root);
    Ok(())
}

/// Builds a unique test root in the system temporary directory.
fn test_root(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "ratsus-plan-filter-{label}-{}",
        uuid::Uuid::new_v4()
    ))
}
