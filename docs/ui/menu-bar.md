# UI menu bar

`src/ui/menu_bar/` owns the application menu bar shown at the top of the TUI.

## Responsibilities

- Build menu bar state for the active main-pane tab.
- Render the menu bar and split its area from the rest of the screen.
- Handle menu mouse input and tab selection.
- Map menu state to file viewer tabs and app actions.

## Child modules

- `input/` handles menu mouse interaction.
- `render/` renders the menu bar and splits the screen area.
- `state/` builds and synchronizes menu state.

## Boundary

Menu bar rendering and input stay here. The behavior behind each tab belongs to the owning feature, such as chat grid, file viewer, diff, or Expo.

## Key files

- `src/ui/menu_bar/state/app_menu_bar.rs`
- `src/ui/menu_bar/input/handle_mouse.rs`
- `src/ui/menu_bar/render/render_app_menu_bar.rs`
- `src/ui/menu_bar/render/split_area.rs`
