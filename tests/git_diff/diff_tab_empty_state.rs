use insta::assert_snapshot;

/// Validates the documented git_diff E2E scenario.
#[test]
fn diff_tab_empty_state() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "git_diff",
        "diff_tab_empty_state",
        "The Diff tab renders a stable empty state when no diff feature data is available.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: git_diff
test: diff_tab_empty_state
description: The Diff tab renders a stable empty state when no diff feature data is available.
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
