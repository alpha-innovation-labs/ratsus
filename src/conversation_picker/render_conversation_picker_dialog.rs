use ratatui::layout::Alignment;
use ratatui::style::Color;
use ratatui::Frame;
use ratkit::primitives::dialog::{Dialog, DialogModalMode, DialogWidget};

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_body::ConversationPickerBody;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;

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
        app.conversation_picker.folder_filter.as_deref(),
    );
    let body = ConversationPickerBody::new(
        app.conversation_picker.query.clone(),
        items,
        app.conversation_picker.selected_position,
    );
    let mut dialog = Dialog::new(" conversations ", "")
        .width_percent(0.72)
        .height_percent(0.68)
        .buttons(vec![])
        .content_padding(2, 1)
        .message_alignment(Alignment::Left)
        .no_backdrop()
        .modal_mode(DialogModalMode::Blocking)
        .border_color(Color::Cyan)
        .body_renderer(Box::new(body));

    frame.render_widget(DialogWidget::new(&mut dialog), frame.area());
}
