# App events

`src/app/events/` owns top-level Ratkit coordinator event handling and periodic tick work.

## Responsibilities

- Route keyboard, mouse, resize, and tick events from Ratkit.
- Convert handled events into `CoordinatorAction` values.
- Drain async deletion results.
- Poll session refresh results.
- Poll observation watchers and preview caches.
- Request redraws when visible state changes.

## Flow

`handle_app_event` is the single entry point used by `CoordinatorApp`. It delegates keyboard and mouse events to `app/input`, turns resize events into redraws, and sends ticks to `handle_tick_event`.

## Key files

- `src/app/events/handle_app_event.rs`
- `src/app/events/handle_tick_event.rs`
- `src/app/events/drain_delete_session_receiver.rs`
- `src/app/events/redraw_action.rs`
