use insta::assert_snapshot;

/// Validates the documented app_orchestration E2E scenario.
#[test]
fn app_startup_real_catalog() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "app_orchestration",
        "app_startup_real_catalog",
        "Startup creates a usable app with real Nexus sessions and renders the initial catalog.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: app_orchestration
test: app_startup_real_catalog
description: Startup creates a usable app with real Nexus sessions and renders the initial catalog.
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
