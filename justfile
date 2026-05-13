# Run the Nexus TUI wrapper.
dev:
    cargo run

# Run the TUI with deterministic stub data.
dev-stub:
    RATSUS_BACKEND=stub cargo run

# Build and install the optimized ratsus binary on the local Cargo PATH.
release:
    cargo install --path . --locked --force

# Run the test suite.
test:
    cargo test

# Check formatting.
fmt-check:
    cargo fmt --check

# Format source files.
fmt:
    cargo fmt
