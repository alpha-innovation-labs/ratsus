/// Returns Ghostty keybind overrides required by Ratsus shortcuts.
pub fn required_ghostty_keybind_lines() -> Vec<String> {
    let mut lines = vec![
        "keybind = ctrl+tab=csi:9;5u".to_string(),
        "keybind = ctrl+shift+tab=csi:9;6u".to_string(),
        "keybind = ctrl+`=csi:96;5u".to_string(),
        "keybind = ctrl+grave_accent=csi:96;5u".to_string(),
        "keybind = shift+tab=unbind".to_string(),
        "keybind = super+n=unbind".to_string(),
        "keybind = cmd+n=unbind".to_string(),
    ];
    for index in 1..=9 {
        lines.push(format!("keybind = super+{index}=unbind"));
        lines.push(format!("keybind = super+digit_{index}=unbind"));
        lines.push(format!("keybind = cmd+{index}=unbind"));
        lines.push(format!("keybind = cmd+digit_{index}=unbind"));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::required_ghostty_keybind_lines;

    /// Verifies Ratsus unbinds Ghostty shortcuts consumed before Ratsus can handle them.
    #[test]
    fn includes_tab_command_chat_and_command_number_unbinds() {
        let lines = required_ghostty_keybind_lines();

        assert!(lines.contains(&"keybind = ctrl+tab=csi:9;5u".to_string()));
        assert!(lines.contains(&"keybind = ctrl+shift+tab=csi:9;6u".to_string()));
        assert!(lines.contains(&"keybind = ctrl+`=csi:96;5u".to_string()));
        assert!(lines.contains(&"keybind = ctrl+grave_accent=csi:96;5u".to_string()));
        assert!(lines.contains(&"keybind = shift+tab=unbind".to_string()));
        assert!(lines.contains(&"keybind = super+n=unbind".to_string()));
        assert!(lines.contains(&"keybind = cmd+n=unbind".to_string()));
        assert!(lines.contains(&"keybind = super+1=unbind".to_string()));
        assert!(lines.contains(&"keybind = super+digit_9=unbind".to_string()));
        assert!(lines.contains(&"keybind = cmd+1=unbind".to_string()));
        assert!(lines.contains(&"keybind = cmd+digit_9=unbind".to_string()));
    }
}
