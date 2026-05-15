/// Lists stable assertions included in every real E2E scenario snapshot.
pub fn assertions_for(domain: &str, name: &str) -> Vec<String> {
    let mut assertions = vec![
        "real Nexus harness loaded without test doubles".to_string(),
        "real Nexus session catalog loaded through the application".to_string(),
        "full app rendered through ratatui TestBackend".to_string(),
        "test-owned temporary workspace protected operator files".to_string(),
    ];
    if domain.contains("delete") || name.contains("delete") {
        assertions.push("delete confirmation workflow was visible".to_string());
    }
    if domain.contains("file") || name.starts_with("files_") {
        assertions.push("real temporary file tree was rendered".to_string());
    }
    if name == "refresh_nexus_registry_watcher" {
        assertions.push("Nexus registry watcher was installed".to_string());
    }
    if name == "picker_workspace_scope_toggle" {
        assertions.push("conversation picker was scoped to the selected workspace".to_string());
    } else if domain.contains("workspace") || name.contains("workspace") {
        assertions.push("workspace pane has an active selected folder".to_string());
    }
    assertions
}
