use anyhow::Result;
use ratkit::{run, RunnerConfig};

use crate::app::nexus_demo_state::NexusDemo;

/// Starts the Nexus TUI application with the default Ratkit runner configuration.
pub fn run_nexus_app() -> Result<()> {
    let app = NexusDemo::new()?;
    run(app, RunnerConfig::default())?;
    Ok(())
}
