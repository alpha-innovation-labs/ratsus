# Run the TUI with deterministic stub data.
# Usage: just dev-stub
# Example: just dev-stub

dev-stub:
    RATSUS_BACKEND=stub cargo run
