//! Backend-neutral chat session refresh and creation workflow helpers.

pub mod drain_session_refreshes;
pub mod focused_chat_session_working_dir;
pub mod new_chat_insert_index;
pub mod promote_new_chat_folder;
pub mod spawn_session_refresh_worker;
pub mod start_new_chat;
pub mod start_new_chat_in_dir;
