mod support;

#[path = "session_deletion/delete_bulk_confirmation.rs"]
mod delete_bulk_confirmation;
#[path = "session_deletion/delete_cancel_preserves_catalog.rs"]
mod delete_cancel_preserves_catalog;
#[path = "session_deletion/delete_confirm_removes_sessions.rs"]
mod delete_confirm_removes_sessions;
#[path = "session_deletion/delete_focus_restoration.rs"]
mod delete_focus_restoration;
#[path = "session_deletion/delete_guard_blocks_foreign_nexus_id.rs"]
mod delete_guard_blocks_foreign_nexus_id;
#[path = "session_deletion/delete_owned_nexus_bulk.rs"]
mod delete_owned_nexus_bulk;
#[path = "session_deletion/delete_owned_nexus_single.rs"]
mod delete_owned_nexus_single;
#[path = "session_deletion/delete_single_confirmation.rs"]
mod delete_single_confirmation;
