# External Session Inclusion + Legacy Mode Removal Plan

## Scope

Two coupled changes in one branch:

1. **External session inclusion and ordering** — Ratsus must learn about Nexus chat sessions that were not started inside Ratsus, and must place every new session in the right slot of its own ordering scheme.
2. **Legacy grouped folder mode removal** — delete the `workspace_view_enabled = false` code path, the `+ more` overflow row, and every test/lock-in that depends on the ten-row per-folder cap. Workspace view becomes the only left-pane mode.

Both ship together because the legacy mode hides the same external sessions the inclusion change is meant to surface.

## Goals

- A Nexus session that appears in the chat-status watcher payload (or in `nexus --sessions-all --json` at startup) but was never spawned by Ratsus is added to `app.session_terminals` instead of being silently dropped.
- Every new session is placed in the user-reorder order when its id is in `SessionOrderPreferences::session_ids`; otherwise it is placed by creation timestamp, with newer-than-top unpositioned ids landing at position 0 of `session_terminals`.
- After a refresh that adds a session, its resolved slot is written back to `SessionOrderPreferences::session_ids` so subsequent refreshes keep it in place.
- The legacy grouped folder mode and the `+ more` row are gone. Workspace view is the only left-pane mode, persisted as the default.
- No regression in: focused/active session preservation, day-group separators, the normal terminal slot, or `keep_focused_session_visible`.

## Current State

- `apply_session_refresh` (`src/extensions/harness/nexus/refresh/apply_session_refresh.rs`) updates existing sessions by id and replaces placeholders by working dir. Unknown ids are silently dropped (line 49-62).
- `apply_session_refreshes` (`src/extensions/harness/sessions/refresh/apply_session_refreshes.rs`) just calls `merge_session_refresh`, then syncs folder/workspace state and calls `keep_focused_session_visible`.
- `apply_session_id_order` (`src/ui/left_panel/order/apply_session_id_order.rs`) reorders so that any id missing from `session_ids` gets `usize::MAX` and is appended at the end. Used both at startup and after any insert path.
- `new_chat_insert_index` (`src/extensions/harness/sessions/creation/new_chat_insert_index.rs`) puts a new in-ratsus chat at the first index of its working dir, or at 0.
- Legacy mode: `visible_session_rows_with_folders` (`src/ui/left_panel/session/visible_rows.rs:53-99`) caps each folder at `RECENT_SESSION_LIMIT = 10`, splices running/focused/normal-terminal sessions back in, and emits `SessionListRow::FolderMore`. `SessionListRow::FolderMore` is rendered by `folder_more_row_line` (`src/ui/left_panel/render/session_row_line.rs:177`) and activated by `open_folder_history_modal` (`src/ui/left_panel/input/activate_focused_row.rs:17` and `src/app/input/handle_app_mouse.rs:142`).
- `visible_rows_cache_methods.rs` branches on `workspace_view_enabled` to pick `visible_session_rows_with_folders` vs `visible_session_rows_cache.rows()`. The toggle is `toggle_workspace_view` bound to `AppHotkey::ToggleWorkspaceView`, persisted in `PersistedWorkspaceState`.

## Design

### 1. Additive merge in `apply_session_refresh`

Convert the per-refresh loop from "update or placeholder-replace" into "update, placeholder-replace, or insert":

- For each refreshed session:
  - If id matches an existing entry → update in place (today's behavior).
  - Else if a placeholder (`is_new_nexus_chat_session`) with the same working dir exists → replace placeholder with the refreshed session.
  - Else → append a new `SessionTerminal::dormant_with_harness(refreshed_session, harness)` to the slice.
- Track inserted ids in a `BTreeSet<String>` returned to the caller so the order helper can promote them.

The function signature changes from `bool` to `(bool, BTreeSet<String>)`. The bool keeps its meaning ("anything changed"); the set reports newly inserted ids.

### 2. Order recompute helper

Add `order_with_user_preferences` in `src/ui/left_panel/order/`:

- Inputs: `&mut Vec<SessionTerminal>`, `&[String]` (saved session ids in their saved order), `&BTreeSet<String>` (ids that were newly inserted in this refresh).
- Output: `Vec<String>` — the new saved session id list to persist, in the resolved order.

Algorithm:

1. Build `positions: HashMap<&str, usize>` from the input saved ids.
2. For each id in the input order, pull the matching terminal from the working set and push to `output_terminals` and `output_ids`. Skip ids that aren't in the working set (stale preference).
3. Collect the remaining terminals (unpositioned) into a buffer.
4. Sort the buffer by `Reverse(session_created_timestamp)` (newest first).
5. Unpositioned sessions are always prepended above the positioned block, sorted newest-first within the unpositioned block. The prepended block's resolved ids are persisted to `SessionOrderPreferences::session_ids`.
6. Replace `*session_terminals` with `output_terminals`. Return `output_ids`.

`apply_session_id_order` becomes a thin wrapper that calls this helper with an empty insertion set (preserves the current startup-time behavior). `new_chat_insert_index` becomes a thin wrapper that calls this helper with the new chat's id as the only "inserted" id, and returns the resolved index of that id.

### 3. Persistence on refresh

In `apply_session_refreshes`:

- After the merge, if `changed`:
  - Read the current `SessionOrderPreferences`.
  - Call `order_with_user_preferences(&mut session_terminals, &preferences.session_ids, &inserted_ids)`.
  - Replace `preferences.session_ids` with the returned vector.
  - Persist preferences via `persist_session_order_preferences`.
  - Continue with the existing `sync_folder_order` / `sync_selected_workspace` / `keep_focused_session_visible` steps.
- The active and focused indexes are remapped by the new `session_terminals` order — they are looked up by id after the recompute (same pattern `apply_initial_sessions` uses for `active_session_id`).

### 4. Initial load alignment

`apply_initial_sessions` (`src/app/sessions/drain_initial_sessions_receiver.rs:44`) currently does `sort_sessions_by_creation_date` then `apply_session_id_order`. Replace with the new helper so startup-time placement matches refresh-time placement. Same id, same slot. This is the change that makes "the order of sessions is managed by Ratsus" true in one place.

### 5. Legacy mode deletion

Mechanical removal:

- `SessionListRow::FolderMore` variant and all match arms that destructure it.
- `visible_session_rows_with_folders` and its tests in `visible_rows_tests.rs` and `visible_rows_running_tests.rs`.
- `RECENT_SESSION_LIMIT`, `visible_folder_session_indexes`, `push_normal_terminal_after_limit`, `push_pinned_session`, `push_running_sessions`, `push_unique_index` — all in `src/ui/left_panel/session/visible_rows.rs`.
- `folder_more_row_line` and `more_line_style` in `src/ui/left_panel/render/session_row_line.rs`.
- The `SessionListRow::FolderMore` arm in `src/ui/left_panel/render/session_lines.rs`, `src/ui/left_panel/render/rendered_rows.rs`, `src/ui/left_panel/folder/open_focused_project.rs`, `src/ui/left_panel/folder/collapse_focused_project.rs`, `src/ui/left_panel/input/handle_session_drag_mouse.rs`, `src/ui/left_panel/focus/focus_row.rs`, `src/ui/left_panel/input/activate_focused_row.rs`, `src/app/input/handle_app_mouse.rs`, `src/app/state/app_state_methods.rs`.
- `open_folder_history_modal` and its whole file: confirmed dead after `FolderMore` deletion. Two callers, both gone — `src/app/input/handle_app_mouse.rs:142` (`SessionListRow::FolderMore { path } => open_folder_history_modal(app, path)`) and `src/ui/left_panel/input/activate_focused_row.rs:17` (same arm). The sibling `open_history_modal` in `src/extensions/harness/history_modal/actions/open.rs` is the `Ctrl+H` path, takes `selected_workspace_path`, and is unrelated — it stays. Delete `src/extensions/harness/history_modal/actions/open_folder.rs` and remove `pub mod open_folder;` from `src/extensions/harness/history_modal/actions/mod.rs`.
- `AppHotkey::ToggleWorkspaceView`, `toggle_workspace_view`, `workspace_view_enabled`, `PersistedWorkspaceState::workspace_view_enabled`, the `default_workspace_view_enabled` helper, the field in `AppState`, and the test cases that toggle it (`handle_keyboard_event`, `select_workspace_by_visible_index`).
- The `if !workspace_view_enabled` branches in `visible_rows_cache_methods.rs`, `restore_multiplexer_state_into_app.rs`, `capture_multiplexer_state.rs`, `render_app.rs`, `select_workspace_by_visible_index.rs`, `app_state_methods.rs` (for `FolderMore`), and `session_lines.rs:129`.
- `select_workspace_by_visible_index` legacy branch (the test that sets `workspace_view_enabled = false` goes away with the field).
- Mouse and layout paths that branch on `workspace_view_enabled` (`handle_app_mouse.rs`, `visible_layout_widget_state`, `handle_mouse.rs`, `render_resizable_grid_overlay.rs`).

Persistence migration:

- The `persisted_multiplexer_state` workspace block currently stores `workspace_view_enabled`. After deletion, drop the field. Old persisted files still deserialize via `#[serde(default)]` on the other fields, so no migration is required for users who had the legacy default false or the workspace true — they all just land in workspace view.

### 6. Documentation

- `docs/ui/left-panel.md`: drop the legacy bullets, drop `visible_rows_with_folders` references, drop the `focused/` and `order/` sub-bullet that describes the cap.
- `docs/extensions/command-bar.md`: drop the "Toggle workspace view" sentence.
- `docs/ui/workspace-pane.md`: drop the "two modes" framing; document workspace view as the only mode.
- `docs/plan/source-layout-domains.md`: no domain-boundary change, but mention the deletion under "App → UI → left panel" if it lists capabilities.
- `docs/app/state.md`: drop `workspace_view_enabled` from the field list.
- `AGENTS.md`: the "App" section's `visible_rows_cache_methods.rs` line stays; no link to the deleted legacy file.
- `docs/plan/e2e_tests.md`: add the new E2E scenarios below.

## Test Strategy

### Unit tests (test-first, written before code changes)

`src/extensions/harness/nexus/refresh/apply_session_refresh_tests.rs` (extend):

- `adds_external_session_with_unknown_id` — refresh with one id that is not in the slice ends with the new id in the slice; returned set contains that id.
- `updates_existing_session_by_id` — existing test, unchanged behavior.
- `replaces_new_chat_placeholder_by_working_dir` — existing test, unchanged.
- `reports_inserted_ids_for_placeholder_replacements` — placeholder replacement also reports the new id so the order helper can promote it.
- `preserves_unknown_id_for_external_session_with_matching_placeholder_working_dir` — when both id-lookup and placeholder-lookup would match, id wins.

`src/ui/left_panel/order/order_with_user_preferences_tests.rs` (new):

- `positions_known_ids_in_saved_order`
- `appends_unpositioned_ids_in_creation_date_descending_order`
- `prepends_unpositioned_when_newest_exceeds_positioned_top`
- `drops_saved_ids_not_in_working_set`
- `returns_persistable_id_list_in_resolved_order`
- `promotes_inserted_ids_to_positioned_slots` — passes an id in the inserted set; assert it appears in the returned saved-ids list at its resolved slot.
- `keeps_existing_known_ids_unchanged_when_inserted_set_is_empty`

`src/extensions/harness/sessions/refresh/apply_session_refreshes_tests.rs` (extend):

- `external_session_appears_in_session_terminals_after_refresh`
- `external_newer_than_top_lands_at_position_zero`
- `refresh_persists_new_session_id_in_session_order_preferences`
- `focused_session_remains_focused_after_external_insert`

`src/extensions/harness/sessions/creation/new_chat_insert_index_tests.rs` (extend):

- `new_in_ratsus_chat_lands_at_top_when_newer_than_everything` — verifies the new wrapper behavior for the in-ratsus path.

`src/app/sessions/drain_initial_sessions_receiver_tests.rs` (extend or add):

- `initial_load_uses_order_with_user_preferences` — verifies the unified path.

`src/ui/left_panel/session/visible_rows_tests.rs` and `visible_rows_running_tests.rs`:

- Delete the two tests that lock the 10-cap and `FolderMore` behavior:
  - `legacy_folder_rows_show_more_after_ten_sessions`
  - `legacy_folder_rows_include_focused_and_running_sessions_after_limit`
  - `legacy_folder_rows_returns_all_folders` (in `visible_rows_cache_methods`)

`src/app/state/visible_rows_cache_methods.rs` tests:

- The `legacy_workspace_view_returns_all_folders` test goes away with the field.

Mouse and rendering tests that asserted `FolderMore` interactions:

- Grep `FolderMore` in tests and remove the affected test cases (likely in `src/ui/left_panel/tests/mouse_lag_regression_tests.rs`, `src/ui/left_panel/tests/scrollbar_render_tests.rs`, `src/ui/left_panel/scroll/scroll_regression_tests.rs`).

### E2E tests (extend `docs/plan/e2e_tests.md` and add scenarios)

New scenarios:

1. **External Nexus session appears without restart** — start Ratsus with workspace view, run `nexus` in a shell to start a new chat, wait for the watcher tick, assert the new chat is in the session list at the top.
2. **External older Nexus session slots by date** — start Ratsus, run `nexus` with a backdated `created_at`, assert it slots in below the positioned entries rather than at the top.
3. **Workspace view is the only mode** — start Ratsus, assert no `+ more` row in the sidebar; assert the `Toggle workspace view` command is absent from the command bar.
4. **User-reorder persists across external insert** — reorder two sessions, then trigger an external session, assert the user-reordered pair stays in their saved positions and the new session slots in by creation date.

### Verification

- `just fmt-check` after every code change.
- `just clippy` after every code change.
- `just test` after every code change. New tests must fail before implementation, pass after.
- Manual smoke: `just dev-stub` (uses the stub harness so no real Nexus CLI is required); the stub harness needs a way to simulate an external session in the refresh payload — extend it with a `simulate_external_session` test hook if not present, otherwise a real `nexus` shell is needed.

## File Touch Points (summary)

Behavior:

- `src/extensions/harness/nexus/refresh/apply_session_refresh.rs` — additive insert, return inserted-id set.
- `src/extensions/harness/sessions/refresh/apply_session_refreshes.rs` — call order helper, persist preferences.
- `src/app/sessions/drain_initial_sessions_receiver.rs` — use order helper instead of `sort_sessions_by_creation_date` + `apply_session_id_order`.
- `src/ui/left_panel/order/apply_session_id_order.rs` — keep as a thin wrapper or delete (used at startup; the wrapper call in `apply_initial_sessions` will be replaced).
- New: `src/ui/left_panel/order/order_with_user_preferences.rs` (+ tests).
- `src/extensions/harness/sessions/creation/new_chat_insert_index.rs` — wrap the helper.

Deletion (legacy mode):

- `src/ui/left_panel/session/visible_rows.rs` — drop `visible_session_rows_with_folders` and helpers.
- `src/ui/left_panel/session/visible_rows_tests.rs`, `visible_rows_running_tests.rs` — drop legacy tests.
- `src/ui/left_panel/session/list_row.rs` — drop `FolderMore` variant.
- `src/ui/left_panel/render/session_row_line.rs` — drop `folder_more_row_line` and `more_line_style`.
- `src/ui/left_panel/render/session_lines.rs`, `rendered_rows.rs` — drop `FolderMore` arms.
- `src/ui/left_panel/folder/open_focused_project.rs`, `collapse_focused_project.rs` — drop `FolderMore` arms.
- `src/ui/left_panel/input/handle_session_drag_mouse.rs`, `focus_row.rs`, `activate_focused_row.rs` — drop `FolderMore` arms.
- `src/app/input/handle_app_mouse.rs`, `state/app_state_methods.rs` — drop `FolderMore` arms.
- `src/app/state/visible_rows_cache_methods.rs` — drop the `!workspace_view_enabled` branch.
- `src/ui/grid_layout/persistence/persisted_workspace_state.rs` — drop `workspace_view_enabled` and `default_workspace_view_enabled`.
- `src/ui/grid_layout/persistence/restore_workspace_state.rs`, `capture_workspace_state.rs` — drop the field copy.
- `src/ui/grid_layout/persistence/restore_multiplexer_state_into_app.rs`, `capture_multiplexer_state.rs` — drop the `workspace_view_enabled` references in the `workspace` block.
- `src/app/state/app_state.rs` — drop `pub workspace_view_enabled: bool`.
- `src/app/state/new_app_state.rs` — drop the field initialization and any branches.
- `src/app/test_support/app_fixture.rs` — drop the field initialization.
- `src/app/input/handle_keyboard_event.rs` — drop the `AppHotkey::ToggleWorkspaceView` arm and the two tests.
- `src/app/input/handle_app_mouse.rs` — drop the `workspace_view_enabled` branches.
- `src/ui/workspace_pane/toggle_workspace_view.rs` — delete the file and its `mod.rs` entry.
- `src/ui/workspace_pane/select_workspace_by_visible_index.rs` — drop the legacy branch and the test that sets the field false.
- `src/ui/left_panel/render/session_lines.rs:129` and `rendered_rows.rs:172` — drop the `workspace_view_enabled` branches.
- `src/ui/layout/resizable_grid/visible_layout_widget_state.rs`, `handle_mouse.rs` — drop the `workspace_view_enabled` parameter.
- `src/core/rendering/screen/render_app.rs` — drop the `workspace_view_enabled` branches.
- `src/core/rendering/resize/render_resizable_grid_overlay.rs` — drop the `workspace_view_enabled` argument.
- `src/extensions/command_bar/input/handle_keyboard.rs` — drop the `ToggleWorkspaceView` command test.
- Hotkey registry / command bar data: drop the `Toggle workspace view` command entry (search `ToggleWorkspaceView` and `Toggle workspace view`).
- Test fixtures and snapshot tests that assert `workspace_view_enabled`:
  - `src/ui/left_panel/tests/mouse_lag_regression_tests.rs`
  - `src/ui/left_panel/tests/scrollbar_render_tests.rs`
  - `src/ui/left_panel/scroll/scroll_regression_tests.rs`
  - `src/core/rendering/screen/render_app_snapshot_tests.rs`
- `src/extensions/harness/history_modal/actions/open_folder.rs` — delete the file.
- `src/extensions/harness/history_modal/actions/mod.rs` — drop `pub mod open_folder;`.
- `docs/ui/left-panel.md`, `docs/extensions/command-bar.md`, `docs/ui/workspace-pane.md`, `docs/app/state.md`, `AGENTS.md`, `docs/plan/e2e_tests.md` — update per section 6 above.

## Risks and Open Questions

- The stub harness (`src/extensions/harness/stub/adapter/stub_harness.rs`) is used by `just dev-stub` and by all the unit tests via `app_fixture`. The new code paths are exercised by tests against the real `NexusHarness::merge_session_refresh` and by the helper directly, so the stub harness should not need behavior changes for tests. If a smoke test needs to simulate an external session, the stub may grow a `set_refresh_payload` test hook.
- `is_new_nexus_chat_session` checks for `title == "New Nexus chat" && id.starts_with("new-")`. External sessions that happen to match that pattern (extremely unlikely but possible after data manipulation) would be treated as placeholders. Mitigation: the id-match in `apply_refreshed_session` runs first, so a real external id always wins.
- The order helper mutates `session_terminals`. Callers that hold indexes into the slice (active_index, focused_index) must re-resolve by id after the call. Today `apply_initial_sessions` already does this for `active_index`; the refresh path will get the same treatment.
- `VisibleSessionRowsSignature` is keyed on `(id, working_dir, is_running)`. Adding a session changes the id set, so the cache invalidates. We will not need to add `date` to the signature for correctness, but we may add it for a future perf pass.
- The `open_folder_history_modal` action had a "verify callers" risk in the previous draft. Resolved: read both call sites in `src/app/input/handle_app_mouse.rs:142` and `src/ui/left_panel/input/activate_focused_row.rs:17`. Both are `FolderMore` arms. The `Ctrl+H` picker (`open_history_modal`) is a separate function and is unaffected. The whole `open_folder.rs` file deletes with `FolderMore`.

## Definition of Done

- All new unit tests fail before the implementation and pass after.
- All legacy tests listed for deletion are removed.
- `just check` is green.
- `just dev-stub` launches without panics.
- E2E scenarios 1-4 are documented in `docs/plan/e2e_tests.md` and at least scenario 3 (workspace view is the only mode) is automated.
- Docs updates landed; AGENTS.md index still resolves.
- `git grep FolderMore` returns no matches.
- `git grep workspace_view_enabled` returns no matches.
- `git grep ToggleWorkspaceView` returns no matches.
- `git grep open_folder_history_modal` returns no matches.
- `git grep open_folder::` returns no matches (the `mod.rs` `pub mod open_folder;` entry is gone).
