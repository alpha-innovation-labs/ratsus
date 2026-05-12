use crate::terminal::shell_command_from_env_value::shell_command_from_env_value;

/// Returns the shell executable used for normal terminal sessions.
pub fn default_shell_command() -> String {
    shell_command_from_env_value(std::env::var("SHELL").ok())
}
