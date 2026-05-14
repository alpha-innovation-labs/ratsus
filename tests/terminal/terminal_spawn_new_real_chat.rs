use insta::assert_snapshot;

/// Validates the documented terminal E2E scenario.
#[test]
fn terminal_spawn_new_real_chat() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "terminal",
        "terminal_spawn_new_real_chat",
        "New real Nexus chat renders terminal output and selected catalog row when enabled.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: terminal
test: terminal_spawn_new_real_chat
description: New real Nexus chat renders terminal output and selected catalog row when enabled.
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
