use insta::assert_snapshot;

/// Validates the documented terminal E2E scenario.
#[test]
fn terminal_spawn_real_normal_terminal() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "terminal",
        "terminal_spawn_real_normal_terminal",
        "Real normal terminal renders local terminal text through a PTY-backed shell.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: terminal
test: terminal_spawn_real_normal_terminal
description: Real normal terminal renders local terminal text through a PTY-backed shell.
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
