use std::sync::Arc;

use anyhow::Result;
use ratsus::app::run_app;
use ratsus::harness::ChatHarness;
use ratsus::harnesses::nexus_harness;
use ratsus::harnesses::stub::StubHarness;

fn main() -> Result<()> {
    let chat_harness: Arc<dyn ChatHarness> = match std::env::var("RATSUS_BACKEND").as_deref() {
        Ok("stub") => Arc::new(StubHarness::new()),
        _ => nexus_harness(),
    };
    run_app(chat_harness)
}
