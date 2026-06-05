use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders};

const CONVERSATION_PICKER_WIDTH_PERCENT: f32 = 0.72;
const CONVERSATION_PICKER_HEIGHT_PERCENT: f32 = 0.68;
const CONVERSATION_PICKER_HORIZONTAL_PADDING: u16 = 2;
const CONVERSATION_PICKER_VERTICAL_PADDING: u16 = 1;

/// Returns the body area used by the conversation picker dialog for a frame area.
pub fn conversation_picker_dialog_body_area(frame_area: Rect) -> Rect {
    let dialog_area = conversation_picker_dialog_area(frame_area);
    let inner = Block::default().borders(Borders::ALL).inner(dialog_area);
    inset_rect(
        inner,
        CONVERSATION_PICKER_HORIZONTAL_PADDING,
        CONVERSATION_PICKER_VERTICAL_PADDING,
    )
}

/// Returns the full dialog area used by Ratkit's centered percentage layout.
pub fn conversation_picker_dialog_area(area: Rect) -> Rect {
    let width = (area.width as f32 * CONVERSATION_PICKER_WIDTH_PERCENT) as u16;
    let height = (area.height as f32 * CONVERSATION_PICKER_HEIGHT_PERCENT) as u16;
    Rect::new(
        area.x + area.width.saturating_sub(width) / 2,
        area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    )
}

/// Insets a rectangle by horizontal and vertical padding without underflow.
fn inset_rect(area: Rect, horizontal: u16, vertical: u16) -> Rect {
    Rect::new(
        area.x.saturating_add(horizontal),
        area.y.saturating_add(vertical),
        area.width.saturating_sub(horizontal.saturating_mul(2)),
        area.height.saturating_sub(vertical.saturating_mul(2)),
    )
}
