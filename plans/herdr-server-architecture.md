# Ratsus Headless Server Architecture Plan

**Date:** 2026-06-04  
**Goal:** Clean-room rewrite of Herdr's session-server model into Ratsus, including terminal rendering.  
**Status:** Approved — decisions locked. Implementation begins after technical design is complete.  

---

## This Is a Clean-Room Rewrite, Not a Port

The previous version of this document attempted to "adapt" Herdr concepts into Ratsus's existing domain boundaries. That approach is discarded. The architecture decisions below require a **clean-room rewrite** of Herdr's session-server model.

**What clean-room means:**
- We study Herdr's observable behavior, protocol, and architecture.
- We read Herdr's source to understand the design.
- We **do not copy code** — every line is written fresh.
- Ratsus remains MIT-licensed.
- The wire protocol may be compatible (protocols are not copyrightable) but the implementation is original.
- Data structures may be similar where they model the same domain, but the code that manipulates them is new.

Ratsus's existing `app/`, `core/`, `ui/`, and `extensions/` hierarchy remains for non-server features (Expo, file viewer, harness backends) but is **subordinate** to the server runtime when server mode is active.

---

## Locked Decisions

### 1. Server renders to a virtual buffer; the TUI is a thin client

The server runs the full Ratkit render loop into a `ratatui::Buffer` in memory. After each render tick, it serializes the buffer into `FrameData` and streams it to connected clients. The TUI does not run Ratkit locally. It receives frames, diffs them against the last frame, and blits ANSI escape codes to the host terminal.

**Consequences:**
- Ratsus's `core/rendering/` is bypassed in server mode.
- Ratsus's `ui/grid_layout/` is replaced by the server's workspace/tab/pane model.
- The TUI binary becomes a thin client (~1,744 lines equivalent to `client/mod.rs`).

### 2. Implement a binary wire protocol compatible with Herdr's design

The protocol uses `bincode`-serialized enums with a `u32` little-endian length prefix. Protocol version starts at 1 (independent of Herdr's v12). Max frame size is 2 MiB (32 MiB for Kitty graphics). Message types are `ClientMessage` and `ServerMessage` with equivalent semantic coverage.

**Consequences:**
- No JSON for the TUI stream. The existing JSON API idea is abandoned.
- The separate automation API implements equivalent methods to the reference API schema (~40 methods).
- Version negotiation, handshake timeout, and render encoding negotiation (`SemanticFrame` vs `TerminalAnsi`) follow Herdr's design but are reimplemented.

### 3. Code lives in parallel `src/herdr/` modules

The server subsystem is implemented in a new `src/herdr/` directory. It does not map into Ratsus's existing domains. The existing domains (`app/`, `core/`, `ui/`, `extensions/`, `shared/`) remain for Ratsus-specific features that run alongside or on top of the server runtime.

**Proposed tree:**

```
src/
  herdr/
    server/
      headless.rs           — event loop, virtual render, client mgmt
      client_transport.rs   — read/write threads, ServerEvent dispatch
      clients.rs            — HashMap of connections, input routing
      client_accept.rs      — handshake, reject, accept_pending
      render_stream.rs      — FrameData serialization, broadcast
      autodetect.rs         — socket detection, auto-spawn
      handoff.rs            — live server handoff for updates
      socket_paths.rs       — socket path logic, permission restrict
      keybindings.rs        — keybinding profile application
      notifications.rs      — toast forwarding to clients
      clipboard_image.rs    — clipboard bridging
      terminal_attach.rs    — direct terminal attach helper
      mod.rs
    client/
      mod.rs                — thin client: connect, blit, input, resize
      input.rs              — attach escape, prefix handling
    protocol/
      wire.rs               — ClientMessage, ServerMessage, framing
      render_ansi.rs        — BlitEncoder, diff, ANSI escape generation
      mod.rs
    api/
      schema.rs             — API methods, request/response types
      server.rs             — API request handler, dispatch
      client.rs             — API client for internal use
      event_hub.rs          — event pub/sub
      subscriptions.rs      — event subscription mgmt
      status.rs             — status endpoint
      wait.rs               — wait-for-condition helper
      mod.rs
    pane/
      mod.rs                — PTY spawn, child mgmt, scrollback, agent detect
    workspace/
      mod.rs                — workspace/tab/pane hierarchy
    session/
      mod.rs                — session lifecycle, restore
    persist/
      mod.rs                — module root
      io.rs                 — load/save session.json
      restore.rs            — restore workspaces, tabs, panes
      snapshot.rs           — serialize current state
    terminal/
      mod.rs                — terminal emulation bridge
    vt/                     — VT parser / terminal emulation
      mod.rs
  app/                      — Ratsus-specific orchestration (non-server features)
  core/                     — Ratsus rendering (bypassed in server mode)
  ui/                       — Ratsus widgets (bypassed in server mode)
  extensions/               — Expo, file viewer, harness backends (overlay on server)
  shared/                   — Cross-domain helpers
```

### 4. Full multi-client support from day one

The server maintains a `HashMap<u64, ClientConnection>` with per-client `control` and `render` channels. The render channel has capacity 1 so slow clients cannot build lag. Input is routed from the latest app client. Direct terminal attach mode (`TerminalAttach`) is supported for individual pane access without the full app chrome.

**Consequences:**
- `server/clients.rs` with `latest_app_client()`, `render_targets()`, `terminal_attach_client_ids()` must be implemented.
- Frame broadcasting to all clients after each render tick is required.
- Client-specific keybinding profiles (`ClientKeybindings::Server` vs `Local`) must be supported.

---

## License

Herdr is AGPL-3.0-or-later. Ratsus is MIT. Because this is a **clean-room rewrite**, no Herdr code is copied. Ratsus remains MIT. The wire protocol is reverse-engineered from observable behavior (socket traffic, log files) and documented design — protocols are functional requirements, not copyrightable expression.

---

## Dependency Audit

### Shared dependencies (no conflict)
- `serde` + `derive`
- `serde_json`
- `libc`

### Version conflicts to resolve

| Crate | Ratsus | Herdr | Resolution |
|-------|--------|-------|------------|
| `crossterm` | 0.28 | 0.29 | Upgrade Ratsus to 0.29 |
| `ratatui` | 0.29 | 0.30 | Upgrade Ratsus to 0.30 |
| `portable-pty` | 0.8 | 0.9 | Upgrade Ratsus to 0.9 |

### New dependencies required

| Crate | Purpose | Notes |
|-------|---------|-------|
| `base64` | OSC 52 clipboard, image payloads | Already used indirectly via `ratatui-image` |
| `bincode` | Wire protocol serialization | Version 2 with `serde` feature |
| `bytes` | Byte buffer handling | Lightweight |
| `ctrlc` | SIGINT handling for graceful shutdown | Optional if we handle signals manually |
| `png` | Clipboard image encoding | Only needed if clipboard bridging is enabled |
| `regex` | Agent detection heuristics | For matching process names in agent detection |
| `sha2` | Checksum verification | For update/hotpatch logic if we implement it |
| `tokio` | Async runtime for API server | `rt-multi-thread`, `macros`, `sync`, `time` |
| `toml` | Config parsing | For keybinding and theme configs |
| `tracing` + `tracing-subscriber` | Structured logging | May replace or supplement existing logging |
| `unicode-width` | Cell width calculation | Critical for terminal rendering correctness |

### Terminal emulation engine: `libghostty-vt` via Zig build

**Decision:** Use Herdr's vendored `libghostty-vt` library, compiled via `zig build` in `build.rs`.

`libghostty-vt` is a terminal emulator library written in Zig/C that handles ANSI escape sequence parsing, screen buffer management, VT state machine, and scrollback. It is the same engine used by Ghostty, a modern terminal emulator, making it battle-tested for complex terminal apps (Neovim, Tmux, Emacs, etc.).

**Build integration:**
- `build.rs` calls `zig build -Demit-lib-vt -Doptimize=ReleaseFast`
- Produces `libghostty-vt.a` in `vendor/libghostty-vt/zig-out/lib/`
- Linked statically into the Rust binary

**Consequences:**
- Every developer must install Zig (`brew install zig`, `pacman -S zig`, etc.)
- CI must install Zig
- Build times increase by ~10-30 seconds on first compile
- Cross-compilation requires Zig target mapping (see `build.rs::zig_target()`)},{

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  ratsusd (server process)                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  Event loop (server/headless.rs)                                    │    │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐  │    │
│  │  │ AppState     │  │ Virtual Buf  │  │ PTY Manager (pane.rs)    │  │    │
│  │  │ (workspace/  │  │ (ratatui)    │  │ (libghostty-vt)          │  │    │
│  │  │  session.rs) │  │              │  │                          │  │    │
│  │  └──────────────┘  └──────────────┘  └──────────────────────────┘  │    │
│  │         │                 │                    │                    │    │
│  │         └─────────────────┴────────────────────┘                    │    │
│  │                           │                                         │    │
│  │                    render tick                                      │    │
│  │                           │                                         │    │
│  │                    ┌──────────────┐                                 │    │
│  │                    │ FrameData    │                                 │    │
│  │                    │ (cells,      │                                 │    │
│  │                    │  cursor)     │                                 │    │
│  │                    └──────────────┘                                 │    │
│  │                           │                                         │    │
│  │              ┌────────────┼────────────┐                          │    │
│  │              ▼            ▼            ▼                          │    │
│  │         Client 1     Client 2     Client N                        │    │
│  │         (render)     (render)     (render)                        │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │  API server (api/server.rs) — herdr.sock                            │    │
│  │  session.list, session.spawn, server.stop, etc.                     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
              ┌──────────────────────┼──────────────────────┐
              ▼                      ▼                      ▼
       ┌─────────────┐       ┌─────────────┐       ┌─────────────┐
       │ ratsus TUI  │       │ ratsus TUI  │       │ herdr --attach
       │ (client)    │       │ (client)    │       │ (direct pane)
       └─────────────┘       └─────────────┘       └─────────────┘
```

### Server event loop

Herdr's `server/headless.rs` implements a manual event loop that multiplexes:
1. `Timer` — periodic tasks (update checks, persist auto-save)
2. `Internal(AppEvent)` — PTY output, child process exits, agent state changes
3. `Api(api::ApiRequestMessage)` — external API requests
4. `ServerEvent(ServerEvent)` — client connect, disconnect, input, resize
5. `RenderRequested` — explicit render trigger

The loop renders to a virtual `ratatui::Buffer`, converts it to `FrameData`, and broadcasts to all clients.

### Client thin-client logic

Herdr's `client/mod.rs`:
1. Connects to `herdr-client.sock`
2. Sends `ClientMessage::Hello` with terminal size, protocol version, render encoding
3. Enters raw mode, enables mouse capture, bracketed paste
4. Spawns a reader thread for `ServerMessage` frames
5. Spawns a writer thread for `ClientMessage` input
6. Diffs incoming `FrameData` against last frame using `BlitEncoder`
7. Writes ANSI escape codes to stdout
8. Restores terminal on exit

### Wire protocol details

```rust
// Length-prefix framing
[ u32 little-endian length ][ bincode-serialized message ]

// Client → Server
ClientMessage::Hello { version: u32, cols: u16, rows: u16, ... }
ClientMessage::Input { data: Vec<u8> }
ClientMessage::Resize { cols: u16, rows: u16, ... }
ClientMessage::ClipboardImage { extension: String, data: Vec<u8> }
// ... 15+ variants

// Server → Client
ServerMessage::Frame { data: FrameData }
ServerMessage::Shutdown { reason: String }
ServerMessage::Notification { kind: NotifyKind, message: String }
ServerMessage::ClipboardWrite { data: String }
// ... 10+ variants

// FrameData
struct FrameData {
    width: u16,
    height: u16,
    cells: Vec<CellData>,
    cursor: CursorData,
}

struct CellData {
    text: String,        // single character (may be multi-byte)
    fg: ColorData,
    bg: ColorData,
    bold: bool,
    italic: bool,
    underline: UnderlineStyle,
    // ... additional style flags
}
```

---

## Persistence Model

The server persistence is **not** an extension of Ratsus's `ui/grid_layout/persistence/`. It is a separate system with its own schema.

**File:** `~/.config/ratsus/session.json` (or `~/.config/herdr/session.json` if we keep Herdr's paths)

**Schema (version 3):**
```json
{
  "version": 3,
  "workspaces": [
    {
      "id": "w...",
      "custom_name": null,
      "identity_cwd": "/path",
      "tabs": [
        {
          "custom_name": null,
          "panes": [
            { "id": 1, "terminal_type": "shell", "cwd": "/path" }
          ]
        }
      ]
    }
  ]
}
```

**Ratsus persistence is deprecated in server mode.** The server owns all layout, session, and workspace state. Ratsus's local UI preferences (theme, keybindings) may still be stored separately.

---

## Integration with Ratsus-Specific Features

**Decision:** All Ratsus features survive as **client-side overlays**.

The server streams terminal frames; the client composites local UI on top. The server knows nothing about overlays. Input is intercepted by the client before being forwarded to the server.

| Ratsus Feature | Server Equivalent | Integration Strategy |
|----------------|-------------------|----------------------|
| Harness backends (Nexus, stub) | Agent detection (`pane.rs` agent_changed) | Implement agent detection heuristics; add Nexus-specific patterns |
| Expo view | None | **Client-side overlay** — intercepts input, renders locally over frame stream |
| File viewer / file tree | None | **Client-side overlay** — local Ratkit rendering over server frames |
| Command bar | None | **Client-side modal** — intercepts prefix keys, renders locally |
| Left panel / workspace pane | Server sidebar | Replace with the server's sidebar model |
| Resizable grid | Server pane layout | Adopt the server's split model (vertical/horizontal) |
| Menu bar | None | **Client-side overlay** — rendered locally by thin client |
| Notifications / toasts | Server notifications | Forward server notifications + Ratsus local toasts |

---

## Security Model

Herdr's actual security model (from source code analysis):

1. **Unix sockets** are created with `0600` permissions (`server/socket_paths.rs::restrict_socket_permissions`).
2. **Socket path** is in `~/.config/herdr/herdr.sock` and `~/.config/herdr/herdr-client.sock`.
3. **Stale socket cleanup** uses PID-based lockfiles (`ipc.rs::remove_socket_file_if_owned`).
4. **No `SO_PEERCRED`** — the previous plan incorrectly claimed this. Herdr relies on filesystem permissions only.
5. **Windows is unsupported** — Herdr is Unix-only (`std::os::unix::net::UnixStream`).

The security model follows Herdr's design.

---

## Startup Flow

**Decision:** Single binary with auto-detect mode (Herdr model).

```
ratsus [args]
    │
    ├─► Check RATSUS_ENV_VAR to prevent nested invocation
    │
    ├─► Parse CLI args (config path, cwd, command to spawn)
    │
    ├─► Try to connect to existing server socket
    │       ├─► Success → Thin client mode (attach to running server)
    │       └─► Failure → Continue to server start
    │
    ├─► Initialize logging
    │
    ├─► Load config (config.toml)
    │
    ├─► Spawn server child process (re-exec with --server-child)
    │       ├─► Create API socket (ratsus.sock)
    │       ├─► Create client socket (ratsus-client.sock)
    │       ├─► Load session.json or start fresh
    │       ├─► Restore workspaces, tabs, panes
    │       └─► Enter event loop
    │
    └─► Start thin client (connect to client socket)
```

The server is **not** a traditional Unix daemon (no `fork()`, `setsid()`). It is a normal child process spawned via `std::process::Command` re-exec. It survives client disconnect because it is a separate OS process with its own process group. The client and server share the same binary — the server child detects `--server-child` and enters headless mode.

---

## Test Strategy

| Layer | Strategy | Herdr Source Reference |
|-------|----------|------------------------|
| Wire protocol | Unit tests for `protocol/wire.rs` framing, serde round-trip | `protocol/wire.rs` (no tests visible in scan; must add) |
| ANSI blitting | Unit tests for `BlitEncoder` diff accuracy | `protocol/render_ansi.rs` (no tests visible; must add) |
| Server event loop | Integration test: mock client, verify frame broadcast | `server/headless.rs` (large; needs fixture) |
| Client handshake | Unit test: mock server socket, verify Hello message | `client/mod.rs` (extract handshake logic) |
| Multi-client | Integration test: two mock clients, verify independent frames | `server/clients.rs` |
| Persistence | Round-trip test: save session.json, stop server, restore | `persist/` |
| API methods | Unit tests against `api/server.rs` with mock state | `api/schema.rs` + `api/server.rs` |
| PTY spawn | Integration test with stub shell; verify child process | `pane.rs` |
| E2E | Start server, spawn pane via API, attach client, verify output | Full stack |

**Constraint:** Herdr's source has minimal visible test coverage. We must add tests as we implement.

---

## Implementation Phases

### Phase 0: Technical foundation
- Decide on terminal emulation engine (see open questions)
- Upgrade `crossterm` 0.28→0.29, `ratatui` 0.29→0.30, `portable-pty` 0.8→0.9
- Add new dependencies to `Cargo.toml`
- Set up `src/herdr/` module tree

### Phase 1: Protocol & thin client skeleton
- Implement `protocol/wire.rs` (message types, framing)
- Implement `protocol/render_ansi.rs` (BlitEncoder, diff, ANSI generation)
- Create minimal `client/mod.rs` that connects and prints frames
- **Test:** Handshake, frame receipt, ANSI blitting

### Phase 2: Server event loop (headless)
- Implement `server/headless.rs` core event loop (without PTY/agent logic)
- Implement `server/client_transport.rs` (accept, read, write threads)
- Implement `server/clients.rs` (multi-client management)
- Implement virtual `ratatui::Buffer` render
- **Test:** Two clients connect, receive frames, resize

### Phase 3: Pane / PTY / terminal emulation
- Implement `pane.rs` or integrate with Ratsus's `portable-pty`
- Implement chosen terminal emulation engine
- Implement PTY spawn, child lifecycle, scrollback
- **Test:** Spawn shell, send input, receive output

### Phase 4: Workspace / session / persistence
- Implement `workspace.rs`, `session.rs`, `persist/`
- Implement `session.json` save/restore
- **Test:** Create workspace, add tabs/panes, stop server, restore

### Phase 5: API layer
- Implement `api/schema.rs`, `api/server.rs`, `api/client.rs`
- Expose Unix socket API
- **Test:** `session.list`, `session.spawn`, `server.stop`

### Phase 6: Ratsus integration
- Bridge server workspaces to Ratsus harness backends (Nexus, stub)
- Decide fate of Expo, file viewer, command bar
- Implement client-side overlays for Ratsus-specific UI
- **Test:** E2E harness session via server

### Phase 7: Polish & optimization
- Copy mode, selection, scrollback search
- Agent detection heuristics for Nexus
- Performance profiling (frame rate, memory)
- **Test:** Snapshot tests for frame output

---

## What We Implement (Herdr-equivalent behavior)

| Herdr Module | Herdr Lines | Ratsus Equivalent (to implement) | Ratsus Equivalent (deprecated) |
|--------------|-------------|----------------------------------|--------------------------------|
| `server/headless.rs` | 6,267 | `herdr/server/headless.rs` | `app/` event loop |
| `server/client_transport.rs` | 753 | `herdr/server/client_transport.rs` | None (new) |
| `server/clients.rs` | 385 | `herdr/server/clients.rs` | None (new) |
| `server/render_stream.rs` | 351 | `herdr/server/render_stream.rs` | None (new) |
| `server/autodetect.rs` | 523 | `herdr/server/autodetect.rs` | None (new) |
| `server/handoff.rs` | 464 | `herdr/server/handoff.rs` | None (new) |
| `server/socket_paths.rs` | 244 | `herdr/server/socket_paths.rs` | None (new) |
| `client/mod.rs` | 1,744 | `herdr/client/mod.rs` | `core/rendering/` |
| `client/input.rs` | ~150 | `herdr/client/input.rs` | `app/input/` |
| `protocol/wire.rs` | 1,576 | `herdr/protocol/wire.rs` | None (new) |
| `protocol/render_ansi.rs` | 1,518 | `herdr/protocol/render_ansi.rs` | None (new) |
| `api/schema.rs` | 1,565 | `herdr/api/schema.rs` | None (new) |
| `api/server.rs` | 963 | `herdr/api/server.rs` | None (new) |
| `api/subscriptions.rs` | 721 | `herdr/api/subscriptions.rs` | None (new) |
| `pane.rs` | 3,096 | `herdr/pane/mod.rs` | `extensions/terminal/` |
| `workspace.rs` | 845 | `herdr/workspace/mod.rs` | `ui/grid_layout/` |
| `session.rs` | 971 | `herdr/session/mod.rs` | `app/sessions/` |
| `persist/` | TBD | `herdr/persist/` | `ui/grid_layout/persistence/` |
| `layout.rs` | 845 | `herdr/layout.rs` | `ui/grid_layout/` |
| `selection.rs` | 971 | `herdr/selection.rs` | None (new) |

---

## Open Questions

### Resolved

| # | Question | Decision |
|---|----------|----------|
| 1 | License (AGPL vs MIT) | Clean-room rewrite preserves MIT |
| 2 | Terminal emulation engine | `libghostty-vt` via Zig build |
| 3 | Feature survival | All features as client-side overlays |
| 4 | Wire protocol | Herdr-compatible from day one |
| 5 | Process model | Single binary, auto-detect server spawn |

### Remaining

1. **Config migration.** The server uses `config.toml`. Ratsus currently has no user config file. Do we adopt Herdr's config system for all settings?
2. **Update mechanism.** The reference implementation has an in-app updater with handoff logic. Do we implement this or rely on external package managers?
3. **Remote SSH bridge.** Herdr has `remote.rs` (79KB) for `herdr --remote`. Is this in scope?
4. **Agent integrations.** Herdr detects Claude, Codex, OpenCode, Kiro, etc. Do we add Nexus-specific detection patterns?
5. **Copy mode / selection.** The selection system (reference: ~17KB in Herdr) handles linewise, block, and semantic selection. Do we implement this exactly or adapt?
6. **Sound / toast notifications.** Herdr forwards notifications to clients. Ratsus has its own toast system. Which wins?

---

## Summary

This is a **clean-room rewrite** of Herdr's architecture for Ratsus. The server renders to a virtual buffer and streams frames to thin clients over a binary serde protocol. Code lives in `src/herdr/`. Multi-client support is required from day one. The existing Ratsus domain boundaries are preserved only for non-server features.

**Resolved decisions:**
1. ✅ Clean-room rewrite (MIT preserved)
2. ✅ Terminal emulation: `libghostty-vt` via Zig build
3. ✅ Feature survival: All features as client-side overlays
4. ✅ Wire protocol: Herdr-compatible from day one
5. ✅ Process model: Single binary, auto-detect server spawn

**Remaining blockers before implementation:**
1. Install Zig toolchain for all developers and CI
2. Upgrade conflicting dependencies (`crossterm` 0.28→0.29, `ratatui` 0.29→0.30, `portable-pty` 0.8→0.9)
3. Vendor `libghostty-vt` or depend on it as a submodule

**Estimated implementation size:** ~15,000–20,000 lines of original code, based on Herdr's scope as a reference.
