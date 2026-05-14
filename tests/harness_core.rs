mod support;

#[path = "harness_core/harness_nexus_load_read_only.rs"]
mod harness_nexus_load_read_only;
#[path = "harness_core/harness_nexus_owned_mutation_only.rs"]
mod harness_nexus_owned_mutation_only;
#[path = "harness_core/harness_real_delete_session.rs"]
mod harness_real_delete_session;
#[path = "harness_core/harness_real_load_sessions.rs"]
mod harness_real_load_sessions;
#[path = "harness_core/harness_real_refresh_stability.rs"]
mod harness_real_refresh_stability;
#[path = "harness_core/harness_real_spawn_new_chat.rs"]
mod harness_real_spawn_new_chat;
#[path = "harness_core/harness_real_spawn_normal_terminal.rs"]
mod harness_real_spawn_normal_terminal;
