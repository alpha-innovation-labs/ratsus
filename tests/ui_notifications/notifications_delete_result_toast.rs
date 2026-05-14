use insta::assert_snapshot;

/// Validates the documented ui_notifications E2E scenario.
#[test]
fn notifications_delete_result_toast() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_notifications",
        "notifications_delete_result_toast",
        "Delete completion or failure displays the correct user-facing notification.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_notifications
test: notifications_delete_result_toast
description: Delete completion or failure displays the correct user-facing notification.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- delete confirmation workflow was visible
"###);
    Ok(())
}
