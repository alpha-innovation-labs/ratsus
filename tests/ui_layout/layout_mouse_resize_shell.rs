use insta::assert_snapshot;

/// Validates the documented ui_layout E2E scenario.
#[test]
fn layout_mouse_resize_shell() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_layout",
        "layout_mouse_resize_shell",
        "Mouse resizing changes pane dimensions predictably and preserves content.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_layout
test: layout_mouse_resize_shell
description: Mouse resizing changes pane dimensions predictably and preserves content.
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
