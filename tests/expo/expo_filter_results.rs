use insta::assert_snapshot;

/// Validates the documented expo E2E scenario.
#[test]
fn expo_filter_results() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "expo",
        "expo_filter_results",
        "Expo filtering changes visible cards and result counts predictably.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: expo
test: expo_filter_results
description: Expo filtering changes visible cards and result counts predictably.
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
