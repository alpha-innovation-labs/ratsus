use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::session_list_row::SessionListRow;

impl NexusDemo {
    /// Returns the currently visible left-panel tree rows from the row cache.
    pub fn visible_rows(&self) -> Vec<SessionListRow> {
        self.visible_rows_cache
            .borrow_mut()
            .rows(
                &self.session_terminals,
                &self.collapsed_folders,
                &self.folder_order,
                Some(self.active_index),
            )
            .to_vec()
    }

    /// Returns the currently visible left-panel tree row count from the row cache.
    pub fn visible_row_count(&self) -> usize {
        self.visible_rows_cache.borrow_mut().row_count(
            &self.session_terminals,
            &self.collapsed_folders,
            &self.folder_order,
            Some(self.active_index),
        )
    }

    /// Returns how many times the visible row cache has rebuilt rows in this test app.
    #[cfg(test)]
    pub fn visible_rows_rebuild_count(&self) -> usize {
        self.visible_rows_cache.borrow().rebuild_count()
    }
}
