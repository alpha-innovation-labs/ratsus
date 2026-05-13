//! PTY-backed terminal sessions, input encoding, and terminal output processing.

pub mod chat_session_from_persisted_normal_terminal_session;
pub mod chat_terminal;
pub mod default_shell_command;
pub mod encode_key_event;
pub mod is_chat_session;
pub mod is_normal_terminal_session;
pub mod load_normal_terminal_sessions;
pub mod load_persisted_normal_terminal_sessions;
pub mod normal_terminal_legacy_registry_path;
pub mod normal_terminal_registry_path;
pub mod normal_terminal_session_info;
pub mod normal_terminal_sessions_from_session_terminals;
pub mod persist_normal_terminal_sessions;
pub mod persisted_normal_terminal_session;
pub mod persisted_normal_terminal_session_from_chat_session;
pub mod process_terminal_output;
pub mod pty_terminal;
pub mod save_normal_terminal_sessions;
pub mod scroll_delta_for_mouse_kind;
pub mod session_terminal;
pub mod session_terminal_has_exited;
pub mod shell_command_from_env_value;
pub mod spawn_normal_terminal_session;
pub mod spawn_terminal_reader;
pub mod stub_terminal;
pub mod write_terminal_replies;
