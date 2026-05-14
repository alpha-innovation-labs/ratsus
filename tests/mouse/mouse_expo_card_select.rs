use insta::assert_snapshot;

/// Validates the documented mouse E2E scenario.
#[test]
fn mouse_expo_card_select() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "mouse",
        "mouse_expo_card_select",
        "Clicking an expo card focuses the intended card.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: mouse
test: mouse_expo_card_select
description: Clicking an expo card focuses the intended card.
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
