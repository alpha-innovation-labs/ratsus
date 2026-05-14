use insta::assert_snapshot;

/// Validates the documented ui_notifications E2E scenario.
#[test]
fn notifications_spawn_failure_toast() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_notifications",
        "notifications_spawn_failure_toast",
        "A failed user action displays a clear toast instead of failing silently.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_notifications
test: notifications_spawn_failure_toast
description: A failed user action displays a clear toast instead of failing silently.
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
