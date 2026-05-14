use insta::assert_snapshot;

/// Validates the documented expo E2E scenario.
#[test]
fn expo_observation_preview_real() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "expo",
        "expo_observation_preview_real",
        "Real observation previews render Nexus preview text when available.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: expo
test: expo_observation_preview_real
description: Real observation previews render Nexus preview text when available.
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
