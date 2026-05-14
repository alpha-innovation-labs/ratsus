# Terminal extension

`src/extensions/terminal/` owns terminal runtime behavior. It converts sessions into interactive PTY-backed or stub-backed terminal views and keeps normal terminal metadata persistent when the active harness allows it.

## Responsibilities

- Spawn PTY-backed chat and shell processes.
- Resolve the default shell command from the environment.
- Encode keyboard input into terminal byte sequences.
- Process PTY output into Ratkit terminal screen state.
- Reply to terminal protocol requests through the PTY writer.
- Resize PTYs and terminal parsers.
- Track whether backing terminal processes have exited.
- Support mouse and keyboard copy mode selection.
- Load and save app-owned normal terminal sessions.

## Terminal runtime

`PtyTerminal` opens a native PTY, spawns the requested command in a working directory, creates a parser, takes a writer, clones a reader, and starts a reader worker. Rendering uses Ratkit terminal screen rendering from parser state.

`ChatTerminal` wraps either a PTY terminal or a stub terminal. `SessionTerminal` pairs a `ChatSession` with a terminal that may be dormant until the UI activates it.

## Normal terminal persistence

Normal terminal sessions are app-owned and separate from Nexus chat sessions. They are persisted only when the active harness enables normal-terminal persistence. Stub mode disables loading and saving so deterministic runs do not touch user state.

The normal terminal registry path is owned by Ratsus and is distinct from Nexus conversation transcripts.

## Copy mode

Copy mode owns selection bounds, mouse hit conversion, keyboard and mouse handling, selected text extraction, clipboard copying, and rendering a selected screen region.

## Key files

- `src/extensions/terminal/process/pty_terminal.rs`
- `src/extensions/terminal/process/spawn_terminal_reader.rs`
- `src/extensions/terminal/process/process_terminal_output.rs`
- `src/extensions/terminal/process/default_shell_command.rs`
- `src/extensions/terminal/input/encode_key_event.rs`
- `src/extensions/terminal/session/chat_terminal.rs`
- `src/extensions/terminal/session/session_terminal.rs`
- `src/extensions/terminal/persistence/registry_path.rs`
- `src/extensions/terminal/persistence/load_normal_terminal_sessions.rs`
- `src/extensions/terminal/persistence/save_normal_terminal_sessions.rs`
- `src/extensions/terminal/copy_mode/`
