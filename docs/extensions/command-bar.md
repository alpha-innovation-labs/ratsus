# Command bar extension

`src/extensions/command_bar/` owns the modal command palette for discovering and executing app commands.

## Responsibilities

- Track command bar open state, filter query, and selected command row.
- Render available commands with their display hotkeys.
- Handle shared list keyboard input for movement, filtering, activation, and close.
- Map selected command rows onto existing app actions, including commands without direct hotkeys.

## Behavior

`Ctrl+K` opens the command bar in filter mode. `Ctrl+H` opens chat history directly through the conversation picker. `Next session` and `Previous session` expose `Ctrl+Tab` and `Ctrl+Shift+Tab`. Enter on a command closes the command bar and executes the mapped app action. Escape clears the active filter first, then closes the command bar when filtering is inactive. `Select workspace 1` through `Select workspace 9` expose `Ctrl+1` through `Ctrl+9`; in grouped folder mode they activate the first session in the matching folder. `Toggle workspace view` has no direct hotkey and switches between workspace-pane mode and the legacy all-folders left-pane view.

## Key files

- `src/extensions/command_bar/data/filtered_commands.rs`
- `src/extensions/command_bar/input/handle_keyboard.rs`
- `src/extensions/command_bar/render/render_dialog.rs`
- `src/extensions/command_bar/command/app_hotkey_for_command.rs`
