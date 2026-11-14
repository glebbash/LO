# LO Playground

A vscode.dev-style playground for the LO language that runs entirely in the browser. It is a static site: a VS Code web build with the LO extension, wasm-wasi-core and the GitHub theme preloaded, a persistent scratch workspace (saved in IndexedDB), a shell in the terminal view and a `.wasm` to WAT viewer. More extensions can be installed from Open VSX.

## AI Use Disclosure

Most of this folder was AI generated and edited afterwards by hand where needed. The playground is almost entirely plumbing between existing libraries and services (the VS Code web build, wasm-wasi-core, wabt.js, Open VSX, Cloudflare Pages), and the work is mostly integration details that only show up at runtime. Author chose not to write that by hand or spend many hours debugging those integration issues. The code was reviewed and tested, but don't really expect to see any technical miracles in the code, only care about the result here, not the potentially messy code.

## Local

Requirements: node 18+, `tar` and `unzip` on `PATH`, and network access. The build downloads the VS Code web build and `wabt` from the npm registry and extensions from `open-vsx.org`. There are no npm dependencies. Downloads are cached in `.cache/`, the output goes to `dist/` (about 100MB).

```sh
npm run build
npm run serve          # http://localhost:8080
./serve.sh [port]      # same, builds first if dist/ is missing (REBUILD=1 forces it)
```

Open it as `localhost`, the page needs a secure context.

## Layout

| Path                      | What                                                                                                                                   |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `package.json` `config`   | Pinned versions (VS Code web build, wabt), the preloaded extensions and the default theme                                              |
| `scripts.js`              | `build` creates `dist/`, `serve [port]` is a static server for it that sends the headers from `public/_headers`                        |
| `src/`                    | `index.html`, `main.js` (workbench setup), `about.js` (About dialog) and `playground.code-profile` (default profile)                   |
| `extensions/scratch-fs/`  | The `scratch:` file system, file and text search, first start seeding, workspace zip download                                          |
| `extensions/lo-shell/`    | Shell in the terminal view                                                                                                             |
| `extensions/wasm4/`       | Runs WASM-4 games written in LO in an editor tab, rebuilds them on save, exports a standalone HTML page                                |
| `extensions/wasm-viewer/` | Opens `.wasm` files as WAT, WAT highlighting, `wasm2wat` and `wat2wasm` for the shell                                                  |
| `public/_headers`         | Headers for Cloudflare Pages                                                                                                           |

## Deploying to Cloudflare Pages

Connect the repo in the Cloudflare dashboard (Workers & Pages, Create, Pages, Connect to Git) with these settings:

| Setting                | Value                       |
| ---------------------- | --------------------------- |
| Production branch      | `main`                      |
| Root directory         | `extras/playground`         |
| Build command          | `npm run build`             |
| Build output directory | `dist`                      |
| Environment variable   | `NODE_VERSION=22` (any 18+) |

No secrets, wrangler config or CI are needed, and `_headers` is picked up from the output directory. Cloudflare Pages allows 20000 files and 25 MiB per file. The build has about 1700 files and the largest is the 17 MiB workbench bundle.

## Default profile

`src/playground.code-profile` is the starting point of a new visitor: user settings, UI state (hidden buttons and views, panel layout) and optionally keybindings and snippets. It is a VS Code profile export that `src/main.js` passes as the `profile` option, and the build fails without it. The few `configurationDefaults` in `src/main.js` (theme, remote indicator color, window title, AI features off) apply underneath it.

To change it:

1. Configure the playground the way you want it. Settings are exported from the user `settings.json`, defaults from `main.js` are not part of it.
2. Run **Preferences: Export Profile...**, select Settings, UI State and whatever else you need (not Extensions, they are preloaded by the build) and save it into the workspace.
3. Download the file (see below) and replace `src/playground.code-profile` with it. Delete `recently.opened` from `globalState`, it is the list of recent folders.
4. Rebuild.

A profile is applied once, to a browser without saved state, so test it in a private window. Changing the profile name (in `src/main.js` and in the file) gives everybody a fresh profile.

## Seeding

On the first start `scratch-fs` downloads `lo.wasm` from `raw.githubusercontent.com/glebbash/LO/main/lo.wasm`, writes a `main.lo` and runs `lo.wasm eject`, which unpacks `src/`, `lib/` and `test/` into the workspace. Nothing is saved until this succeeded, a failure shows an error and the next reload tries again. `Scratch: Reset workspace to examples` clears the saved workspace and seeds again.

## Downloading files

- **Whole workspace:** `Scratch: Download workspace as zip`. In a single-folder workspace the root cannot be selected in the Explorer, so the built-in Download cannot do it.
- **A file or folder:** the Explorer's **Download...**. A folder is written file by file into a directory you pick, which needs the File System Access API, so it works in Chromium based browsers only.

## WASM-4

Open a LO file for the [WASM-4](https://wasm4.org) fantasy console (the ejected `test/demos/wasm4/src/` has samples) and use the game icon in the editor title, or `WASM-4: Run current file`. The file is compiled with the `lo.wasm` in the workspace root and the game opens in a tab next to the editor. Click the game to give it the keyboard (arrow keys, `X` and `Z`, `R` restarts). While the tab is open, saving any `.lo` file rebuilds the game and reloads it. The title bar of the game tab has three buttons: rebuild and restart the game now (always a full reload, `debug-rerun` icon), a toggle for reloading on save (setting `wasm4.rebuildOnSave`, `sync` when on and `sync-ignored` when off) and a toggle for hot swap (setting `wasm4.hotSwap`, keeps the memory of the game on reload, `beaker` when on and `beaker-stop` when off). VS Code's own toggle highlight is not used because themes do not render it clearly, so each toggle is a pair of commands with an icon per state. Build errors are in the "WASM-4" output.

`WASM-4: Export HTML` writes a standalone page next to the source file (`dark-maze.lo` becomes `dark-maze.html`) with the runtime and the cart inside, the same format as `w4 bundle --html`. The message that follows has a Download button.

## Implementation notes

**VS Code build.** The web client is the `@github1s/vscode-web` npm package: Code - OSS (MIT), built from the `microsoft/vscode` sources with a few patches by the github1s project, and a few months behind stable. Microsoft's own `web-standalone` build is not used, its license (https://code.visualstudio.com/license/server) does not allow to "host, share, publish, rent or lease the software", so it is fine for local development only. The fallback is building Code - OSS from source (`vscode-web-min` gulp task). The package's `vscode/` folder is served as `/vscode/out/`, its `dependencies/` and `extensions/` folders keep their names, the workbench loads files from them by path.

**Extension gallery.** The Marketplace is only for Microsoft's products, so `productConfiguration` in `src/main.js` points the gallery at Open VSX. `product.json` is merged deeply, which is why the unused Microsoft endpoints are set to empty strings.

**Built-in extensions.** The manifests of all built-in extensions are part of the workbench bundle, so deleting an extension's folder leaves it registered and VS Code logs an activation error. The build handles unwanted built-ins in two ways. The JS/TS, HTML and CSS language servers and the chat features get a no-op stub at the entry point of their manifest (grammars stay, so highlighting works). The Mermaid extension (59MB of bundles) is removed completely: its folder is deleted and its entry is cut out of the list in the bundle (`dropBuiltinExtension` in `scripts.js`). The API proposals for the search providers of `scratch-fs` are added in `src/main.js`.

**Cross-origin isolation.** wasm-wasi-core needs `SharedArrayBuffer`, so every response carries COOP `same-origin` and COEP `require-corp` (`public/_headers`, and the same in `scripts.js serve`). `credentialless` would be more forgiving but Safari does not support it, and `require-corp` works everywhere, iPad included. The price: a cross-origin `<img>` only loads if the server sends a `Cross-Origin-Resource-Policy` header or CORS headers and the request uses CORS mode. Open VSX sends CORS headers only (https://github.com/eclipse-openvsx/openvsx/issues/2047), so the build patches the extension icon widget in the workbench bundle to request its icons with `crossorigin="anonymous"` (`patchExtensionIcons` in `scripts.js`). Images in webviews, like those in an extension's README, are not patched, they load only from servers that send CORP or CORS headers.

**Webviews and extension host.** The build has no `vscode-cdn.net` endpoint and static hosting has no wildcard subdomains, so webviews are served from the page's origin (`webviewEndpoint`) and the extension host iframe is same-origin as well, which is VS Code's default without `webEndpointUrlTemplate`. They are not isolated from the page by origin.

**Explorer title.** The workspace folder is `scratch:/Scratch FS`. VS Code shows the last path segment, and "/" for a root folder. `scratch-fs` keeps its files below that folder.

**Path separators.** VS Code formats paths in labels (Copy Path, tooltips) with the separator of the OS the browser runs on, which gives backslashes on Windows. `scratch-fs` contributes a `resourceLabelFormatters` entry for the `scratch` scheme with `/` as the separator.

**Search.** Quick Open and the Search view need a file and a text search provider for the scheme, otherwise they only show recent files. `scratch-fs` implements both with the `fileSearchProvider` and `textSearchProvider` proposals.

**Terminal.** There is no pty host in the browser, so the terminal only works through extensions:

- webshell 0.13.0 (the only version on Open VSX) bundles the pre-1.0 `@vscode/wasm-wasi` client and expects `wasm-wasi-core.activate()` to return the API directly. wasm-wasi-core 1.x returns a loader (`.load(1)`) instead, so webshell fails with `Unable to parse WASM WASI Core extension version`. `lo-shell` replaces it, using webshell's `coreutils.wasm`. The build takes that file out of the webshell vsix.
- The terminal "+" button runs `workbench.action.terminal.new`, which here ends in a profile picker with one entry. A command registered through `create({ commands })` is placed in front of the built-in one, so `src/main.js` overrides it to open `lo-shell`. `workbench.action.terminal.newWithProfile` must not be overridden, VS Code registers it again on every profile refresh and throws if it exists.
- The toggle terminal keybinding only closes the panel once the terminal has focus and does nothing without a terminal, so `lo-shell` rebinds it (`contributes.keybindings`): close when a terminal has focus, focus it otherwise, create a shell if there is none.
- A terminal panel restored as open from the saved UI state is empty, `src/main.js` calls `lo-shell.ensure` for that case.
- Programs get the current directory mounted at `/`, relative paths like `compile main.lo` need that. `argv[0]` is added by wasm-wasi-core, so the arguments passed to `createProcess` start after the program name.

**WASM-4.** The extension uses two builds of the runtime from the `wasm4` npm package (ISC license, version in `package.json` `config.wasm4`): the developer build with devtools in the editor tab, and the slim build in the exported page. The developer build expects the `w4 watch` server: it fetches `cart.wasm` and waits for `reload` and `hotswap` messages on a WebSocket. The tab replaces both with a small shim (`devServerShim` in the extension), so a rebuild only posts the new cart to the webview. The cart is compiled in the extension host by running `lo.wasm compile` with wasm-wasi-core, like the seeding does with `eject`. Only saved files are compiled, so the editor saves the active file before running. The export is checked against the committed `extras/assets/dark-maze.html` made by `w4 bundle`. The developer runtime also registers its key handler on `top`. For a cross-origin webview that throws and is ignored, but here the webview is same-origin, so the game would receive and swallow every key typed in the editor (it claims letters, space, `2`, `4` and `R`). The build removes that registration from the vendored copy and fails if it cannot find it, which happens when the runtime version changes.

**Downloads.** An extension runs in a web worker and cannot start a browser download. `src/main.js` registers `playground.download`, which `scratch-fs` calls with the zip.

**AI features.** `chat.disableAIFeatures` only hides the Copilot setup UI, which is already hidden in the web, so a few more `chat.*` settings are set in `src/main.js`.
