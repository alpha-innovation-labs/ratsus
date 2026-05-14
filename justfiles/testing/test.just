# Run the Rust test suite with cargo-nextest and sccache using one test thread for real Nexus stability.
# Usage: just test
# Example: just test

test: _require-sccache _require-nextest
    RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0 cargo nextest run --test-threads 1
