use anyhow::ensure;
use ratsus::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState;
use ratsus::app::state::app_state::AppState;

/// Opens a visible delete confirmation dialog for deletion scenarios.
pub fn open_delete_dialog(app: &mut AppState) -> anyhow::Result<()> {
    ensure!(
        !app.session_terminals.is_empty(),
        "real Nexus catalog must have a session for delete dialog scenarios"
    );
    let first = app.session_terminals[0].session.clone();
    app.delete_confirmation = DeleteSessionConfirmationState::default();
    app.delete_confirmation.open(0, first.id, first.title);
    Ok(())
}
