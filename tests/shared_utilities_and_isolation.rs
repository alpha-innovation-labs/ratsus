mod support;

#[path = "shared_utilities_and_isolation/cleanup_removes_test_artifacts.rs"]
mod cleanup_removes_test_artifacts;
#[path = "shared_utilities_and_isolation/isolation_temp_workdir_for_owned_nexus.rs"]
mod isolation_temp_workdir_for_owned_nexus;
#[path = "shared_utilities_and_isolation/isolation_temp_workspace_for_real.rs"]
mod isolation_temp_workspace_for_real;
#[path = "shared_utilities_and_isolation/snapshot_redaction_nexus_ids.rs"]
mod snapshot_redaction_nexus_ids;
#[path = "shared_utilities_and_isolation/snapshot_redaction_paths.rs"]
mod snapshot_redaction_paths;
