/// Copies selected terminal text to the system clipboard.
pub fn copy_text_to_clipboard(text: &str) {
    if text.is_empty() {
        return;
    }
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        let _ = clipboard.set_text(text.to_owned());
    }
}
