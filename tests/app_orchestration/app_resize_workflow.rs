use insta::assert_snapshot;

/// Validates the documented app_orchestration E2E scenario.
#[test]
fn app_resize_workflow() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "app_orchestration",
        "app_resize_workflow",
        "Resize events redraw the app with stable pane boundaries and no broken borders.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: app_orchestration
test: app_resize_workflow
description: Resize events redraw the app with stable pane boundaries and no broken borders.
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
