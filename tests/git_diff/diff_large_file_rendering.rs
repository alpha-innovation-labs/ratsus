use insta::assert_snapshot;

/// Validates the documented git_diff E2E scenario.
#[test]
fn diff_large_file_rendering() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "git_diff",
        "diff_large_file_rendering",
        "Large diffs scroll predictably and preserve visible hunk boundaries.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: git_diff
test: diff_large_file_rendering
description: Large diffs scroll predictably and preserve visible hunk boundaries.
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
