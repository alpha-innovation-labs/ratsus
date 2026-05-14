# Expo extension

`src/extensions/expo/` owns the folder-level conversation overview. It presents sessions as cards grouped by the selected project folder and enriches them with observation previews.

## Responsibilities

- Build Expo card models from sessions and observation previews.
- Filter cards by query and selected folder.
- Keep focused cards visible during keyboard navigation.
- Handle Expo keyboard and mouse input.
- Calculate masonry column layout and card placement.
- Render cards, hotkey footer text, and empty states.
- Load and refresh observation previews through the active harness.

## Data flow

The app records the selected Expo folder in `AppState`. Expo derives visible session indices for that folder, applies filtering, builds card models, lays them out with masonry placement, and stores card areas for mouse hit testing.

Observation preview requests are built from visible sessions and loaded through `ChatHarness::load_observation_previews`. The Nexus harness reads real observation state; the stub harness returns deterministic fake previews.

## Input behavior

Expo supports keyboard navigation, mouse selection, scrolling, filter entry, and card-width adjustment. Focus movement clamps to visible cards and scrolls the view to keep the focused card on screen.

## Key files

- `src/extensions/expo/card/model.rs`
- `src/extensions/expo/card/models.rs`
- `src/extensions/expo/card/render_conversation.rs`
- `src/extensions/expo/filter/filter_cards.rs`
- `src/extensions/expo/filter/filtered_session_indices.rs`
- `src/extensions/expo/input/handle_keyboard.rs`
- `src/extensions/expo/input/handle_mouse.rs`
- `src/extensions/expo/input/keep_focused_card_visible.rs`
- `src/extensions/expo/layout/masonry.rs`
- `src/extensions/expo/render/render_view.rs`
- `src/extensions/expo/observations/preview_requests.rs`
- `src/extensions/expo/observations/spawn_cache_worker.rs`
