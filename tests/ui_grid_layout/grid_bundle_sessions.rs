use insta::assert_snapshot;

/// Validates the documented ui_grid_layout E2E scenario.
#[test]
fn grid_bundle_sessions() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_grid_layout",
        "grid_bundle_sessions",
        "Bundled sessions render branch markers and preserve active bundle selection.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_grid_layout
test: grid_bundle_sessions
description: Bundled sessions render branch markers and preserve active bundle selection.
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
