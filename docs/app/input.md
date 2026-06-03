# App input

`src/app/input/` owns top-level keyboard and mouse dispatch. It decides which domain receives an input event based on focused pane, active tab, dialog state, and shell mode.

## Responsibilities

- Dispatch keyboard events to dialogs, command bar, left panel, terminal, file viewer, Expo, or menu behavior.
- Dispatch mouse events to shell layout, left panel, terminal panes, tabs, and active extension views.
- Convert keyboard shortcuts into navigation, tab changes, pane focus changes, deletion prompts, or terminal input.
- Let normal shell terminals receive reserved terminal editing keys such as Ctrl+E instead of treating them as app shortcuts.
- Map `Ctrl+Tab` and `Ctrl+Shift+Tab` into session cycling commands.
- Map Ctrl+backtick into cycling the left pane through Sessions, Plans, and Files.
- Map `Alt+N` to a vertical split, `Alt+Shift+N` to a horizontal split, and `Cmd+N` to a new chat.

## Boundary

Input-specific logic for a feature should live in the feature domain after top-level dispatch. For example, command bar keyboard behavior belongs in `src/extensions/command_bar/input/`, Expo keyboard behavior belongs in `src/extensions/expo/input/`, and left-panel keyboard behavior belongs in `src/ui/left_panel/input/`.

## Key files

- `src/app/input/handle_keyboard_event.rs`
- `src/app/input/handle_app_mouse.rs`
- `src/app/input/handle_left_keyboard.rs`
- `src/app/input/handle_terminal_keyboard.rs`
- `src/app/input/session_cycle_direction_for_keyboard.rs`
