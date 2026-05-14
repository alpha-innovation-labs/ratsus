# Run the full local verification suite.
# Usage: just check
# Example: just check

check:
    #!/usr/bin/env bash
    set -euo pipefail

    just fmt-check
    just cargo-check
    just clippy
    just test
