use crate::ui::grid_layout::group::default_split_pane_session_group_state::default_split_pane_session_group_state;
use crate::ui::grid_layout::group::ensure_group_for_split::ensure_group_for_split;

/// Verifies the first split creates a named group with source and child panes.
#[test]
fn first_split_creates_group() {
    let mut state = default_split_pane_session_group_state();

    let group_id = ensure_group_for_split(&mut state, 10, 20);

    let group = state.groups.get(&group_id).expect("split group");
    assert_eq!(group.name, "Group 1");
    assert_eq!(group.panes, vec![10, 20]);
}

/// Verifies later splits from a grouped pane extend the same group.
#[test]
fn later_split_extends_existing_group() {
    let mut state = default_split_pane_session_group_state();
    let group_id = ensure_group_for_split(&mut state, 10, 20);

    let second_group_id = ensure_group_for_split(&mut state, 10, 30);

    assert_eq!(second_group_id, group_id);
    assert_eq!(state.groups.get(&group_id).unwrap().panes, vec![10, 20, 30]);
}
