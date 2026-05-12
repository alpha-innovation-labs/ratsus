use ratkit::CoordinatorAction;

/// Returns the coordinator action for a boolean redraw decision.
pub fn redraw_action(should_redraw: bool) -> CoordinatorAction {
    if should_redraw {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}
