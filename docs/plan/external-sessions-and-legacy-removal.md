# Consolidated Folder View + Session Ordering Plan

## Scope

Two coupled changes in one branch:

1. **External session inclusion and ordering** — Ratsus must learn about Nexus chat sessions that were not started inside Ratsus, and must place every new session at the top of its folder. This is already implemented in the working tree (from the previous round) and stays.
2. **Restore the all-sessions-per-folder view, drop the workspace split** — replace the three-pane Workspaces | Sessions | Terminal layout with the older two-pane Folders+Sessions | Terminal layout. The left pane is a single flat list of folders, each followed by every session in that folder. The workspace split (`workspaces` pane on the far left, `sessions` pane in the middle) is removed entirely. The `+ more` per-folder cap is also removed — every session in a folder is shown.

The view to ship: **single left pane, all folders listed, every session per folder shown, sessions within each folder ordered by the new rules (newest-first, new sessions always at the top)**. No `+ more`. No Workspaces pane. No `Toggle workspace view` command. This is the consolidated folder view the user asked for.

## Goals

- A Nexus session that appears in the chat-status watcher payload (or in `nexus --sessions-all --json` at startup) but was never spawned by Ratsus is added to `app.session_terminals` instead of being silently dropped. *(Already implemented; do not regress.)*
- Within each folder's session slice, every new (unpositioned) session lands at index 0. The rest of the folder's sessions follow in the user-reorder order for ids in `SessionOrderPreferences::session_ids`, and in newest-creation-first order for unpositioned ones. *(Already implemented at the global level; needs to apply per-folder, see §3.)*
- After a refresh that adds a session, its resolved slot is written back to `SessionOrderPreferences::session_ids` so subsequent refreshes keep it in place. *(Already implemented.)*
- The left pane renders every folder in `folder_order`, and under each folder every session for that working dir. No per-folder cap. No `+ more` row.
- The Workspaces pane (`workspace_pane`) and all of its plumbing is removed. There is no `selected_workspace_path`, no `workspace_view_enabled`, no `workspace_focused_session_ids`, no `Toggle workspace view` command, no workspace order persistence. The shell layout collapses from three panes to two.
- No regression in: focused/active session preservation, day-group separators, the normal terminal slot, `keep_focused_session_visible`, drag-to-reorder (both folders and sessions), folder collapse/expand.

## Current State (post previous round)

What the previous round left in the working tree:

- `order_with_user_preferences` helper at `src/ui/left_panel/order/order_with_user_preferences.rs` works at the global `session_terminals` level: known ids in saved slots, unpositioned prepended at the top, sorted newest-first within the unpositioned block. The "always prepend" rule was applied as the final fix.
- `apply_session_refresh` is additive and returns the inserted-id set.
- `apply_session_refreshes`, `apply_initial_sessions`, and `new_chat_insert_index` all route through the helper.
- `apply_session_refreshes` re-resolves `active_index` and `focused_index` by id after the recompute.
- The `+ more` row (`SessionListRow::FolderMore`), `RECENT_SESSION_LIMIT`, `visible_session_rows_with_folders`, `folder_more_row_line`, `more_line_style`, `open_folder_history_modal`, `AppHotkey::ToggleWorkspaceView`, `toggle_workspace_view`, `AppState::workspace_view_enabled`, `PersistedWorkspaceState::workspace_view_enabled`, the workspace view `VisibleSessionRowsCache`, and the legacy test cases were deleted in the previous round. They now need to come back **only for the consolidated folder view** (folder headers, all sessions per folder, no cap) — not the legacy ten-cap mode. The ten-cap and `+ more` stay gone.
- The shell is currently three panes (Workspaces | Sessions | Terminal). It needs to become two panes (Folders+Sessions | Terminal).

## Design

### 1. Restore the all-sessions-per-folder view (no cap, no `+ more`)

Reintroduce a row builder that emits `SessionListRow::Folder { path, ... }` followed by one `SessionListRow::Session { index }` per session in that folder. There is no `SessionListRow::FolderMore` and no per-folder session cap.

- Add back `visible_session_rows_with_folders` in `src/ui/left_panel/session/visible_rows.rs`, but **without** the `RECENT_SESSION_LIMIT` logic, the `push_normal_terminal_after_limit`, `push_pinned_session`, `push_running_sessions`, and `push_unique_index` helpers, and the `FolderMore` emission. The function iterates `folder_order`, for each folder looks up the session indexes in `sessions_by_folder` (preserving the current `session_terminals` order), and emits a `Folder` row followed by a `Session` row per index.
- `SessionListRow::Folder` already exists in `src/ui/left_panel/session/list_row.rs`. The `current_session_count` / `total_session_count` fields are both equal to the folder's session count (no cap to compare against). This is the row that already exists and it stays.
- The `append_visible_folder_session_rows` helper at `src/ui/left_panel/session/split_groups/append_visible_folder_session_rows.rs` is reused as-is — it already accepts an arbitrary list of indexes and emits rows. It does not need changes.

### 2. Make the consolidated view the only view (delete the workspace split)

The shell is now two panes. Drop everything workspace-related:

- `AppState::workspace_view_enabled` field stays deleted (it was removed in the previous round; it does not come back).
- `PersistedWorkspaceState::workspace_view_enabled` stays deleted.
- `selected_workspace_path`, `workspace_focused_session_ids`, `workspace_scroll`, `last_workspace_area`, `last_workspace_list_area`, `last_left_session_toggle_area` (and the file-viewer / plans equivalents if they exist) — delete the fields and the render/mouse code that reads them.
- `AppHotkey::ToggleWorkspaceView` stays deleted.
- `toggle_workspace_view` stays deleted.
- `select_workspace_by_visible_index`, `select_workspace_first_session`, `scroll_workspace_view`, `select_workspace`, `sync_selected_workspace`, `remember_active_workspace_session` — delete the files and the `mod.rs` entries.
- The `VisibleSessionRowsCache` struct + its `rows()` / `row_count()` methods — delete it from `src/ui/left_panel/session/visible_rows_cache.rs` (and the file's `mod.rs` entry). `visible_rows_cache_methods.rs` simplifies to a single `visible_rows()` that calls `visible_session_rows_with_folders` directly.
- `visible_session_rows` (the new-mode row builder in `src/ui/left_panel/session/visible_rows.rs`) — delete. Its only caller was the cache, which is gone.
- The shell layout drops the Workspaces pane. The left pane is wider, the terminal pane is right. Adjust the layout widget if needed.
- The day-group separator logic in `src/ui/left_panel/render/rendered_rows.rs` was conditional on `selected_workspace_path.is_some()`. After deletion, day-group separators are always shown. Remove the `workspace_day_separators_enabled` gate.
- `handle_app_mouse.rs` workspace drag/drop branches — delete.
- `render_app.rs` workspace-pane branches — delete.
- `docs/ui/workspace-pane.md` — delete the file or replace it with a brief note that the workspace pane is gone and the folder view is the only one. AGENTS.md index may need the link removed.

### 3. Order fix must apply per folder, not just globally

The `order_with_user_preferences` helper currently operates on the whole `session_terminals` slice, partitioning by saved id list. With the consolidated view, the visible order is `[folder, session..., folder, session..., ...]`. The order within each folder's session slice is what the user sees in the sidebar.

Two implementation choices — pick **A**:

**(A) Global helper, per-folder projection.** Keep `order_with_user_preferences` operating on the whole `session_terminals` slice exactly as it does today. The helper reorders the flat slice so unpositioned ids are at the top globally. Then `visible_session_rows_with_folders` groups by working dir, preserving the global order within each folder. **Downside**: the per-folder visible order is the global unpositioned-then-positioned order, not "newest chats at the top of their folder regardless of which folder." For the user's stated goal — "any chat that happens that is newer than the latest historical session should be placed on top of the list" — this is exactly right: newest chat goes to the top of the global list, and the global list is grouped by folder, so the newest chat is at the top of its folder. **This is the intended behavior.**

**(B) Per-folder helper.** Add a second helper that takes a folder and its sessions and applies the same "unpositioned at top" rule per folder. This would let a chat created in folder B go to the top of folder B even if folder A has unpositioned sessions newer than it. The user said "go on top of the list" — singular list, top of list. **(A) is what the words say.**

Choose (A). No new helper. `visible_session_rows_with_folders` reads `session_terminals` in the order the helper left them and groups by `working_dir`. Within each folder, the order is the global order restricted to that folder.

The unit test `always_prepends_unpositioned_even_when_older_than_newest_positioned` (already in the tree) is the global lock; no per-folder lock needed.

### 4. Persistence

`apply_session_refreshes` and `apply_initial_sessions` already persist `SessionOrderPreferences::session_ids` after a recompute. No change.

The persisted multiplexer state in `~/.config/ratsus/session-status` (or whatever the new path is — the previous round renamed it) no longer has a `workspace` block. `PersistedMultiplexerState::workspace` becomes a struct with no fields, or is removed entirely. Open question: see Risks.

### 5. Drag-to-reorder still works

`handle_session_drag_mouse.rs`, `start_session_drag`, `move_dragged_session`, `reorder_session_to_index`, `persist_session_order_preferences` — all stay. The order helper re-runs on the next refresh / startup with the new saved order, putting the user-reordered sessions in their slots.

Folder drag (`start_folder_drag`, `move_dragged_folder`, `move_folder_order`, `reorder_folder_to_index`) — all stay.

Workspace drag (`start_workspace_drag`, `move_dragged_workspace`, `finish_workspace_drag`) — delete; the workspace concept is gone.

### 6. Expo and the folder click

Clicking a folder row in the consolidated view opens Expo scoped to that folder, exactly as the legacy grouped mode did. `activate_expo_folder` is the existing function for this; it stays. The `Folder` row arm in `activate_focused_left_row` and `handle_app_mouse` is the only thing the click does. No behavior change needed.

## Test Strategy

### Unit tests

The new `visible_session_rows_with_folders` needs tests that lock the "all sessions per folder, no cap, no `+ more`" behavior:

`src/ui/left_panel/session/visible_rows_tests.rs` (extend):

- `folder_rows_emit_all_sessions_without_more_row` — folder with 20 sessions emits a `Folder` row followed by 20 `Session` rows, no `FolderMore`.
- `folder_rows_preserve_session_terminals_order_within_a_folder` — given a global order, each folder's session rows appear in that order.
- `unpositioned_session_lands_at_top_of_its_folder` — given two folders and a brand-new unpositioned session in folder B, the first session row of folder B is the unpositioned one.
- `running_session_lands_in_its_own_folder` — sanity check that the partition is by working dir.
- `collapsed_folder_emits_no_session_rows` — collapsed folder only emits the `Folder` row.
- `hidden_folder_emits_nothing` — folder not in `folder_order` emits nothing.

The existing legacy tests that lock the ten-cap and `FolderMore` (`legacy_folder_rows_show_more_after_ten_sessions`, `legacy_folder_rows_include_focused_and_running_sessions_after_limit`) stay deleted — the cap and `+ more` are gone.

The existing `shows_all_folder_sessions_without_more_row` and `shows_chat_beyond_old_visible_limit` tests in the workspace view are replaced by the new consolidated-view tests above. `visible_session_rows` (the workspace builder) is gone, so its tests go too.

`src/app/state/visible_rows_cache_methods.rs`:

- Delete the `legacy_workspace_view_returns_all_folders` test.
- Add `consolidated_view_returns_folder_headers_and_all_sessions` — verifies `AppState::visible_rows()` returns `[Folder, Session, Session, ..., Folder, Session, ...]` for two folders with mixed session counts.

The unit tests for `order_with_user_preferences`, `apply_session_refreshes`, `apply_session_refresh`, `new_chat_insert_index`, and `apply_initial_sessions` are already in the tree and pass. They do not need changes for this round.

### E2E tests (`docs/plan/e2e_tests.md`)

Replace the previous round's E2E scenarios. New scenarios:

1. **Consolidated folder view is the only view** — start Ratsus, assert no Workspaces pane in the shell, assert the left pane shows folder headers followed by every session per folder, assert no `+ more` row anywhere.
2. **New chat (in-ratsus or external) lands at the top of its folder** — start a new Nexus chat, assert the new session is the first row under its folder header.
3. **Today sessions within a folder appear in date order** — pre-seed sessions with known dates, assert the order within each folder matches newest-first.
4. **User-reorder persists across refreshes** — drag a session to a new position, refresh, assert the new position is preserved.

### Verification

- `just fmt-check` after every code change.
- `just clippy` after every code change.
- `just test` after every code change.
- `git grep` for the deletion targets must all return zero matches (see Definition of Done).

## File Touch Points

Restore (consolidated view):

- `src/ui/left_panel/session/visible_rows.rs` — add back `visible_session_rows_with_folders` (no cap, no `+ more`). Delete `visible_session_rows` (the workspace builder).
- `src/ui/left_panel/session/visible_rows_tests.rs` — add consolidated-view tests; remove the workspace-view tests that no longer apply.

Delete (workspace split):

- `src/ui/left_panel/session/visible_rows_cache.rs` — delete the file; remove `pub mod visible_rows_cache;` from `mod.rs`.
- `src/app/state/visible_rows_cache_methods.rs` — delete the cache-mediated methods; `visible_rows()` calls `visible_session_rows_with_folders` directly. `visible_row_count()` delegates to the same builder.
- `src/ui/workspace_pane/select_workspace_by_visible_index.rs` — delete the file and its `mod.rs` entry.
- `src/ui/workspace_pane/select_workspace_first_session.rs` — already deleted in the previous round; stays deleted.
- `src/ui/workspace_pane/scroll_workspace_view.rs` — delete the file and its `mod.rs` entry.
- `src/ui/workspace_pane/toggle_workspace_view.rs` — already deleted; stays deleted.
- `src/ui/workspace_pane/select_workspace.rs` — delete the file and its `mod.rs` entry.
- `src/ui/workspace_pane/sync_selected_workspace.rs` — delete the file and its `mod.rs` entry.
- `src/ui/workspace_pane/remember_active_workspace_session.rs` — delete the file and its `mod.rs` entry.
- `src/app/expo/activate_expo_folder.rs` — keep; this is the folder-row click handler and stays.
- `src/app/state/app_state.rs` — delete the fields: `selected_workspace_path`, `workspace_focused_session_ids`, `workspace_scroll`, `last_workspace_area`, `last_workspace_list_area`, `last_left_session_toggle_area`, `last_left_plan_toggle_area`, `last_left_file_toggle_area`, `pending_left_g`, `workspace_drag`, `workspace_drag_moved`.
- `src/app/state/new_app_state.rs` — drop the field initializations and any `sync_selected_workspace` / `select_workspace` calls.
- `src/app/test_support/app_fixture.rs` — drop the field initializations and the stub `selected_workspace_path`.
- `src/app/state/app_state_methods.rs` — delete `start_workspace_drag`, `mark_workspace_drag_moved`, `move_dragged_workspace`, `finish_workspace_drag`. Drop `remember_active_workspace_session` from `activate_focused_session`.
- `src/app/sessions/drain_initial_sessions_receiver.rs` — drop the `sync_selected_workspace` call and the `selected_workspace_path` plumbing in `apply_initial_sessions`.
- `src/ui/grid_layout/persistence/persisted_workspace_state.rs` — the file is now either empty or removed. The `PersistedMultiplexerState::workspace` field is removed.
- `src/ui/grid_layout/persistence/restore_workspace_state.rs` — delete the file and its `mod.rs` entry.
- `src/ui/grid_layout/persistence/capture_workspace_state.rs` — same.
- `src/ui/grid_layout/persistence/restore_multiplexer_state_into_app.rs` — drop the workspace block reads.
- `src/ui/grid_layout/persistence/capture_multiplexer_state.rs` — same.
- `src/app/input/handle_app_mouse.rs` — drop the workspace drag/drop branches.
- `src/core/rendering/screen/render_app.rs` — drop the workspace-pane branches.
- `src/ui/left_panel/render/rendered_rows.rs` — remove the `workspace_day_separators_enabled` gate; day-group separators are always shown now.
- `src/ui/left_panel/render/session_lines.rs` — drop the workspace-only render branches.
- `src/ui/layout/resizable_grid/visible_layout_widget_state.rs`, `handle_mouse.rs` — drop the `workspace_view_enabled` parameter (already removed in the previous round; verify and clean up any stragglers).
- `src/core/rendering/resize/render_resizable_grid_overlay.rs` — drop the `workspace_view_enabled` argument.
- `src/extensions/command_bar/input/handle_keyboard.rs` — verify the `ToggleWorkspaceView` command test stays gone.
- Hotkey registry / command bar data: verify the `Toggle workspace view` command entry stays gone.
- `src/ui/left_panel/tests/mouse_lag_regression_tests.rs`, `src/ui/left_panel/tests/scrollbar_render_tests.rs`, `src/ui/left_panel/scroll/scroll_regression_tests.rs`, `src/core/rendering/screen/render_app_snapshot_tests.rs` — drop the workspace-view-specific test cases. Keep the consolidated-view regression tests.
- `docs/ui/workspace-pane.md` — delete or replace with a note that the workspace pane is gone.
- `AGENTS.md` — remove the `docs/ui/workspace-pane.md` link, remove any `select_workspace_by_visible_index` / `toggle_workspace_view` references.
- `docs/ui/left-panel.md` — rewrite to describe the consolidated folder view: folders as headers, all sessions per folder, day-group separators, the order helper, drag-to-reorder. No mention of `+ more` (it doesn't exist), no mention of legacy modes (none exist), no mention of workspaces (gone).
- `docs/extensions/command-bar.md` — verify no "Toggle workspace view" remains.
- `docs/app/state.md` — drop the deleted fields.
- `docs/plan/e2e_tests.md` — replace the previous round's scenarios with the four new ones.

## Risks and Open Questions

- **Persisted multiplexer state migration.** The persisted state file at `~/.config/ratsus/multiplexer.json` (or similar) currently has a `workspace` block with the deleted fields. With `#[serde(default)]` on the parent struct, the deserializer will ignore unknown fields and old files will load cleanly. The workspace block becomes vestigial data in old files. The user explicitly said "Never delete, rewrite, or migrate user-owned data" — so we *do not* rewrite the file. The vestigial workspace block sits in old files harmlessly. This matches the previous round's "field removed, file unchanged" pattern.
- **`is_new_nexus_chat_session` placeholder check.** Unchanged from the previous round. The id-match in `apply_refreshed_session` runs first, so a real external id always wins.
- **`VisibleSessionRowsSignature`** no longer exists — the cache is gone. Per-call recomputation is the cost; for the user's session counts (hundreds per workspace) this is fine. If it ever becomes a problem, a per-folder cache can come back later.
- **Layout widget** currently has three pane IDs (workspaces, left, terminal). After deletion it has two. The layout widget code in `src/ui/layout/resizable_grid/` needs the workspace pane ID removed from every place it's referenced. The shell splitter percentage for the workspace pane is removed from persisted state.
- **Folder selection** after the workspace pane is gone: there is no "selected workspace" concept. The folder click in the consolidated view opens Expo scoped to that folder, which is the same UX as the legacy grouped mode. No selection state is needed in the left pane for the folder view; selecting a folder = opening Expo. The file-viewer and plans modes still need to know which folder they're showing — they fall back to the active session's working dir or the first folder in `folder_order`.

## Definition of Done

- New consolidated-view unit tests fail before the implementation and pass after.
- All workspace-split tests are removed.
- `just check` is green (`fmt-check` + `clippy` + `test`).
- `just dev-stub` launches without panics and shows the consolidated folder view in the left pane.
- E2E scenarios 1-4 are documented in `docs/plan/e2e_tests.md` and at least scenario 1 is automated.
- Docs updates landed; AGENTS.md index still resolves.
- `git grep workspace_view_enabled` returns no matches.
- `git grep ToggleWorkspaceView` returns no matches.
- `git grep selected_workspace_path` returns no matches.
- `git grep workspace_pane` (in `src/`) returns no matches except for the directory itself.
- `git grep VisibleSessionRowsCache` returns no matches.
- `git grep open_folder_history_modal` returns no matches (still gone, not coming back).
- The 587 passing tests from the previous round still pass; the 1 pre-existing snapshot failure (`picker_place_existing_in_split_group`) is unchanged (it fails on baseline too, not caused by this change).
