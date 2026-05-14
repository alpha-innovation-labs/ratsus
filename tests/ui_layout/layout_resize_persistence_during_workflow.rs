use insta::assert_snapshot;

/// Validates the documented ui_layout E2E scenario.
#[test]
fn layout_resize_persistence_during_workflow() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "ui_layout",
        "layout_resize_persistence_during_workflow",
        "Layout remains stable after resize followed by picker, delete modal, and tab changes.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: ui_layout
test: layout_resize_persistence_during_workflow
description: Layout remains stable after resize followed by picker, delete modal, and tab changes.
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
