use insta::assert_snapshot;

/// Validates the documented mouse E2E scenario.
#[test]
fn mouse_grid_resize() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "mouse",
        "mouse_grid_resize",
        "Dragging the shell divider resizes panes safely.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: mouse
test: mouse_grid_resize
description: Dragging the shell divider resizes panes safely.
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
