use insta::assert_snapshot;

/// Validates the documented ui_layout E2E scenario.
#[test]
fn layout_left_pane_visibility_toggle() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_layout",
        "layout_left_pane_visibility_toggle",
        "Toggling the left pane expands and restores the main pane.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_layout
test: layout_left_pane_visibility_toggle
description: Toggling the left pane expands and restores the main pane.
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
