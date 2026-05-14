use insta::assert_snapshot;

/// Validates the documented ui_grid_layout E2E scenario.
#[test]
fn grid_place_session_in_active_split() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_grid_layout",
        "grid_place_session_in_active_split",
        "The placement picker assigns the selected session to the active pane.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_grid_layout
test: grid_place_session_in_active_split
description: The placement picker assigns the selected session to the active pane.
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
