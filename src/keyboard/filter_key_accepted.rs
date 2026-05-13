use crossterm::event::KeyModifiers;

/// Returns true when a typed key can be inserted into a list filter query.
pub fn filter_key_accepted(modifiers: KeyModifiers) -> bool {
    modifiers.is_empty() || modifiers == KeyModifiers::SHIFT
}
