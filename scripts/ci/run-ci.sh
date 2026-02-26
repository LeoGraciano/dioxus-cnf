#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"

(
  cd "$root/apps/frontend/packages/web"
  dx build --platform web
  dx build --platform server
)
(
  cd "$root/apps/frontend/packages/mobile"
  dx build --platform android
  dx build --platform ios
)
(
  cd "$root/apps/frontend/packages/desktop"
  dx build --platform desktop
)
