# Ratkit Adapting Plan

## Goal

Prefer Ratkit-native services and primitives where they reduce custom Ratsus code, improve consistency, and match existing Ratkit event-loop contracts.

## Recommendations

### 1. Watch file viewer selections with `FileWatcher`

- Add a Ratkit `FileWatcher` for the active file preview and/or file tree root.
- Poll it from the existing tick flow, matching the session and observation watcher pattern.
- Refresh the file preview only when the selected file changes.
- Refresh the tree only when root-level directory changes affect visible rows.

Expected benefit: file previews stay current without manual reopening, while redraws remain event-driven.

### 2. Centralize global shortcuts with `HotkeyService`

- Move top-level shortcuts from `app/input/handle_keyboard_event.rs` into a Ratkit hotkey registry.
- Define scopes for global, terminal, left panel, picker, and modal contexts.
- Keep focused-pane handlers for text/terminal input after scoped hotkey resolution.

Expected benefit: fewer scattered modifier checks and a single source for shortcut labels/help.

### 3. Use `Dialog.footer` for picker help text

- Move the conversation picker help line out of custom body rendering.
- Use Ratkit dialog footer support for static keyboard hints.
- Keep only filter text and result rows inside `ConversationPickerBody`.

Expected benefit: cleaner body layout and consistent modal chrome with delete confirmation.

### 4. Review split-pane chrome for Ratkit `Pane` and `Button`

- Evaluate replacing custom split pane borders in `ui/grid_layout/render/render_chat_sessions.rs` with Ratkit `Pane` styling.
- Evaluate replacing custom close-title hit testing with Ratkit `Button` behavior if it can preserve current TUI layout.
- Keep `ResizableGrid` as the layout owner.

Expected benefit: less custom border/button rendering while preserving grid behavior.

## Validation checklist

- Add or update E2E coverage before user-visible behavior changes.
- Keep tick handlers returning `Continue` unless visible state changes.
- Store all Ratkit widget/service state in `AppState`, never in render loops.
- Run `just fmt-check` and `just test` after each implementation step.
