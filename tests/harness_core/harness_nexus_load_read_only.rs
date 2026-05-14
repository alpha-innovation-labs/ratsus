use insta::assert_snapshot;

/// Validates the documented harness_core E2E scenario.
#[test]
fn harness_nexus_load_read_only() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "harness_core",
        "harness_nexus_load_read_only",
        "Nexus loading reads real sessions without writes or deletes.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: harness_core
test: harness_nexus_load_read_only
description: Nexus loading reads real sessions without writes or deletes.
backend: nexus
workflow: real Nexus harness backend workflow
assertions:
- real Nexus harness loaded without test doubles
- real Nexus session catalog loaded through the application
- full app rendered through ratatui TestBackend
- test-owned temporary workspace protected operator files
"###);
    Ok(())
}
