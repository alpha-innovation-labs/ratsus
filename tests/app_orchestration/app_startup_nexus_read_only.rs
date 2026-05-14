use insta::assert_snapshot;

/// Validates the documented app_orchestration E2E scenario.
#[test]
fn app_startup_nexus_read_only() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "app_orchestration",
        "app_startup_nexus_read_only",
        "Startup loads real Nexus sessions in read-only mode without mutating Nexus data.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: app_orchestration
test: app_startup_nexus_read_only
description: Startup loads real Nexus sessions in read-only mode without mutating Nexus data.
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
