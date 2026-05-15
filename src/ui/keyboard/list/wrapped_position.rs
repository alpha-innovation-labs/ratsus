/// Returns the wrapped list position after applying a signed movement.
pub fn wrapped_list_position(current: usize, direction: isize, item_count: usize) -> usize {
    if item_count == 0 {
        return 0;
    }
    let item_count = item_count as isize;
    (current as isize + direction).rem_euclid(item_count) as usize
}

#[cfg(test)]
mod tests {
    use super::wrapped_list_position;

    /// Moving backward from the first item wraps to the last item.
    #[test]
    fn wraps_backward_from_first_to_last() {
        assert_eq!(wrapped_list_position(0, -1, 3), 2);
    }

    /// Moving forward from the last item wraps to the first item.
    #[test]
    fn wraps_forward_from_last_to_first() {
        assert_eq!(wrapped_list_position(2, 1, 3), 0);
    }

    /// Empty lists always resolve to the zero position.
    #[test]
    fn empty_list_returns_zero() {
        assert_eq!(wrapped_list_position(7, -1, 0), 0);
    }
}
