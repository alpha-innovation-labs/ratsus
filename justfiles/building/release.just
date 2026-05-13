# Build and install the optimized ratsus binary on the local Cargo PATH.
# Usage: just release
# Example: just release

release:
    cargo install --path . --locked --force
