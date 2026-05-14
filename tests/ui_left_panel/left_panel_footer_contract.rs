use insta::assert_snapshot;

/// Validates the documented ui_left_panel E2E scenario.
#[test]
fn left_panel_footer_contract() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_left_panel",
        "left_panel_footer_contract",
        "Footer keys and status match the active left-pane content.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_left_panel
test: left_panel_footer_contract
description: Footer keys and status match the active left-pane content.
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
