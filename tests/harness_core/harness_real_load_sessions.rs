use insta::assert_snapshot;

/// Validates the documented harness_core E2E scenario.
#[test]
fn harness_real_load_sessions() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "harness_core",
        "harness_real_load_sessions",
        "The real Nexus harness returns the session catalog used by E2E snapshots.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: harness_core
test: harness_real_load_sessions
description: The real Nexus harness returns the session catalog used by E2E snapshots.
backend: nexus
workflow: real Nexus harness backend workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
