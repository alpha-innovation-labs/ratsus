//! Left session panel rows, grouping, scrolling, and drag handling.

pub mod activate_focused_left_row;
pub mod apply_session_id_order;
pub mod clamp_visible_offset;
pub mod collapse_focused_project;
pub mod current_session_order_preferences;
pub mod focus_adjacent_folder;
pub mod focus_left_panel_end;
pub mod focus_left_panel_row;
pub mod focus_left_panel_start;
pub mod focused_left_row;
pub mod folder_blue_color;
pub mod folder_click_hits_label;
pub mod folder_compact_display_name;
pub mod folder_display_name;
pub mod folder_has_running_session;
pub mod folder_icon;
pub mod folder_row_index_after;
pub mod format_session_age;
pub mod handle_session_drag_mouse;
pub mod left_panel_hotkey_footer;
pub mod left_panel_key_behavior;
#[cfg(test)]
pub mod left_panel_mouse_lag_regression_tests;
#[cfg(test)]
pub mod left_panel_scroll_regression_tests;
pub mod left_panel_scroll_state;
pub mod left_panel_scrollbar_area;
pub mod left_panel_scrollbar_config;
#[cfg(test)]
pub mod left_panel_scrollbar_render_tests;
pub mod left_panel_text_width;
pub mod load_session_order_preferences;
pub mod move_folder_order;
pub mod open_focused_project;
pub mod persist_session_order_preferences;
pub mod pinned_left_panel_rows;
pub mod pinned_running_left_panel_rows;
pub mod render_left_panel_scrollbar;
pub mod rendered_left_panel_row;
pub mod rendered_left_panel_rows;
pub mod reordered_index_after_move;
pub mod running_session_indicator;
pub mod save_session_order_preferences;
pub mod scroll_left_panel_view;
pub mod session_created_timestamp;
pub mod session_drag_state;
pub mod session_folder_order;
pub mod session_icon;
pub mod session_index_for_click;
pub mod session_is_active_chat_row;
pub mod session_is_recent;
pub mod session_lines;
pub mod session_list_row;
pub mod session_order_preferences;
pub mod session_order_preferences_path;
pub mod session_row_for_click;
pub mod session_row_for_rendered_click;
pub mod session_row_line;
#[cfg(test)]
mod session_row_line_tests;
pub mod session_title_color;
pub mod session_visible_row_index;
pub mod should_focus_left_pane_for_mouse;
pub mod should_toggle_folder_on_drop;
pub mod sort_sessions_by_creation_date;
pub mod sync_folder_order;
pub mod toggle_focused_left_conversation_selection;
pub mod toggle_session_folder;
pub mod truncate_folder_path_to_width;
pub mod truncate_text_to_width;
pub mod visible_session_rows;
pub mod visible_session_rows_cache;
#[cfg(test)]
mod visible_session_rows_tests;
