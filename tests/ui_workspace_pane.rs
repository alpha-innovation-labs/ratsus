mod support;

use insta::assert_snapshot;

/// Validates the documented workspace-pane E2E scenario.
#[test]
fn workspace_pane_startup_catalog() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_workspace_pane",
        "workspace_pane_startup_catalog",
        "The workspace pane shows folder workspaces beside a folder-filtered left panel.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_workspace_pane
test: workspace_pane_startup_catalog
description: The workspace pane shows folder workspaces beside a folder-filtered left panel.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- workspace pane has an active selected folder
"###);
    Ok(())
}
