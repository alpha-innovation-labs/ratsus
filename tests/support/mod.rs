//! Shared E2E support for Ratsus integration tests.

pub mod case_report;
pub mod create_real_app;
pub mod real_case;
pub mod render_app_text;
pub mod run_case;
pub mod temporary_workspace;
pub mod workflow;

pub use run_case::run_case;
