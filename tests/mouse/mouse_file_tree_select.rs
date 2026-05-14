use insta::assert_snapshot;

/// Validates the documented mouse E2E scenario.
#[test]
fn mouse_file_tree_select() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "mouse",
        "mouse_file_tree_select",
        "Clicking a file-tree row selects and previews the intended path.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: mouse
test: mouse_file_tree_select
description: Clicking a file-tree row selects and previews the intended path.
backend: nexus
workflow: real Nexus mouse input workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
