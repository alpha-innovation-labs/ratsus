use crate::app::nexus_demo_state::NexusDemo;
use crate::app::selected_delete_targets::selected_delete_targets;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::conversation_picker::selected_conversation_picker_item::selected_conversation_picker_item;

/// Opens delete confirmation for selected picker sessions or the highlighted session.
pub fn open_selected_conversation_delete_confirmation(app: &mut NexusDemo) {
    let selected_targets = selected_delete_targets(app);
    if !selected_targets.is_empty() {
        app.delete_confirmation.open_many(0, selected_targets);
        return;
    }
    open_highlighted_conversation_delete_confirmation(app);
}

/// Opens delete confirmation for the highlighted picker session.
fn open_highlighted_conversation_delete_confirmation(app: &mut NexusDemo) {
    let Some(item) = selected_conversation_picker_item(app) else {
        return;
    };
    let ConversationPickerItemKind::Session { index, .. } = item.kind else {
        return;
    };
    let Some(entry) = app.session_terminals.get(index) else {
        return;
    };
    app.delete_confirmation
        .open(index, entry.session.id.clone(), entry.session.title.clone());
}
