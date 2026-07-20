# Build a knowledge graph of a project using graphify.
# Usage: just graphify
# Example: just graphify

graphify:
    @echo "\033[1;36mBuilding knowledge graph...\033[0m"
    @if command -v graphify >/dev/null 2>&1; then \
        OPENAI_BASE_URL='http://localhost:4000/v1' \
        OPENAI_API_KEY='sk-1234' \
        OPENAI_MODEL='qwen/qwen3.6-35b-a3b' \
        graphify . && \
        graphify cluster-only . && \
        graphify export html; \
    else \
        echo "graphify not found. Install with: uv tool install graphifyy"; \
    fi
