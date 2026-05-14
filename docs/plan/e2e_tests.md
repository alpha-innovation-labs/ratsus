# Ratsus E2E Tests and Desired Outcomes

## App orchestration domain

1. `app_startup_real_catalog` — Startup creates a usable app with real Nexus sessions and renders the initial catalog.
2. `app_startup_nexus_read_only` — Startup loads real Nexus sessions in read-only mode without mutating Nexus data.
3. `app_keyboard_global_shortcuts` — Global shortcuts route to the correct UI workflow before focused-pane handling.
4. `app_tick_refresh_workflow` — Tick events refresh sessions and previews without changing unrelated visible state.
5. `app_resize_workflow` — Resize events redraw the app with stable pane boundaries and no broken borders.
6. `app_quit_shortcut` — The quit shortcut returns a user-visible quit outcome without corrupting state.

## Core rendering domain

1. `render_full_screen_baseline` — The full shell renders menu bar, left pane, main pane, borders, and footer consistently.
2. `render_delete_confirmation_dialog` — Delete confirmation appears centered with the expected titles and actions.
3. `render_conversation_picker_dialog` — Picker modal overlays the app without damaging the underlying screen.
4. `render_resize_overlay` — Active resizing displays the resize overlay and returns to normal rendering after release.
5. `render_small_terminal_layout` — Small terminal sizes clip content safely without panics or malformed borders.
6. `render_snapshot_regression_baseline` — Stable snapshots catch visible regressions in layout, text, and symbols.

## UI keyboard domain

1. `keyboard_list_contract_chat` — Chat lists honor `j/k`, arrows, `gg/G`, Enter, `/`, `d`, space, and `q`.
2. `keyboard_list_contract_files` — File tree lists honor the same shared list keys where the actions apply.
3. `keyboard_list_contract_picker` — Picker navigation, filtering, activation, and close behavior follow the shared list contract.
4. `keyboard_filter_text_entry` — Filter mode treats typed characters as query text instead of navigation keys.
5. `keyboard_pending_g_reset` — A pending `g` prefix resets correctly after handled and unhandled keys.

## UI layout domain

1. `layout_left_pane_visibility_toggle` — Toggling the left pane expands and restores the main pane.
2. `layout_focus_toggle_borders` — Focus changes update visible pane focus styling and footer context.
3. `layout_mouse_resize_shell` — Mouse resizing changes pane dimensions predictably and preserves content.
4. `layout_resize_persistence_during_workflow` — Layout remains stable after resize followed by picker, delete modal, and tab changes.

## UI menu bar domain

1. `menu_switch_to_chat` — Menu navigation activates Chat and renders chat sessions in the main pane.
2. `menu_switch_to_files` — Menu navigation activates Files and renders file tree plus preview.
3. `menu_switch_to_expo` — Menu navigation activates Expo and renders folder cards.
4. `menu_inactive_tabs_render_stably` — Inactive or empty tabs render stable placeholders without panics.

## UI left panel domain

1. `left_panel_startup_catalog` — The left panel shows grouped sessions with deterministic folder order and active markers.
2. `left_panel_long_folder_scroll` — Large folders scroll predictably with stable focus and row boundaries.
3. `left_panel_folder_collapse_expand` — Collapse and expand actions update visible rows without losing focus.
4. `left_panel_multi_select` — Space toggles selection markers for bulk workflows.
5. `left_panel_delete_shortcut` — Delete starts the confirmation flow for the focused or selected rows.
6. `left_panel_mouse_select_row` — Clicking a row focuses and activates the intended session.
7. `left_panel_footer_contract` — Footer keys and status match the active left-pane content.

## UI grid layout domain

1. `grid_split_right` — The right split shortcut creates a stable side-by-side terminal layout.
2. `grid_split_bottom` — The bottom split shortcut creates a stable stacked terminal layout.
3. `grid_place_session_in_active_split` — The placement picker assigns the selected session to the active pane.
4. `grid_bundle_sessions` — Bundled sessions render branch markers and preserve active bundle selection.
5. `grid_cycle_bundle_session` — Cycling within a bundle updates the pane and left-panel marker.
6. `grid_close_pane` — Closing a pane restores focus and leaves remaining sessions visible.
7. `grid_no_unassigned_placeholders` — Split workflows never leave visible `No session assigned` placeholders after valid placement.

## UI notifications domain

1. `notifications_spawn_failure_toast` — A failed user action displays a clear toast instead of failing silently.
2. `notifications_delete_result_toast` — Delete completion or failure displays the correct user-facing notification.
3. `notifications_toast_expiry` — Expired toasts disappear on redraw without affecting layout.

## Harness core domain

1. `harness_real_load_sessions` — The real Nexus harness returns the session catalog used by E2E snapshots.
2. `harness_real_refresh_stability` — Real Nexus refresh preserves row order and running status for loaded sessions.
3. `harness_real_spawn_new_chat` — Real Nexus new-chat creation adds a visible session and terminal body when enabled.
4. `harness_real_spawn_normal_terminal` — Real normal-terminal creation adds a visible terminal through a PTY-backed shell.
5. `harness_real_delete_session` — Real Nexus deletion flow targets only explicitly selected sessions.
6. `harness_nexus_load_read_only` — Nexus loading reads real sessions without writes or deletes.
7. `harness_nexus_owned_mutation_only` — Nexus mutations apply only to manifest-owned sessions.

## Conversation picker domain

1. `picker_catalog_grouping` — The picker lists grouped conversations in deterministic order.
2. `picker_filter_results` — Typing a query narrows visible picker results predictably.
3. `picker_activate_session` — Enter opens the selected session and closes the picker.
4. `picker_place_in_split` — Split-placement mode places the selected session in the active pane.
5. `picker_cancel_preserves_state` — Cancelling the picker returns to the previous app state.

## Session deletion domain

1. `delete_single_confirmation` — Single-session delete opens confirmation with the correct title.
2. `delete_bulk_confirmation` — Bulk delete opens confirmation with exactly the selected titles.
3. `delete_cancel_preserves_catalog` — Cancelling deletion leaves all sessions present.
4. `delete_confirm_removes_sessions` — Confirming deletion removes selected sessions and clears stale selection markers.
5. `delete_focus_restoration` — Focus moves to a sensible remaining row after deletion.
6. `delete_guard_blocks_foreign_nexus_id` — Nexus cleanup rejects IDs not recorded in the test manifest.
7. `delete_owned_nexus_single` — Nexus single delete removes only the manifest-owned session.
8. `delete_owned_nexus_bulk` — Nexus bulk delete removes only manifest-owned selected sessions.

## Session refresh domain

1. `refresh_real_stays_stable` — Real Nexus refresh does not reorder sessions or corrupt running state.
2. `refresh_nexus_adopts_owned_id` — Nexus refresh maps an owned placeholder to its real session ID and title.
3. `refresh_nexus_running_status` — Nexus refresh updates running status for owned sessions without changing foreign rows.
4. `refresh_nexus_registry_watcher` — Nexus session refreshes are triggered by the registry file watcher instead of a polling loop.
5. `refresh_preserves_active_terminal` — Refresh keeps the active terminal pane attached to the correct session.
6. `refresh_preserves_left_panel_focus` — Refresh keeps visible focus stable when the focused session still exists.

## Terminal domain

1. `terminal_open_existing_real_session` — Opening a dormant real Nexus session renders its terminal pane.
2. `terminal_spawn_new_real_chat` — New real Nexus chat renders terminal output and selected catalog row when enabled.
3. `terminal_spawn_real_normal_terminal` — Real normal terminal renders local terminal text through a PTY-backed shell.
4. `terminal_input_feedback` — Typed input reaches the active real terminal and remains observable in the terminal pane.
5. `terminal_scrollback_keys` — Scrollback commands keep terminal rendering valid.
6. `terminal_close_exited_session` — Exited sessions close cleanly and restore pane focus.
7. `terminal_resume_owned_nexus_session` — Resuming a manifest-owned Nexus session opens only that owned terminal.
8. `terminal_persistence_isolated` — Normal terminal persistence writes only inside test-owned Ratsus paths.

## Expo domain

1. `expo_open_folder_cards` — Opening Expo for a folder renders cards for that folder.
2. `expo_filter_results` — Expo filtering changes visible cards and result counts predictably.
3. `expo_card_navigation` — Keyboard navigation keeps the focused card visible.
4. `expo_card_width_resize` — Card width clamps correctly across terminal sizes.
5. `expo_masonry_layout` — Cards flow into stable masonry columns without overlap.
6. `expo_observation_preview_real` — Real observation previews render Nexus preview text when available.
7. `expo_observation_preview_owned_nexus` — Nexus observation previews render only for manifest-owned sessions in mutating tests.
8. `expo_mouse_select_card` — Clicking a card focuses or opens the intended conversation.

## File viewer domain

1. `files_open_tab_tree_and_preview` — Files tab renders the tree in the left pane and preview in the main pane.
2. `files_navigate_tree` — Tree navigation moves focus through files and folders predictably.
3. `files_expand_collapse_folder` — Folder expand/collapse updates visible tree rows and preview state.
4. `files_open_file_preview` — Opening a file renders the expected code or markdown preview.
5. `files_filter_paths` — Filtering narrows visible paths and keeps a valid selected path.
6. `files_quit_filter` — Quit closes filter mode without leaving Files.
7. `files_footer_status` — Footer status displays the selected path and shared hotkeys.
8. `files_missing_or_binary_preview` — Unsupported files render a safe user-visible fallback.

## Git diff domain

1. `diff_tab_empty_state` — The Diff tab renders a stable empty state when no diff feature data is available.
2. `diff_open_changed_file` — When diff data exists, selecting a changed file renders its diff preview.
3. `diff_navigation` — Diff navigation moves between hunks or files without breaking layout.
4. `diff_large_file_rendering` — Large diffs scroll predictably and preserve visible hunk boundaries.

## Observation domain

1. `observations_real_preview_load` — Real observation loading produces Nexus preview lines when available.
2. `observations_nexus_owned_preview_load` — Nexus observation loading reads preview state for owned sessions only.
3. `observations_watcher_refresh` — Watcher or tick refresh updates visible previews after observation changes.
4. `observations_missing_state_fallback` — Missing observation files render a safe empty preview state.

## Mouse domain

1. `mouse_left_panel_select` — Clicking a left-panel row selects the intended row.
2. `mouse_left_panel_scroll` — Wheel scrolling moves the left-panel viewport predictably.
3. `mouse_grid_resize` — Dragging the shell divider resizes panes safely.
4. `mouse_terminal_pane_activate` — Clicking a terminal pane makes it active.
5. `mouse_expo_card_select` — Clicking an expo card focuses the intended card.
6. `mouse_file_tree_select` — Clicking a file-tree row selects and previews the intended path.
7. `mouse_modal_buttons` — Clicking modal buttons confirms or cancels the visible modal action.

## Nexus safety domain

1. `nexus_read_only_catalog` — Read-only Nexus tests inspect existing sessions without mutation.
2. `nexus_owned_session_manifest_create` — Mutating tests record owned session IDs before cleanup is allowed.
3. `nexus_delete_guard_blocks_foreign` — Foreign or missing manifest IDs cannot be deleted.
4. `nexus_owned_single_delete` — A single owned session can be deleted safely.
5. `nexus_owned_bulk_delete` — Multiple owned sessions can be deleted safely.
6. `nexus_cleanup_idempotent` — Cleanup can run more than once without deleting foreign sessions.
7. `nexus_redacted_snapshots` — Nexus snapshots redact IDs, paths, and volatile status text.

## Shared utilities and isolation domain

1. `isolation_temp_workspace_for_real` — Real E2E tests use test-owned workspaces and do not mutate operator config or data files.
2. `isolation_temp_workdir_for_owned_nexus` — Nexus mutating tests create sessions only from test-owned working directories.
3. `snapshot_redaction_paths` — Snapshot output redacts temp paths consistently.
4. `snapshot_redaction_nexus_ids` — Snapshot output redacts Nexus IDs consistently.
5. `cleanup_removes_test_artifacts` — Test-owned temp directories, manifests, and files are removed after each test.
