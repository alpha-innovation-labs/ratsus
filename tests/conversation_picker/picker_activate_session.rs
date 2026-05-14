use insta::assert_snapshot;

/// Validates the documented conversation_picker E2E scenario.
#[test]
fn picker_activate_session() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "conversation_picker",
        "picker_activate_session",
        "Enter opens the selected session and closes the picker.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: conversation_picker
test: picker_activate_session
description: Enter opens the selected session and closes the picker.
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
