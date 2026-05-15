# App state

`src/app/state/` owns `AppState` and construction of the cross-domain state graph used by Ratkit.

## Responsibilities

- Store shell layout, terminal pane maps, menu bar, toasts, and dialogs.
- Store session terminals, active/focused indexes, selected ids, and closed ids.
- Store left-panel ordering, scrolling, focus, drag, collapse state, and per-workspace remembered focused sessions.
- Store active main-pane tab and extension state for Expo and file viewer.
- Store observation preview cache state and watcher handles.
- Store async receivers for session refresh and deletion work.
- Store app diagnostics counters for FPS and redraws.
- Build initial state with an injected `ChatHarness`.

## Startup construction

`AppState::new_with_harness` loads sessions from the harness, optionally appends persisted normal terminals, applies saved left-panel preferences, initializes the file-system tree view, starts observation preview loading, starts observation watching, and starts periodic session refresh.

## Key files

- `src/app/diagnostics/`
- `src/app/state/app_state.rs`
- `src/app/state/new_app_state.rs`
- `src/app/state/app_state_methods.rs`
- `src/app/state/visible_rows_cache_methods.rs`
