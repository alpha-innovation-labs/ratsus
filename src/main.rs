use anyhow::Result;
use ratatui::Frame;
use ratkit::{
    run_with_diagnostics, CoordinatorAction, CoordinatorApp, CoordinatorEvent, RunnerConfig,
};

#[path = "nexus/apply_cursor_style.rs"]
mod apply_cursor_style;
#[path = "nexus/apply_session_refresh.rs"]
mod apply_session_refresh;
#[path = "nexus/clamp_visible_offset.rs"]
mod clamp_visible_offset;
#[path = "nexus/clear_terminal_copy_selection.rs"]
mod clear_terminal_copy_selection;
#[path = "nexus/copy_text_to_clipboard.rs"]
mod copy_text_to_clipboard;
#[path = "nexus/drain_session_refreshes.rs"]
mod drain_session_refreshes;
#[path = "nexus/encode_key_event.rs"]
mod encode_key_event;
#[path = "nexus/finish_terminal_copy_selection.rs"]
mod finish_terminal_copy_selection;
#[path = "nexus/focused_nexus_session_working_dir.rs"]
mod focused_nexus_session_working_dir;
#[path = "nexus/focused_pane.rs"]
mod focused_pane;
#[path = "nexus/focused_pane_after_left_pane_toggle.rs"]
mod focused_pane_after_left_pane_toggle;
#[path = "nexus/folder_display_name.rs"]
mod folder_display_name;
#[path = "nexus/handle_nexus_demo_event.rs"]
mod handle_nexus_demo_event;
#[path = "nexus/handle_resizable_grid_mouse.rs"]
mod handle_resizable_grid_mouse;
#[path = "nexus/handle_session_drag_mouse.rs"]
mod handle_session_drag_mouse;
#[path = "nexus/handle_terminal_copy_keyboard.rs"]
mod handle_terminal_copy_keyboard;
#[path = "nexus/handle_terminal_copy_mouse.rs"]
mod handle_terminal_copy_mouse;
#[path = "nexus/is_new_nexus_chat_session.rs"]
mod is_new_nexus_chat_session;
#[path = "nexus/is_resizing_layout.rs"]
mod is_resizing_layout;
#[path = "nexus/is_terminal_copy_selection_active.rs"]
mod is_terminal_copy_selection_active;
#[path = "nexus/load_nexus_sessions.rs"]
mod load_nexus_sessions;
#[path = "nexus/new_nexus_chat_session.rs"]
mod new_nexus_chat_session;
#[path = "nexus/nexus_demo_state.rs"]
mod nexus_demo_state;
#[path = "nexus/nexus_terminal.rs"]
mod nexus_terminal;
#[path = "nexus/pane_area_by_id.rs"]
mod pane_area_by_id;
#[path = "nexus/parse_nexus_sessions.rs"]
mod parse_nexus_sessions;
#[path = "nexus/parse_nexus_sessions_json.rs"]
mod parse_nexus_sessions_json;
#[path = "nexus/process_terminal_output.rs"]
mod process_terminal_output;
#[path = "nexus/redraw_action.rs"]
mod redraw_action;
#[path = "nexus/render_nexus_demo.rs"]
mod render_nexus_demo;
#[path = "nexus/render_resize_placeholder.rs"]
mod render_resize_placeholder;
#[path = "nexus/render_screen_with_selection.rs"]
mod render_screen_with_selection;
#[path = "nexus/reordered_index_after_move.rs"]
mod reordered_index_after_move;
#[path = "nexus/scroll_delta_for_mouse_kind.rs"]
mod scroll_delta_for_mouse_kind;
#[path = "nexus/selected_text_from_terminal_copy_selection.rs"]
mod selected_text_from_terminal_copy_selection;
#[path = "nexus/selection_bounds.rs"]
mod selection_bounds;
#[path = "nexus/selection_contains_position.rs"]
mod selection_contains_position;
#[path = "nexus/selection_position.rs"]
mod selection_position;
#[path = "nexus/selection_position_for_mouse.rs"]
mod selection_position_for_mouse;
#[path = "nexus/session_drag_state.rs"]
mod session_drag_state;
#[path = "nexus/session_folder_order.rs"]
mod session_folder_order;
#[path = "nexus/session_index_for_click.rs"]
mod session_index_for_click;
#[path = "nexus/session_info.rs"]
mod session_info;
#[path = "nexus/session_lines.rs"]
mod session_lines;
#[path = "nexus/session_list_row.rs"]
mod session_list_row;
#[path = "nexus/session_row_for_click.rs"]
mod session_row_for_click;
#[path = "nexus/session_terminal.rs"]
mod session_terminal;
#[path = "nexus/session_visible_row_index.rs"]
mod session_visible_row_index;
#[path = "nexus/should_resize_active_terminal.rs"]
mod should_resize_active_terminal;
#[path = "nexus/show_copied_to_clipboard_toast.rs"]
mod show_copied_to_clipboard_toast;
#[path = "nexus/show_failed_to_start_new_chat_toast.rs"]
mod show_failed_to_start_new_chat_toast;
#[path = "nexus/spawn_new_nexus_session_terminal.rs"]
mod spawn_new_nexus_session_terminal;
#[path = "nexus/spawn_session_refresh_worker.rs"]
mod spawn_session_refresh_worker;
#[path = "nexus/spawn_terminal_reader.rs"]
mod spawn_terminal_reader;
#[path = "nexus/split_terminal_demo_layout.rs"]
mod split_terminal_demo_layout;
#[path = "nexus/start_new_nexus_chat.rs"]
mod start_new_nexus_chat;
#[path = "nexus/start_terminal_copy_selection.rs"]
mod start_terminal_copy_selection;
#[path = "nexus/terminal_copy_selection.rs"]
mod terminal_copy_selection;
#[path = "nexus/toggle_left_pane_visibility.rs"]
mod toggle_left_pane_visibility;
#[path = "nexus/toggle_session_folder.rs"]
mod toggle_session_folder;
#[path = "nexus/update_terminal_copy_selection.rs"]
mod update_terminal_copy_selection;
#[path = "nexus/visible_session_rows.rs"]
mod visible_session_rows;
#[path = "nexus/write_terminal_replies.rs"]
mod write_terminal_replies;

use handle_nexus_demo_event::handle_nexus_demo_event;
use nexus_demo_state::NexusDemo;
use render_nexus_demo::render_nexus_demo;

impl CoordinatorApp for NexusDemo {
    fn on_event(&mut self, event: CoordinatorEvent) -> ratkit::LayoutResult<CoordinatorAction> {
        handle_nexus_demo_event(self, event)
    }

    fn on_draw(&mut self, frame: &mut Frame) {
        render_nexus_demo(self, frame);
    }
}

fn main() -> Result<()> {
    let app = NexusDemo::new()?;
    run_with_diagnostics(app, RunnerConfig::default())?;
    Ok(())
}
