use insta::assert_snapshot;

/// Validates the documented ui_notifications E2E scenario.
#[test]
fn notifications_toast_expiry() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_notifications",
        "notifications_toast_expiry",
        "Expired toasts disappear on redraw without affecting layout.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_notifications
test: notifications_toast_expiry
description: Expired toasts disappear on redraw without affecting layout.
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
