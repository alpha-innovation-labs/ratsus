use insta::assert_snapshot;

/// Validates the documented ui_menu_bar E2E scenario.
#[test]
fn menu_inactive_tabs_render_stably() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_menu_bar",
        "menu_inactive_tabs_render_stably",
        "Inactive or empty tabs render stable placeholders without panics.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_menu_bar
test: menu_inactive_tabs_render_stably
description: Inactive or empty tabs render stable placeholders without panics.
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
