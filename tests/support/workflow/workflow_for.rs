/// Describes the high-level real application workflow under test.
pub fn workflow_for(domain: &str, name: &str) -> &'static str {
    if domain.contains("render") {
        "real Nexus terminal rendering workflow"
    } else if domain.contains("keyboard") {
        "real Nexus keyboard input workflow"
    } else if domain.contains("mouse") {
        "real Nexus mouse input workflow"
    } else if domain.contains("harness") {
        "real Nexus harness backend workflow"
    } else {
        let _ = name;
        "real Nexus full-app workflow"
    }
}
