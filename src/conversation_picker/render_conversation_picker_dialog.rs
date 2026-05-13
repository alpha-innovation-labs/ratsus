use ratatui::layout::Alignment;
use ratatui::Frame;
use ratkit::primitives::dialog::{Dialog, DialogModalMode, DialogWidget};

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_body::ConversationPickerBody;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;
use crate::conversation_picker::conversation_picker_mode::ConversationPickerMode;
use crate::rendering::default_border_color::default_border_color;

/// Renders the centered conversation picker modal dialog when it is open.
pub fn render_conversation_picker_dialog(app: &NexusDemo, frame: &mut Frame) {
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
        ConversationPickerMode::PlaceInActiveSplit => " place conversation in split ",
    };
    let mut dialog = Dialog::new(title, "")
        .width_percent(0.72)
        .height_percent(0.68)
        .buttons(vec![])
        .content_padding(2, 1)
        .message_alignment(Alignment::Left)
        .no_backdrop()
        .modal_mode(DialogModalMode::Blocking)
        .border_color(default_border_color())
        .body_renderer(Box::new(body));

    frame.render_widget(DialogWidget::new(&mut dialog), frame.area());
}
