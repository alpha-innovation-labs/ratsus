# Compile all Rust targets without producing final binaries, using sccache.
# Usage: just cargo-check
# Example: just cargo-check

cargo-check: _require-sccache
    RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0 cargo check --workspace --all-targets
