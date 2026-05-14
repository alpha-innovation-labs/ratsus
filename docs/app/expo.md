# App Expo bridge

`src/app/expo/` owns app-level entry points that open the Expo extension from existing navigation state.

## Responsibilities

- Activate Expo for a selected folder.
- Open Expo for the focused conversation's project folder.
- Keep app-level state changes small and delegate Expo behavior to `src/extensions/expo/`.

## Boundary

This module bridges shell navigation to the Expo extension. Card modeling, filtering, layout, rendering, and observation previews remain inside `src/extensions/expo/`.

## Key files

- `src/app/expo/activate_expo_folder.rs`
- `src/app/expo/open_expo_for_focused_conversation.rs`
