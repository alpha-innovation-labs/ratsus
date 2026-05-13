use crate::app::state::app_state::AppState;
use crate::ui::left_panel::render::rendered_row::RenderedLeftPanelRow;
use crate::ui::left_panel::scroll::pinned_rows::pinned_left_panel_rows;
use crate::ui::left_panel::scroll::pinned_running_rows::pinned_running_left_panel_rows;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Builds the rendered left-panel rows, including pinned sections and separators.
pub fn rendered_left_panel_rows(app: &AppState) -> Vec<RenderedLeftPanelRow> {
    let rows = app.visible_rows();
    let visible_height = usize::from(app.last_session_list_area.height);
    rendered_left_panel_rows_from_rows(app, &rows, visible_height)
}

/// Builds rendered rows from a precomputed visible row list.
pub fn rendered_left_panel_rows_from_rows(
    app: &AppState,
    rows: &[SessionListRow],
    visible_height: usize,
) -> Vec<RenderedLeftPanelRow> {
    let full_end = (app.session_scroll + visible_height).min(rows.len());
    let running_pinned = pinned_running_left_panel_rows(app, rows, full_end);
    let active_end = visible_end_after_pins(app, rows.len(), visible_height, &running_pinned, &[]);
    let active_pinned = pinned_left_panel_rows(app, rows, active_end);
    let final_end = visible_end_after_pins(
        app,
        rows.len(),
        visible_height,
        &running_pinned,
        &active_pinned,
    );
    let running_pinned = pinned_running_left_panel_rows(app, rows, final_end);
    let active_end = visible_end_after_pins(app, rows.len(), visible_height, &running_pinned, &[]);
    let active_pinned = pinned_left_panel_rows(app, rows, active_end);
    let final_end = visible_end_after_pins(
        app,
        rows.len(),
        visible_height,
        &running_pinned,
        &active_pinned,
    );
    let mut rendered = Vec::new();
    append_pinned_section(&mut rendered, &running_pinned);
    append_pinned_section(&mut rendered, &active_pinned);
    append_scrolled_rows(&mut rendered, rows, app.session_scroll, final_end);
    rendered
}

/// Returns the viewport end after reserving pinned row sections.
fn visible_end_after_pins(
    app: &AppState,
    row_count: usize,
    visible_height: usize,
    running_pinned: &[(usize, SessionListRow)],
    active_pinned: &[(usize, SessionListRow)],
) -> usize {
    let reserved = pinned_section_height(running_pinned) + pinned_section_height(active_pinned);
    let list_height = visible_height.saturating_sub(reserved);
    (app.session_scroll + list_height).min(row_count)
}

/// Returns the rendered height for a pinned section including its separator.
fn pinned_section_height(rows: &[(usize, SessionListRow)]) -> usize {
    if rows.is_empty() {
        0
    } else {
        rows.len() + 1
    }
}

/// Appends one pinned section and separator.
fn append_pinned_section(
    rendered: &mut Vec<RenderedLeftPanelRow>,
    pinned: &[(usize, SessionListRow)],
) {
    if pinned.is_empty() {
        return;
    }
    rendered.extend(pinned.iter().map(|(source_row_index, row)| {
        RenderedLeftPanelRow::SessionListRow {
            source_row_index: *source_row_index,
            row: row.clone(),
        }
    }));
    rendered.push(RenderedLeftPanelRow::Separator);
}

/// Appends normal scrolled rows.
fn append_scrolled_rows(
    rendered: &mut Vec<RenderedLeftPanelRow>,
    rows: &[SessionListRow],
    start: usize,
    end: usize,
) {
    rendered.extend(rows[start..end].iter().enumerate().map(|(offset, row)| {
        RenderedLeftPanelRow::SessionListRow {
            source_row_index: start + offset,
            row: row.clone(),
        }
    }));
}

#[cfg(test)]
mod tests {
    use super::pinned_section_height;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies pinned sections reserve rows plus one separator.
    #[test]
    fn pinned_section_height_includes_separator() {
        assert_eq!(pinned_section_height(&[]), 0);
        assert_eq!(
            pinned_section_height(&[(0, SessionListRow::Session { index: 0 })]),
            2
        );
    }
}
