use insta::assert_snapshot;

/// Validates the documented mouse E2E scenario.
#[test]
fn mouse_modal_buttons() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "mouse",
        "mouse_modal_buttons",
        "Clicking modal buttons confirms or cancels the visible modal action.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: mouse
test: mouse_modal_buttons
description: Clicking modal buttons confirms or cancels the visible modal action.
backend: nexus
workflow: real Nexus mouse input workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
