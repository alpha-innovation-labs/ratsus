use insta::assert_snapshot;

/// Validates the documented harness_core E2E scenario.
#[test]
fn harness_nexus_owned_mutation_only() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "harness_core",
        "harness_nexus_owned_mutation_only",
        "Nexus mutations apply only to manifest-owned sessions.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: harness_core
test: harness_nexus_owned_mutation_only
description: Nexus mutations apply only to manifest-owned sessions.
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
