# LO language support

This extension adds the following commands:
- `LO: Initialize project in the current workspace`
- `LO: Run current file`

To compile `.lo` files to `.wasm`:
  - Open Web Shell using Command Palette: `Terminal: Create New Web Shell`
  - Run: `run lo.wasm <input>.lo > <output>.wasm`

> Check [the source](https://github.com/glebbash/LO) for more info

## Install the VS Code extension

> NOTE: This extension depends on [wasm-wasi-core](https://marketplace.visualstudio.com/items?itemName=ms-vscode.wasm-wasi-core) and [webshell](https://marketplace.visualstudio.com/items?itemName=ms-vscode.webshell) which currently have no release versions so you'll need to click `Install Pre-Release Version` to install them.

1. Open the command palette in VS Code: `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
2. Type and select `Extensions: Install from VSIX...`. The file selection prompt should open.
3. Navigate to the extension directory, `vscode-ext`.
4. Select `lo.vsix`.
5. Click `Install`.
