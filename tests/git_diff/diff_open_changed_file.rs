use insta::assert_snapshot;

/// Validates the documented git_diff E2E scenario.
#[test]
fn diff_open_changed_file() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "git_diff",
        "diff_open_changed_file",
        "When diff data exists, selecting a changed file renders its diff preview.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: git_diff
test: diff_open_changed_file
description: When diff data exists, selecting a changed file renders its diff preview.
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
