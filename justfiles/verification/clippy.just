# Run Clippy across all Rust targets, using sccache.
# Usage: just clippy
# Example: just clippy

clippy: _require-sccache
    RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0 cargo clippy --workspace --all-targets -- -D warnings
