# File viewer extension

`src/extensions/file_viewer/` owns file browsing and file preview behavior for the navbar-selected Files mode.

## Responsibilities

- Maintain the file-system tree view state.
- Show workspace folders as grouped roots in the left-pane Files mode.
- Persist grouped workspace open/closed file-tree folders in the multiplexer state file.
- Render grouped workspace file trees as left-pane content when Files mode is active.
- Update selected paths from keyboard and mouse input.
- Render file preview content in the main pane.
- Avoid filesystem watchers for file rows and previews; files refresh only through explicit navigation or reload paths.
- Route Markdown files to `MarkdownWidget`.
- Route non-Markdown files to `CodeWidget`.
- Handle preview scrolling and widget input.
- Render file-viewer tab titles and tab hit testing.

## Preview routing

Preview state is loaded from a selected path. Directories, empty files, and unreadable files produce text previews. Files ending in `.md` or `.markdown` use Ratkit `MarkdownWidget`; other readable files use Ratkit `CodeWidget`.

Code previews use configured code state so line-focused navigation remains visible while moving with `j/k`. Markdown previews use the shared Markdown widget so rendered Markdown scrolls consistently with other Ratkit document viewers.

## Tree behavior

`FileSystemTreeView` owns preview state plus grouped workspace file-tree focus, filtering, expansion, and selection. The left pane shows each workspace folder as a root row and file entries underneath it.

The legacy single-root Ratkit tree state is retained for persisted expansion compatibility and preview loading. Files mode syncs roots from `AppState::folder_order`, stores grouped open/closed state in `FileSystemTreeView`, and persists it through `PersistedFileSystemTreeState`.

Files mode does not start directory, selected-file, or repository watchers. This keeps startup and tick handling free of broad filesystem watching; grouped file rows and previews update when the user navigates, opens, or explicitly reloads file state.

## Key files

- `src/extensions/file_viewer/tree/view.rs`
- `src/extensions/file_viewer/tree/render_view.rs`
- `src/extensions/file_viewer/tree/git_status/`
- `src/extensions/file_viewer/tree/git_status_methods.rs`
- `src/extensions/file_viewer/tree/update_selection.rs`
- `src/extensions/file_viewer/tree/left_pane_content.rs`
- `src/extensions/file_viewer/tree/workspace_file_row.rs`
- `src/extensions/file_viewer/tree/workspace_methods.rs`
- `src/extensions/file_viewer/tree/workspace_rows.rs`
- `src/extensions/file_viewer/tree/sync_workspace_root.rs`
- `src/extensions/file_viewer/tree/persisted_file_system_tree_state.rs`
- `src/extensions/file_viewer/preview/file_preview_state.rs`
- `src/extensions/file_viewer/preview/preview_state_for_path.rs`
- `src/extensions/file_viewer/preview/is_markdown_path.rs`
- `src/extensions/file_viewer/preview/markdown_widget_for_path.rs`
- `src/extensions/file_viewer/preview/configured_code_state.rs`
- `src/extensions/file_viewer/preview/render_preview.rs`
- `src/extensions/file_viewer/tabs/tab.rs`
- `src/extensions/file_viewer/tabs/title_line.rs`
