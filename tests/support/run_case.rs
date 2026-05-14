use crate::support::case_report::CaseReport;
use crate::support::real_case::run_real_case::run_real_case;

/// Runs one documented E2E case against the real Nexus-backed application.
pub fn run_case(domain: &str, name: &str, description: &str) -> anyhow::Result<CaseReport> {
    run_real_case(domain, name, description)
}
