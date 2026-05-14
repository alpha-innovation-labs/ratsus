# App focus

`src/app/focus/` owns app-level focus toggling between major shell regions.

## Responsibilities

- Switch the focused pane between left navigation and terminal/main-pane content.
- Preserve domain-specific focus behavior inside the owning UI or extension module.

## Boundary

This module only changes top-level focused-pane state. Left-panel row focus belongs in `src/ui/left_panel/`, terminal pane activation belongs in `src/ui/grid_layout/`, and file or Expo focus belongs to their extensions.

## Key files

- `src/app/focus/toggle_focused_pane.rs`
