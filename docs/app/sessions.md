# App sessions

`src/app/sessions/` owns app-level session list operations that are not backend-specific and not terminal implementation details.

## Responsibilities

- Clamp active and focused session indexes.
- Adjust indexes after removals.
- Detect exited session entries.
- Close or remove exited sessions from app state.
- Restore focus after bulk deletion.
- Choose normal-terminal insertion points and working directories.
- Start new normal terminal sessions through the active harness policy.

## Boundary

Backend session loading and deletion belong in `src/extensions/harness/`. PTY process behavior belongs in `src/extensions/terminal/`. This module coordinates the app's session collection and visible selection state.

## Key files

- `src/app/sessions/clamp_session_index.rs`
- `src/app/sessions/adjust_index_after_removals.rs`
- `src/app/sessions/exited_session_indices.rs`
- `src/app/sessions/close_exited_sessions.rs`
- `src/app/sessions/remove_exited_sessions.rs`
- `src/app/sessions/restore_focus_after_bulk_delete.rs`
- `src/app/sessions/start_new_normal_terminal.rs`
