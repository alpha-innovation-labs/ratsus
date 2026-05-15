# UI keyboard

`src/ui/keyboard/` owns shared keyboard contracts and helpers used by list-like controls and filter input.

## Responsibilities

- Define `ListKeyBehavior` for consistent list navigation.
- Define shared list key outcomes.
- Provide wrapped list-position movement for list-like controls.
- Provide filter key acceptance helpers.
- Test common keyboard behavior once and reuse it across UI surfaces.

## List contract

List-like panes, panels, and modals must use `ListKeyBehavior` so these controls stay consistent: `j/k`, arrow keys, `gg/G`, Enter, `/`, `d`, space when selection applies, and `q`. Row movement should wrap at list edges through `wrapped_list_position`.

Filter mode must treat typed characters as query text instead of navigation commands.

## Child modules

- `filter/` contains filter/editing key helpers.
- `list/` contains list behavior and outcomes.

## Key files

- `src/ui/keyboard/list/behavior.rs`
- `src/ui/keyboard/list/outcome.rs`
- `src/ui/keyboard/list/wrapped_position.rs`
- `src/ui/keyboard/filter/key_accepted.rs`
