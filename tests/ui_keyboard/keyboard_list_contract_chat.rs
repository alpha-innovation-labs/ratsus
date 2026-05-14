use insta::assert_snapshot;

/// Validates the documented ui_keyboard E2E scenario.
#[test]
fn keyboard_list_contract_chat() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_keyboard",
        "keyboard_list_contract_chat",
        "Chat lists honor `j/k`, arrows, `gg/G`, Enter, `/`, `d`, space, and `q`.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_keyboard
test: keyboard_list_contract_chat
description: Chat lists honor `j/k`, arrows, `gg/G`, Enter, `/`, `d`, space, and `q`.
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
