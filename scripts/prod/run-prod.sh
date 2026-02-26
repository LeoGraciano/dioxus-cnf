#!/usr/bin/env bash
set -euo pipefail

platform="${1:-all}"
root="$(git rev-parse --show-toplevel)"

case "$platform" in
  web)
    cd "$root/apps/frontend/packages/web"
    exec dx build --platform web --release
    ;;
  server)
    cd "$root/apps/frontend/packages/web"
    exec dx build --platform server --release
    ;;
  android)
    cd "$root/apps/frontend/packages/mobile"
    exec dx build --platform android --release
    ;;
  ios)
    cd "$root/apps/frontend/packages/mobile"
    exec dx build --platform ios --release
    ;;
  desktop)
    cd "$root/apps/frontend/packages/desktop"
    exec dx build --platform desktop --release
    ;;
  all)
    (
      cd "$root/apps/frontend/packages/web"
      dx build --platform web --release
      dx build --platform server --release
    )
    (
      cd "$root/apps/frontend/packages/mobile"
      dx build --platform android --release
      dx build --platform ios --release
    )
    (
      cd "$root/apps/frontend/packages/desktop"
      dx build --platform desktop --release
    )
    ;;
  *)
    echo "Usage: scripts/start-prod.sh [all|web|server|android|ios|desktop]" >&2
    exit 1
    ;;
 esac
