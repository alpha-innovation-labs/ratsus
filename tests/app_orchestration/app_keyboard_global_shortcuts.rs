use insta::assert_snapshot;

/// Validates the documented app_orchestration E2E scenario.
#[test]
fn app_keyboard_global_shortcuts() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "app_orchestration",
        "app_keyboard_global_shortcuts",
        "Global shortcuts route to the correct UI workflow before focused-pane handling.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: app_orchestration
test: app_keyboard_global_shortcuts
description: Global shortcuts route to the correct UI workflow before focused-pane handling.
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
