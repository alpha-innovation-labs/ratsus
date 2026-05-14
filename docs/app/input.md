# App input

`src/app/input/` owns top-level keyboard and mouse dispatch. It decides which domain receives an input event based on focused pane, active tab, dialog state, and shell mode.

## Responsibilities

- Dispatch keyboard events to dialogs, left panel, terminal, file viewer, Expo, or menu behavior.
- Dispatch mouse events to shell layout, left panel, terminal panes, tabs, and active extension views.
- Convert keyboard shortcuts into navigation, tab changes, pane focus changes, deletion prompts, or terminal input.
- Map keyboard direction commands used for chat cycling.

## Boundary

Input-specific logic for a feature should live in the feature domain after top-level dispatch. For example, Expo keyboard behavior belongs in `src/extensions/expo/input/`, and left-panel keyboard behavior belongs in `src/ui/left_panel/input/`.

## Key files

- `src/app/input/handle_keyboard_event.rs`
- `src/app/input/handle_app_mouse.rs`
- `src/app/input/handle_left_keyboard.rs`
- `src/app/input/handle_terminal_keyboard.rs`
- `src/app/input/chat_cycle_direction_for_keyboard.rs`
