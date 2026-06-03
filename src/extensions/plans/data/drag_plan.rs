use crate::extensions::plans::data::plan_drag_state::PlanDragState;
use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Starts dragging a plan by source index.
    pub fn start_drag(&mut self, index: usize) {
        if index >= self.plans.len() {
            return;
        }
        self.drag = Some(PlanDragState::new(index));
        self.active_index = Some(index);
        self.focus_row_for_plan_index(index);
    }

    /// Moves the dragged plan before the target plan index.
    pub fn move_dragged_plan(&mut self, target_index: usize) {
        let Some(drag) = self.drag else {
            return;
        };
        if target_index >= self.plans.len() || target_index == drag.current_index {
            return;
        }
        let plan = self.plans.remove(drag.current_index);
        let adjusted_target = adjusted_drop_index(drag.current_index, target_index);
        self.plans.insert(adjusted_target, plan);
        self.drag = Some(PlanDragState {
            source_index: drag.source_index,
            current_index: adjusted_target,
        });
        self.active_index = Some(adjusted_target);
        self.focus_row_for_plan_index(adjusted_target);
    }

    /// Reorders the focused visible plan by a signed row delta.
    pub fn reorder_focused_by(&mut self, direction: isize) {
        let rows = self.visible_rows();
        let Some(source_index) = self.focused_plan_index() else {
            return;
        };
        let target_row = self
            .focused_row
            .saturating_add_signed(direction)
            .min(rows.len().saturating_sub(1));
        let Some(target_index) = plan_index_at_or_near_row(&rows, target_row, direction) else {
            return;
        };
        self.drag = Some(PlanDragState::new(source_index));
        self.move_dragged_plan(target_index);
        self.drag = None;
    }

    /// Ends any active drag operation.
    pub fn finish_drag(&mut self) {
        self.drag = None;
    }
}

/// Returns the nearest plan index to a grouped row for keyboard reordering.
fn plan_index_at_or_near_row(
    rows: &[crate::extensions::plans::data::plan_list_row::PlanListRow],
    row: usize,
    direction: isize,
) -> Option<usize> {
    rows.get(row).and_then(|candidate| match candidate {
        crate::extensions::plans::data::plan_list_row::PlanListRow::Plan { index } => Some(*index),
        crate::extensions::plans::data::plan_list_row::PlanListRow::Folder { .. } => {
            nearest_plan_index(rows, row, direction)
        }
    })
}

/// Searches away from a grouped folder row for the next reorderable plan row.
fn nearest_plan_index(
    rows: &[crate::extensions::plans::data::plan_list_row::PlanListRow],
    row: usize,
    direction: isize,
) -> Option<usize> {
    if direction < 0 {
        return rows[..row].iter().rev().find_map(plan_index_for_row);
    }
    rows.iter().skip(row + 1).find_map(plan_index_for_row)
}

/// Returns the plan index represented by one grouped row.
fn plan_index_for_row(
    row: &crate::extensions::plans::data::plan_list_row::PlanListRow,
) -> Option<usize> {
    match row {
        crate::extensions::plans::data::plan_list_row::PlanListRow::Plan { index } => Some(*index),
        crate::extensions::plans::data::plan_list_row::PlanListRow::Folder { .. } => None,
    }
}

/// Returns the insertion index after removing the dragged item.
fn adjusted_drop_index(source_index: usize, target_index: usize) -> usize {
    if source_index < target_index {
        target_index - 1
    } else {
        target_index
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::extensions::plans::data::plan_list_state::PlanListState;

    /// Drag reordering moves the source plan before the target plan.
    #[test]
    fn drag_reorders_plans() -> anyhow::Result<()> {
        let root = test_root("drag");
        let plans_dir = root.join("plans");
        fs::create_dir_all(&plans_dir)?;
        fs::write(plans_dir.join("alpha.md"), "# Alpha")?;
        fs::write(plans_dir.join("beta.md"), "# Beta")?;
        fs::write(plans_dir.join("gamma.md"), "# Gamma")?;
        let mut state = PlanListState::with_root(root.clone())?;

        state.start_drag(0);
        state.move_dragged_plan(2);
        state.finish_drag();

        assert_eq!(
            state
                .plans
                .iter()
                .map(|plan| plan.title.as_str())
                .collect::<Vec<_>>(),
            vec!["beta", "alpha", "gamma"]
        );
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Builds a unique test root in the system temporary directory.
    fn test_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("ratsus-plan-drag-{label}-{}", uuid::Uuid::new_v4()))
    }
}
