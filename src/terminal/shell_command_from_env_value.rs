/// Returns the shell command from an optional environment value.
pub fn shell_command_from_env_value(shell: Option<String>) -> String {
    shell
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "/bin/sh".to_string())
}

#[cfg(test)]
mod tests {
    use super::shell_command_from_env_value;

    /// Empty or missing shell values fall back to `/bin/sh`.
    #[test]
    fn falls_back_for_empty_shell_values() {
        assert_eq!(shell_command_from_env_value(None), "/bin/sh");
        assert_eq!(shell_command_from_env_value(Some("".into())), "/bin/sh");
    }

    /// A present shell value is used directly.
    #[test]
    fn uses_present_shell_value() {
        assert_eq!(
            shell_command_from_env_value(Some("/bin/zsh".into())),
            "/bin/zsh"
        );
    }
}
