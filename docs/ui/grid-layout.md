# UI grid layout

`src/ui/grid_layout/` owns visible pane organization inside the main area. It is currently used for terminal-backed chat sessions, but any extension may place content in the grid.

## Responsibilities

- Track which content or session is assigned to each pane.
- Place existing sessions or extension content into active panes.
- Bundle multiple sessions per pane when needed.
- Split the active pane horizontally or vertically.
- Maintain stable split-pane session group identities for left-panel hierarchy.
- Resize pane-owned terminal views when pane sizes change.
- Activate panes and update active pane session state.
- Persist focused session, folder, plan, file, pane, tab, and workspace state for restart restore.
- Hit-test pane close buttons.
- Render assigned pane content; current code renders chat sessions inside pane areas.

## Child modules

- `bundle/` manages pane session bundles.
- `group/` tracks split-pane session groups by stable group id and pane ids.
- `pane/` maps panes, close buttons, and session ids.
- `render/` renders chat sessions.
- `persistence/` captures and restores split-pane layout plus restart focus state.
- `split/` manages split direction, active pane assignment, pane closing, and resizing.

## Boundary

Grid layout owns pane organization, splitting, resizing, active pane state, and hit-testing. It does not own terminal process internals or the feature rendered in a pane. PTY spawning, input encoding, and process state belong in `src/extensions/terminal/`; feature-specific content such as Expo, file preview, or git diff behavior belongs in its extension.

## Key files

- `src/ui/grid_layout/bundle/bundle_new_session.rs`
- `src/ui/grid_layout/bundle/place_existing_session.rs`
- `src/ui/grid_layout/pane/pane_id_for_session.rs`
- `src/ui/grid_layout/persistence/capture_multiplexer_state.rs`
- `src/ui/grid_layout/persistence/restore_multiplexer_state_into_app.rs`
- `src/ui/grid_layout/render/render_chat_sessions.rs`
- `src/ui/grid_layout/split/split_active_pane.rs`
- `src/ui/grid_layout/split/resize_session.rs`
- `src/ui/grid_layout/split/close_terminal_pane.rs`
