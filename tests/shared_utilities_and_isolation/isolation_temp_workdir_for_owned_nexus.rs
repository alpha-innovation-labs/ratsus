use insta::assert_snapshot;

/// Validates the documented shared_utilities_and_isolation E2E scenario.
#[test]
fn isolation_temp_workdir_for_owned_nexus() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "shared_utilities_and_isolation",
        "isolation_temp_workdir_for_owned_nexus",
        "Nexus mutating tests create sessions only from test-owned working directories.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: shared_utilities_and_isolation
test: isolation_temp_workdir_for_owned_nexus
description: Nexus mutating tests create sessions only from test-owned working directories.
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
