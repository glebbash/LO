# LO Playground

A self-hosted, vscode.dev-style playground for LO: a static site that serves the VS Code web build, with `glebbash.lo`, `ms-vscode.wasm-wasi-core` and the GitHub theme preloaded, plus two extensions of our own (below), and a persistent scratch workspace (`scratch:/`, in-memory, saved to IndexedDB, seeded on first start). Extra extensions can be installed from Open VSX.

## AI Use Disclosure

Most of this folder was AI generated and edited afterwards by hand where needed. The playground is almost entirely plumbing between existing libraries and services (the VS Code web build, wasm-wasi-core, wabt.js, Open VSX, Cloudflare Pages), and the work is mostly integration details that only show up at runtime. Author chose not to write that by hand or spend many hours debugging those integration issues. The code was reviewed and tested, but don't really expect to see any technical miracles in the code, only care about the result here, not the potentially messy code.

## Seeding

On the first start `scratch-fs` downloads `lo.wasm` from `raw.githubusercontent.com/glebbash/LO/main/lo.wasm` (so the playground always starts with the current compiler, and needs network), writes a `main.lo`, and runs `lo.wasm eject` through wasm-wasi-core, which unpacks `src/`, `lib/` and `test/` into the workspace. The result is persisted only after this succeeded; on failure an error is shown and a reload retries. `Scratch: Reset workspace to examples` wipes everything and seeds again.

## Default settings

`src/settings.json` holds the default user settings for everyone. To export your own: run **Preferences: Open User Settings (JSON)** in the playground, paste its content (or the keys you want) into `src/settings.json`, rebuild. They are defaults only, visitors can still change them, and they win over the ones hardcoded in `src/main.js` (object values like `workbench.colorCustomizations` are merged one level deep). The build converts the JSONC file to plain JSON for the page.

Not settings, so not covered: keybindings (use `contributes.keybindings` in an extension, see `extensions/lo-shell/package.json`) and layout state (panel/sidebar position and size, opened views; use `defaultLayout` in `create(...)` in `src/main.js`).

## Layout

| Path                      | What                                                                                                                                                                                                                                  |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `package.json` `config`   | The one place to pin the VS Code commit, the preloaded extensions and the default theme                                                                                                                                               |
| `scripts.js`              | `node scripts.js build`: builds `dist/` (downloads the pinned VS Code web build and the extensions from Open VSX, copies everything else). `node scripts.js serve [port]`: local static server for `dist/` with the COOP/COEP headers |
| `src/`                    | `index.html`, `main.js` (workbench bootstrap) and `about.js` (the About dialog opened by the bottom-left "LO Playground" indicator; a plain HTML `<dialog>` styled with theme variables, text and links are in there)                 |
| `extensions/scratch-fs/`  | `scratch:` file system (IndexedDB), explorer title "Scratch FS", `Scratch: Download workspace as zip`                                                                                                                                 |
| `extensions/wasm-viewer/` | Opens `.wasm` as WAT (read-only), WAT syntax highlighting, and the `wasm2wat`/`wat2wasm` used by lo-shell. wabt.js (Apache-2.0, npm `wabt`, pinned in `package.json` `config.wabt`) is fetched by the build into `vendor/wabt/`       |
| `extensions/lo-shell/`    | Shell in the terminal view (replaces `ms-vscode.webshell`, see below)                                                                                                                                                                 |
| `public/_headers`         | Cloudflare Pages headers (COOP/COEP)                                                                                                                                                                                                  |

## Local

Requirements: node 18+ (global `fetch`), `tar` and `unzip` on `PATH`, and network access (the build downloads the VS Code web build from `update.code.visualstudio.com`, extensions from `open-vsx.org` and `wabt` from the npm registry). There are no npm dependencies to install. Downloads are cached in `.cache/` (gitignored), output goes to `dist/` (about 110MB).

```sh
npm run build
npm run serve   # http://localhost:8080 (localhost counts as a secure context)
# or from anywhere: ./extras/playground/serve.sh [port]  (builds first if dist/ is missing, REBUILD=1 to force)
```

## Cloudflare Pages

Dashboard: Workers & Pages -> Create -> Pages -> Connect to Git -> pick the repo.

| Setting                | Value                       |
| ---------------------- | --------------------------- |
| Production branch      | `main`                      |
| Root directory         | `extras/playground`         |
| Build command          | `npm run build`             |
| Build output directory | `dist`                      |
| Environment variable   | `NODE_VERSION=22` (any 18+) |

What you need:

- A Cloudflare account and the GitHub repo connected to it.
- Nothing else: no secrets, no wrangler config, no GitHub Actions. Cloudflare builds on every push. `_headers` is picked up from the output dir automatically.
- Optional: a custom domain under the Pages project's Custom domains tab.

Limits checked: ~2.4k files (limit 20k) and the largest file is 22MB (limit 25MiB). The agents-window bundle is dropped from the build for that reason.

## Why not ms-vscode.webshell

webshell 0.13.0 (the only version on Open VSX and the marketplace) bundles the pre-1.0 `@vscode/wasm-wasi` client, which expects `wasm-wasi-core.activate()` to return the API directly. wasm-wasi-core 1.x returns an `APILoader` (`.load(1)`), so webshell fails with `Unable to parse WASM WASI Core extension version: undefined`. `lo-shell` is a ~150 line replacement modeled after webshell's source (it only needs the `.load(1)` call): builtins `cd pwd clear help exit`, coreutils (`ls cat mkdir rm ...`) via `coreutils.wasm`, which the build takes out of the webshell vsix, and `wasm2wat` / `wat2wasm`, and any `*.wasm` file in the workspace (`./lo.wasm compile in.lo > out.wasm`). It registers a terminal profile and is the default one, so the terminal view works out of the box (a shell is only created when you open the terminal; the only startup exception: if the terminal panel is restored as open from the saved UI state, which would be blank, `src/main.js` calls `lo-shell.ensure` to create one).

## Notes

- **Do not deploy this publicly as is.** The VS Code web build comes from `update.code.visualstudio.com` (Microsoft's `web-standalone` build, same as `@vscode/test-web` uses). Its bundled `LICENSE` ("Microsoft Pre-release Software License Terms, Visual Studio Code Remote Development Extensions") forbids to "host, share, publish, rent or lease the software" and to "provide the software as a stand-alone or integrated offering ... for others to use" (see also https://code.visualstudio.com/license/server). Running it locally for development is fine. To host it publicly the build base has to be switched to Code - OSS built from the MIT-licensed `microsoft/vscode` sources (`vscode-web-min` gulp task), which is not done yet. The Marketplace gallery that ships with the Microsoft build is replaced by Open VSX in `src/main.js`, for the same reason.
- Webviews use VS Code's default `{{uuid}}.vscode-cdn.net` origin (Cloudflare Pages has no wildcard subdomains for same-site isolated iframes). This only works for a commit that Microsoft has deployed to that CDN, so keep `vscodeCommit` at a released stable commit.
- Locally (`localhost`), `main.js` points the extension host/webview iframes at `{{uuid}}.localhost` instead of `vscode-cdn.net`, otherwise Chrome blocks them as Local Network Access requests. Open the page as `http://localhost:8080` (not the LAN IP).
- The page is cross-origin isolated (COOP `same-origin`, COEP `credentialless`) for `SharedArrayBuffer` in wasm-wasi-core. Check `crossOriginIsolated` in the console. `credentialless` instead of `require-corp` so that cross-origin images without a `Cross-Origin-Resource-Policy` header (extension icons from Open VSX, README images from GitHub in the extension details) still load. Browsers without `credentialless` support (Safari, as far as I know) won't get isolation, so wasm-wasi-core (running LO, the shell) won't work there.
- The terminal "+" button runs `workbench.action.terminal.new`, which in the web (no pty host) ends in a profile picker; `src/main.js` overrides that command (host commands from `create({ commands })` win over built-in ones) to open LO Shell directly. `...terminal.newWithProfile` must not be overridden, VS Code re-registers it on every profile refresh and throws.
- `Ctrl+\`` (toggle terminal) is rebound by `lo-shell`(the stock toggle misbehaves without a pty host) to mimic default VS Code: if a terminal has focus the panel closes; otherwise the terminal is focused (panel opened if hidden), and a LO Shell is created first when there is none. See`contributes.keybindings`in`extensions/lo-shell/package.json`.
- AI: `chat.disableAIFeatures` only affects the Copilot setup/entitlement UI (it is already hidden in the web), so the build also removes the chat-related built-in extensions and `main.js` sets a few `chat.*` defaults. Untested whether the chat panel/title-bar entries are fully gone.
