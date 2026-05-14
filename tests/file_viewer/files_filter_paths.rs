use insta::assert_snapshot;

/// Validates the documented file_viewer E2E scenario.
#[test]
fn files_filter_paths() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "file_viewer",
        "files_filter_paths",
        "Filtering narrows visible paths and keeps a valid selected path.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: file_viewer
test: files_filter_paths
description: Filtering narrows visible paths and keeps a valid selected path.
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
