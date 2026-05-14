use insta::assert_snapshot;

/// Validates the documented ui_keyboard E2E scenario.
#[test]
fn keyboard_filter_text_entry() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_keyboard",
        "keyboard_filter_text_entry",
        "Filter mode treats typed characters as query text instead of navigation keys.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_keyboard
test: keyboard_filter_text_entry
description: Filter mode treats typed characters as query text instead of navigation keys.
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
