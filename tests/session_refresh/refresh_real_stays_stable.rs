use insta::assert_snapshot;

/// Validates the documented session_refresh E2E scenario.
#[test]
fn refresh_real_stays_stable() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_refresh",
        "refresh_real_stays_stable",
        "Real Nexus refresh does not reorder sessions or corrupt running state.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_refresh
test: refresh_real_stays_stable
description: Real Nexus refresh does not reorder sessions or corrupt running state.
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
