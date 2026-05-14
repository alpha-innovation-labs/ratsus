# Install local Rust tooling used by repository commands.
# Usage: just install-tools
# Example: just install-tools

install-tools:
    #!/usr/bin/env bash
    set -euo pipefail

    rustup component add clippy rustfmt
    cargo install sccache --locked
    cargo install cargo-nextest --locked
