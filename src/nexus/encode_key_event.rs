use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

/// Encodes a coordinator keyboard event into terminal input bytes.
pub fn encode_key_event(event: &KeyboardEvent) -> Option<Vec<u8>> {
    if !event.is_key_down() {
        return None;
    }
    match event.key_code {
        KeyCode::Char(mut c) => {
            if event.modifiers.contains(KeyModifiers::CONTROL) {
                if c == 'x' {
                    return None;
                }
                c = c.to_ascii_lowercase();
                if c.is_ascii() {
                    return Some(vec![(c as u8) & 0x1f]);
                }
                return None;
            }
            if event.modifiers.contains(KeyModifiers::ALT) {
                if c.is_ascii() {
                    return Some(vec![0x1b, c as u8]);
                }
                return None;
            }
            if c.is_ascii() {
                Some(vec![c as u8])
            } else {
                Some(c.to_string().into_bytes())
            }
        }
        KeyCode::Enter => Some(vec![b'\r']),
        KeyCode::Backspace => Some(vec![0x7f]),
        KeyCode::Tab => Some(vec![b'\t']),
        KeyCode::Esc => Some(vec![0x1b]),
        KeyCode::Up => Some(b"\x1b[A".to_vec()),
        KeyCode::Down => Some(b"\x1b[B".to_vec()),
        KeyCode::Right => Some(b"\x1b[C".to_vec()),
        KeyCode::Left => Some(b"\x1b[D".to_vec()),
        _ => None,
    }
}
