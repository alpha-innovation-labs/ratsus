use insta::assert_snapshot;

/// Validates the documented ui_left_panel E2E scenario.
#[test]
fn left_panel_delete_shortcut() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_left_panel",
        "left_panel_delete_shortcut",
        "Delete starts the confirmation flow for the focused or selected rows.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_left_panel
test: left_panel_delete_shortcut
description: Delete starts the confirmation flow for the focused or selected rows.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- delete confirmation workflow was visible
"###);
    Ok(())
}
