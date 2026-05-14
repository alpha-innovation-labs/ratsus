use insta::assert_snapshot;

/// Validates the documented terminal E2E scenario.
#[test]
fn terminal_resume_owned_nexus_session() -> anyhow::Result<()> {
    let report = crate::support::run_case(
        "terminal",
        "terminal_resume_owned_nexus_session",
        "Resuming a manifest-owned Nexus session opens only that owned terminal.",
    )?;
    assert_snapshot!(report.to_snapshot(), @r###"
domain: terminal
test: terminal_resume_owned_nexus_session
description: Resuming a manifest-owned Nexus session opens only that owned terminal.
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
