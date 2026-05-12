/// Returns where an index points after moving one list item from `from` to `to`.
pub fn reordered_index_after_move(index: usize, from: usize, to: usize) -> usize {
    if index == from {
        return to;
    }
    if from < to && index > from && index <= to {
        return index - 1;
    }
    if to < from && index >= to && index < from {
        return index + 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::reordered_index_after_move;

    /// Verifies the moved item follows its new position.
    #[test]
    fn moves_selected_index_to_target() {
        assert_eq!(reordered_index_after_move(1, 1, 3), 3);
    }

    /// Verifies items between a downward move shift upward.
    #[test]
    fn shifts_items_up_for_downward_move() {
        assert_eq!(reordered_index_after_move(3, 1, 4), 2);
    }

    /// Verifies items between an upward move shift downward.
    #[test]
    fn shifts_items_down_for_upward_move() {
        assert_eq!(reordered_index_after_move(2, 4, 1), 3);
    }

    /// Verifies unrelated indexes remain unchanged.
    #[test]
    fn leaves_unaffected_index_unchanged() {
        assert_eq!(reordered_index_after_move(5, 1, 3), 5);
    }
}
