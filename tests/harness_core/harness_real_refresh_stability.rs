use insta::assert_snapshot;

/// Validates the documented harness_core E2E scenario.
#[test]
fn harness_real_refresh_stability() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "harness_core",
        "harness_real_refresh_stability",
        "Real Nexus refresh preserves row order and running status for loaded sessions.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: harness_core
test: harness_real_refresh_stability
description: Real Nexus refresh preserves row order and running status for loaded sessions.
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
