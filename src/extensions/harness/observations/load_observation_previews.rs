use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io;

use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;
use crate::extensions::expo::observations::preview_request::ObservationPreviewRequest;
use crate::extensions::harness::observations::nexus_observations_dir::nexus_observations_dir;
use crate::extensions::harness::observations::parse_observation_preview::parse_observation_preview;

/// Loads observation previews for requested conversations by scanning observations once.
pub fn load_observation_previews(
    requests: impl IntoIterator<Item = ObservationPreviewRequest>,
) -> io::Result<HashMap<String, ConversationObservationPreview>> {
    let requests_by_id = requests_by_id(requests);
    if requests_by_id.is_empty() {
        return Ok(HashMap::new());
    }
    let paths_by_id = observation_state_paths_by_id(&requests_by_id.keys().cloned().collect())?;
    let mut previews = HashMap::new();
    for (conversation_id, path) in paths_by_id {
        let Some(request) = requests_by_id.get(&conversation_id) else {
            continue;
        };
        let input = fs::read_to_string(path)?;
        if let Ok(preview) = parse_observation_preview(&input, &request.title) {
            previews.insert(conversation_id, preview);
        }
    }
    Ok(previews)
}

/// Builds request lookup values keyed by UUID conversation id.
fn requests_by_id(
    requests: impl IntoIterator<Item = ObservationPreviewRequest>,
) -> BTreeMap<String, ObservationPreviewRequest> {
    requests
        .into_iter()
        .map(|request| (request.conversation_id.clone(), request))
        .collect()
}

/// Returns matching observation JSON paths keyed by requested conversation id.
fn observation_state_paths_by_id(
    wanted: &BTreeSet<String>,
) -> io::Result<BTreeMap<String, std::path::PathBuf>> {
    let mut paths = BTreeMap::new();
    for entry in fs::read_dir(nexus_observations_dir())? {
        let path = entry?.path();
        let Some(conversation_ids) = observation_conversation_ids(&path) else {
            continue;
        };
        for conversation_id in conversation_ids {
            if wanted.contains(&conversation_id) {
                paths.insert(conversation_id, path.clone());
            }
        }
    }
    Ok(paths)
}

/// Extracts possible session ids from a consolidated observation JSON path.
fn observation_conversation_ids(path: &std::path::Path) -> Option<Vec<String>> {
    let name = path.file_name()?.to_str()?;
    let stem = name.strip_suffix(".json")?;
    let mut ids = vec![stem.to_string()];
    if let Some((_, uuid)) = stem.rsplit_once('_') {
        ids.push(uuid.to_string());
    }
    Some(ids)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::observation_conversation_ids;

    /// Verifies timestamp-prefixed observation JSON names expose the UUID session id.
    #[test]
    fn extracts_conversation_id_from_json_name() {
        assert_eq!(
            observation_conversation_ids(Path::new("2026-04-20T15-18-32-751Z_abc.json"))
                .unwrap()
                .get(1),
            Some(&"abc".to_string())
        );
    }

    /// Verifies timestamp-prefixed observation JSON names expose the full session id.
    #[test]
    fn extracts_full_conversation_id_from_json_name() {
        assert_eq!(
            observation_conversation_ids(Path::new("2026-04-20T15-18-32-751Z_abc.json"))
                .unwrap()
                .first(),
            Some(&"2026-04-20T15-18-32-751Z_abc".to_string())
        );
    }
}
