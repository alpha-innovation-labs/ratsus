# Ratsus

Nexus session TUI wrapper built with the public `ratkit` crate from crates.io.

## Run

```bash
just dev
```

## Repository organization

- `src/main.rs` wires the Ratkit coordinator app and delegates event/render work.
- `src/nexus/` holds one concern per file for session state, terminal IO, layout, rendering, selection, clipboard, and mouse/keyboard handling.
- `.agents/skills/ratkit/` bundles the local Ratkit agent reference used by Nexus contributors.
