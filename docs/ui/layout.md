# UI layout

`src/ui/layout/` owns reusable shell layout behavior that is not specific to one extension.

## Responsibilities

- Track the focused shell pane.
- Toggle left-pane visibility.
- Handle resizable-grid mouse operations.
- Track whether the shell layout is currently resizing.
- Resolve pane areas and pane ids for rendering and input routing.
- Decide whether active terminal panes should resize.

## Child modules

- `focus/` owns focused-pane state and left-pane visibility transitions.
- `resizable_grid/` owns resize mouse handling, pane area lookup, pane ids, and resize predicates.

## Boundary

Extension-specific layout, such as Expo masonry columns or file tree rows, belongs in the owning extension. Terminal pane placement belongs in `src/ui/grid_layout/`.

## Key files

- `src/ui/layout/focus/focused_pane.rs`
- `src/ui/layout/focus/toggle_left_pane_visibility.rs`
- `src/ui/layout/resizable_grid/handle_mouse.rs`
- `src/ui/layout/resizable_grid/pane_area_by_id.rs`
- `src/ui/layout/resizable_grid/pane_ids.rs`
- `src/ui/layout/resizable_grid/should_resize_active_terminal.rs`
