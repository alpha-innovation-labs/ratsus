# App navigation

`src/app/navigation/` owns session navigation helpers that coordinate left-panel order, active indexes, focused indexes, and selection state.

## Responsibilities

- Find adjacent chat sessions in left-panel order.
- Cycle between sessions shown in the left panel.
- Build ordered session indexes from visible left-panel rows.
- Reorder sessions by target index.
- Toggle conversation selection by session index.
- Clamp next active session indexes after movement or deletion.

## Boundary

This module works with app-level session order and active selection. Row rendering and row hit testing belong in `src/ui/left_panel/`; terminal pane placement belongs in `src/ui/grid_layout/`.

## Key files

- `src/app/navigation/adjacent_chat_index_in_left_pane_order.rs`
- `src/app/navigation/cycle_chat_in_left_pane_order.rs`
- `src/app/navigation/ordered_chat_indices_from_left_rows.rs`
- `src/app/navigation/reorder_session_to_index.rs`
- `src/app/navigation/toggle_conversation_selection_by_index.rs`
- `src/app/navigation/next_chat_index.rs`
