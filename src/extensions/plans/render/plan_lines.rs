use ratatui::text::Line;

use crate::extensions::plans::data::plan_list_row::PlanListRow;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::ui::left_panel::render::session_row_line::{
    folder_row_line, session_row_line, ChatSessionRowLineConfig, FolderRowLineConfig,
};
use crate::ui::left_panel::render::text_width::left_panel_text_width;

/// Builds visible grouped plan-tree lines for the shared left pane.
pub fn plan_lines(state: &PlanListState) -> Vec<Line<'static>> {
    let rows = state.visible_rows();
    if rows.is_empty() {
        if state.filter_query.is_empty() {
            return vec![Line::from("No workspace plan folders")];
        }
        return vec![Line::from(format!(
            "No plans matching {}",
            state.filter_query
        ))];
    }
    let height = usize::from(state.last_area.height);
    let width = left_panel_text_width(state.last_area, rows.len());
    rows.iter()
        .skip(state.scroll)
        .take(height)
        .enumerate()
        .map(|(offset, row)| plan_tree_line(state, row, state.scroll + offset, width))
        .collect()
}

/// Builds one visible grouped plan-tree row.
fn plan_tree_line(
    state: &PlanListState,
    row: &PlanListRow,
    row_index: usize,
    width: u16,
) -> Line<'static> {
    match row {
        PlanListRow::Folder {
            path,
            current_plan_count,
            total_plan_count,
        } => folder_row_line(FolderRowLineConfig {
            path,
            current_session_count: *current_plan_count,
            total_session_count: *total_plan_count,
            is_collapsed: state.collapsed_folders.contains(path),
            has_running_session: false,
            loader_tick: 0,
            width,
            show_full_path: false,
            is_active_expo_folder: false,
            is_selected: row_index == state.focused_row,
            is_dragging: false,
        }),
        PlanListRow::Plan { index } => plan_row_line(state, *index, row_index, width),
    }
}

/// Builds one Markdown plan row using the same visual contract as session rows.
fn plan_row_line(
    state: &PlanListState,
    index: usize,
    row_index: usize,
    width: u16,
) -> Line<'static> {
    let plan = &state.plans[index];
    session_row_line(ChatSessionRowLineConfig {
        title: &plan.title,
        age: "",
        icon: "󰈙",
        width,
        is_selected: row_index == state.focused_row,
        is_active: state.active_index == Some(index),
        is_running: false,
        is_completed_unseen: false,
        is_dragging: state.drag.is_some_and(|drag| drag.current_index == index),
        is_toggled: false,
        bundle_marker: None,
        tree_prefix: None,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use ratatui::layout::Rect;
    use ratatui::style::{Color, Modifier};

    use super::plan_lines;
    use crate::extensions::plans::data::plan_entry::PlanEntry;
    use crate::extensions::plans::data::plan_list_state::PlanListState;

    /// Builds a plan list state for deterministic render styling tests.
    fn plan_state() -> PlanListState {
        let folder = PathBuf::from("/workspace/plans");
        PlanListState {
            root_path: PathBuf::from("/workspace"),
            plans_dir: folder.clone(),
            workspace_folders: vec![folder.clone()],
            plans: vec![PlanEntry::new(
                folder.join("alpha.md"),
                folder.clone(),
                "Alpha".to_string(),
            )],
            collapsed_folders: BTreeSet::new(),
            folder_order: vec![folder],
            focused_row: 1,
            active_index: None,
            scroll: 0,
            filter_query: String::new(),
            filtering: false,
            pending_g: false,
            drag: None,
            last_area: Rect::new(0, 0, 40, 5),
            preview_state: None,
            selected_plan_watcher: None,
        }
    }

    /// Verifies selected plan rows match session rows by using foreground emphasis only.
    #[test]
    fn selected_plan_has_no_background_color() {
        let lines = plan_lines(&plan_state());

        let selected_line = lines.get(1).expect("selected plan line");
        for span in &selected_line.spans {
            assert_eq!(span.style.bg, None);
            if span.content.trim().is_empty() {
                continue;
            }
            assert_eq!(span.style.fg, Some(Color::Red));
            assert!(span.style.add_modifier.contains(Modifier::BOLD));
        }
    }

    /// Verifies plans render below the same folder rows used by sessions.
    #[test]
    fn plans_render_as_grouped_folder_tree() {
        let texts = plan_lines(&plan_state())
            .iter()
            .map(|line| {
                line.spans
                    .iter()
                    .map(|span| span.content.as_ref())
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        assert!(texts[0].contains("plans"));
        assert!(texts[1].contains("Alpha"));
    }
}
