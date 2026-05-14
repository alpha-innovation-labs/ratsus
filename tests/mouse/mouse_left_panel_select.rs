use insta::assert_snapshot;

/// Validates the documented mouse E2E scenario.
#[test]
fn mouse_left_panel_select() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "mouse",
        "mouse_left_panel_select",
        "Clicking a left-panel row selects the intended row.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: mouse
test: mouse_left_panel_select
description: Clicking a left-panel row selects the intended row.
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
