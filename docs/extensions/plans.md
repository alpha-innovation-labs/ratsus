# Plans extension

`src/extensions/plans/` owns Markdown plan browsing for the left-pane `Plan` mode.

## Responsibilities

- Create and load plans from each workspace folder's `plans/` directory.
- List only Markdown plan files (`.md` and `.markdown`), including nested paths under each workspace `plans/` directory.
- Render every workspace folder row, including folders with no plan files, then plan files below each folder.
- Maintain plan focus, active selection, scrolling, filtering, and drag reorder state.
- Implement the shared `ListKeyBehavior` contract through `LeftPaneContent`.
- Render the active plan in the main pane with the existing Markdown/code preview widgets.
- Use a Ratkit `FileWatcher` only for the active/open plan file so external edits refresh its preview.

## Boundary

The plans extension creates missing workspace `plans/` directories when loading plan inventory. It does not watch every plan directory, and it does not edit, rename, or delete plan files; it discovers Markdown files, orders them in memory, and previews the selected file.
