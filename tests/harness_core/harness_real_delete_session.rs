use insta::assert_snapshot;

/// Validates the documented harness_core E2E scenario.
#[test]
fn harness_real_delete_session() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "harness_core",
        "harness_real_delete_session",
        "Real Nexus deletion flow targets only explicitly selected sessions.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: harness_core
test: harness_real_delete_session
description: Real Nexus deletion flow targets only explicitly selected sessions.
backend: nexus
workflow: real Nexus harness backend workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- delete confirmation workflow was visible
"###);
    Ok(())
}
