//! Split-pane session group state and lifecycle helpers.

pub mod add_pane_to_group;
pub mod compact_split_pane_session_groups;
pub mod create_split_pane_session_group;
pub mod default_split_pane_session_group_state;
pub mod ensure_group_for_split;
#[cfg(test)]
mod ensure_group_for_split_tests;
pub mod group_session_count;
pub mod pane_group_id;
pub mod pane_group_name;
pub mod remove_pane_from_groups;
pub mod split_pane_session_group;
pub mod split_pane_session_group_state;
