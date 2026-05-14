use insta::assert_snapshot;

/// Validates the documented mouse E2E scenario.
#[test]
fn mouse_terminal_pane_activate() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "mouse",
        "mouse_terminal_pane_activate",
        "Clicking a terminal pane makes it active.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: mouse
test: mouse_terminal_pane_activate
description: Clicking a terminal pane makes it active.
backend: nexus
workflow: real Nexus mouse input workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
