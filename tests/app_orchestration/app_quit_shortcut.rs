use insta::assert_snapshot;

/// Validates the documented app_orchestration E2E scenario.
#[test]
fn app_quit_shortcut() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "app_orchestration",
        "app_quit_shortcut",
        "The quit shortcut returns a user-visible quit outcome without corrupting state.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: app_orchestration
test: app_quit_shortcut
description: The quit shortcut returns a user-visible quit outcome without corrupting state.
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
