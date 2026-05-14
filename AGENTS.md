# Ratsus Agent Guide

Ratsus is a Ratatui/Ratkit TUI for organizing harness-backed coding sessions. The app layer starts the runtime and coordinates state, core provides app-wide primitive mechanics, UI modules provide visible reusable interface behavior, extensions implement product features like Nexus sessions, terminals, Expo, and file previews, and shared helpers are only for real cross-domain primitives. This file is the high-level index; detailed behavior lives in `docs/`.

## Detailed sections

### App

`src/app/` is the orchestration layer. It owns startup wiring, top-level state, event dispatch, lifecycle coordination, and cross-domain workflows that connect shell UI, extensions, and harness backends without owning their internals.

- [Deletion](docs/app/deletion.md): confirms, deletes, and cleans up chat sessions.
- [Events](docs/app/events.md): routes Ratkit coordinator events and app ticks.
- [Expo bridge](docs/app/expo.md): opens Expo views from app-level navigation.
- [Focus](docs/app/focus.md): toggles active shell focus between panes.
- [Input](docs/app/input.md): dispatches top-level keyboard and mouse input.
- [Lifecycle](docs/app/lifecycle.md): starts the Ratkit application runtime.
- [Navigation](docs/app/navigation.md): moves, selects, orders, and cycles sessions.
- [Sessions](docs/app/sessions.md): manages active session indexes, exited sessions, and normal terminal startup.
- [State](docs/app/state.md): owns `AppState` construction and cross-domain state fields.
- [Test support](docs/app/test-support.md): builds app fixtures for focused tests without real terminals.

### Core

`src/core/` contains app-wide primitive foundations that are not standalone product features and are not reusable visible UI widgets. It focuses on behind-the-scenes shell mechanics such as root composition, global overlays, dialogs, cursor style, and shared shell styling.

- [Rendering](docs/core/rendering.md): composes the root screen, dialogs, overlays, and shared style.

### UI

`src/ui/` owns visible reusable shell interface behavior. It provides layout, navigation, keyboard contracts, grid panes, left-panel rows, menus, and notifications that multiple features can use while avoiding backend and extension-specific logic.

- [Grid layout](docs/ui/grid-layout.md): places, splits, bundles, resizes, and renders reusable grid panes.
- [Keyboard](docs/ui/keyboard.md): provides shared keyboard contracts and list behavior.
- [Layout](docs/ui/layout.md): owns shell focus, left-pane visibility, and resizable-grid mouse state.
- [Left panel](docs/ui/left-panel.md): renders and operates folder/session navigation.
- [Menu bar](docs/ui/menu-bar.md): renders and synchronizes the application menu bar.
- [Notifications](docs/ui/notifications.md): presents user-facing toast notifications.

### Extensions

`src/extensions/` owns product capabilities built on top of the app shell. Each extension should own its feature state, rendering, input behavior, backend integration points, and tests while using shared app and UI contracts.

- [Expo](docs/extensions/expo.md): renders folder-level conversation cards, filtering, layout, and observation previews.
- [File viewer](docs/extensions/file-viewer.md): owns file-tree navigation, tabs, code previews, and Markdown previews.
- [Harness](docs/extensions/harness.md): defines session backends plus Nexus, stub, conversation picker, refresh, deletion, and observations.
- [Terminal](docs/extensions/terminal.md): owns PTY terminals, terminal IO, copy mode, process checks, and normal terminal persistence.

### Shared

`src/shared/` is reserved for dependency-light primitives that are genuinely reused across domains. It should stay empty until real reuse exists and must not become a dumping ground for feature logic, backend behavior, rendering, or app orchestration.

- [Shared utilities](docs/shared.md): reserved for dependency-light primitives used across multiple domains.

### Plans

`docs/plan/` stores planning documents, test strategy, and domain-boundary notes. These files explain intended direction and should be updated when scenarios, source layout, or architectural boundaries change.

- [Testing plan](docs/plan/e2e_tests.md): tracks E2E scenarios, snapshot expectations, and test coverage goals.
- [Source layout plan](docs/plan/source-layout-domains.md): records domain-boundary rationale and migration notes.

## Common commands

- `just dev` runs the app with the real Nexus harness and sccache-backed builds.
- `just dev-stub` runs the app with `RATSUS_BACKEND=stub` and sccache-backed builds.
- `just check` runs format verification, Cargo check, Clippy, and tests.
- `just clippy` runs `cargo clippy --workspace --all-targets -- -D warnings` with sccache.
- `just test` runs `cargo nextest run` with sccache for parallel test-binary scheduling.
- `just fmt-check` runs `cargo fmt --check`.
- `just fmt` formats Rust source.
- `just install-tools` installs Clippy, rustfmt, sccache, and cargo-nextest.

## Required rules

- Start user-visible features with E2E coverage and start bug fixes by reproducing the bug in a test.
- Before committing code, review the work and update relevant docs when architecture, behavior, scenarios, or source layout changed.
- Do not claim a command passed unless that exact command was run.
- Never delete, rewrite, or migrate user-owned Nexus data unless the target is proven test-owned or explicitly app-owned.
- List-like panes, panels, and modals must implement the shared `ListKeyBehavior` contract.
- Keep files small, domain-scoped, documented, and split by single-purpose functions.
