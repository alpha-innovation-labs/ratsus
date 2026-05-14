# Build and install the optimized ratsus binary on the local Cargo PATH with sccache.
# Usage: just release
# Example: just release

release: _require-sccache
    RUSTC_WRAPPER=sccache CARGO_INCREMENTAL=0 cargo install --path . --locked --force
