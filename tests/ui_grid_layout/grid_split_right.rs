use insta::assert_snapshot;

/// Validates the documented ui_grid_layout E2E scenario.
#[test]
fn grid_split_right() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_grid_layout",
        "grid_split_right",
        "The right split shortcut creates a stable side-by-side terminal layout.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_grid_layout
test: grid_split_right
description: The right split shortcut creates a stable side-by-side terminal layout.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
