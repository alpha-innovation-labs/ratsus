use insta::assert_snapshot;

/// Validates the documented file_viewer E2E scenario.
#[test]
fn files_missing_or_binary_preview() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "file_viewer",
        "files_missing_or_binary_preview",
        "Unsupported files render a safe user-visible fallback.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: file_viewer
test: files_missing_or_binary_preview
description: Unsupported files render a safe user-visible fallback.
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
