# App deletion

`src/app/deletion/` owns app-level deletion flow for chat sessions. It coordinates confirmation UI state, selected targets, worker startup, and cleanup after backend deletion finishes. The concrete deletion operation remains harness-owned.

## Responsibilities

- Build the delete confirmation message and title.
- Track confirmation dialog state.
- Determine which selected or focused sessions can be removed.
- Spawn the deletion worker.
- Run backend chat deletions one at a time inside that worker so harness CLIs that mutate shared session state do not race.
- Complete deletion by killing open terminals, removing sessions, restoring focus, and syncing folder order.
- Handle delete confirmation keyboard input.

## Safety

Deletion must go through the active `ChatHarness`. Harness-specific deletion belongs in the concrete harness adapter, such as `src/extensions/harness/nexus/` or a future `src/extensions/harness/<harness_name>/`. Nexus deletion uses the Nexus CLI and must never rewrite raw Nexus storage directly.

## Key files

- `src/app/deletion/open_delete_session_confirmation.rs`
- `src/app/deletion/selected_delete_targets.rs`
- `src/app/deletion/removable_delete_session_ids.rs`
- `src/app/deletion/spawn_delete_sessions_worker.rs`
- `src/app/deletion/complete_delete_sessions.rs`
- `src/app/deletion/handle_delete_session_confirmation_keyboard.rs`
