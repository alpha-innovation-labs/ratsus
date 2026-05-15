use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::ui::left_panel::render::text_width::left_panel_text_width;
use crate::ui::left_panel::render::truncate_text_to_width::truncate_text_to_width;

/// Builds visible plan list lines for the shared left pane.
pub fn plan_lines(state: &PlanListState) -> Vec<Line<'static>> {
    if state.plans.is_empty() {
        return vec![Line::from("No markdown plans in ./plans")];
    }
    let visible = state.visible_indices();
    if visible.is_empty() {
        return vec![Line::from(format!(
            "No plans matching {}",
            state.filter_query
        ))];
    }
    let height = usize::from(state.last_area.height);
    let width = left_panel_text_width(state.last_area, visible.len());
    visible
        .iter()
        .skip(state.scroll)
        .take(height)
        .enumerate()
        .map(|(offset, plan_index)| plan_line(state, *plan_index, state.scroll + offset, width))
        .collect()
}

/// Builds one styled visible row for a Markdown plan.
fn plan_line(
    state: &PlanListState,
    plan_index: usize,
    visible_row: usize,
    width: u16,
) -> Line<'static> {
    let plan = &state.plans[plan_index];
    let selected = visible_row == state.focused_row;
    let active = state.active_index == Some(plan_index);
    let dragging = state
        .drag
        .is_some_and(|drag| drag.current_index == plan_index);
    let title = truncate_text_to_width(&plan.title, usize::from(width.saturating_sub(3)));
    let style = row_style(selected, active, dragging);
    Line::from(vec![Span::styled("󰈙 ", style), Span::styled(title, style)])
}

/// Returns the display style for one plan row state.
fn row_style(selected: bool, active: bool, dragging: bool) -> Style {
    let mut style = Style::default().fg(Color::White);
    if active {
        style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
    }
    if selected {
        style = style.bg(Color::DarkGray);
    }
    if dragging {
        style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
    }
    style
}
