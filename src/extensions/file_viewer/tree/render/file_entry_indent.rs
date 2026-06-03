/// Returns two-space indentation for a grouped workspace file-tree depth.
pub fn file_entry_indent(depth: usize) -> String {
    "  ".repeat(depth)
}

#[cfg(test)]
mod tests {
    use super::file_entry_indent;

    /// Verifies direct children under a workspace folder are visibly indented.
    #[test]
    fn indents_workspace_children() {
        assert_eq!(file_entry_indent(0), "");
        assert_eq!(file_entry_indent(1), "  ");
        assert_eq!(file_entry_indent(2), "    ");
    }
}
