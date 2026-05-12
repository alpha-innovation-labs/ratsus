/// Focus targets available in the terminal demo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusedPane {
    Left,
    Terminal,
}

impl FocusedPane {
    /// Returns true when keyboard input should be forwarded into the PTY.
    pub fn allows_terminal_input(self) -> bool {
        matches!(self, FocusedPane::Terminal)
    }
}

#[cfg(test)]
mod tests {
    use super::FocusedPane;

    /// Verifies that only terminal focus forwards keyboard input into the PTY.
    #[test]
    fn only_terminal_focus_allows_terminal_input() {
        assert!(!FocusedPane::Left.allows_terminal_input());
        assert!(FocusedPane::Terminal.allows_terminal_input());
    }
}
