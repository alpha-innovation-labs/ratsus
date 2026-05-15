//! Left-panel session row data and session metadata helpers.

pub mod activation;
pub mod created_timestamp;
pub mod day_group_label;
pub mod format_age;
pub mod icon;
pub mod is_active_chat_row;
pub mod is_recent;
pub mod list_row;
pub mod running_indicator;
pub mod running_indicator_frame_changed;
pub mod sort_by_creation_date;
pub mod split_groups;
pub mod title_color;
pub mod visible_rows;
pub mod visible_rows_cache;
#[cfg(test)]
mod visible_rows_running_tests;
#[cfg(test)]
mod visible_rows_tests;
