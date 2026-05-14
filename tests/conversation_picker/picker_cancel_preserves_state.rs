use insta::assert_snapshot;

/// Validates the documented conversation_picker E2E scenario.
#[test]
fn picker_cancel_preserves_state() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "conversation_picker",
        "picker_cancel_preserves_state",
        "Cancelling the picker returns to the previous app state.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: conversation_picker
test: picker_cancel_preserves_state
description: Cancelling the picker returns to the previous app state.
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
