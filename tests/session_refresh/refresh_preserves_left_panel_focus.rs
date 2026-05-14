use insta::assert_snapshot;

/// Validates the documented session_refresh E2E scenario.
#[test]
fn refresh_preserves_left_panel_focus() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_refresh",
        "refresh_preserves_left_panel_focus",
        "Refresh keeps visible focus stable when the focused session still exists.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_refresh
test: refresh_preserves_left_panel_focus
description: Refresh keeps visible focus stable when the focused session still exists.
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
