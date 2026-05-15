use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::core::rendering::style::apply_cursor_style::apply_cursor_style;
use crate::extensions::terminal::copy_mode::render::render_screen_with_selection::render_screen_with_selection;
use crate::extensions::terminal::copy_mode::selection::is_selection_active::is_terminal_copy_selection_active;
use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
use crate::ui::grid_layout::pane::session_index_for_pane::session_index_for_pane;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Renders the session assigned to a terminal pane into a visible pane area.
pub fn render_pane_session(app: &AppState, frame: &mut Frame, pane_id: PaneId, area: Rect) {
    let Some(index) = session_index_for_pane(app, pane_id) else {
        frame.render_widget(Paragraph::new("No session assigned"), area);
        return;
    };
    let Some(entry) = app.session_terminals.get(index) else {
        frame.render_widget(Paragraph::new("Session unavailable"), area);
        return;
    };
    if is_terminal_copy_selection_active(&entry.copy_selection) {
        if let Some(snapshot) = entry.copy_selection.snapshot.as_ref() {
            render_screen_with_selection(snapshot, &entry.copy_selection, area, frame.buffer_mut());
        }
    } else if let Some(terminal) = entry.terminal.as_ref() {
        render_terminal(app, frame, pane_id, terminal, area);
    } else {
        frame.render_widget(Paragraph::new("Starting chat session…"), area);
    }
}

/// Renders a live terminal and places the cursor when this pane is focused.
fn render_terminal(
    app: &AppState,
    frame: &mut Frame,
    pane_id: PaneId,
    terminal: &ChatTerminal,
    area: Rect,
) {
    if !terminal.render_to_buffer(area, frame.buffer_mut()) {
        terminal.render(frame, area);
    }
    if app.focused_pane != FocusedPane::Terminal || pane_id != app.active_terminal_pane_id {
        return;
    }
    let Some(cursor) = terminal.cursor_state() else {
        return;
    };
    if cursor.col >= area.width || cursor.row >= area.height {
        return;
    }
    frame.set_cursor_position((area.x + cursor.col, area.y + cursor.row));
    apply_cursor_style(cursor.style);
}
