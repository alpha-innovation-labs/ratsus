use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratkit::primitives::dialog::{Dialog, DialogModalMode, DialogWidget};

use crate::app::state::app_state::AppState;
use crate::core::rendering::style::default_border_color::default_border_color;
use crate::extensions::history_modal::data::items::history_modal_items;
use crate::extensions::history_modal::data::mode::HistoryModalMode;
use crate::extensions::history_modal::layout::body::HistoryModalBody;
use crate::extensions::history_modal::layout::footer_text::history_modal_footer_text;
use crate::extensions::history_modal::render::render_scope_header::render_history_modal_scope_header;

/// Renders the centered conversation picker modal dialog when it is open.
pub fn render_history_modal_dialog(app: &AppState, frame: &mut Frame) {
    if !app.history_modal.is_open {
        return;
    }

    let items = history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        &app.history_modal.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    let body = HistoryModalBody::new(
        app.history_modal.query.clone(),
        items,
        app.history_modal.selected_position,
        app.history_modal.is_filtering,
        app.loader_tick,
        app.session_drag.map(|drag| drag.current_index),
    );
    let title = match app.history_modal.mode {
        HistoryModalMode::Open => " conversations ",
        HistoryModalMode::PlaceInActiveSplit(_) => " place conversation in split ",
    };
    let mut dialog = Dialog::new(title, "")
        .width_percent(0.72)
        .height_percent(0.68)
        .buttons(vec![])
        .content_padding(2, 1)
        .message_alignment(Alignment::Left)
        .footer(history_modal_footer_text())
        .footer_alignment(Alignment::Center)
        .footer_style(Style::default().fg(Color::DarkGray))
        .no_backdrop()
        .modal_mode(DialogModalMode::Blocking)
        .border_color(default_border_color())
        .body_renderer(Box::new(body));

    frame.render_widget(DialogWidget::new(&mut dialog), frame.area());
    render_history_modal_scope_header(app, frame);
}
