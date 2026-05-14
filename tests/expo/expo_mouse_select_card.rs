use insta::assert_snapshot;

/// Validates the documented expo E2E scenario.
#[test]
fn expo_mouse_select_card() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "expo",
        "expo_mouse_select_card",
        "Clicking a card focuses or opens the intended conversation.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: expo
test: expo_mouse_select_card
description: Clicking a card focuses or opens the intended conversation.
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
