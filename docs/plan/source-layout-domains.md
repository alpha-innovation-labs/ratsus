# Source Layout Domains

## Purpose

This document defines the intended `src/` organization for Ratsus. The goal is to group modules by product domain and responsibility, while keeping the project in a single Rust crate until the boundaries are stable enough to split into workspace crates.

## Proposed Tree

```text
src/
├── app/
├── core/
│   ├── rendering/
│   └── state/
├── ui/
│   ├── grid_layout/
│   ├── keyboard/
│   ├── layout/
│   ├── left_panel/
│   ├── menu_bar/
│   └── notifications/
├── extensions/
│   ├── expo/
│   ├── file_viewer/
│   ├── git_diff/
│   ├── harness/
│   │   ├── core/
│   │   ├── nexus/
│   │   ├── stub/
│   │   └── observations/
│   └── terminal/
└── shared/
```

## Domain Responsibilities

### `app/`

`app/` is the orchestration layer. It owns startup wiring, top-level event routing, lifecycle handling, extension registration, and cross-domain coordination. It is not a primitive UI or backend domain; it connects the app shell to extensions without owning their internals.

It may contain:

- App bootstrap and run loop integration.
- Top-level event dispatch.
- App state shell and coordinator implementation.
- Wiring between `core`, `ui`, and `extensions`.
- Lifecycle actions such as startup, tick handling, and shutdown.

It should not contain:

- Feature-specific rendering.
- Backend adapter internals.
- Terminal process implementation details.
- Reusable UI behavior that belongs in `ui/`.

### `core/`

`core/` contains app-level primitive mechanics that other domains build on top of. These modules are not standalone features and are not reusable visible UI widgets; they provide behind-the-scenes application foundations.

#### `core/rendering/`

`core/rendering/` owns root screen composition and global drawing plumbing. It composes visible regions, overlays, dialogs, cursor style, and shared shell styling without becoming the owner of reusable UI widgets.

It may contain:

- Root app render orchestration.
- Cursor styling.
- Shared border styling.
- Resize overlays and placeholders.
- Generic modal/dialog composition used by the app shell.
- High-level area splitting that composes multiple UI regions.

It should not contain:

- Expo card rendering.
- Terminal rendering internals.
- File viewer or git diff rendering.
- Left panel row-specific rendering if that rendering is owned by `ui/left_panel/`.

#### `core/state/`

`core/state/` is reserved for app-wide state types once they are separated from orchestration.

It may contain:

- Shared app state structs.
- Cross-domain state snapshots.
- State transition helpers that are not feature-specific.

This folder should only be created when state has been clearly separated from `app/`.

### `ui/`

`ui/` contains visible interface building blocks and reusable UI behavior. These modules are part of the shell, can be used by multiple extensions, and should not own backend or product-specific feature behavior.

#### `ui/grid_layout/`

`ui/grid_layout/` replaces the current `session_panes/` name. Its responsibility is organizing visible panes in a grid, independent of any single feature. Any extension may place content in those panes; grid layout owns pane mechanics, not the feature rendered inside them.

It may contain:

- Pane splitting.
- Active pane tracking.
- Pane close button hit-testing.
- Pane-to-content placement helpers, including session placement when the content is harness-backed.
- Pane resize behavior.
- Grid area lookup.

It should avoid terminal process details and extension-specific content behavior. Terminal-specific runtime behavior belongs in `extensions/terminal/`, while future feature content such as git diffs belongs in its own extension.

#### `ui/keyboard/`

`ui/keyboard/` contains shared keyboard behavior for UI controls.

It may contain:

- `ListKeyBehavior`.
- List key outcomes.
- Shared filtering/editing key helpers.
- Tests for common key contracts.

List-like panes, panels, and modals should use the shared keyboard contract so navigation hotkeys stay consistent.

#### `ui/layout/`

`ui/layout/` contains generic shell layout behavior.

It may contain:

- Focused pane state.
- Left pane visibility toggling.
- Root pane ID helpers.
- Resizable shell-grid mouse handling.

It should not own extension-specific layout such as Expo masonry placement.

#### `ui/left_panel/`

`ui/left_panel/` owns the navigation/sidebar experience.

It may contain:

- Folder rows.
- Session rows.
- Sidebar scrolling.
- Sidebar drag and drop.
- Sidebar selection.
- Sidebar-specific rendering.
- Sidebar-specific mouse and keyboard behavior.

The left panel is a navigation surface. It does not own terminal pane layout or backend session loading.

#### `ui/menu_bar/`

`ui/menu_bar/` owns the application menu bar.

It may contain:

- Menu state synchronization.
- Menu item mapping.
- Menu mouse handling.
- Menu rendering.

#### `ui/notifications/`

`ui/notifications/` owns UI notification helpers.

It may contain:

- Toast helper functions.
- User-facing notification messages.
- Notification styling helpers.

It should not perform the work that caused the notification. It should only present the result.

### `extensions/`

`extensions/` contains product capabilities unique to this app and built on top of the app shell. Each extension should own its state, input handling, rendering, backend integration points, and tests where practical.

Extensions should depend on `app`, `core`, `ui`, and `shared` contracts, but the shell should avoid depending on extension internals except through explicit integration points.

#### `extensions/expo/`

`extensions/expo/` owns the Expo experience.

It may contain:

- Expo card models.
- Expo filtering.
- Expo masonry layout.
- Expo keyboard and mouse handling.
- Observation preview loading hooks.
- Expo-specific rendering.

#### `extensions/file_viewer/`

`extensions/file_viewer/` owns file browsing and file preview behavior.

It may contain:

- File tree view state.
- File selection updates.
- File viewer input handling.
- File viewer rendering.

The current file-system tree code from `main_pane/` is a candidate for this extension.

#### `extensions/git_diff/`

`extensions/git_diff/` is reserved for git diff viewing.

It may contain:

- Diff loading.
- Diff state.
- Diff navigation.
- Diff rendering.

This folder should be created when the feature exists.

#### `extensions/harness/`

`extensions/harness/` combines the harness contract, concrete harness adapters, and session metadata lifecycle because these concerns share a backend/session boundary. Harness-specific operations, including session deletion, belong in the concrete harness adapter such as `extensions/harness/nexus/` or a future `extensions/harness/<harness_name>/`.

It may contain:

- Harness trait and session types.
- Nexus adapter.
- Stub adapter.
- Session loading and refresh.
- Session deletion through the active concrete backend.
- New chat/session creation.
- Observation support used by harness-backed sessions.

Suggested children:

```text
extensions/harness/
├── core/
├── nexus/
├── stub/
└── observations/
```

`core/` contains backend-neutral contracts. `nexus/` and `stub/` contain concrete implementations. `observations/` contains observation files, parsing, and watching.

#### `extensions/terminal/`

`extensions/terminal/` owns terminal runtime behavior.

It may contain:

- PTY spawning.
- Shell command resolution.
- Terminal input encoding.
- Terminal output processing.
- Reader/writer workers.
- Terminal process exit checks.
- Normal terminal session persistence.
- Conversion between persisted terminal data and app session records when required.

It should not own grid splitting. Pane organization belongs in `ui/grid_layout/`.

### `shared/`

`shared/` is for dependency-light primitives used by multiple domains. It may remain empty when no real shared primitive exists; the goal is correct ownership, not filling the folder.

It may contain:

- Pure formatting helpers.
- Path helpers.
- ID helpers.
- Small test helpers that are not app-specific.

It should not contain:

- App state.
- Rendering ownership.
- Backend behavior.
- Feature logic.
- UI widgets.

If a helper grows meaningful domain behavior, it should move into the relevant domain instead of staying in `shared/`.

## Documentation rule

Before committing code, review whether the work changed architecture, behavior, scenarios, or source layout. Update the relevant `docs/` files and this plan when those changes affect project understanding.

## Migration Notes

The project should be reorganized inside the existing crate first. A workspace split into `apps/` and `crates/` should wait until the module boundaries are clean and the public APIs are obvious.

A later workspace could look like this:

```text
apps/
└── ratsus/
crates/
├── ratsus-core/
├── ratsus-ui/
├── ratsus-extensions/
└── ratsus-shared/
```

This should not be the first migration step because the current modules still have many cross-domain dependencies. Moving to folders first keeps the refactor reversible and avoids designing crate APIs before the boundaries are proven.
