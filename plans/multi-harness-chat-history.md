# Multi-Harness Chat History Reference

Where each external agent harness stores its chat transcripts, and how to read them.

---

## 1. Claude Code

**Storage Location:**
- Primary: `~/.claude/projects/<project>/<session-id>.jsonl`
- Config override: `CLAUDE_CONFIG_DIR` env var
- `<project>` is derived from the absolute working directory path (encoded, not the raw path)

**File Format:** JSONL (one JSON object per line)

```json
{"type": "user", "message": {"role": "user", "content": "Hello"}, "isMeta": false}
{"type": "assistant", "message": {"role": "assistant", "content": "Hi there"}, "isMeta": false}
{"type": "user", "message": {"role": "user", "content": "Write a function"}, "isMeta": false}
```

**Schema:**
- `type`: `"user" | "assistant"` (skip if `"isMeta": true`)
- `message.role`: mirrors `type`
- `message.content`: string or structured content

**How to Fetch:**
1. Enumerate `~/.claude/projects/*/*.jsonl`
2. Sort by mtime (newest first)
3. Stream-read lines, parse JSON
4. Filter out `isMeta: true` records

**Resume Command:** `claude --resume <session-id-or-name>`

**Source:** [code.claude.com/docs/en/sessions.md](https://code.claude.com/docs/en/sessions.md)

---

## 2. Codex

**Storage Location:**
- Primary: `~/.codex/sessions/<YYYY>/<MM>/<DD>/rollout-<uuid>.jsonl` (dated subdirs)
- Archived: `~/.codex/archived_sessions/`
- Index: `~/.codex/state.db` (SQLite, separate from the rollout files)
- Override: `codex_home` from `Config` (set via `CODEX_HOME`)

**File Format:** JSONL with response items

```json
{"type": "response_item", "payload": {"type": "message", "role": "user", "content": "Hello"}}
{"type": "response_item", "payload": {"type": "message", "role": "assistant", "content": "Hi"}}
{"type": "response_item", "payload": {"type": "function_call", "name": "read_file", "arguments": {"path": "src/main.rs"}}}
```

**Schema:**
- `payload.type`: `"message" | "function_call" | "function_call_output"`
- For messages: `payload.role` (`"user" | "assistant"`), `payload.content`
- For tool calls: `payload.name`, `payload.arguments`

**How to Fetch:**
1. Open `~/.codex/state.db` read-only (use `StateDbHandle`) to list threads
2. For each thread, resolve its rollout path from the index
3. Stream-read the dated JSONL, parse lines
4. Map `function_call` → tool role, `message` → user/assistant role

**Resume Command:** `codex resume <session-id>`

**Source:** [github.com/openai/codex](https://github.com/openai/codex) — `codex-rs/rollout/src/lib.rs` defines `SESSIONS_SUBDIR`, `ARCHIVED_SESSIONS_SUBDIR`, and the `state_db` module.

---

## 3. OpenCode

**Storage Location:**
- SQLite database at the OpenCode data directory:
  - macOS: `~/Library/Application Support/opencode/opencode.db`
  - Linux: `~/.local/share/opencode/opencode.db`
  - Windows: `%APPDATA%\opencode\opencode.db`
- Resolved at runtime via `Global.Path.data` from `@opencode-ai/core/global`

**Access Pattern:** Drizzle ORM over the schema in `@opencode-ai/core/session/sql` and `@opencode-ai/core/project/sql`. Do **not** write raw `SELECT` statements — the schema is owned by Drizzle migrations.

**Tables (logical columns):**
- `SessionTable` — `id`, `slug`, `project_id`, `workspace_id`, `parent_id`, `directory`, `path`, `title`, `agent`, `model`, `version`, `share_url`, `summary_*`, `cost`, `tokens_*`, `metadata`, `permission`, `revert`, `time_created`, `time_updated`, `time_compacting`, `time_archived`
- `PartTable` — `id`, `session_id`, `message_id`, `data` (JSON-encoded part payload)
- `ProjectTable` — `id`, `name`, `worktree`
- `MessageV2` — managed via `MessageV2.page({ sessionID, limit, before })` for pagination

**How to Fetch:**
1. Resolve the DB path from `Global.Path.data + "/opencode.db"`
2. Open the SQLite database read-only (Drizzle SQLite driver)
3. Use `SessionTable` to list sessions (filter by `directory`, `time_updated`, `parent_id` as needed)
4. For each session, page through messages with `MessageV2.page(...)`
5. For each message, load parts from `PartTable` and decode the `data` JSON

**Resume Command:** `opencode --session <session-id>`

**Source:** [github.com/anomalyco/opencode](https://github.com/anomalyco/opencode) — `packages/opencode/src/session/session.ts` and `@opencode-ai/core/session/sql`.

---

## 4. Hermes Agent

**Storage Location:**
- Linux / WSL2: `~/.hermes/state.db`
- Native Windows: `%LOCALAPPDATA%\hermes\state.db`
- Override: `get_hermes_home()` from `hermes_constants.py` (verify the env var name in that module — `HERMES_HOME` is the expected name but unverified)

**Database Schema:**

```sql
-- sessions
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL,           -- 'cli' | 'telegram' | 'discord' | ...
    user_id TEXT,
    model TEXT,
    model_config TEXT,
    system_prompt TEXT,
    parent_session_id TEXT REFERENCES sessions(id),
    started_at REAL NOT NULL,
    ended_at REAL,
    end_reason TEXT,
    message_count INTEGER DEFAULT 0,
    tool_call_count INTEGER DEFAULT 0,
    input_tokens INTEGER DEFAULT 0,
    output_tokens INTEGER DEFAULT 0,
    cache_read_tokens INTEGER DEFAULT 0,
    cache_write_tokens INTEGER DEFAULT 0,
    reasoning_tokens INTEGER DEFAULT 0,
    cwd TEXT,
    title TEXT,
    api_call_count INTEGER DEFAULT 0,
    archived INTEGER NOT NULL DEFAULT 0
);

-- messages
CREATE TABLE messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL REFERENCES sessions(id),
    role TEXT NOT NULL,
    content TEXT,
    tool_call_id TEXT,
    tool_calls TEXT,
    tool_name TEXT,
    timestamp REAL NOT NULL,
    token_count INTEGER,
    finish_reason TEXT,
    reasoning TEXT,
    reasoning_content TEXT,
    reasoning_details TEXT,
    platform_message_id TEXT,
    observed INTEGER DEFAULT 0,
    active INTEGER NOT NULL DEFAULT 1
);
```

Full-text search via virtual tables `messages_fts` and `messages_fts_trigram` (trigram tokenizer for CJK substring matching).

**How to Fetch:**
1. Open `state.db` with `sqlite3.connect(...)`, `check_same_thread=False`, `timeout=1.0`
2. Set `PRAGMA journal_mode=WAL` — falls back to `DELETE` mode on NFS/SMB/FUSE (locking-protocol errors)
3. Read sessions with `SELECT ... FROM sessions WHERE source IN ('cli', 'tui') ORDER BY started_at DESC`
4. Read transcripts with `SELECT ... FROM messages WHERE session_id = ? ORDER BY timestamp, id`
5. Decode `messages.content` — may be prefixed with `\x00json:` for JSON payloads
6. Do **not** snapshot the DB to temp — open in place with WAL/DELETE fallback

**Resume Command:** Varies by source (CLI: `hermes --resume <id>`; gateway platforms have their own resume UX)

**Source:** [github.com/NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) — `hermes_state.py` (full `SessionDB` class, schema, WAL fallback logic).

---

## 5. Pi (Pi Coding Agent)

**Storage Location:**
- Storage directory resolved by `getAgentDir()` from `packages/coding-agent/src/config.ts`
- Session layout: per-session JSONL files under the agent dir (subpath shape to verify against the `SessionManager` implementation)
- Override: any env var or config key defined in `config.ts` for the agent dir (verify the exact name — `PI_CODING_AGENT_DIR` is the expected name but unverified)

**File Format:** JSONL with structured session entries (`SessionEntry` discriminated union: `SessionMessageEntry`, `FileEntry`, `CompactionEntry`, `ModelChangeEntry`, `ThinkingLevelChangeEntry`, `CustomEntry`, etc.)

```json
{"type": "session", "version": "..."}
{"type": "message", "id": "...", "role": "user", "content": "Hello"}
{"type": "message", "id": "...", "role": "assistant", "content": "Hi"}
{"type": "part", "id": "...", "messageID": "...", "data": {"type": "text", "text": "..."}}
```

**How to Fetch:**
1. Resolve the agent directory via `getAgentDir()`
2. Enumerate session files (verify glob pattern against `SessionManager.list()`)
3. For each session, parse the JSONL stream of `SessionEntry` records
4. Walk `SessionMessageEntry` rows to get the transcript; join against part rows for full content

**Resume Command:** `pi --session <session-id-or-path>`

**Source:** [github.com/earendil-works/pi](https://github.com/earendil-works/pi) — `packages/coding-agent/src/index.ts` exports `getAgentDir` and `SessionManager`. Exact per-session subpath and env-var override names need confirmation from the `SessionManager` source.

**Verification status:** Storage root confirmed; per-session subpath and env-var override name need to be checked against the actual `SessionManager` source before relying on them.

---

## 6. Gemini CLI

**Storage Location:**
- Base dir: `~/.gemini/` (or `$GEMINI_CLI_HOME/.gemini/`)
- Override: `GEMINI_CLI_HOME` env var (changes the home used to resolve `~/.gemini/`)
- Project key: SHA256 of the absolute working directory path, hex-encoded

**File Format:** JSONL with typed records

```json
{"id": "msg-1", "timestamp": "2025-01-15T10:30:00Z", "content": [{"text": "Hello"}], "type": "user"}
{"id": "msg-2", "timestamp": "2025-01-15T10:30:05Z", "content": [{"text": "Hi there"}], "type": "gemini", "model": "gemini-2.5-flash", "tokens": {"input": 1000, "output": 500, "total": 1500}}
```

**Schema:**
- `type`: `"user" | "gemini" | "info" | "error" | "warning"`
- `content`: array of Part objects (usually `[{"text": "..."}]`)
- `toolCalls`: array of tool call records (for gemini type)
- `model`: model name
- `tokens`: token usage summary

**How to Fetch:**
1. Compute `project_hash = sha256(absolute_cwd).hex()` to find the project-scoped subdir
2. Enumerate the per-project subdir under `~/.gemini/` (verify the exact subpath against the history module — likely `tmp/<hash>/` or `history/<hash>/`)
3. Stream session JSONL files, parse JSON
4. Map `type: "gemini"` → assistant role, `type: "user"` → user role

**Resume Command:** Not yet standardized (Gemini CLI uses checkpointing)

**Source:** [github.com/google-gemini/gemini-cli](https://github.com/google-gemini/gemini-cli) — `packages/core/src/utils/paths.ts` defines `GEMINI_DIR = '.gemini'`, `GEMINI_CLI_HOME` override, and `getProjectHash()` as `crypto.createHash('sha256').update(projectRoot).digest('hex')`.

**Verification status:** Hash algorithm and base dir confirmed; exact history subpath under `~/.gemini/` (e.g. `tmp/` vs `history/`) needs confirmation from the CLI's history module.

---

## References

- cmux Vault implementation: `Packages/CMUXAgentVault/Sources/`
- cmux SessionIndexStore: `Sources/SessionIndexStore.swift`
- herdr integration assets: `src/integration/assets/`
