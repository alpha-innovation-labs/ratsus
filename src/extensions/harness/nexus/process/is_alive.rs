/// Returns true when the operating system reports a process id as alive.
pub fn process_is_alive(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    let signal_result = unsafe { libc::kill(pid as libc::pid_t, 0) };
    signal_result == 0
}

#[cfg(test)]
mod tests {
    use super::process_is_alive;

    /// Verifies the current test process is reported as alive without spawning a command.
    #[test]
    fn current_process_is_alive() {
        assert!(process_is_alive(std::process::id()));
    }

    /// Verifies pid zero is not treated as an application process.
    #[test]
    fn zero_pid_is_not_alive() {
        assert!(!process_is_alive(0));
    }
}
