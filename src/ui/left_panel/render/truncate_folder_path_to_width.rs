/// Truncates a folder path by preserving the rightmost location components.
pub fn truncate_folder_path_to_width(path: &str, max_width: usize) -> String {
    let char_count = path.chars().count();
    if char_count <= max_width {
        return path.to_string();
    }
    if max_width == 0 {
        return String::new();
    }
    if max_width <= 3 {
        return ".".repeat(max_width);
    }
    let tail_width = max_width - 3;
    let tail = rightmost_chars(path, tail_width);
    format!("...{}", tail)
}

/// Returns the rightmost characters from a string.
fn rightmost_chars(text: &str, width: usize) -> String {
    let mut chars = text.chars().rev().take(width).collect::<Vec<_>>();
    chars.reverse();
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::truncate_folder_path_to_width;

    /// Keeps paths that fit in the available width.
    #[test]
    fn keeps_fitting_path() {
        assert_eq!(
            truncate_folder_path_to_width("~/workspace/app", 15),
            "~/workspace/app"
        );
    }

    /// Truncates long paths with an ASCII ellipsis and right-side location suffix.
    #[test]
    fn truncates_path_with_right_suffix() {
        assert_eq!(
            truncate_folder_path_to_width("~/workspace/alpha/project", 18),
            "...e/alpha/project"
        );
    }

    /// Handles very narrow widths with a clipped ellipsis.
    #[test]
    fn handles_narrow_width() {
        assert_eq!(truncate_folder_path_to_width("~/workspace", 2), "..");
    }
}
