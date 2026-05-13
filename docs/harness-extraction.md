# Harness Extraction Inventory

## Objective

Document every current Nexus-specific seam that must move behind a harness/backend boundary so the app can run against:

1. the real Nexus backend,
2. a deterministic stub backend for safe local development, and
3. future backends such as opencode or Claude Code.

## Current state

The UI is not backend-neutral yet. Nexus is encoded in four different ways:

1. **Names and domain types**: `NexusDemo`, `NexusSession`, `NexusTerminal`, `nexus_sessions`.
2. **External process contract**: direct `nexus` CLI calls and `nexus --resume <id>` PTY spawning.
3. **Persistence contract**: Nexus-specific registry, observation, and terminal-session paths under `.local/share/nexus`.
4. **Data contract**: Nexus JSON fields, cmux registry fields, observation state files, and placeholder IDs/titles.

The clean extraction target is a backend-neutral app state using a `ChatHarness` plus a terminal-session abstraction.

## Proposed target modules

```text
src/
  harness/
    mod.rs
    chat_harness.rs
    chat_session.rs
    chat_terminal.rs
    chat_backend_event.rs
  harnesses/
    nexus/
      mod.rs
      nexus_harness.rs
      load_sessions.rs
      parse_sessions_json.rs
      parse_registry.rs
      observation_previews.rs
      terminal_command.rs
    stub/
      mod.rs
      stub_harness.rs
      stub_sessions.rs
      stub_terminal.rs
  app/
    app_state.rs
    run_app.rs
```

Recommended renames during extraction:

| Current | Target |
|---|---|
| `NexusDemo` | `AppState` or `RatsusApp` |
| `NexusSession` | `ChatSession` |
| `NexusTerminal` | `ChatTerminal` or `TerminalHandle` |
| `SessionTerminal` | `SessionRuntime` |
| `nexus_sessions` | `harnesses::nexus` + generic `harness` contracts |

## Harness methods the app currently uses

These are the capabilities the current app already consumes from Nexus, directly or indirectly.

### Session catalog methods

| Needed method | Current implementation | Current files |
|---|---|---|
| `load_sessions() -> Result<Vec<ChatSession>>` | `nexus --sessions-all --json` | `src/nexus_sessions/load_nexus_sessions.rs` |
| `refresh_sessions() -> Result<Vec<ChatSession>>` | read cmux registry JSON and derive running state | `src/nexus_sessions/load_nexus_session_registry.rs`, `src/nexus_sessions/spawn_session_refresh_worker.rs` |
| `merge_session_refresh(existing, refreshed)` | preserve stable title, update running status | `src/nexus_sessions/apply_session_refresh.rs`, `src/nexus_sessions/merge_registry_session_metadata.rs` |
| `new_session_placeholder(cwd) -> ChatSession` | `New Nexus chat`, `new-<uuid>` | `src/nexus_sessions/new_nexus_chat_session.rs` |
| `is_new_session_placeholder(session) -> bool` | title/id check | `src/nexus_sessions/is_new_nexus_chat_session.rs` |

### Session lifecycle methods

| Needed method | Current implementation | Current files |
|---|---|---|
| `spawn_new_chat(cwd, rows, cols) -> SessionRuntime` | spawn `nexus` in a PTY | `src/nexus_sessions/spawn_new_nexus_session_terminal.rs` |
| `spawn_existing_chat(session, rows, cols) -> ChatTerminal` | spawn `nexus --resume <id>` in a PTY | `src/terminal/session_terminal.rs` |
| `delete_chat_session(id) -> Result<()>` | `nexus --delete-session <id>` | `src/nexus_sessions/delete_nexus_session.rs`, `src/app/spawn_delete_sessions_worker.rs` |
| `working_dir_for_new_chat(app) -> PathBuf` | focused session cwd or current dir | `src/nexus_sessions/focused_nexus_session_working_dir.rs` |

### Terminal methods used by the UI

The current concrete type is `NexusTerminal`. A stub harness must provide equivalents without spawning real Nexus.

| Needed method | Current implementation | Current callers |
|---|---|---|
| `ensure_terminal(rows, cols)` | lazy PTY spawn in `SessionTerminal` | `src/app/nexus_demo_methods.rs`, `src/session_panes/resize_terminal_pane_session.rs` |
| `render(frame, area)` | render Ratkit termtui screen | `src/session_panes/render_chat_sessions.rs` |
| `resize(rows, cols)` | resize parser and PTY | `src/app/nexus_demo_methods.rs`, `src/session_panes/resize_terminal_pane_session.rs` |
| `write_input(bytes)` | write to PTY | `src/app/handle_terminal_keyboard.rs` |
| `scrollback_up(rows)` | scroll terminal history | `src/app/handle_nexus_demo_mouse.rs` |
| `scrollback_down(rows)` | scroll terminal history | `src/app/handle_nexus_demo_mouse.rs` |
| `reset_scrollback()` | return to live terminal view | `src/app/handle_terminal_keyboard.rs` |
| `take_needs_redraw() -> bool` | drain redraw flag | `src/app/handle_tick_event.rs` |
| `screen_snapshot() -> Option<Screen>` | copy-mode snapshot | `src/copy_mode/handle_terminal_copy_mouse.rs` |
| `cursor_state() -> Option<TerminalCursorState>` | cursor rendering | `src/session_panes/render_chat_sessions.rs` |
| `has_exited() -> bool` | process exit check | `src/terminal/session_terminal_has_exited.rs`, `src/app/close_exited_sessions.rs` |
| `kill()` | terminate process | `src/app/complete_delete_sessions.rs` |

### Observation/Expo methods

These are Nexus-specific today. A future backend can either implement them or return empty previews.

| Needed method | Current implementation | Current files |
|---|---|---|
| `load_observation_previews(requests)` | scan `.local/share/nexus/agent/observations` | `src/expo/load_observation_previews.rs` |
| `load_observation_preview(id, title)` | find `_*id.state.json`, parse topics | `src/expo/load_observation_preview.rs`, `src/expo/observation_state_path.rs` |
| `watch_observations() -> Option<FileWatcher>` | watch Nexus observations directory | `src/expo/start_observation_watcher.rs` |
| `is_observation_state_path(path) -> bool` | file suffix `.state.json` | `src/expo/path_is_observation_state.rs` |
| `parse_observation_preview(input, title)` | Nexus state JSON with `topics[].assistantBullets` | `src/expo/parse_observation_preview.rs` |

### Persistence/config methods

| Needed method | Current implementation | Current files |
|---|---|---|
| `session_order_preferences_path()` | `~/.config/ratsus/session-order.json` | `src/left_panel/session_order_preferences_path.rs` |
| `load_session_order_preferences()` | read Ratsus preference JSON | `src/left_panel/load_session_order_preferences.rs` |
| `save_session_order_preferences()` | write Ratsus preference JSON | `src/left_panel/save_session_order_preferences.rs` |
| `normal_terminal_registry_path()` | `~/.local/share/nexus/terminal-sessions.json` | `src/terminal/normal_terminal_registry_path.rs` |
| `load_normal_terminal_sessions()` | read local normal-terminal metadata | `src/terminal/load_normal_terminal_sessions.rs` |
| `save_normal_terminal_sessions()` | write local normal-terminal metadata | `src/terminal/save_normal_terminal_sessions.rs` |

The normal-terminal registry currently lives under a Nexus path even though it is Ratsus state. It should move to a Ratsus-owned path before adding stub mode.

## Exhaustive extraction inventory

### Entrypoint and app identity

| File | Coupling | Extraction work |
|---|---|---|
| `src/main.rs` | imports `run_nexus_app` | call generic `run_app(BackendKind)` or env/config-selected harness |
| `src/app/run_nexus_app.rs` | constructs `NexusDemo::new()` | replace with `run_app(harness)` and add `run_stub_app()` for `just dev-stub` |
| `src/app/mod.rs` | `NexusDemo`, `render_nexus_demo`, `handle_nexus_demo_event` | rename app/event/render modules or keep compatibility wrappers |
| `src/app/nexus_demo_state.rs` | central state loads Nexus sessions, normal terminals, observation watcher, refresh worker | inject harness and move external loading/spawning/watching out of constructor |
| `src/app/nexus_demo_methods.rs` | exposes `NexusTerminal`, lazily ensures concrete PTY | use terminal trait/object/enum owned by generic `SessionRuntime` |
| `Cargo.toml` | package description says Nexus wrapper | update once backend abstraction exists |
| `README.md` | describes Nexus wrapper | update docs for real/stub backend commands |
| `justfile` | only `just dev` runs real app | add `just dev-stub`; eventually `just dev BACKEND=nexus|stub` |

### Nexus backend module

Everything under `src/nexus_sessions/` is backend-specific except the generic ideas of session metadata and refresh application.

| File | Coupling | Extraction work |
|---|---|---|
| `src/nexus_sessions/mod.rs` | module is the backend boundary by name only | split into generic `harness` contracts and `harnesses::nexus` implementation |
| `session_info.rs` | `NexusSession` is used app-wide | rename to generic `ChatSession`; add `kind: ChatSessionKind` instead of ID-prefix checks |
| `load_nexus_sessions.rs` | executes `nexus --sessions-all --json` | Nexus harness `load_sessions()` implementation |
| `parse_nexus_sessions_json.rs` | Nexus JSON fields `id,title,cwd,created,modified` | keep inside Nexus harness parser |
| `parse_nexus_sessions.rs` | old Nexus table parser | keep only if Nexus harness still needs legacy parser |
| `sanitize_nexus_sessions_json.rs` | Nexus CLI JSON cleanup | keep inside Nexus harness parser |
| `sanitize_json_control_characters.rs` | parser helper | can stay generic utility or move under Nexus parser |
| `delete_nexus_session.rs` | executes `nexus --delete-session` | Nexus harness `delete_session(id)` |
| `spawn_new_nexus_session_terminal.rs` | spawns `nexus` PTY | Nexus harness `spawn_new_chat()` or terminal factory |
| `new_nexus_chat_session.rs` | placeholder title/id format | move to Nexus harness; stub/opencode can provide own placeholder |
| `is_new_nexus_chat_session.rs` | title/id prefix detection | replace with generic `session.lifecycle == Placeholder` or `SessionOrigin` |
| `focused_nexus_session_working_dir.rs` | name-specific utility | rename generic; keep logic in app layer |
| `start_new_nexus_chat.rs` | app action hardwired to Nexus | call `app.harness.start_new_chat(...)` or generic workflow |
| `start_new_nexus_chat_in_dir.rs` | app workflow uses Nexus spawner | generic `start_new_chat_in_dir` with injected harness |
| `spawn_session_refresh_worker.rs` | background worker calls Nexus registry loader | generic worker should call harness `refresh_sessions()` |
| `load_nexus_session_registry.rs` | reads Nexus cmux registry | Nexus harness refresh implementation |
| `nexus_cmux_session_registry_path.rs` | `NEXUS_CMUX_SESSION_REGISTRY`, `.local/share/nexus/agent/cmux-session-registry.json` | Nexus harness config/path provider |
| `parse_nexus_session_registry.rs` | cmux JSON fields and title fallback `New Session` | Nexus harness parser |
| `merge_registry_session_metadata.rs` | Nexus placeholder title rules | generic refresh merge plus backend-specific title specificity |
| `apply_session_refresh.rs` | refresh merge depends on new Nexus placeholder detection | make generic over `ChatSession` and placeholder/session kind |
| `process_is_alive.rs` | calls `kill -0` for registry pids | Nexus harness runtime activity implementation |
| `session_file_is_active.rs` | parses Nexus jsonl message roles/stop reasons | Nexus harness activity implementation |

### Terminal runtime module

| File | Coupling | Extraction work |
|---|---|---|
| `src/terminal/nexus_terminal.rs` | concrete PTY-backed terminal named Nexus | rename to `PtyTerminal`; implement `ChatTerminal` trait or enum variant |
| `src/terminal/session_terminal.rs` | owns `NexusSession`, `Option<NexusTerminal>`, spawns Nexus or shell by ID prefix | rename to `SessionRuntime`; delegate spawn to harness/terminal factory |
| `src/terminal/is_chat_session.rs` | chat means not `terminal-*` | replace with `ChatSessionKind::Chat` |
| `src/terminal/is_normal_terminal_session.rs` | normal terminal detected by `terminal-` prefix | replace with `ChatSessionKind::NormalTerminal` |
| `src/terminal/normal_terminal_session_info.rs` | returns `NexusSession` with `terminal-*` ID | return generic `ChatSession` with kind field |
| `src/terminal/spawn_normal_terminal_session.rs` | spawns real shell | decide whether stub mode uses fake shell terminal or disables normal terminals |
| `src/terminal/normal_terminal_registry_path.rs` | stores Ratsus terminal state under `.local/share/nexus` | move to `.local/share/ratsus` or backend-scoped storage |
| `src/terminal/normal_terminal_legacy_registry_path.rs` | legacy Ratsus path | keep migration only until path is fixed |
| `src/terminal/load_normal_terminal_sessions.rs` | returns `Vec<NexusSession>` | return generic sessions and isolate storage path |
| `src/terminal/nexus_session_from_persisted_normal_terminal_session.rs` | conversion name/type | rename to generic conversion |
| `src/terminal/persisted_normal_terminal_session_from_nexus_session.rs` | conversion name/type | rename to generic conversion |
| `src/terminal/normal_terminal_sessions_from_session_terminals.rs` | reads generic app session runtime | rename after `SessionRuntime` extraction |
| `src/terminal/persist_normal_terminal_sessions.rs` | persists local terminal metadata | point at Ratsus-owned storage, not Nexus-owned storage |
| `src/terminal/session_terminal_has_exited.rs` | concrete terminal exit polling | call terminal trait method |
| `src/terminal/default_shell_command.rs` | local shell from `$SHELL` | not Nexus, but unsafe for fully stubbed mode unless disabled/faked |
| `src/terminal/encode_key_event.rs` | terminal input encoding | backend-neutral; keep |
| `src/terminal/process_terminal_output.rs` | terminal stream processing | backend-neutral; keep with PTY terminal |
| `src/terminal/spawn_terminal_reader.rs` | PTY reader | backend-neutral for real PTY terminal; unused by fake terminal |
| `src/terminal/write_terminal_replies.rs` | PTY reply writer | backend-neutral for real PTY terminal |
| `src/terminal/scroll_delta_for_mouse_kind.rs` | UI behavior | backend-neutral |
| `src/terminal/shell_command_from_env_value.rs` | shell parsing | backend-neutral but only real-shell mode needs it |

### App workflows that call backend operations

| File | Coupling | Extraction work |
|---|---|---|
| `src/app/handle_keyboard_event.rs` | Ctrl+N calls `start_new_nexus_chat`; split errors say new chat | call generic new-chat action through harness |
| `src/session_panes/split_active_terminal_pane.rs` | split always creates a new Nexus chat | call generic harness spawner |
| `src/session_panes/bundle_new_session_in_active_terminal_pane.rs` | bundle always creates a new Nexus chat | call generic harness spawner |
| `src/conversation_picker/activate_selected_conversation.rs` | folder activation starts a Nexus chat in dir | call generic new-chat-in-dir action |
| `src/app/restore_focus_after_removals.rs` | if all chats exited, starts a replacement Nexus chat | call generic replacement-chat policy or make backend-configurable |
| `src/app/confirm_delete_session.rs` | filters chat targets and starts Nexus delete worker | call harness delete for backend chat sessions |
| `src/app/spawn_delete_sessions_worker.rs` | thread calls `delete_nexus_session` | generic delete worker over harness handle/command |
| `src/app/complete_delete_sessions.rs` | tracks `closed_chat_session_ids` for deleted chats | generic closed conversation ids; use `kind` not prefix checks |
| `src/app/drain_delete_session_receiver.rs` | result names `deleted_chat_ids` | generic delete result naming |
| `src/app/removable_delete_session_ids.rs` | removal rules depend on `is_chat_session` | use `ChatSessionKind` and backend delete semantics |
| `src/app/remove_exited_sessions.rs` | removes exited non-chat terminals differently | use `ChatSessionKind` and terminal policy |
| `src/app/close_exited_sessions.rs` | persists normal terminals after exit cleanup | keep local terminal persistence separate from chat harness |
| `src/app/start_new_normal_terminal.rs` | real shell terminal creation and persistence | decide stub behavior; use generic session kind |

### Refresh and tick loop

| File | Coupling | Extraction work |
|---|---|---|
| `src/app/handle_tick_event.rs` | drains Nexus refreshes and observation watcher every tick | call generic refresh receiver and optional observation provider |
| `src/nexus_sessions/drain_session_refreshes.rs` | drains Nexus session refresh channel | move to generic refresh drain or app service |
| `src/nexus_sessions/apply_session_refresh.rs` | mutates `SessionTerminal` using Nexus metadata | generic session refresh application |
| `src/app/nexus_demo_state.rs` | owns `Receiver<SessionRefreshResult>` from Nexus worker | own generic refresh receiver from selected harness |

### Observation and Expo

| File | Coupling | Extraction work |
|---|---|---|
| `src/expo/nexus_observations_dir.rs` | hardcoded `.local/share/nexus/agent/observations` | Nexus harness observation path provider |
| `src/expo/observation_state_path.rs` | timestamp-prefixed `_<id>.state.json` lookup | Nexus harness implementation detail |
| `src/expo/load_observation_preview.rs` | reads Nexus observation file | harness observation provider method |
| `src/expo/load_observation_previews.rs` | scans Nexus observation dir and parses state | harness observation provider method |
| `src/expo/parse_observation_preview.rs` | Nexus observation JSON schema | Nexus harness parser |
| `src/expo/start_observation_watcher.rs` | watches Nexus observation dir | harness optional watcher |
| `src/expo/path_is_observation_state.rs` | Nexus state filename suffix | harness-specific changed-path filter |
| `src/expo/spawn_observation_cache_worker.rs` | always calls Nexus loader | generic worker over provider callback |
| `src/expo/start_observation_cache_load.rs` | schedules observation loads from app sessions | keep generic but call provider-backed worker |
| `src/expo/poll_observation_watcher.rs` | watcher assumes Nexus state files | use harness watcher/filter |
| `src/expo/drain_observation_cache_receiver.rs` | generic cache drain | keep after receiver type remains generic |
| `src/expo/observation_preview_request.rs` | generic request model despite “conversation” naming | keep or move to harness contract |
| `src/expo/observation_preview_requests.rs` | builds requests from `SessionTerminal` | rename after `SessionRuntime`; filter only sessions with observations if needed |
| `src/expo/observation_conversation_ids.rs` | extracts IDs from sessions | generic after `ChatSession` rename |
| Expo layout/render files | assume session titles/ids/previews exist | mostly backend-neutral once model is generic |

### UI text, naming, and rendering

| File | Coupling | Extraction work |
|---|---|---|
| `src/rendering/render_nexus_demo.rs` | function name and left-pane title `nexus sessions` | rename and use `harness.display_name()` in titles |
| `src/session_panes/render_chat_sessions.rs` | `Starting Nexus session…`, `NexusTerminal` parameter | generic loading text and terminal trait |
| `src/left_panel/session_lines.rs` | empty text `No Nexus sessions`; function `nexus_session_row_line` | generic text and function naming |
| `src/left_panel/session_row_line.rs` | `NexusSessionRowLineConfig`, `nexus_session_row_line` | rename to generic session row line |
| `src/conversation_picker/conversation_picker_lines.rs` | imports `NexusSessionRowLineConfig` | rename imports/types |
| `src/menu_bar/nexus_menu_bar.rs` | menu builder name | rename to generic menu bar; content may stay backend-neutral |
| `src/menu_bar/render_nexus_menu_bar.rs` | render function name | rename generic |
| `src/menu_bar/handle_nexus_menu_bar_mouse.rs` | event function name | rename generic |
| `src/menu_bar/split_nexus_menu_bar_area.rs` | layout function name | rename generic |
| `src/menu_bar/style_nexus_menu_bar_border.rs` | style function name | rename generic |
| `src/menu_bar/sync_nexus_menu_bar_selection.rs` | sync function name | rename generic |
| `src/notifications/show_failed_to_start_new_chat_toast.rs` | text `Failed to start Nexus chat` | use `harness.display_name()` or generic text |
| Snapshot files under `src/rendering` | expected text includes Nexus | update after UI text rename |

### Conversation picker and left panel model references

These modules are mostly backend-neutral, but they consume `NexusDemo`, `NexusSession`, or `SessionTerminal`, so they must be touched during type extraction.

| Area | Files | Extraction work |
|---|---|---|
| Conversation picker state/render/input | `src/conversation_picker/*.rs` | rename model imports; keep behavior; backend-specific only where new chat is started |
| Left-panel sorting/grouping/rendering | `src/left_panel/*.rs` | rename model imports; replace `is_chat_session` / `is_normal_terminal_session` with `ChatSessionKind` |
| Session panes | `src/session_panes/*.rs` | rename model imports; route new chat creation and terminal operations through harness/terminal abstraction |
| Copy mode | `src/copy_mode/*.rs` | terminal trait must expose `screen_snapshot`; otherwise backend-neutral |
| Rendering | `src/rendering/*.rs` | rename app/render functions and backend display strings |

### Test support and tests

| File/area | Coupling | Extraction work |
|---|---|---|
| `src/test_support/nexus_demo_fixture.rs` | fixture constructs `NexusDemo` with generic-looking fake sessions | rename to app fixture and optionally use `StubHarness` |
| `src/test_support/dormant_session.rs` | creates `NexusSession` | create generic `ChatSession` fixture |
| `src/nexus_sessions/*_tests.rs` | Nexus parser/refresh assumptions | keep under Nexus harness tests |
| `src/rendering/render_nexus_demo_snapshot_tests.rs` | `NexusDemo`, `NexusSession`, snapshot strings | update to generic app or stub harness fixtures |
| `src/left_panel/*tests.rs` | `NexusSession`, `SessionTerminal`, refresh result type | rename types; keep behavior tests |
| `src/conversation_picker/*tests.rs` | `NexusSession`, `SessionTerminal` | rename types; keep behavior tests |
| pending snapshot files | contain Nexus text | regenerate or remove pending snapshots after rename |

## Backend-specific details to keep inside `NexusHarness`

The following must not leak into generic app code after extraction:

1. CLI name: `nexus`.
2. CLI arguments: `--sessions-all --json`, `--resume <id>`, `--delete-session <id>`.
3. Env var: `NEXUS_CMUX_SESSION_REGISTRY`.
4. Registry path: `.local/share/nexus/agent/cmux-session-registry.json`.
5. Registry schema: `entries[].sessionId`, `sessionTitle`, `updatedAt`, `cwd`, `pid`, `sessionFile`.
6. Activity heuristic: `kill -0 <pid>` plus jsonl session-file last message status.
7. Stable-session JSON schema: `id`, `title`, `cwd`, `created`, `modified`.
8. Placeholder title/id: `New Nexus chat`, `new-<uuid>`.
9. Registry placeholder title: `New Session`.
10. Observation directory: `.local/share/nexus/agent/observations`.
11. Observation filename convention: timestamp prefix plus `_<session_id>.state.json`.
12. Observation schema: `topics[].title`, `topics[].assistantBullets`.

## Stub harness requirements

A useful `StubHarness` should implement the same methods without touching Nexus, user chat data, or Nexus-owned directories.

Minimum behavior:

1. Return deterministic fake `ChatSession` rows across several folders.
2. Provide fake running/idle states and refresh updates without reading cmux registry files.
3. Create new fake sessions in memory when Ctrl+N, split, or bundle actions run.
4. Delete fake sessions in memory without CLI calls.
5. Return fake observation previews or empty previews.
6. Provide a fake terminal implementing render/input/resize/scroll/copy/redraw/cursor methods.
7. Store preferences and normal-terminal metadata in a stub-specific temp/Ratsus path, or disable persistence.
8. Never execute `nexus`, never read `.local/share/nexus`, and never resume real sessions.

## Suggested implementation order

1. Add generic `ChatSession`, `ChatSessionKind`, `SessionRuntime`, and `ChatTerminal` contracts while preserving current behavior.
2. Rename `NexusTerminal` to `PtyTerminal` and make it implement the terminal contract.
3. Move all `src/nexus_sessions/*` external calls/parsers into `harnesses::nexus`.
4. Inject a harness into app construction instead of calling `NexusDemo::new()` with globals.
5. Convert new/delete/refresh/observation workers to call harness methods.
6. Add `harnesses::stub` with deterministic sessions and fake terminal.
7. Add `just dev-stub` and ensure it cannot call the Nexus CLI or Nexus data paths.
8. Update UI strings, tests, snapshots, README, and Cargo description.

## Done criteria for extraction

1. `just dev` still launches the real Nexus-backed app.
2. `just dev-stub` launches the same UI with only fake sessions and fake terminal output.
3. Stub mode performs no `nexus` command execution.
4. Stub mode reads no `.local/share/nexus` files.
5. UI modules do not import `harnesses::nexus` directly.
6. Only `harnesses::nexus` contains Nexus CLI args, Nexus paths, Nexus JSON parsers, and Nexus observation parsing.
7. Future backends can be added by implementing the harness contract and selecting it at startup.
