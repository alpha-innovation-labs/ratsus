# UI left panel

`src/ui/left_panel/` owns the navigation sidebar for folders and sessions.

## Responsibilities

- Build visible folder and session rows.
- Render folder rows, session rows, scrollbars, hotkey footer, and active left-pane content.
- Track focused row and visible row cache.
- Handle keyboard and mouse input for navigation, collapse, selection, opening, reordering, and dragging.
- Persist session and folder ordering preferences.
- Maintain scroll position and keep focused rows visible.

## Child modules

- `focus/` moves and resolves focused rows.
- `folder/` renders and operates project folder rows.
- `input/` dispatches left-panel keyboard and mouse behavior.
- `order/` loads, saves, applies, and syncs ordering preferences.
- `render/` draws footer, scrollbar, and row content.
- `scroll/` manages left-panel scroll behavior.
- `session/` renders and sorts session rows and visible row data.
- `tests/` contains left-panel regression tests.

## Storage

Left-panel ordering preferences are stored at `~/.config/ratsus/session-order.json`.

## Boundary

The left panel is a navigation surface. It does not load backend sessions, delete Nexus data directly, or manage PTY process internals.

## Key files

- `src/ui/left_panel/active_content.rs`
- `src/ui/left_panel/content.rs`
- `src/ui/left_panel/input/key_behavior.rs`
- `src/ui/left_panel/input/dispatch_left_pane_keyboard.rs`
- `src/ui/left_panel/order/preferences_path.rs`
- `src/ui/left_panel/render/render_rows.rs`
- `src/ui/left_panel/scroll/`
- `src/ui/left_panel/session/visible_rows.rs`
