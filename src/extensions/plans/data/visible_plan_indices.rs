use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Returns plan indexes visible under the current filter query.
    pub fn visible_indices(&self) -> Vec<usize> {
        let query = self.filter_query.to_ascii_lowercase();
        self.plans
            .iter()
            .enumerate()
            .filter(|(_, plan)| plan_matches_filter(plan, &query))
            .map(|(index, _)| index)
            .collect()
    }
}

/// Returns true when a plan matches the already-normalized query.
fn plan_matches_filter(plan: &PlanEntry, query: &str) -> bool {
    query.is_empty() || plan.title.to_ascii_lowercase().contains(query)
}
