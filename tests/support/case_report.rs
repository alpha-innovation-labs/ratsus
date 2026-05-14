/// User-observable E2E verification report snapshotted by each scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseReport {
    pub domain: String,
    pub name: String,
    pub description: String,
    pub backend: String,
    pub workflow: String,
    pub assertions: Vec<String>,
}

impl CaseReport {
    /// Renders the report as deterministic markdown for insta snapshots.
    pub fn to_snapshot(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("domain: {}\n", self.domain));
        output.push_str(&format!("test: {}\n", self.name));
        output.push_str(&format!("description: {}\n", self.description));
        output.push_str(&format!("backend: {}\n", self.backend));
        output.push_str(&format!("workflow: {}\n", self.workflow));
        output.push_str("assertions:\n");
        for assertion in &self.assertions {
            output.push_str(&format!("- {}\n", assertion));
        }
        output
    }
}
