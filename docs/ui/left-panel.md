# UI left panel

`src/ui/left_panel/` owns the navigation sidebar for folders, sessions, plans, and files selected from the top navbar.

## Responsibilities

- Build visible session and split-session group rows for the selected workspace.
- Host the navbar-selected `Sessions`, `Plans`, or `Files` content mode, and cycle those modes with Ctrl+backtick.
- Render selected-workspace session rows left-aligned with day-group separator lines, plus split-group hierarchy rows, scrollbars, hotkey footer, and active left-pane content.
- Style active session titles and running animations distinctly from static icons.
- Mark completed unseen sessions in teal until the user opens them.
- Use the shared folder blue for folder surfaces; workspace folders render as rounded boxes without folder icons.
- Track focused row and visible row cache.
- Show all sessions for the selected workspace without a `+ more` overflow row.
- In legacy grouped mode, show ten recent sessions per folder, keep focused/running sessions visible, and add `+ more` when older sessions remain.
- Handle keyboard and mouse input for navigation, Cmd+number current-workspace item selection, collapse, selection, opening, reordering, dragging, filtering, hover focus, and main-pane focus handoff.
- Persist session and folder ordering preferences.
- Maintain scroll position and keep focused rows visible.

## Child modules

- `focus/` moves and resolves focused rows.
- `folder/` renders and operates project folder rows.
- `input/` dispatches left-panel keyboard and mouse behavior.
- `mode/` defines left-pane modes and the selected-mode pane title.
- `order/` loads, saves, applies, and syncs ordering preferences.
- `render/` draws footer, scrollbar, and row content.
- `scroll/` manages left-panel scroll behavior.
- `session/` renders, sorts, and day-groups session rows, split-group child rows, selected-workspace rows, and legacy grouped folder rows.
- `tests/` contains left-panel regression tests.

## Storage

Left-panel ordering preferences are stored at `~/.config/ratsus/session-order.json`.

## Boundary

The left panel is a navigation surface. It does not load backend sessions, delete Nexus data directly, edit plan files, or manage PTY process internals.

## Key files

- `src/ui/left_panel/active_content.rs`
- `src/ui/left_panel/content.rs`
- `src/ui/left_panel/input/key_behavior.rs`
- `src/ui/left_panel/mode/left_pane_mode.rs`
- `src/ui/left_panel/input/dispatch_left_pane_keyboard.rs`
- `src/ui/left_panel/order/preferences_path.rs`
- `src/ui/left_panel/render/render_rows.rs`
- `src/ui/left_panel/scroll/`
- `src/ui/left_panel/session/visible_rows.rs`
