#!/usr/bin/env bash
# Serves the playground, building it first if needed. Usage: [REBUILD=1] ./serve.sh [port]
set -euo pipefail
cd "$(dirname "$0")"
if [ ! -d dist ] || [ -n "${REBUILD:-}" ]; then npm run build; fi
exec node scripts.js serve "${1:-8080}"
