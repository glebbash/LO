#!/usr/bin/env bash
set -e

main() {
    npm version patch
    npm run build
    npx vsce package -o lo.vsix
}

# cwd will be restored after subshell exits
(
  cd "$(dirname "${BASH_SOURCE[0]}")"
  main
)
