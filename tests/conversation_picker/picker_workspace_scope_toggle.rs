use insta::assert_snapshot;

/// Validates the documented conversation picker workspace-scope scenario.
#[test]
fn picker_workspace_scope_toggle() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "conversation_picker",
        "picker_workspace_scope_toggle",
        "Ctrl+H opens conversations scoped to the selected workspace and shows the scope in the header.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: conversation_picker
test: picker_workspace_scope_toggle
description: Ctrl+H opens conversations scoped to the selected workspace and shows the scope in the header.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- conversation picker was scoped to the selected workspace
"###);
    Ok(())
}
