# LO language support

> NOTE: This extension depends on [wasm-wasi-core](https://marketplace.visualstudio.com/items?itemName=ms-vscode.wasm-wasi-core) which currently has no release version so you'll need to click `Install Pre-Release Version` to install it.

This extension adds the following commands:
- `LO: Initialize project in the current workspace`
- `LO: Run current file`

To compile `.lo` files to `.wasm` (using [WebShell](https://marketplace.visualstudio.com/items?itemName=ms-vscode.webshell)):
  - Open Web Shell using Command Palette: `Terminal: Create New Web Shell`
  - Run: `run lo.wasm <input>.lo > <output>.wasm`

To compile `.lo` files to `.wasm` (using [wasmtime](https://wasmtime.dev/)):
  - Run: `wasmtime --dir=. lo.wasm <input>.lo > <output>.wasm`

> Check [the source](https://github.com/glebbash/LO) for more info
