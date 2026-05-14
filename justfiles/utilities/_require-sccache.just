# Verify that sccache is available before running cached Cargo builds.
# Usage: just _require-sccache
# Example: just _require-sccache

_require-sccache:
    #!/usr/bin/env bash
    set -euo pipefail

    if ! command -v sccache >/dev/null 2>&1; then
        echo "sccache is required for cached Rust builds." >&2
        echo "Install it with: cargo install sccache --locked" >&2
        exit 1
    fi
