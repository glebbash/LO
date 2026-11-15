// Runs LO programs for the WASM-4 fantasy console (https://wasm4.org) in an editor tab. The program is compiled with the
// lo.wasm of the workspace, shown with the WASM-4 developer runtime (vendor/wasm4/) and rebuilt when a file is saved.
const vscode = require("vscode");

const Z85 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ.-:+=^!/*?&<>()[]{}@%$#";

/** z85 encoding of the cart, the way `w4 bundle` embeds it. */
function z85(src) {
    const padded = src.length + ((4 - (src.length % 4)) % 4);
    let out = "";
    let value = 0;
    for (let i = 0; i < padded; i++) {
        value = value * 256 + (i < src.length ? src[i] : 0);
        if ((i + 1) % 4 === 0) {
            for (let divisor = 85 ** 4; divisor >= 1; divisor /= 85) out += Z85[Math.floor(value / divisor) % 85];
            value = 0;
        }
    }
    return out;
}

const escapeJs = (s) => s.replace(/<\//g, "<\\u002F").replace(/<!/g, "<\\u0021");
const escapeCss = (s) => s.replace(/<\//g, "<\\002F").replace(/<!/g, "<\\0021");
const escapeHtml = (s) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
const stripSourceMap = (s) => s.replace(/\/\/# sourceMappingURL=.*$/m, "");

function toBase64(bytes) {
    let s = "";
    for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
    return btoa(s);
}

/**
 * Stands in for the `w4 watch` server of the developer runtime: it serves the cart, sends the reload messages and passes
 * the `trace` output (which the runtime sends to that server) on to the extension. The source file is kept in the state
 * of the page, VS Code hands it back when it restores the panel after a reload.
 */
const devServerShim = (cartBase64, source) => `
(() => {
    const api = acquireVsCodeApi();
    api.setState({ source: ${JSON.stringify(source)} });
    let cart = Uint8Array.from(atob("${cartBase64}"), (c) => c.charCodeAt(0));
    let socket;
    const realFetch = window.fetch.bind(window);
    window.fetch = (input, init) =>
        /(^|\\/)cart\\.wasm(\\?|$)/.test(String(input?.url ?? input)) ? Promise.resolve(new Response(cart)) : realFetch(input, init);
    // The runtime opens the dev server connection first (when it is evaluated); later sockets (netplay) stay real.
    const RealWebSocket = window.WebSocket;
    window.WebSocket = function (...args) {
        if (socket) return new RealWebSocket(...args);
        socket = Object.assign(new EventTarget(), { readyState: 1, send: (text) => api.postMessage({ type: "trace", text: String(text) }), close() {} });
        return socket;
    };
    window.addEventListener("message", (e) => {
        if (e.data?.type !== "cart") return;
        api.setState({ source: e.data.source });
        cart = new Uint8Array(e.data.cart);
        socket?.dispatchEvent(new MessageEvent("message", { data: e.data.hot ? "hotswap" : "reload" }));
    });
})();`;

exports.activate = (context) => {
    const output = vscode.window.createOutputChannel("WASM-4", "log");
    // Lines look like the ones of a log channel, `time [level] text`. The channel is plain, so that nothing is added to
    // them, and has the `log` language for the colors (the built-in log grammar colors the time and the level).
    const pad = (n, width = 2) => String(n).padStart(width, "0");
    const log = (level, text) => {
        const d = new Date();
        const stamp = `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}:${pad(d.getSeconds())}.${pad(d.getMilliseconds(), 3)}`;
        output.appendLine(`${stamp} [${level}] ${text}`);
    };
    const status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 0);
    const vendorCache = new Map();
    let panel;
    let gameLoaded = false; // the page of the panel has the runtime in it
    let entry; // uri of the .lo file that is built
    let building = false;
    let rebuildAgain = false;
    let rebuildFull = false; // a queued rebuild has to restart the game
    let rebuildTimer;
    let loModule; // { key, module }

    const workspaceFolder = () => vscode.workspace.workspaceFolders?.[0]?.uri;
    const isLo = (uri) => uri.path.endsWith(".lo");

    async function vendorText(name) {
        if (!vendorCache.has(name)) {
            const url = vscode.Uri.joinPath(context.extensionUri, "vendor", "wasm4", name).toString();
            vendorCache.set(
                name,
                fetch(url).then((r) => {
                    if (!r.ok) throw new Error(`${name}: ${r.status}`);
                    return r.text();
                }),
            );
        }
        return vendorCache.get(name);
    }

    /** Compiles an .lo file to a cart with the lo.wasm of the workspace root. */
    async function compile(uri) {
        const folder = workspaceFolder();
        if (!folder) throw new Error("Open a workspace first.");
        const loUri = vscode.Uri.joinPath(folder, "lo.wasm");
        let stat;
        try {
            stat = await vscode.workspace.fs.stat(loUri);
        } catch {
            throw new Error("lo.wasm not found in the root of the workspace.");
        }
        const key = `${stat.mtime}:${stat.size}`;
        if (loModule?.key !== key) {
            loModule = { key, module: WebAssembly.compile(await vscode.workspace.fs.readFile(loUri)) };
        }

        const loader = await vscode.extensions.getExtension("ms-vscode.wasm-wasi-core").activate();
        const wasm = typeof loader.load === "function" ? loader.load(1) : loader;
        const stdout = [];
        const stderr = [];
        const out = wasm.createReadable();
        out.onData((c) => stdout.push(c));
        const err = wasm.createReadable();
        err.onData((c) => stderr.push(new TextDecoder().decode(c)));

        const process = await wasm.createProcess("lo", await loModule.module, {
            args: ["compile", vscode.workspace.asRelativePath(uri, false)],
            mountPoints: [{ kind: "vscodeFileSystem", uri: folder, mountPoint: "/" }],
            stdio: {
                in: { kind: "pipeIn", pipe: wasm.createWritable() },
                out: { kind: "pipeOut", pipe: out },
                err: { kind: "pipeOut", pipe: err },
            },
        });
        const code = await process.run();
        if (code !== 0) throw new Error(stderr.join("") || `lo.wasm compile exited with ${code}`);

        const cart = new Uint8Array(stdout.reduce((n, c) => n + c.length, 0));
        let offset = 0;
        for (const c of stdout) {
            cart.set(c, offset);
            offset += c.length;
        }
        return cart;
    }

    async function build(uri) {
        const started = Date.now();
        try {
            const cart = await compile(uri);
            log("info", `Built ${vscode.workspace.asRelativePath(uri, false)}: ${cart.length} bytes in ${Date.now() - started} ms`);
            status.text = "$(check) WASM-4";
            status.tooltip = `Built ${new Date().toLocaleTimeString()}`;
            status.backgroundColor = undefined;
            status.show();
            return cart;
        } catch (e) {
            log("error", `Build of ${vscode.workspace.asRelativePath(uri, false)} failed:\n${String(e?.message ?? e).trim()}`);
            status.text = "$(error) WASM-4";
            status.tooltip = "The build failed, see the WASM-4 output";
            status.backgroundColor = new vscode.ThemeColor("statusBarItem.errorBackground");
            status.command = "wasm4.showOutput";
            status.show();
            throw e;
        }
    }

    async function webviewHtml(cart, source) {
        const [css, js] = await Promise.all([vendorText("dev/wasm4.css"), vendorText("dev/wasm4.js")]);
        return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1, user-scalable=no">
<style>${escapeCss(css)}
/* the default webview styles give the body a 20px padding */
body { padding: 0; }</style>
</head>
<body>
<div id="devtools"></div>
<script>${escapeJs(devServerShim(toBase64(cart), source))}</script>
<script>${escapeJs(stripSourceMap(js))}</script>
<wasm4-app></wasm4-app>
</body>
</html>`;
    }

    /** Shows a cart in the panel: the first one loads the page, later ones are sent to the running game. */
    async function showCart(cart, hot) {
        if (!gameLoaded) {
            gameLoaded = true;
            panel.webview.html = await webviewHtml(cart, entry.toString());
        } else {
            panel.webview.postMessage({ type: "cart", cart, hot, source: entry.toString() });
        }
    }

    /** Creates the panel to the left of the editor that has the source. */
    async function createPanel(uri) {
        const column = vscode.window.activeTextEditor?.viewColumn;
        let viewColumn = vscode.ViewColumn.Beside;
        if (column && column > 1) {
            viewColumn = column - 1;
        } else if (column) {
            await vscode.commands.executeCommand("workbench.action.newGroupLeft");
            viewColumn = vscode.ViewColumn.Active;
        }
        const created = vscode.window.createWebviewPanel(
            "wasm4",
            `WASM-4: ${uri.path.split("/").pop()}`,
            { viewColumn, preserveFocus: !column },
            { enableScripts: true, retainContextWhenHidden: true },
        );
        // The new group has the focus, give it back to the editor.
        if (column) await vscode.commands.executeCommand(column > 1 ? "workbench.action.focusNextGroup" : "workbench.action.focusRightGroup");
        return created;
    }

    function adopt(created) {
        panel = created;
        gameLoaded = false;
        panel.onDidDispose(() => (panel = undefined), null, context.subscriptions);
        panel.webview.onDidReceiveMessage((m) => m?.type === "trace" && log("trace", m.text), null, context.subscriptions);
    }

    async function run(uri) {
        entry = uri;
        const cart = await build(uri).catch((e) => {
            vscode.window.showErrorMessage("WASM-4: the build failed", "Show output").then((a) => a && output.show());
            throw e;
        });
        if (!panel) adopt(await createPanel(uri));
        panel.title = `WASM-4: ${uri.path.split("/").pop()}`;
        await showCart(cart, false);
    }

    /** Rebuilds the running game. It is hot swapped when the setting is on, unless `full` asks for a restart. */
    async function rebuild({ full = false } = {}) {
        if (building) {
            rebuildAgain = true;
            rebuildFull ||= full;
            return;
        }
        building = true;
        try {
            const cart = await build(entry);
            if (panel) await showCart(cart, !full && vscode.workspace.getConfiguration("wasm4").get("hotSwap", false));
        } catch {
            // the error is in the output
        } finally {
            building = false;
            if (rebuildAgain) {
                const again = { full: rebuildFull };
                rebuildAgain = false;
                rebuildFull = false;
                rebuild(again);
            }
        }
    }

    async function exportHtml() {
        const uri = entry ?? (vscode.window.activeTextEditor && isLo(vscode.window.activeTextEditor.document.uri) ? vscode.window.activeTextEditor.document.uri : undefined);
        if (!uri) return vscode.window.showErrorMessage("WASM-4: open the .lo file of the game first.");
        const base = uri.path.split("/").pop().replace(/\.lo$/, "");
        const title = await vscode.window.showInputBox({ title: "WASM-4: Export HTML", prompt: "Title of the game", value: base });
        if (title === undefined) return;

        try {
            const cart = await build(uri);
            const [css, js, version] = await Promise.all([vendorText("slim/wasm4.css"), vendorText("slim/wasm4.js"), vendorText("version.txt")]);
            const cartJson = escapeJs(JSON.stringify({ WASM4_CART: z85(cart), WASM4_CART_SIZE: cart.length }));
            const html = `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1, minimum-scale=1, maximum-scale=1, user-scalable=no">
  <meta name="generator" content="WASM-4 ${version.trim()}">
  <script id="wasm4-disk-prefix" type="text/plain">${escapeJs(title)}</script>
  <title>${escapeHtml(title)}</title>
  <style>${escapeCss(css)}</style>
</head>
<body>
  <script id="wasm4-cart-json" type="application/json">${cartJson}</script>
  <script>${escapeJs(js)}</script>
  <wasm4-app></wasm4-app>
</body>
</html>
`;
            const bytes = new TextEncoder().encode(html);
            const target = uri.with({ path: uri.path.replace(/\.lo$/, "") + ".html" });
            await vscode.workspace.fs.writeFile(target, bytes);
            const action = await vscode.window.showInformationMessage(
                `Exported ${target.path.split("/").pop()} (${Math.round(bytes.length / 1024)} KB)`,
                "Download",
            );
            if (action === "Download") {
                await vscode.commands.executeCommand("playground.download", target.path.split("/").pop(), "text/html", toBase64(bytes));
            }
        } catch (e) {
            vscode.window.showErrorMessage("WASM-4: the export failed", "Show output").then((a) => a && output.show());
        }
    }

    function toggleSetting(name, defaultValue) {
        const config = vscode.workspace.getConfiguration("wasm4");
        return config.update(name, !config.get(name, defaultValue), vscode.ConfigurationTarget.Global);
    }

    function updateContext() {
        const doc = vscode.window.activeTextEditor?.document;
        const isCart = !!doc && isLo(doc.uri) && /wasm4/.test(doc.getText());
        vscode.commands.executeCommand("setContext", "wasm4.isCart", isCart);
    }

    context.subscriptions.push(
        output,
        status,
        vscode.commands.registerCommand("wasm4.showOutput", () => output.show()),
        vscode.commands.registerCommand("wasm4.run", async () => {
            const doc = vscode.window.activeTextEditor?.document;
            const uri = doc && isLo(doc.uri) ? doc.uri : entry;
            if (!uri) return vscode.window.showErrorMessage("WASM-4: open the .lo file of the game first.");
            if (doc && doc.uri.toString() === uri.toString()) await doc.save();
            await run(uri).catch(() => {});
        }),
        vscode.commands.registerCommand("wasm4.exportHtml", exportHtml),
        vscode.commands.registerCommand("wasm4.reload", () => {
            if (!entry) return vscode.window.showErrorMessage("WASM-4: run a .lo file first.");
            return rebuild({ full: true });
        }),
        // The title bar shows a different icon for each state of a toggle, so each toggle is a pair of commands (see package.json).
        ...["wasm4.toggleReloadOnSave", "wasm4.reloadOnSaveIsOn", "wasm4.reloadOnSaveIsOff"].map((id) =>
            vscode.commands.registerCommand(id, () => toggleSetting("rebuildOnSave", true)),
        ),
        ...["wasm4.toggleHotSwap", "wasm4.hotSwapIsOn", "wasm4.hotSwapIsOff"].map((id) =>
            vscode.commands.registerCommand(id, () => toggleSetting("hotSwap", false)),
        ),
        vscode.workspace.onDidSaveTextDocument((doc) => {
            if (!panel || !entry || !isLo(doc.uri) || !vscode.workspace.getConfiguration("wasm4").get("rebuildOnSave", true)) return;
            clearTimeout(rebuildTimer);
            rebuildTimer = setTimeout(rebuild, 150);
        }),
        // After a reload of the page VS Code brings the panel back (in its editor group) and asks for its content.
        vscode.window.registerWebviewPanelSerializer("wasm4", {
            async deserializeWebviewPanel(restored, state) {
                restored.webview.options = { enableScripts: true };
                adopt(restored);
                if (!state?.source) {
                    restored.webview.html = "<p>Run a WASM-4 game to fill this tab.</p>";
                    return;
                }
                entry = vscode.Uri.parse(state.source);
                restored.title = `WASM-4: ${entry.path.split("/").pop()}`;
                try {
                    await showCart(await build(entry), false);
                } catch {
                    restored.webview.html = "<p>The build failed, see the WASM-4 output.</p>";
                }
            },
        }),
        vscode.window.onDidChangeActiveTextEditor(updateContext),
        vscode.workspace.onDidChangeTextDocument((e) => e.document === vscode.window.activeTextEditor?.document && updateContext()),
    );
    updateContext();
};
