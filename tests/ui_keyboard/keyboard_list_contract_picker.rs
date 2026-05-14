use insta::assert_snapshot;

/// Validates the documented ui_keyboard E2E scenario.
#[test]
fn keyboard_list_contract_picker() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_keyboard",
        "keyboard_list_contract_picker",
        "Picker navigation, filtering, activation, and close behavior follow the shared list contract.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_keyboard
test: keyboard_list_contract_picker
description: Picker navigation, filtering, activation, and close behavior follow the shared list contract.
backend: nexus
workflow: real Nexus keyboard input workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
