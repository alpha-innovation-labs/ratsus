# Harness extension

`src/extensions/harness/` owns backend-neutral chat session contracts and concrete harness integrations. It is the boundary between the UI and external or fake session providers. Harness-specific behavior belongs under the concrete harness folder.

## Responsibilities

- Define `ChatHarness`, `ChatSession`, and `ChatSessionKind` in `harness/core`.
- Load and refresh chat session metadata.
- Spawn new chats and existing chat terminals.
- Delete backend-owned chat sessions through the active concrete harness adapter.
- Provide conversation picker state, input, rendering, selection behavior, workspace/all scope toggling, and folder-local pinning of active/running chats.
- Provide observation preview loading and optional file watching.
- Provide the real Nexus harness and deterministic stub harness.

## Harness contract

`ChatHarness` exposes the operations the UI needs: display name, initial session load, lightweight refresh, new chat spawn, existing chat spawn, normal terminal policy hooks, deletion, refresh merge semantics, observation preview loading, observation watcher startup, and observation path matching. Watcher-triggered session refreshes are scheduled on a background worker and drained from the tick loop when ready.

`ChatSession` stores display and routing metadata: modified date, created date, title, id, working directory, running state, and session kind.

## Nexus harness

`NexusHarness` delegates to the Nexus CLI and Nexus data files. Nexus is a TypeScript harness built on top of Pi and is shipped independently from Ratsus. We created it, and its source lives at `/Users/alpha/workspace/alpha-innovation-labs/nexus-tui-awesome`.

When behavior needs to change inside Nexus itself, update or suggest updating Nexus in its own repository instead of working around it in Ratsus.

- Initial session loading runs `nexus --sessions-all --json` in `nexus/sessions/load_chat_sessions.rs`.
- If the JSON CLI output cannot be parsed, loading falls back to the cmux registry.
- Existing sessions spawn with `nexus --resume <session-id>` in the session working directory.
- New sessions spawn through `spawn_new_nexus_session_terminal` and preserve the existing left-panel folder order.
- Deletion runs `nexus --delete-session <session-id>`.
- Refresh reads the chat status file and merges running state into existing app sessions from an async worker, so slow Nexus file or process checks do not block the UI tick.
- The chat status file path is resolved once from `$NEXUS_CHAT_STATUS_FILE`, `nexus --chat-status-file-location`, or the default path, then cached for watcher matching and refreshes.

## Nexus conversation storage

Ratsus reads the Nexus cmux registry from `$NEXUS_CMUX_SESSION_REGISTRY` when set, otherwise from `~/.local/share/nexus/agent/cmux-session-registry.json`.

The registry contains entries with `sessionId`, optional `sessionTitle`, `updatedAt`, `cwd`, `pid`, and `sessionFile`. The `sessionFile` field points to the JSONL transcript file for that Nexus conversation.

Ratsus uses those session files for activity detection by reading from the tail and finding the newest JSON line whose `type` is `message`. A latest `user` or `toolResult` message is treated as active. A latest `assistant` message is active unless its `stopReason` is `stop`.

Process liveness is checked with `kill -0 <pid>`, which sends no signal and does not terminate the process. It only checks whether the PID exists and is accessible.

## Nexus observations

Observation previews are loaded from consolidated Nexus observation JSON files under `~/.local/share/nexus/agent/observations`. The Nexus harness starts a file watcher and asks Expo to reload previews when a watched observation JSON path changes.

## Conversation picker

`Ctrl+H` opens the conversation picker scoped to the selected workspace folder. Pressing `Tab` toggles the picker between `Workspace` and `All` scopes, and the active scope is rendered in the top-right dialog header. `All` preserves the previous unfiltered catalog behavior. Pressing `/` inside the session left pane opens this picker with filter entry already active; normal picker opens still require `/` before typed text filters.

## Stub harness

`StubHarness` is deterministic and in-memory. It does not run Nexus, read Nexus data paths, spawn a real shell, or persist normal terminal sessions. It supports fake sessions, fake chat terminals, fake normal terminals, deletion from memory, and fake observation previews.

## Key files

- `src/extensions/harness/core/chat_harness.rs`
- `src/extensions/harness/core/chat_session.rs`
- `src/extensions/harness/nexus/adapter/nexus_harness.rs`
- `src/extensions/harness/nexus/config/cmux_session_registry_path.rs`
- `src/extensions/harness/nexus/registry/load_session_registry.rs`
- `src/extensions/harness/nexus/registry/parse_session_registry.rs`
- `src/extensions/harness/nexus/process/session_file_is_active.rs`
- `src/extensions/harness/nexus/process/is_alive.rs`
- `src/extensions/harness/nexus/sessions/load_chat_sessions.rs`
- `src/extensions/harness/stub/adapter/stub_harness.rs`
- `src/extensions/harness/conversation_picker/`
- `src/extensions/harness/observations/`
