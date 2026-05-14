# Verify that cargo-nextest is available before running the test suite.
# Usage: just _require-nextest
# Example: just _require-nextest

_require-nextest:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! cargo nextest --version >/dev/null 2>&1; then
        echo "cargo-nextest is required for parallel test-binary execution." >&2
        echo "Install it with: cargo install cargo-nextest --locked" >&2
        exit 1
    fi
