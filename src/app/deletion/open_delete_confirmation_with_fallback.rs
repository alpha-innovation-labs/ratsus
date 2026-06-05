use crate::app::state::app_state::AppState;

/// Opens the delete confirmation for selected targets, falling back to a
/// caller-supplied focused-row opener when no targets are selected.
pub(crate) fn open_delete_confirmation_with_fallback<F>(app: &mut AppState, fallback: F)
where
    F: FnOnce(&mut AppState),
{
    let selected = crate::app::deletion::selected_delete_targets::selected_delete_targets(app);
    if !selected.is_empty() {
        app.delete_confirmation.open_many(selected);
        return;
    }
    fallback(app);
}
