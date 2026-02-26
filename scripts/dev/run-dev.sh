#!/usr/bin/env bash
set -euo pipefail

platform="${1:-web}"

case "$platform" in
  web)
    cd "$(git rev-parse --show-toplevel)/apps/frontend/packages/web"
    exec dx serve
    ;;
  mobile)
    cd "$(git rev-parse --show-toplevel)/apps/frontend/packages/mobile"
    exec dx serve
    ;;
  desktop)
    cd "$(git rev-parse --show-toplevel)/apps/frontend/packages/desktop"
    exec dx serve
    ;;
  *)
    echo "Usage: scripts/start-dev.sh [web|mobile|desktop]" >&2
    exit 1
    ;;
 esac
