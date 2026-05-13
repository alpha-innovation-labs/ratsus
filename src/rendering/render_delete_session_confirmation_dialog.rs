use ratatui::layout::Alignment;
use ratatui::Frame;
use ratkit::primitives::dialog::{Dialog, DialogModalMode, DialogWidget};

use crate::app::app_state::AppState;
use crate::app::delete_session_confirmation_message::delete_session_confirmation_message;
use crate::left_panel::running_session_indicator::running_session_indicator;
use crate::rendering::default_border_color::default_border_color;

/// Renders the delete confirmation modal when sessions are pending deletion.
pub fn render_delete_session_confirmation_dialog(app: &AppState, frame: &mut Frame) {
    if !app.delete_confirmation.is_open() {
        return;
    }
    let message = delete_session_confirmation_message(app);
    let deleting_footer = format!("Deleting {}", running_session_indicator(app.loader_tick));
    let mut dialog = if app.delete_confirmation.is_deleting {
        deleting_delete_dialog(&message, &deleting_footer)
    } else {
        pending_delete_dialog(&message)
    };
    frame.render_widget(DialogWidget::new(&mut dialog), frame.area());
}

/// Builds the delete dialog while deletion is running.
fn deleting_delete_dialog<'a>(message: &'a str, footer: &'a str) -> Dialog<'a> {
    base_delete_dialog(message).footer(footer)
}

/// Builds the delete dialog while waiting for confirmation.
fn pending_delete_dialog(message: &str) -> Dialog<'_> {
    base_delete_dialog(message).footer("Enter/y: Yes    Esc/n: No")
}

/// Builds the shared delete dialog shell.
fn base_delete_dialog(message: &str) -> Dialog<'_> {
    Dialog::new(" confirm delete ", message)
        .width_percent(0.48)
        .height_percent(0.34)
        .content_padding(2, 1)
        .message_alignment(Alignment::Left)
        .no_backdrop()
        .modal_mode(DialogModalMode::Blocking)
        .border_color(default_border_color())
}
