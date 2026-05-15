# Plans extension

`src/extensions/plans/` owns Markdown plan browsing for the left-pane `Plan` mode.

## Responsibilities

- Create and load plans from `./plans` relative to the app working directory.
- List only Markdown plan files (`.md` and `.markdown`).
- Maintain plan focus, active selection, scrolling, filtering, and drag reorder state.
- Implement the shared `ListKeyBehavior` contract through `LeftPaneContent`.
- Render the active plan in the main pane with the existing Markdown/code preview widgets.

## Boundary

The plans extension does not create, edit, rename, or delete plan files. It only discovers Markdown files, orders them in memory, and previews the selected file.
