use crate::app::nexus_demo_state::NexusDemo;
use crate::app::open_delete_session_confirmation::open_delete_session_confirmation;
use crate::conversation_picker::open_conversation_picker::open_conversation_picker;
use crate::keyboard::list_key_behavior::ListKeyBehavior;
use crate::left_panel::activate_focused_left_row::activate_focused_left_row;
use crate::left_panel::collapse_focused_project::collapse_focused_project;
use crate::left_panel::focus_left_panel_end::focus_left_panel_end;
use crate::left_panel::focus_left_panel_start::focus_left_panel_start;
use crate::left_panel::open_focused_project::open_focused_project;
use crate::left_panel::toggle_focused_left_conversation_selection::toggle_focused_left_conversation_selection;

/// Adapts the left session pane to the shared list keyboard contract.
pub struct LeftPanelKeyBehavior<'a> {
    app: &'a mut NexusDemo,
}

impl<'a> LeftPanelKeyBehavior<'a> {
    /// Creates a shared-key adapter for the left session pane.
    pub fn new(app: &'a mut NexusDemo) -> Self {
        Self { app }
    }
}

impl ListKeyBehavior for LeftPanelKeyBehavior<'_> {
    /// Moves left-pane focus by a signed row delta.
    fn move_selection(&mut self, direction: isize) {
        self.app.select_relative_session(direction);
    }

    /// Moves left-pane focus to the first visible row.
    fn focus_first(&mut self) {
        focus_left_panel_start(self.app);
    }

    /// Moves left-pane focus to the last visible row.
    fn focus_last(&mut self) {
        focus_left_panel_end(self.app);
    }

    /// Activates the focused left-pane row.
    fn activate_selection(&mut self) {
        activate_focused_left_row(self.app);
    }

    /// Collapses the focused project folder.
    fn collapse_selection(&mut self) {
        collapse_focused_project(self.app);
    }

    /// Opens the focused project folder.
    fn open_selection(&mut self) {
        open_focused_project(self.app);
    }

    /// Opens delete confirmation for the focused session row.
    fn delete_selection(&mut self) {
        open_delete_session_confirmation(self.app);
    }

    /// Toggles bulk-selection state for the focused session row.
    fn toggle_selection(&mut self) {
        toggle_focused_left_conversation_selection(self.app);
    }

    /// Opens the conversation picker from the left pane.
    fn start_filtering(&mut self) {
        open_conversation_picker(self.app);
    }

    /// Returns whether the left pane is waiting for a second `g`.
    fn has_pending_g(&self) -> bool {
        self.app.pending_left_g
    }

    /// Updates whether the left pane is waiting for a second `g`.
    fn set_pending_g(&mut self, pending: bool) {
        self.app.pending_left_g = pending;
    }
}
