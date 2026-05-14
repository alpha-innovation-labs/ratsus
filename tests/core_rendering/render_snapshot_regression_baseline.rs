use insta::assert_snapshot;

/// Validates the documented core_rendering E2E scenario.
#[test]
fn render_snapshot_regression_baseline() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "core_rendering",
        "render_snapshot_regression_baseline",
        "Stable snapshots catch visible regressions in layout, text, and symbols.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: core_rendering
test: render_snapshot_regression_baseline
description: Stable snapshots catch visible regressions in layout, text, and symbols.
backend: nexus
workflow: real Nexus terminal rendering workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
