//! Terminal process spawning, IO, and exit handling.

pub mod default_shell_command;
pub mod normal_terminal_shell_argv;
pub mod process_terminal_output;
pub mod pty_terminal;
pub mod session_terminal_has_exited;
pub mod shell_command_from_env_value;
pub mod spawn_normal_terminal_session;
pub mod spawn_terminal_reader;
pub mod write_terminal_replies;
