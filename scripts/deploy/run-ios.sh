#!/usr/bin/env bash
set -euo pipefail

# Placeholder for iOS deploy pipeline
# Expected: build artifacts from dx build --platform ios

root="$(git rev-parse --show-toplevel)"
cd "$root/apps/frontend/packages/mobile"

echo "TODO: deploy iOS artifacts (e.g., upload via Xcode/Transporter)"
