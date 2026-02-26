#!/usr/bin/env bash
set -euo pipefail

# Placeholder for desktop deploy pipeline
# Expected: build artifacts from dx build --platform desktop

root="$(git rev-parse --show-toplevel)"
cd "$root/apps/frontend/packages/desktop"

echo "TODO: deploy desktop artifacts (e.g., package + upload)"
