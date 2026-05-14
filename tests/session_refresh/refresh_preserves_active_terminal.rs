use insta::assert_snapshot;

/// Validates the documented session_refresh E2E scenario.
#[test]
fn refresh_preserves_active_terminal() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "session_refresh",
        "refresh_preserves_active_terminal",
        "Refresh keeps the active terminal pane attached to the correct session.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: session_refresh
test: refresh_preserves_active_terminal
description: Refresh keeps the active terminal pane attached to the correct session.
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
