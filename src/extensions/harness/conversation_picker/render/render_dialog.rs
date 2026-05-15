use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::Frame;
use ratkit::primitives::dialog::{Dialog, DialogModalMode, DialogWidget};

use crate::app::state::app_state::AppState;
use crate::core::rendering::style::default_border_color::default_border_color;
use crate::extensions::harness::conversation_picker::data::items::conversation_picker_items;
use crate::extensions::harness::conversation_picker::data::mode::ConversationPickerMode;
use crate::extensions::harness::conversation_picker::layout::body::ConversationPickerBody;
use crate::extensions::harness::conversation_picker::layout::footer_text::conversation_picker_footer_text;
use crate::extensions::harness::conversation_picker::render::render_scope_header::render_conversation_picker_scope_header;

/// Renders the centered conversation picker modal dialog when it is open.
pub fn render_conversation_picker_dialog(app: &AppState, frame: &mut Frame) {
    if !app.conversation_picker.is_open {
        return;
    }

    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    let body = ConversationPickerBody::new(
        app.conversation_picker.query.clone(),
        items,
        app.conversation_picker.selected_position,
        app.conversation_picker.is_filtering,
        app.loader_tick,
        app.session_drag.map(|drag| drag.current_index),
    );
    let title = match app.conversation_picker.mode {
        ConversationPickerMode::Open => " conversations ",
        ConversationPickerMode::PlaceInActiveSplit(_) => " place conversation in split ",
    };
    let mut dialog = Dialog::new(title, "")
        .width_percent(0.72)
        .height_percent(0.68)
        .buttons(vec![])
        .content_padding(2, 1)
        .message_alignment(Alignment::Left)
        .footer(conversation_picker_footer_text())
        .footer_alignment(Alignment::Center)
        .footer_style(Style::default().fg(Color::DarkGray))
        .no_backdrop()
        .modal_mode(DialogModalMode::Blocking)
        .border_color(default_border_color())
        .body_renderer(Box::new(body));

    frame.render_widget(DialogWidget::new(&mut dialog), frame.area());
    render_conversation_picker_scope_header(app, frame);
}
