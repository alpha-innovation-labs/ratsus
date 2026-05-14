use insta::assert_snapshot;

/// Validates the documented observation E2E scenario.
#[test]
fn observations_missing_state_fallback() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "observation",
        "observations_missing_state_fallback",
        "Missing observation files render a safe empty preview state.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: observation
test: observations_missing_state_fallback
description: Missing observation files render a safe empty preview state.
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
