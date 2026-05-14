# Run the TUI with deterministic stub data and sccache-backed builds.
# Usage: just dev-stub
# Example: just dev-stub

dev-stub: _require-sccache
    RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0 RATSUS_BACKEND=stub cargo run
