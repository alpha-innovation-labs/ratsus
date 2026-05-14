use insta::assert_snapshot;

/// Validates the documented ui_left_panel E2E scenario.
#[test]
fn left_panel_startup_catalog() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_left_panel",
        "left_panel_startup_catalog",
        "The left panel shows grouped sessions with deterministic folder order and active markers.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_left_panel
test: left_panel_startup_catalog
description: The left panel shows grouped sessions with deterministic folder order and active markers.
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
