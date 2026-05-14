use insta::assert_snapshot;

/// Validates the documented conversation_picker E2E scenario.
#[test]
fn picker_place_in_split() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "conversation_picker",
        "picker_place_in_split",
        "Split-placement mode places the selected session in the active pane.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: conversation_picker
test: picker_place_in_split
description: Split-placement mode places the selected session in the active pane.
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
