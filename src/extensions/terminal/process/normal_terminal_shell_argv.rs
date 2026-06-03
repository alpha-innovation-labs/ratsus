use std::ffi::OsString;
use std::path::Path;

const NORMAL_TERMINAL_SHELL_LAUNCHER: &str = "ratsus-normal-terminal";
const CD_AND_EXEC_CONFIGURED_SHELL: &str = "cd \"$1\" && exec \"$2\" -i";

/// Builds argv that puts the configured interactive shell in the requested working directory.
pub fn normal_terminal_shell_argv(shell: &str, working_dir: &Path) -> Vec<OsString> {
    if shell_is_fish(shell) {
        return fish_shell_argv(shell, working_dir);
    }
    vec![
        OsString::from("/bin/sh"),
        OsString::from("-lc"),
        OsString::from(CD_AND_EXEC_CONFIGURED_SHELL),
        OsString::from(NORMAL_TERMINAL_SHELL_LAUNCHER),
        working_dir.as_os_str().to_owned(),
        OsString::from(shell),
    ]
}

/// Returns whether the configured shell path points at fish.
fn shell_is_fish(shell: &str) -> bool {
    Path::new(shell)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name == "fish")
}

/// Builds fish argv that cd's after fish startup config has loaded.
fn fish_shell_argv(shell: &str, working_dir: &Path) -> Vec<OsString> {
    vec![
        OsString::from(shell),
        OsString::from("--init-command"),
        OsString::from(fish_cd_init_command(working_dir)),
        OsString::from("-i"),
    ]
}

/// Builds the fish startup command that changes to the requested working directory.
fn fish_cd_init_command(working_dir: &Path) -> String {
    format!("cd {}", fish_single_quoted_path(working_dir))
}

/// Quotes a path for use in a fish single-quoted string.
fn fish_single_quoted_path(working_dir: &Path) -> String {
    format!("'{}'", working_dir.to_string_lossy().replace('\'', "\\'"))
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::path::Path;

    use super::normal_terminal_shell_argv;

    /// Non-fish shells use a POSIX launcher to cd first, then exec the configured shell.
    #[test]
    fn builds_cd_then_exec_shell_argv() {
        let argv = normal_terminal_shell_argv("/bin/zsh", Path::new("/tmp/work space"));

        assert_eq!(argv[0], OsStr::new("/bin/sh"));
        assert_eq!(argv[1], OsStr::new("-lc"));
        assert_eq!(argv[2], OsStr::new("cd \"$1\" && exec \"$2\" -i"));
        assert_eq!(argv[3], OsStr::new("ratsus-normal-terminal"));
        assert_eq!(argv[4], OsStr::new("/tmp/work space"));
        assert_eq!(argv[5], OsStr::new("/bin/zsh"));
    }

    /// Fish shells use init-command so user config cannot leave the terminal in another folder.
    #[test]
    fn builds_fish_init_command_argv() {
        let argv =
            normal_terminal_shell_argv("/opt/homebrew/bin/fish", Path::new("/tmp/work space"));

        assert_eq!(argv[0], OsStr::new("/opt/homebrew/bin/fish"));
        assert_eq!(argv[1], OsStr::new("--init-command"));
        assert_eq!(argv[2], OsStr::new("cd '/tmp/work space'"));
        assert_eq!(argv[3], OsStr::new("-i"));
    }

    /// Fish cd init command escapes single quotes in workspace paths.
    #[test]
    fn escapes_fish_single_quotes() {
        let argv = normal_terminal_shell_argv("fish", Path::new("/tmp/work'space"));

        assert_eq!(argv[2], OsStr::new("cd '/tmp/work\\'space'"));
    }
}
