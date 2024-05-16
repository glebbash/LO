<p align="center">
  <img src="./vscode-ext/assets/icons/lo.svg" />
</p>

LO - Low level programming language for WASM with focus on simplicity

## 🪵 Development Log

🧾 [Parts [0..6] - Building the initial compiler. (text based blog)](https://carrot-blog.deno.dev/?tag=lo)

📺 [Parts [7..] - Building the self-hosted compiler. (YouTube playlist)](https://youtube.com/playlist?list=PL6qyEx0ybzWqkc0zG6jVgRx63nZkdu3DP&si=X8OyuWQ8TNDrfikL)

## 👀 Overview

- Hello world [(source)](examples/test/demos/hello-world.lo):

![Hello World sample](./docs/assets/hello-world.png)

- Advent of Code 2020 Day 1 [(source)](examples/test/demos/hello-world.lo):

![AOC 2020 sample](./docs/assets/aoc-2020-day1.png)

## 🚀 Getting started

### Option 1 (recommended): Using VS Code extension only

> This option also works in vscode.dev

- Install the [LO VS Code extension](https://marketplace.visualstudio.com/items?itemName=glebbash.lo)
- To run currently open file: Command Palette: `LO: Run current file`, or press ▶️ button in the top toolbar
- Compiling files:
  - Open Web Shell using Command Palette: `Terminal: Create New Web Shell`
  - Run: `run lo.wasm <input>.lo > <output>.wasm`

### Option 2: Using wasmtime

- Install [wasmtime](https://github.com/bytecodealliance/wasmtime)
- Compiling files: `wasmtime --dir=. lo.wasm <input>.lo > <output>.wasm`
- Getting diagnostics (in json format): `wasmtime --dir=. lo.wasm <input>.lo --inspect`

### Option 3: Using Node.js

- Install [Node.js](https://github.com/bytecodealliance/wasmtime)
- Compiling files: `./utils.mjs compile <input>.lo > <output>.wasm`
- Compiling & running files: `./utils.mjs run <input>.lo`

## 🦀 Building the initial compiler

- Requirements:
  - Install [rustup](https://www.rust-lang.org/tools/install)
  - Switch to nightly: `rustup toolchain install nightly`
  - Add WASM target: `rustup target add  wasm32-unknown-unknown`
  - You can also find configs for GitHub Codespaces and GitPod in this repo
- Run `./build.sh`

  > This will build the compiler with cargo, putting resulting WASM binary into `lo.wasm`

## 🧪 Running tests

- Requirements:
  - Install [Node.js](https://nodejs.org/en/download/package-manager) for running tests
- Run `./utils.mjs test`

  > This runs tests defined in `utils.mjs`. Test programs are located in `examples/test`
