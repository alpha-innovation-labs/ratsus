# Ratsus

Harness-backed session TUI built with the public `ratkit` crate from crates.io.

## Run

```bash
just dev       # real Nexus harness
just dev-stub  # deterministic stub harness; no Nexus CLI or Nexus data paths
```

## Verification

Install the local tooling once:

```bash
just install-tools
```

`just check` runs formatting verification, Cargo check, Clippy, and tests. Cargo build/test recipes use `RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0`; `just test` uses `cargo-nextest` for parallel test-binary scheduling.

## Repository organization

- `src/main.rs` selects the requested harness and starts the Ratkit app.
- `src/harness/` defines backend-neutral chat contracts.
- `src/harnesses/` contains concrete Nexus and stub harnesses.
- `src/app/`, `src/terminal/`, and UI folders keep event, render, and terminal orchestration code.
