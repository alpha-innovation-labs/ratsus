use insta::assert_snapshot;

/// Validates the documented ui_menu_bar E2E scenario.
#[test]
fn menu_switch_to_expo() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_menu_bar",
        "menu_switch_to_expo",
        "Menu navigation activates Expo and renders folder cards.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_menu_bar
test: menu_switch_to_expo
description: Menu navigation activates Expo and renders folder cards.
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
