use insta::assert_snapshot;

/// Validates the documented observation E2E scenario.
#[test]
fn observations_watcher_refresh() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "observation",
        "observations_watcher_refresh",
        "Watcher or tick refresh updates visible previews after observation changes.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: observation
test: observations_watcher_refresh
description: Watcher or tick refresh updates visible previews after observation changes.
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
