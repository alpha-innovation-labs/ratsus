use insta::assert_snapshot;

/// Validates the documented terminal E2E scenario.
#[test]
fn terminal_open_existing_real_session() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "terminal",
        "terminal_open_existing_real_session",
        "Opening a dormant real Nexus session renders its terminal pane.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: terminal
test: terminal_open_existing_real_session
description: Opening a dormant real Nexus session renders its terminal pane.
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
