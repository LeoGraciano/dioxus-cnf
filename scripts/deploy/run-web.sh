#!/usr/bin/env bash
set -euo pipefail

# Placeholder for web deploy pipeline
# Expected: build artifacts in apps/frontend/packages/web or a CI artifact.

root="$(git rev-parse --show-toplevel)"
cd "$root/apps/frontend/packages/web"

echo "TODO: deploy web artifacts (e.g., upload to CDN / static host)"
