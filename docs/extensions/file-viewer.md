# File viewer extension

`src/extensions/file_viewer/` owns file browsing and file preview behavior for the Files main-pane tab.

## Responsibilities

- Maintain the file-system tree view state.
- Render the file tree as left-pane content when the Files tab is active.
- Update selected paths from keyboard and mouse input.
- Render file preview content in the main pane.
- Route Markdown files to `MarkdownWidget`.
- Route non-Markdown files to `CodeWidget`.
- Handle preview scrolling and widget input.
- Render file-viewer tab titles and tab hit testing.

## Preview routing

Preview state is loaded from a selected path. Directories, empty files, and unreadable files produce text previews. Files ending in `.md` or `.markdown` use Ratkit `MarkdownWidget`; other readable files use Ratkit `CodeWidget`.

Code previews use configured code state so line-focused navigation remains visible while moving with `j/k`. Markdown previews use the shared Markdown widget so rendered Markdown scrolls consistently with other Ratkit document viewers.

## Tree behavior

`FileSystemTreeView` owns tree state and selection. It exposes left-pane content for rendering and methods for updating preview state when selection changes.

## Key files

- `src/extensions/file_viewer/tree/view.rs`
- `src/extensions/file_viewer/tree/render_view.rs`
- `src/extensions/file_viewer/tree/update_selection.rs`
- `src/extensions/file_viewer/tree/left_pane_content.rs`
- `src/extensions/file_viewer/preview/file_preview_state.rs`
- `src/extensions/file_viewer/preview/preview_state_for_path.rs`
- `src/extensions/file_viewer/preview/is_markdown_path.rs`
- `src/extensions/file_viewer/preview/markdown_widget_for_path.rs`
- `src/extensions/file_viewer/preview/configured_code_state.rs`
- `src/extensions/file_viewer/preview/render_preview.rs`
- `src/extensions/file_viewer/tabs/tab.rs`
- `src/extensions/file_viewer/tabs/title_line.rs`
