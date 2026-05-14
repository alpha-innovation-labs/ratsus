use crate::support::case_report::CaseReport;
use crate::support::create_real_app::create_real_app;
use crate::support::real_case::prepare_real_case::prepare_real_case;
use crate::support::real_case::verify_real_case::verify_real_case;
use crate::support::real_case::verify_rendered_app::verify_rendered_app;
use crate::support::render_app_text::render_app_text;
use crate::support::temporary_workspace::TemporaryWorkspace;
use crate::support::workflow::assertions_for::assertions_for;
use crate::support::workflow::workflow_for::workflow_for;

/// Runs a documented E2E case against the real Nexus-backed application.
pub fn run_real_case(domain: &str, name: &str, description: &str) -> anyhow::Result<CaseReport> {
    let workspace = TemporaryWorkspace::create(name)?;
    let mut app = create_real_app()?;
    prepare_real_case(&mut app, domain, name, &workspace)?;
    let rendered = render_app_text(&mut app, 80, 20)?;
    verify_rendered_app(&rendered)?;
    verify_real_case(&app, domain, name, &workspace)?;
    Ok(CaseReport {
        domain: domain.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        backend: "nexus".to_string(),
        workflow: workflow_for(domain, name).to_string(),
        assertions: assertions_for(domain, name),
    })
}
