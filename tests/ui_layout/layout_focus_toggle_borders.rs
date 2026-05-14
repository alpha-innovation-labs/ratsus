use insta::assert_snapshot;

/// Validates the documented ui_layout E2E scenario.
#[test]
fn layout_focus_toggle_borders() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_layout",
        "layout_focus_toggle_borders",
        "Focus changes update visible pane focus styling and footer context.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_layout
test: layout_focus_toggle_borders
description: Focus changes update visible pane focus styling and footer context.
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
