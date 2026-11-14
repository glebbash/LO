#!/usr/bin/env bash
# Builds the playground (if dist/ is missing) and serves it. Usage: ./serve.sh [port] ; set REBUILD=1 to force a rebuild
set -euo pipefail
cd "$(dirname "$0")"
if [ ! -d dist ] || [ -n "${REBUILD:-}" ]; then npm run build; fi
exec node scripts.js serve "${1:-8080}"
