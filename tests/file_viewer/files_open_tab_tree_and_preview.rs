use insta::assert_snapshot;

/// Validates the documented file_viewer E2E scenario.
#[test]
fn files_open_tab_tree_and_preview() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "file_viewer",
        "files_open_tab_tree_and_preview",
        "Files tab renders the tree in the left pane and preview in the main pane.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: file_viewer
test: files_open_tab_tree_and_preview
description: Files tab renders the tree in the left pane and preview in the main pane.
backend: nexus
workflow: real Nexus full-app workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
- real temporary file tree was rendered
"###);
    Ok(())
}
