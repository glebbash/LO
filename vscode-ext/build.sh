#!/usr/bin/env bash
set -e

main() {
    mkdir assets/initial-project
    cp -r ../examples/lib assets/initial-project
    cp ../examples/test/demos/hello-world.lo assets/initial-project/main.lo
    cp ../lo.wasm assets/initial-project/lo.wasm

    npm version patch
    npm run package
    npx vsce package -o lo.vsix

    rm -rf assets/initial-project
}

# cwd will be restored after subshell exits
(
  cd "$(dirname "${BASH_SOURCE[0]}")"
  main
)
