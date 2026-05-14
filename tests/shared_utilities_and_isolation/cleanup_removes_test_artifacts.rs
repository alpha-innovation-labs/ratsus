use insta::assert_snapshot;

/// Validates the documented shared_utilities_and_isolation E2E scenario.
#[test]
fn cleanup_removes_test_artifacts() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "shared_utilities_and_isolation",
        "cleanup_removes_test_artifacts",
        "Test-owned temp directories, manifests, and files are removed after each test.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: shared_utilities_and_isolation
test: cleanup_removes_test_artifacts
description: Test-owned temp directories, manifests, and files are removed after each test.
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
