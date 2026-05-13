use std::sync::Arc;

use anyhow::Result;
use ratsus::app::lifecycle::run_app::run_app;
use ratsus::extensions::harness::nexus_harness;
use ratsus::extensions::harness::stub::StubHarness;
use ratsus::extensions::harness::ChatHarness;

fn main() -> Result<()> {
    let chat_harness: Arc<dyn ChatHarness> = match std::env::var("RATSUS_BACKEND").as_deref() {
        Ok("stub") => Arc::new(StubHarness::new()),
        _ => nexus_harness(),
    };
    run_app(chat_harness)
}
