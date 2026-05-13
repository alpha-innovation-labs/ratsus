use std::path::PathBuf;

/// Moves a folder path before the target path in the saved folder order.
pub fn move_folder_order(order: &mut Vec<PathBuf>, from: &PathBuf, to: &PathBuf) -> bool {
    if from == to {
        return false;
    }
    let Some(from_index) = order.iter().position(|path| path == from) else {
        return false;
    };
    let Some(to_index) = order.iter().position(|path| path == to) else {
        return false;
    };
    let folder = order.remove(from_index);
    let adjusted_to = if from_index < to_index {
        to_index - 1
    } else {
        to_index
    };
    order.insert(adjusted_to, folder);
    true
}
