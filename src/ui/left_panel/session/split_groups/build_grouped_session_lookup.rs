use std::collections::BTreeMap;

use ratkit::primitives::resizable_grid::PaneId;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::group_session_count::group_session_count;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;
use crate::ui::left_panel::session::split_groups::grouped_session_child::GroupedSessionChild;
use crate::ui::left_panel::session::split_groups::grouped_session_lookup::GroupedSessionLookup;

/// Builds lookup tables for sessions that should render under split-group parents.
pub fn build_grouped_session_lookup(
    session_terminals: &[SessionTerminal],
    split_groups: &SplitPaneSessionGroupState,
    pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
) -> GroupedSessionLookup {
    let index_by_session_id = session_index_by_id(session_terminals);
    let mut lookup = GroupedSessionLookup::default();
    for (group_id, group) in &split_groups.groups {
        if group_session_count(group, pane_session_bundles) < 2 {
            continue;
        }
        let children = group
            .panes
            .iter()
            .flat_map(|pane_id| {
                children_for_pane(*pane_id, pane_session_bundles, &index_by_session_id)
            })
            .collect::<Vec<_>>();
        if children.len() < 2 {
            continue;
        }
        for child in &children {
            lookup
                .group_id_by_session_index
                .insert(child.session_index, *group_id);
        }
        lookup.children_by_group_id.insert(*group_id, children);
    }
    lookup
}

/// Indexes current session vector positions by stable session id.
fn session_index_by_id(session_terminals: &[SessionTerminal]) -> BTreeMap<String, usize> {
    session_terminals
        .iter()
        .enumerate()
        .map(|(index, entry)| (entry.session.id.clone(), index))
        .collect()
}

/// Resolves the valid session children assigned to one terminal pane.
fn children_for_pane(
    pane_id: PaneId,
    pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
    index_by_session_id: &BTreeMap<String, usize>,
) -> Vec<GroupedSessionChild> {
    pane_session_bundles
        .get(&pane_id)
        .into_iter()
        .flatten()
        .filter_map(|session_id| {
            index_by_session_id
                .get(session_id)
                .map(|session_index| GroupedSessionChild {
                    pane_id,
                    session_index: *session_index,
                })
        })
        .collect()
}
