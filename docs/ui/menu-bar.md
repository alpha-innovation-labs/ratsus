# UI menu bar

`src/ui/menu_bar/` owns the application menu bar shown at the top of the TUI.

## Responsibilities

- Build menu bar state for the active left-pane mode: Sessions, Plans, or Files.
- Render the menu bar and split its area from the rest of the screen.
- Render right-aligned app diagnostics on the menu-bar bottom row.
- Handle menu mouse input and left-pane mode selection.
- Map menu state to Sessions, Plans, and Files app actions.

## Child modules

- `input/` handles menu mouse interaction.
- `render/` renders the menu bar and splits the screen area.
- `state/` builds and synchronizes menu state.

## Boundary

Menu bar rendering and input stay here. The behavior behind each mode belongs to the owning feature, such as sessions, plans, or file viewer.

## Key files

- `src/ui/menu_bar/state/app_menu_bar.rs`
- `src/ui/menu_bar/input/handle_mouse.rs`
- `src/ui/menu_bar/render/render_app_menu_bar.rs`
- `src/ui/menu_bar/render/render_menu_bar_bottom_status.rs`
- `src/ui/menu_bar/render/split_area.rs`
