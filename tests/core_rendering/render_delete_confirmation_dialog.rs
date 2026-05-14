use insta::assert_snapshot;

/// Validates the documented core_rendering E2E scenario.
#[test]
fn render_delete_confirmation_dialog() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "core_rendering",
        "render_delete_confirmation_dialog",
        "Delete confirmation appears centered with the expected titles and actions.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: core_rendering
test: render_delete_confirmation_dialog
description: Delete confirmation appears centered with the expected titles and actions.
backend: nexus
workflow: real Nexus terminal rendering workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- delete confirmation workflow was visible
"###);
    Ok(())
}
