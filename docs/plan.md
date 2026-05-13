# Left Pane Unified Input and Footer Plan

## Goal

Unify left-pane behavior so the left-pane shell owns hotkey routing and footer design, while active left-pane content decides what each action means.

## Current State

- The left-pane border shell is shared in `src/core/rendering/screen/render_app.rs`.
- Chat/session left-pane content uses:
  - Footer: `src/ui/left_panel/render/hotkey_footer.rs`
  - Shared input contract: `src/ui/keyboard/list/behavior.rs`
  - Chat adapter: `src/ui/left_panel/input/key_behavior.rs`
- Files left-pane content uses separate behavior:
  - Footer/status rendering: `src/extensions/file_viewer/tree/render_view.rs`
  - Input handling: `src/extensions/file_viewer/tree/handle_key.rs`
  - Ratkit file-tree handlers are called directly from our file-tree key handler.

## Gap

The code is moderately far from the desired design:

- Footer design is not unified.
- Chat and Files left-pane input paths are separate.
- The left-pane shell does not currently own `j/k`, `h/l`, `gg/G`, Enter, `/`, `d`, `q`, or similar key routing for every content type.
- Files delegates navigation directly to Ratkit instead of receiving normalized left-pane actions from the shell.

## Desired Design

The left-pane shell should:

1. Own the common footer styling and layout.
2. Own common hotkey parsing.
3. Convert keys into semantic actions, such as:
   - Move next
   - Move previous
   - Collapse
   - Expand/open
   - Focus first
   - Focus last
   - Activate
   - Start filtering
   - Delete
   - Toggle selection
   - Quit/close context
4. Pass those semantic actions to the active left-pane content.

Each left-pane content implementation should:

1. Render only its body content.
2. Provide footer items or status text to the shell.
3. Handle semantic left-pane actions in its own domain-specific way.
4. Keep implementation details private, including whether it uses Ratkit internally.

## Proposed Contract

Introduce a shared left-pane content contract, likely under `src/ui/left_panel/`.

Possible shape:

```rust
pub enum LeftPaneAction {
    MoveNext,
    MovePrevious,
    Collapse,
    Expand,
    FocusFirst,
    FocusLast,
    Activate,
    StartFilter,
    Delete,
    ToggleSelection,
    Quit,
}

pub struct LeftPaneFooterItem {
    pub key: &'static str,
    pub description: &'static str,
}

pub trait LeftPaneContent {
    fn handle_left_pane_action(&mut self, action: LeftPaneAction) -> LeftPaneActionOutcome;
    fn footer_items(&self) -> Vec<LeftPaneFooterItem>;
}
```

The exact API should be refined during implementation.

## Migration Steps

1. Add a shared left-pane action enum.
2. Add a shared footer item model and renderer.
3. Move common footer rendering into the left-pane shell.
4. Update Chat/session pane to expose footer items and handle semantic actions.
5. Update Files tree pane to stop owning its footer design.
6. Replace direct Ratkit file-tree key routing with semantic left-pane action handling.
7. Keep Ratkit file-tree operations behind the Files content adapter where needed.
8. Ensure `ListKeyBehavior` remains the shared source of default list hotkey semantics or is adapted into the new left-pane contract.
9. Add regression tests for Chat and Files using the same key-to-action path.

## Validation

Run:

```bash
cargo fmt --check
cargo test
```

Expected result: all existing tests pass, plus new tests confirming Chat and Files left-pane content share the same hotkey routing and footer renderer.
