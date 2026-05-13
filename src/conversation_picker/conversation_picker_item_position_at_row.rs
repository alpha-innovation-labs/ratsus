use ratatui::layout::Rect;

use crate::conversation_picker::conversation_picker_item::ConversationPickerItem;
use crate::conversation_picker::conversation_picker_start_index::conversation_picker_start_index;

const CONVERSATION_PICKER_HEADER_HEIGHT: u16 = 2;
const CONVERSATION_PICKER_FOOTER_HEIGHT: u16 = 1;

/// Resolves a mouse row inside the picker body to a visible picker item position.
pub fn conversation_picker_item_position_at_row(
    body_area: Rect,
    row: u16,
    selected_position: usize,
    items: &[ConversationPickerItem],
) -> Option<usize> {
    let list_area = conversation_picker_list_area(body_area)?;
    if row < list_area.y || row >= list_area.y.saturating_add(list_area.height) {
        return None;
    }
    let list_height = usize::from(list_area.height);
    let start = conversation_picker_start_index(selected_position, items.len(), list_height);
    let position = start.saturating_add(usize::from(row - list_area.y));
    (position < items.len()).then_some(position)
}

/// Returns the body sub-area occupied by result rows.
fn conversation_picker_list_area(body_area: Rect) -> Option<Rect> {
    let list_height = body_area
        .height
        .checked_sub(CONVERSATION_PICKER_HEADER_HEIGHT)?
        .checked_sub(CONVERSATION_PICKER_FOOTER_HEIGHT)?;
    Some(Rect::new(
        body_area.x,
        body_area
            .y
            .saturating_add(CONVERSATION_PICKER_HEADER_HEIGHT),
        body_area.width,
        list_height,
    ))
}
