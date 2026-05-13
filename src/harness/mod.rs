//! Backend-neutral harness contracts for chat backends.

pub mod chat_harness;
pub mod chat_session;

pub use chat_harness::ChatHarness;
pub use chat_session::{ChatSession, ChatSessionKind};
