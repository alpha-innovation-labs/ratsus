use insta::assert_snapshot;

/// Validates the documented observation E2E scenario.
#[test]
fn observations_nexus_owned_preview_load() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "observation",
        "observations_nexus_owned_preview_load",
        "Nexus observation loading reads preview state for owned sessions only.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: observation
test: observations_nexus_owned_preview_load
description: Nexus observation loading reads preview state for owned sessions only.
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
