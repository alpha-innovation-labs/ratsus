# Run the Nexus TUI wrapper with sccache-backed builds.
# Usage: just dev
# Example: just dev

dev: _require-sccache
    RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0 cargo run
