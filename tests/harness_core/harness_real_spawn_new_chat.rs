use insta::assert_snapshot;

/// Validates the documented harness_core E2E scenario.
#[test]
fn harness_real_spawn_new_chat() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "harness_core",
        "harness_real_spawn_new_chat",
        "Real Nexus new-chat creation adds a visible session and terminal body when enabled.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: harness_core
test: harness_real_spawn_new_chat
description: Real Nexus new-chat creation adds a visible session and terminal body when enabled.
backend: nexus
workflow: real Nexus harness backend workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
