use insta::assert_snapshot;

/// Validates the documented session_refresh E2E scenario.
#[test]
fn refresh_nexus_registry_watcher() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_refresh",
        "refresh_nexus_registry_watcher",
        "Nexus session refreshes are triggered by the registry file watcher instead of a polling loop.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_refresh
test: refresh_nexus_registry_watcher
description: Nexus session refreshes are triggered by the registry file watcher instead of a polling loop.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- Nexus registry watcher was installed
"###);
    Ok(())
}
