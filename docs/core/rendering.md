# Core rendering

`src/core/rendering/` owns root screen composition and global shell drawing plumbing. It is core infrastructure, not the owner of reusable visible UI widgets.

## Responsibilities

- Render the complete application screen.
- Split the menu bar, left pane, and main pane into visible regions.
- Render active left-pane content and footer help.
- Delegate main-pane tab rendering to chat grid, file preview, diff placeholder, or Expo.
- Render app-level dialogs.
- Render resize overlays and placeholders.
- Apply shared cursor and border styling.
- Render toast notifications after primary content.

## Render flow

`AppState::on_draw` delegates to `render_app`. The renderer draws the menu bar, computes visible shell panes, renders active left-pane content, draws the selected main tab, then renders overlays, toasts, and dialogs.

## Boundary

This module should stay composition-focused. Terminal internals belong in `extensions/terminal`, file preview behavior belongs in `extensions/file_viewer`, Expo card rendering belongs in `extensions/expo`, and reusable visible shell widgets belong in `ui/`.

## Key files

- `src/core/rendering/screen/render_app.rs`
- `src/core/rendering/screen/split_left_pane_content.rs`
- `src/core/rendering/dialog/render_delete_session_confirmation_dialog.rs`
- `src/core/rendering/resize/render_resizable_grid_overlay.rs`
- `src/core/rendering/resize/render_resize_placeholder.rs`
- `src/core/rendering/style/default_border_color.rs`
- `src/core/rendering/style/left_focused_border_color.rs`
- `src/core/rendering/style/apply_cursor_style.rs`
