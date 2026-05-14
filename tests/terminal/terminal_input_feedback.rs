use insta::assert_snapshot;

/// Validates the documented terminal E2E scenario.
#[test]
fn terminal_input_feedback() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "terminal",
        "terminal_input_feedback",
        "Typed input reaches the active real terminal and remains observable in the terminal pane.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: terminal
test: terminal_input_feedback
description: Typed input reaches the active real terminal and remains observable in the terminal pane.
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
