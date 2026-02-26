#!/usr/bin/env bash
set -euo pipefail

# Placeholder for Android deploy pipeline
# Expected: build artifacts from dx build --platform android

root="$(git rev-parse --show-toplevel)"
cd "$root/apps/frontend/packages/mobile"

echo "TODO: deploy Android artifacts (e.g., upload to Play Console)"
