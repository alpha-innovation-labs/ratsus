# App Layer Code-Quality Plan

## Scope

A single coordinated cleanup pass over everything in `src/app/`. The deletion module
was already cleaned up in a prior round and is referenced here only for context — its
findings are not re-audited.

Five subdomains are *out of scope* for this plan: `src/app/diagnostics/`, `src/app/events/`,
`src/app/focus/`, `src/app/lifecycle/`, `src/app/expo/`. Those came back clean — no dead
code, no O(n²) patterns, no decorative `Result`s, no over-exposed `pub` worth touching.
One small `AppDiagnostics` field-visibility tightening and a `redraw_action` demotion
are the only items in those five, and they're documented under "Low-value remaining
items" at the end of this plan.

Everything else in `src/app/` is in scope:

- `src/app/input/` (including `input/hotkeys/`)
- `src/app/navigation/`
- `src/app/sessions/`
- `src/app/state/`
- `src/app/test_support/`

## Goals

- Delete the four confirmed-dead public functions/files: `cycle_chat_in_left_pane_order`,
  `adjacent_chat_index_in_left_pane_order`, `ordered_chat_indices_from_left_rows`,
  `restore_focus_after_delete`, `resize_active_terminal`. Remove the write-only
  `terminal_layout_widget_state` field on `AppState`.
- Fix the O(n²) iteration in `exited_session_pane_ids.rs` and the three sequential
  passes in `close_exited_sessions.rs` by consolidating into a single walk over
  `session_terminals`.
- Demote ~12 over-exposed `pub` items to `pub(crate)` (or `fn`) where the only callers
  are inside the same parent module.
- Drop the decorative `io::Result<PathBuf>` from `normal_terminal_working_dir` (only
  one of three branches is fallible) and the three `LayoutResult<CoordinatorAction>`
  returns that never produce `Err`.
- Consolidate the chat-vs-session trio (`cycle_*`, `adjacent_*_index`,
  `ordered_*_indices`) by deleting the unused chat half.
- Extract a single `try_drain` helper for the four near-duplicate `drain_*_receiver`
  functions spanning `sessions/`, `events/`, and `extensions/`.
- Tighten the four `AppDiagnostics` fields (write-mostly internal counters).
- Update `docs/app/state.md` and the relevant AGENTS.md index entries to match.

## Current state

| Subdomain       | Files | Notable findings                                                                                          |
| --------------- | ----- | --------------------------------------------------------------------------------------------------------- |
| `deletion/`     | 13    | Already cleaned in prior round. Excluded.                                                                  |
| `diagnostics/`  | 5     | Clean except: 4 over-exposed `AppDiagnostics` fields.                                                       |
| `events/`       | 5     | Clean except: `redraw_action` is module-internal but `pub`.                                                  |
| `focus/`        | 2     | Clean.                                                                                                     |
| `lifecycle/`    | 2     | Clean.                                                                                                     |
| `expo/`         | 4     | Clean.                                                                                                     |
| `input/`        | 15    | 3 over-exposed helpers; 3 decorative `LayoutResult<CoordinatorAction>` returns; 3 unused scope consts.      |
| `navigation/`   | 10    | Entire chat family (3 files) is dead. 1 O(n²) dedup loop. 3 over-exposed helpers.                            |
| `sessions/`     | 14    | 1 dead public function. 1 O(n²) pattern. 1 O(n·m) + 3-pass pattern in `close_exited_sessions`. 1 mostly-decorative `Result`. |
| `state/`        | 5     | 1 write-only field. 1 dead public method. 1 misleading file name. Massive drift between `new_with_harness` and `app_fixture`. |
| `test_support/` | 3     | Test-only fixtures reachable from release builds. Duplicates `dormant_session` pattern in `activate_session_layout_tests.rs`. |

Detailed file:line findings are in the three audit reports
(`plans/app-audit-input-navigation.md`, `plans/app-audit-sessions-state-testsupport.md`,
`plans/app-audit-diagnostics-events-focus-lifecycle-expo.md`). This plan distills the
actionable items.

## Design

### Phase A — Lock current behavior in tests

Per the project rule "start bug fixes by reproducing the bug in a test," pin current
behavior before any rewrite touches the public surface.

- `src/app/sessions/close_exited_sessions.rs` — add a unit test that exercises
  `exited_session_indices`, `exited_session_pane_ids`, and `remove_exited_sessions`
  together on a small fixture, asserting the exact indices and pane ids returned.
  This test pins the behavior the O(n²) → O(n) consolidation must preserve.
- `src/app/sessions/restore_focus_after_removals.rs` — add a unit test for
  `restore_focus_after_removals` and `restore_focus_after_bulk_delete` covering the
  "no sessions left" branch (focus clears) and the "next-session-after-removed"
  branch.
- `src/app/state/activate_session_layout_tests.rs` — add a test that asserts the
  `expo_card_width` value constructed by `new_with_harness` matches the one in
  `app_fixture` (current drift point). Locking this prevents the literal-drift fix
  in Phase G from accidentally changing the value.

### Phase B — Delete confirmed-dead code

Lowest-risk, highest-signal changes. Each is a verified-zero-external-caller removal.

1. Delete `src/app/sessions/restore_focus_after_delete.rs` and remove the
   `pub mod restore_focus_after_delete;` line from `src/app/sessions/mod.rs:13`.
   Grep confirms no callers; the bulk-delete path uses
   `restore_focus_after_bulk_delete` instead.
2. Delete the entire chat family in `src/app/navigation/`:
   - `src/app/navigation/cycle_chat_in_left_pane_order.rs`
   - `src/app/navigation/adjacent_chat_index_in_left_pane_order.rs`
   - `src/app/navigation/ordered_chat_indices_from_left_rows.rs`
   And remove their `pub mod` entries from `src/app/navigation/mod.rs:5,3,8`. The
   `cycle_chat_in_left_pane_order` function has zero external callers; its
   `adjacent_*` and `ordered_*` helpers exist solely to support it.
3. Delete `resize_active_terminal` from `src/app/state/app_state_methods.rs:205` and
   its `should_resize_active_terminal` helper (line 20, line 211) if the helper is
   also unused. Grep first to confirm.
4. Remove the `terminal_layout_widget_state: ResizableGridWidgetState` field from
   `AppState` (`src/app/state/app_state.rs:37`) and from its four initializers
   (`new_app_state.rs:58`, `app_fixture.rs:36`, and the three `*_tests.rs` files).
   Grep across `src/` to confirm zero reads before removal.
5. Inline the trivial private helper `preferred_index`
   (`src/app/sessions/restore_focus_after_removals.rs:79`) at its two call sites
   (lines 25 and 63) and delete the helper.

### Phase C — Fix the O(n²) and O(n·m) iteration patterns

1. `exited_session_pane_ids.rs:9-10` — Replace `Vec<String>::contains` inside the
   `iter().filter_map()` with a `BTreeSet<&String>` built once at the top of the
   function. Or fold the lookup into the consolidated walk below.
2. `close_exited_sessions.rs:13-28` — Three sequential passes over
   `session_terminals` (indices, pane-id lookup, removal) collapse to a single walk
   that yields `(index, working_dir, pane_id)` tuples. The `fallback_working_dir`
   closure inside `remove_exited_sessions` becomes a no-op (every entry has its
   `working_dir` pre-resolved).
3. Drop the `Vec::contains` dedup in
   `ordered_session_indices_from_left_rows.rs:11`. The only caller
   (`adjacent_session_index_in_left_pane_order.rs`) uses `position()` to find the
   first match, so duplicates have no observable effect. The dedup was also an
   inconsistency with the chat variant (which doesn't dedup) — both halves of the
   trio agree once the chat half is deleted in Phase B.

### Phase D — Tighten over-exposed `pub` visibility

Mechanical, no behavior change. Demote the following to `pub(crate)` (still
reachable across `src/`) or `fn` (private). Module-internal-only items become `fn`
where there are no external callers; sibling-only items become `pub(crate)`.

| Item                                          | File:line                                  | New visibility |
| --------------------------------------------- | ------------------------------------------ | -------------- |
| `redraw_action`                               | `events/redraw_action.rs:4`                  | `fn`             |
| `move_focused_left_row`                       | `state/app_state_methods.rs:49`              | `fn`             |
| `handle_left_keyboard`                        | `input/handle_left_keyboard.rs:12`           | `pub(crate)`     |
| `handle_terminal_keyboard`                    | `input/handle_terminal_keyboard.rs:13`       | `pub(crate)`     |
| `active_hotkey_scope`                         | `input/hotkeys/scopes.rs:18`                 | `pub(crate)`     |
| `TERMINAL_SCOPE` const                        | `input/hotkeys/scopes.rs:9`                  | `pub(super)`     |
| `LEFT_PANEL_SCOPE` const                      | `input/hotkeys/scopes.rs:11`                 | `const` (drop `pub`) |
| `PICKER_SCOPE` const                          | `input/hotkeys/scopes.rs:13`                 | `const` (drop `pub`) |
| `MODAL_SCOPE` const                           | `input/hotkeys/scopes.rs:15`                 | `const` (drop `pub`) |
| `adjacent_session_index_in_left_pane_order`   | `navigation/adjacent_session_index_in_left_pane_order.rs:6` | `pub(crate)` |
| `ordered_session_indices_from_left_rows`      | `navigation/ordered_session_indices_from_left_rows.rs:5`  | `pub(crate)` |
| `adjust_index_after_removals`                 | `sessions/adjust_index_after_removals.rs:2` | `pub(crate)`     |
| `clamp_session_index`                         | `sessions/clamp_session_index.rs:2`         | `pub(crate)`     |
| `exited_session_indices`                      | `sessions/exited_session_indices.rs:5`      | `pub(crate)`     |
| `exited_session_pane_ids`                     | `sessions/exited_session_pane_ids.rs:6`     | `pub(crate)`     |
| `normal_terminal_insert_index`                | `sessions/normal_terminal_insert_index.rs:2` | `pub(crate)`    |
| `normal_terminal_working_dir`                 | `sessions/normal_terminal_working_dir.rs:7` | `pub(crate)`     |
| `remove_exited_sessions`                      | `sessions/remove_exited_sessions.rs:14`     | `pub(crate)`     |
| `RemovedExitedSessions` struct + 3 fields     | `sessions/remove_exited_sessions.rs:7`      | `pub(crate)` (struct and fields) |
| `AppDiagnostics` fields (`fps`, `redraws`, `frames_since_fps_update`, `last_fps_update`) | `diagnostics/app_diagnostics.rs:6,8,10,12` | private; add a `#[cfg(test)] pub(crate)` accessor for the `record_app_redraw.rs:42` backdate |

### Phase E — Drop decorative `Result<()>` returns

1. `normal_terminal_working_dir` (`sessions/normal_terminal_working_dir.rs:7`)
   currently returns `io::Result<PathBuf>` but only the last branch
   (`std::env::current_dir()`) can fail. Change to return `PathBuf` and have the
   one caller (`start_new_normal_terminal.rs:16`) handle the failure explicitly
   with `unwrap_or_else(|_| PathBuf::from("."))` (matching the pattern already
   used at `remove_exited_sessions.rs:51`).
2. `handle_keyboard_event` (`input/handle_keyboard_event.rs:43`),
   `handle_left_keyboard` (`input/handle_left_keyboard.rs:15`), and
   `handle_terminal_keyboard` (`input/handle_terminal_keyboard.rs:16`) all return
   `LayoutResult<CoordinatorAction>` (a `Result` type) but every match arm
   returns `Ok(...)`. Change all three to `-> CoordinatorAction` and update the
   one caller in `events/handle_app_event.rs:15` to wrap with `Ok(...)`. This
   removes `.expect("...")` ceremony from 10+ test call sites.

### Phase F — Consolidate near-duplicates

These are larger refactors; each is a single coherent change.

1. **Extract a `try_drain` helper** in a new
   `src/app/events/try_drain_receiver.rs` (or co-locate with
   `drain_delete_session_receiver.rs`):

   ```rust
   pub(crate) fn try_drain<T>(
       slot: &mut Option<Receiver<T>>,
       on_value: impl FnOnce(T) -> bool,
   ) -> bool { … }
   ```

   The four call sites become 3-5 line shims:
   - `src/app/sessions/drain_initial_sessions_receiver.rs:19`
   - `src/app/events/drain_delete_session_receiver.rs:10`
   - `src/extensions/harness/sessions/refresh/drain_session_refresh_receiver.rs:7`
   - `src/extensions/expo/observations/drain_cache_receiver.rs:6`

2. **Extract a `prepare_post_removal` helper** that does
   `sync_folder_order + prune_terminal_pane_session_bundles` and returns whether
   the session list is empty. Add to a new
   `src/app/sessions/focus_recovery.rs` (private). The three
   `restore_focus_after_*` files (`restore_focus_after_removals.rs:19`,
   `restore_focus_after_bulk_delete.rs:12-17`, and the deleted
   `restore_focus_after_delete.rs`) each use this preamble. After Phase B
   removes the dead third, only two files need the helper.

3. **Tighten the `resolve_app_hotkey` `let _ = registry.lookup(...)?`** at
   `input/hotkeys/resolve_app_hotkey.rs:25` to an `if registry.lookup(...).is_none() { return None; }`
   guard. The intent ("the key must be registered in the Ratkit registry before
   we even try to match it") becomes explicit.

### Phase G — Consolidate the `AppState` literal

`new_app_state.rs:54-120` and `app_fixture.rs:32-99` both write 60+ fields with
mostly the same defaults. Drift is already visible (e.g. `expo_card_width` uses
`default_expo_card_width()` in the test fixture but
`clamp_expo_card_width(preferences.expo_card_width)` in production; `focused_row`
is `1` in the test fixture but `0` in production). This is a real "works in
tests, breaks in prod" risk.

**Fix:** extract a private
`fn default_app_state_for_harness(chat_harness: Arc<dyn ChatHarness>) -> anyhow::Result<AppState>`
that builds the full struct, and have both `new_with_harness` and `app_fixture`
call it then patch only the harness-specific differences. The fixture-only
fields (e.g. `initial_sessions_receiver: None` vs a real one in production)
get overridden in the production path; test-only fields get overridden in the
fixture.

The `#[cfg(test)]` gating question for `src/app/test_support/mod.rs:3-4` is
addressed in the same change: add `#[cfg(test)]` to the `pub mod app_fixture;`
and `pub mod dormant_session;` lines so the test helpers are not linked into
release builds.

### Phase H — Update docs

- `docs/app/state.md` — drop the `terminal_layout_widget_state` field from the
  listed fields; document the new `default_app_state_for_harness` helper if it
  becomes `pub(crate)`.
- `AGENTS.md` — the `docs/app/state.md` link stays valid; no index changes
  expected.
- `src/app/state/visible_rows_cache_methods.rs` — the filename is misleading
  (no cache exists). Rename to `visible_rows_methods.rs` and the helpers stay
  the same. The filename is referenced from tests; update the `mod.rs:5` entry
  and any `#[path]` directives.

## Test Strategy

### Unit tests added (Phase A)

- `src/app/sessions/close_exited_sessions.rs` — one new test exercising
  indices/pane-ids/removal together on a 4-session fixture.
- `src/app/sessions/restore_focus_after_removals.rs` — one new test for the
  "no sessions left" branch and one for the "next-session-after-removed"
  branch.
- `src/app/state/activate_session_layout_tests.rs` — one new test asserting
  the `expo_card_width` value from `new_with_harness` matches the one in
  `app_fixture` (current drift point).

### Unit tests kept (existing)

All currently-passing tests stay. The existing 7 `tests/session_deletion/`
snapshot tests and the 5 `tests/session_refresh/` tests must continue to pass
unchanged.

### Unit tests deleted (Phase B)

None — the deleted public functions had no unit tests of their own.

### E2E tests

No new E2E scenarios. The existing snapshot suites in
`tests/session_deletion/`, `tests/session_refresh/`, `tests/app_orchestration/`,
`tests/history_modal/`, `tests/mouse/`, and `tests/ui_keyboard/` must
pass unchanged.

### Verification

After every phase:

- `just fmt-check` (workspace-wide)
- `just clippy`
- `just test`

After Phase H:

- `git grep restore_focus_after_delete` returns no matches in `src/`.
- `git grep cycle_chat_in_left_pane_order` returns no matches in `src/`.
- `git grep adjacent_chat_index_in_left_pane_order` returns no matches.
- `git grep ordered_chat_indices_from_left_rows` returns no matches.
- `git grep terminal_layout_widget_state` returns no matches in `src/`.
- `git grep resize_active_terminal` returns no matches in `src/`.
- `git grep preferred_index` (in `src/app/sessions/`) returns no matches.
- `git grep "io::Result<PathBuf>" src/app/sessions/normal_terminal_working_dir.rs` returns no matches.
- `git grep "LayoutResult<CoordinatorAction>" src/app/input/` returns no matches.
- `git grep "pub struct RemovedExitedSessions"` returns no matches in `src/` (the struct is still there but with `pub(crate)`).

## File Touch Points

### Add

- `src/app/events/try_drain_receiver.rs` + entry in `src/app/events/mod.rs` (Phase F.1)
- `src/app/sessions/focus_recovery.rs` (private module) + entry in `src/app/sessions/mod.rs` (Phase F.2)
- `src/app/test_support/keyboard_event.rs` (optional — see "Open question" below)
- `plans/app-cleanup.md` (this file)

### Modify

- `src/app/sessions/mod.rs` — drop `restore_focus_after_delete`, add `focus_recovery`.
- `src/app/sessions/close_exited_sessions.rs` — consolidate the 3 passes (Phase C.2); add a unit test (Phase A).
- `src/app/sessions/exited_session_pane_ids.rs` — fix the O(n²) `Vec::contains` (Phase C.1) or fold into the consolidated walk.
- `src/app/sessions/restore_focus_after_bulk_delete.rs` — route preamble through `focus_recovery::prepare_post_removal` (Phase F.2); add a unit test.
- `src/app/sessions/restore_focus_after_removals.rs` — same; inline `preferred_index`; add a unit test.
- `src/app/sessions/normal_terminal_working_dir.rs` — drop the decorative `Result` (Phase E.1).
- `src/app/sessions/remove_exited_sessions.rs` — `pub(crate)` visibility on the struct and fields; `pub(crate)` on the function.
- `src/app/sessions/start_new_normal_terminal.rs` — update the `normal_terminal_working_dir(...)` call site to handle the now-`PathBuf` return.
- `src/app/navigation/mod.rs` — drop the three chat-family `pub mod` entries.
- `src/app/navigation/ordered_session_indices_from_left_rows.rs` — drop the dedup O(n²) loop (Phase C.3); `pub(crate)`.
- `src/app/navigation/adjacent_session_index_in_left_pane_order.rs` — `pub(crate)`.
- `src/app/state/app_state.rs` — drop the `terminal_layout_widget_state` field.
- `src/app/state/app_state_methods.rs` — drop `resize_active_terminal`; demote `move_focused_left_row` to `fn`.
- `src/app/state/new_app_state.rs` — drop the field init; route the struct literal through `default_app_state_for_harness` (Phase G).
- `src/app/state/visible_rows_cache_methods.rs` → rename to `src/app/state/visible_rows_methods.rs`; update `mod.rs`.
- `src/app/test_support/app_fixture.rs` — route the struct literal through `default_app_state_for_harness`; drop the field init.
- `src/app/test_support/mod.rs` — add `#[cfg(test)]` to the two child modules (Phase G).
- `src/app/state/activate_session_layout_tests.rs` — drop the local `session_entry` in favor of `dormant_session` (now takes `impl AsRef<Path>`); add the `expo_card_width` drift test (Phase A).
- `src/app/diagnostics/app_diagnostics.rs` — demote the four `pub` fields to private; add a `#[cfg(test)]` accessor for the `record_app_redraw.rs:42` backdate.
- `src/app/events/redraw_action.rs` — demote to `fn`.
- `src/app/events/drain_delete_session_receiver.rs` — adopt the shared `try_drain` helper (Phase F.1).
- `src/app/input/handle_keyboard_event.rs` — demote the return type to `CoordinatorAction`; drop the `.expect("...")` calls in the inline tests.
- `src/app/input/handle_left_keyboard.rs` — demote the return type; `pub(crate)` on the function.
- `src/app/input/handle_terminal_keyboard.rs` — demote the return type; `pub(crate)` on the function.
- `src/app/input/hotkeys/resolve_app_hotkey.rs` — rewrite the `let _ = registry.lookup(...)?` (Phase F.3).
- `src/app/input/hotkeys/scopes.rs` — drop `pub` on the three unused scope consts; demote `TERMINAL_SCOPE` to `pub(super)`; demote `active_hotkey_scope` to `pub(crate)`.
- `src/app/events/handle_app_event.rs` — wrap the new non-`Result` return from `handle_keyboard_event` with `Ok(...)`.
- `src/extensions/harness/sessions/refresh/drain_session_refresh_receiver.rs` — adopt the shared `try_drain` helper.
- `src/extensions/expo/observations/drain_cache_receiver.rs` — adopt the shared `try_drain` helper.
- `src/app/sessions/drain_initial_sessions_receiver.rs` — adopt the shared `try_drain` helper.
- `docs/app/state.md` — drop the `terminal_layout_widget_state` field; document the consolidated literal helper.
- `AGENTS.md` — verify the `docs/app/state.md` link is still valid (no change expected).

### Delete

- `src/app/sessions/restore_focus_after_delete.rs`
- `src/app/navigation/cycle_chat_in_left_pane_order.rs`
- `src/app/navigation/adjacent_chat_index_in_left_pane_order.rs`
- `src/app/navigation/ordered_chat_indices_from_left_rows.rs`

## Risks and Open Questions

- **The pre-existing uncommitted round in `src/extensions/history_modal/`,
  `src/shared/svg/`, and `tests/history_modal/` currently breaks
  `cargo build --workspace --all-targets`** with three errors unrelated to this
  plan. The `just check` and `just test` gates cannot be exercised end-to-end
  until those are resolved (committed, stashed, or fixed). Recommend committing
  or stashing the history-modal work separately before running the test suite.
  This plan does not touch any of those files.
- **Generalizing the chat/session trio** — the trio was identified as a
  near-duplicate pair, but rather than consolidate via a `SessionKind`
  predicate, this plan simply deletes the unused chat half. If a future
  feature needs the chat-filtered variant, it should reintroduce a
  `cycle_chat_in_left_pane_order(include: impl Fn(&ChatSession) -> bool)` and
  share the implementation. **Not a deferred decision; the deletion is final
  unless a future need reverses it.**
- **`handle_keyboard_event` → `CoordinatorAction` signature change** affects
  every test that calls `handle_keyboard_event(...).expect("keyboard event")`.
  All 15+ call sites in `src/app/input/handle_keyboard_event.rs` itself plus
  any external callers must be updated. Mechanical, but the diff is large.
- **`AppState` literal consolidation in Phase G** is a real refactor that
  could expose latent assumptions. The drift-locking test in Phase A is
  essential — without it, the consolidation is unverified.
- **`try_drain` helper placement** — `src/app/events/try_drain_receiver.rs` is
  the natural home (alongside `drain_delete_session_receiver.rs`), but two of
  the four callers live in `extensions/`. The dependency direction
  `extensions → app` is allowed by the project's domain boundaries. The
  helper takes `impl FnOnce(T) -> bool` so each caller's per-type logic
  stays local.
- **Test fixture consolidation** (`key`, `control_key`, `super_key` helpers
  duplicated across 6+ test files) is *not* in this plan as a Phase — it
  would be its own follow-up plan because it touches test-only code that
  has grown organically. Flagged here for the next pass.

## Low-value remaining items (not in this plan)

These came back from the audit but are either too small to be worth a phase or
are judgment calls.

- `src/app/diagnostics/app_diagnostics_status_line.rs:1-6` — could be inlined
  into `render_app.rs:48`, but the dedicated function exists so the format
  string can be unit-tested. Keep.
- `src/app/diagnostics/app_diagnostics_status_line.rs:9-14` and
  `record_app_redraw.rs:24-47` use real assertions, not placeholders. Keep.
- `src/app/events/handle_app_event.rs:9-19` — `LayoutResult<CoordinatorAction>`
  is *not* decorative: the trait signature requires it. The `?` in
  `handle_keyboard_event(...)` (line 15) propagates. Keep.
- `src/app/expo/hide_expo.rs` does not clear `selected_expo_folder` or call
  `persist_multiplexer_state(app)`, unlike `activate_expo_folder`. This is
  an asymmetry that may or may not be intentional; out of scope.
- `src/app/input/hotkeys/normal_terminal_passthrough_hotkey.rs:9-10` hard-codes
  Ctrl+E despite the doc comment saying "a key." Rename to
  `is_normal_terminal_control_e_passthrough` if the function gets revisited.
- `src/app/input/hotkeys/app_hotkey_registry.rs:30` display string vs
  `resolve_app_hotkey.rs:19` binding. The registry is for help metadata, not
  resolution. Add a doc comment to that effect.
- `src/app/navigation/next_chat_index.rs:5` name is slightly misleading
  (returns the first chat at-or-after with single wrap). Rename to
  `next_or_wrapped_chat_index` for clarity. Cosmetic.
- `src/app/state/visible_rows_cache_methods.rs` filename misleads about a
  cache that does not exist. Renaming to `visible_rows_methods.rs` is in
  Phase H but the file's *contents* are not changed.

## Definition of Done

- Phase A tests fail on baseline (i.e. they assert behavior the current
  code does not provide) and pass after their corresponding phase lands.
  Specifically: the `exited_session_indices`+`pane_ids`+`remove` equivalence
  test passes after Phase C.2; the `restore_focus_after_*` tests pass after
  Phase F.2; the `expo_card_width` drift test passes both before and after
  Phase G (the value must not change).
- All four confirmed-dead functions/files are gone.
- `terminal_layout_widget_state` is gone from `AppState` and all four
  initializers.
- The O(n²) `Vec::contains` in `exited_session_pane_ids.rs` is gone, and
  `close_exited_sessions` does at most one pass over `session_terminals`.
- `~12` `pub` items are demoted to `pub(crate)` or `fn`; the `git grep` checks
  in "Verification" all return zero matches.
- `normal_terminal_working_dir` returns `PathBuf`; the call site in
  `start_new_normal_terminal.rs` handles the `current_dir` failure.
- `handle_keyboard_event`, `handle_left_keyboard`, `handle_terminal_keyboard`
  return `CoordinatorAction`; the test call sites drop `.expect("...")`.
- The four `drain_*_receiver` files share a single `try_drain` helper.
- `new_with_harness` and `app_fixture` route through one
  `default_app_state_for_harness`; the `expo_card_width` value is identical
  in both.
- `visible_rows_cache_methods.rs` is renamed to `visible_rows_methods.rs`.
- `just check` is green (or, if the pre-existing history-modal build errors
  are unfixed, every change in this plan is verified by `cargo check`
  filtered to the touched files plus their direct dependents).
- All existing snapshot tests are unchanged.
- `git grep` checks in "Verification" return zero matches.
