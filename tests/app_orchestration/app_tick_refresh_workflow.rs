use insta::assert_snapshot;

/// Validates the documented app_orchestration E2E scenario.
#[test]
fn app_tick_refresh_workflow() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "app_orchestration",
        "app_tick_refresh_workflow",
        "Tick events refresh sessions and previews without changing unrelated visible state.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: app_orchestration
test: app_tick_refresh_workflow
description: Tick events refresh sessions and previews without changing unrelated visible state.
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
