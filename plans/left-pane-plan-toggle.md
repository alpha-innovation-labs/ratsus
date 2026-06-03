---
name: plan
---

# Left pane Plan toggle implementation plan

## Objective

Add a `Session | Plan` toggle to the left-pane top bar. `Session` keeps the current session list. `Plan` shows Markdown plans loaded from `./plans`, creating that directory when missing.

## Scope

- Add app state for the active left-pane mode.
- Add a plans extension that owns plan discovery, filtering, focus, drag reordering, and preview rendering.
- Reuse the shared left-pane keyboard contract so `j/k`, `gg/G`, `/`, `enter`, and quit/filter behavior match other list panes.
- Add mouse support for clicking the top-bar toggle and dragging plan rows to reorder.
- Render the selected plan in the main pane while Plan mode is active.
- Update docs for the new extension and left-panel behavior.

## Non-goals

- Creating or editing plan files from inside the TUI.
- Loading non-Markdown files from `./plans`.
- Replacing the file viewer tree.

## Implementation folders

- `src/extensions/plans/`: plan state, filesystem loading, filtering, row rendering, drag handling, and preview rendering.
- `src/ui/left_panel/`: active content routing, shared top-bar mode state, and left-pane mouse/keyboard integration.
- `src/app/state/`: app-owned mode and plan state wiring.
- `src/app/input/`: top-bar click routing and plan-preview keyboard/mouse routing.
- `src/core/rendering/screen/`: left-pane title/top-bar rendering and main-pane plan preview routing.
- `docs/extensions/` and `docs/ui/left-panel.md`: documentation updates.

## Test plan

1. Unit-test `./plans` creation and Markdown-only loading.
2. Unit-test plan filtering and focus movement.
3. Unit-test drag reorder semantics.
4. Unit-test top-bar active-content routing.
5. Run `just check` after implementation.
