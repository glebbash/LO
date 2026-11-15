import { execFileSync } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = path.dirname(fileURLToPath(import.meta.url));
const REPO_ROOT = path.resolve(ROOT, "../..");
const CFG = JSON.parse(
    fs.readFileSync(path.join(ROOT, "package.json"), "utf8"),
).config;
const CACHE = path.join(ROOT, ".cache");
const DIST = path.join(ROOT, "dist");
const WORKBENCH_JS = path.join(DIST, "vscode/out/vs/workbench/workbench.web.main.internal.js");

await main(process.argv.slice(2));

async function main(args) {
    const [command, ...rest] = args;
    switch (command) {
        case "build":
            await build();
            break;
        case "serve":
            serve(Number(rest[0] ?? 8080));
            break;
        default:
            console.error("usage: node scripts.js build | serve [port]");
            process.exit(1);
    }
}

function sh(cmd, args) {
    return execFileSync(cmd, args, { stdio: "inherit" });
}

async function get(url) {
    const res = await fetch(url, { redirect: "follow" });
    if (!res.ok) throw new Error(`GET ${url} -> ${res.status}`);
    return res;
}

async function download(url, file) {
    fs.mkdirSync(path.dirname(file), { recursive: true });
    fs.writeFileSync(file, Buffer.from(await (await get(url)).arrayBuffer()));
}

/**
 * Removes a built-in extension completely: its folder and its entry in the list of built-in extensions that is baked
 * into the workbench bundle (`{extensionPath:"name",packageJSON:{...},...}`).
 */
function dropBuiltinExtension(name) {
    fs.rmSync(path.join(DIST, "vscode/extensions", name), {
        recursive: true,
        force: true,
    });

    const src = fs.readFileSync(WORKBENCH_JS, "utf8");
    const start = src.indexOf(`{extensionPath:${JSON.stringify(name)},`);
    if (start < 0) throw new Error(`built-in extension ${name} not found in the workbench bundle`);

    // find the end of the object literal, skipping over string literals
    let depth = 0;
    let end = -1;
    for (let i = start; i < src.length; i++) {
        const c = src[i];
        if (c === '"' || c === "'" || c === "`") {
            for (i++; src[i] !== c; i++) if (src[i] === "\\") i++;
        } else if (c === "{") {
            depth++;
        } else if (c === "}" && --depth === 0) {
            end = i + 1;
            break;
        }
    }
    if (end < 0) throw new Error(`end of the ${name} entry not found`);

    // drop the separating comma as well
    const [from, to] = src[start - 1] === "," ? [start - 1, end] : [start, end + 1];
    fs.writeFileSync(WORKBENCH_JS, src.slice(0, from) + src.slice(to));
}

/**
 * Makes the icons of the extension views load in CORS mode. The page is cross-origin isolated with `require-corp`, which
 * blocks a plain `<img>` of a server without a Cross-Origin-Resource-Policy header, like Open VSX. It sends CORS headers.
 */
function patchExtensionIcons() {
    const src = fs.readFileSync(WORKBENCH_JS, "utf8");
    let count = 0;
    const patched = src.replace(/(\w+\("img\.icon",\{alt:"")\}\)/g, (_, head) => {
        count++;
        return `${head},crossorigin:"anonymous"})`;
    });
    if (count === 0) throw new Error("extension icon widget not found in the workbench bundle");
    fs.writeFileSync(WORKBENCH_JS, patched);
}

async function build() {
    fs.rmSync(DIST, { recursive: true, force: true });
    fs.mkdirSync(DIST, { recursive: true });

    // VS Code web client, see README for why this package. Its vscode/ folder becomes out/, the rest keeps its name.
    const vscodeDir = path.join(CACHE, `vscode-web-${CFG.vscodeWeb}`);
    if (!fs.existsSync(path.join(vscodeDir, "vscode"))) {
        const tarball = path.join(CACHE, `vscode-web-${CFG.vscodeWeb}.tgz`);
        const url = `https://registry.npmjs.org/@github1s/vscode-web/-/vscode-web-${CFG.vscodeWeb}.tgz`;
        console.log("downloading VS Code web (Code - OSS)", url);
        await download(url, tarball);
        fs.rmSync(vscodeDir, { recursive: true, force: true });
        fs.mkdirSync(vscodeDir, { recursive: true });
        sh("tar", ["xzf", tarball, "-C", vscodeDir, "--strip-components=1"]);
    }
    for (const [from, to] of [
        ["vscode", "out"],
        ["extensions", "extensions"],
        ["dependencies", "dependencies"],
        ["favicon.ico", "favicon.ico"],
    ]) {
        fs.cpSync(path.join(vscodeDir, from), path.join(DIST, "vscode", to), {
            recursive: true,
        });
    }
    // Built-in extensions we don't ship. They are replaced by a no-op stub at the entry point of their manifest, because
    // the manifests are baked into the workbench, which activates them anyway.
    for (const ext of [
        "TypeScriptTeam.jsts-chat-features",
        "prompt-basics",
        "typescript-language-features",
        "html-language-features",
        "css-language-features",
    ]) {
        const dir = path.join(DIST, "vscode/extensions", ext);
        const manifestFile = path.join(dir, "package.json");
        const entry = fs.existsSync(manifestFile)
            ? JSON.parse(fs.readFileSync(manifestFile, "utf8")).browser
            : undefined;
        fs.rmSync(dir, { recursive: true, force: true });
        if (entry) {
            const file = path.join(dir, entry.endsWith(".js") ? entry : `${entry}.js`);
            fs.mkdirSync(path.dirname(file), { recursive: true });
            fs.writeFileSync(file, "exports.activate = function () {};\nexports.deactivate = function () {};\n");
        }
    }
    dropBuiltinExtension("mermaid-markdown-features");
    patchExtensionIcons();
    // Unused bundle of the agents window, close to Cloudflare's 25 MiB file limit.
    fs.rmSync(path.join(DIST, "vscode/out/vs/sessions"), {
        recursive: true,
        force: true,
    });

    /** Downloads the latest `publisher.name` vsix from Open VSX, unpacks it and returns the `extension/` folder. */
    async function fetchExtension(id) {
        const [publisher, name] = id.split(".");
        const meta = await (
            await get(`https://open-vsx.org/api/${publisher}/${name}/latest`)
        ).json();
        const vsix = path.join(CACHE, "vsix", `${id}-${meta.version}.vsix`);
        if (!fs.existsSync(vsix)) {
            console.log("downloading", id, meta.version);
            await download(meta.files.download, vsix);
        }
        const tmp = path.join(CACHE, "unzip", id);
        fs.rmSync(tmp, { recursive: true, force: true });
        fs.mkdirSync(tmp, { recursive: true });
        sh("unzip", ["-q", vsix, "extension/*", "-d", tmp]);
        return path.join(tmp, "extension");
    }

    const extDirs = [];
    for (const id of CFG.extensions) {
        const tmp = path.dirname(await fetchExtension(id));
        const target = path.join(DIST, "extensions", id.toLowerCase());
        fs.mkdirSync(path.dirname(target), { recursive: true });
        fs.cpSync(path.join(tmp, "extension"), target, { recursive: true });
        const manifest = JSON.parse(
            fs.readFileSync(path.join(target, "package.json"), "utf8"),
        );
        if (!manifest.browser && !manifest.contributes?.themes)
            console.warn(
                `WARNING: ${id} has no "browser" entry, it won't run in the web extension host`,
            );
        extDirs.push(id.toLowerCase());

        if (id === "GitHub.github-vscode-theme") {
            const labels = (manifest.contributes?.themes ?? []).map(
                (t) => t.label,
            );
            if (!labels.includes(CFG.defaultTheme)) {
                throw new Error(
                    `defaultTheme "${CFG.defaultTheme}" not found, available: ${labels.join(", ")}`,
                );
            }
        }
    }

    // Our own extensions.
    for (const name of fs.readdirSync(path.join(ROOT, "extensions"))) {
        fs.cpSync(
            path.join(ROOT, "extensions", name),
            path.join(DIST, "extensions", name),
            { recursive: true },
        );
        extDirs.push(name);
    }
    // coreutils.wasm for lo-shell is taken from the webshell extension.
    {
        const src = path.join(
            await fetchExtension(CFG.coreutilsFrom),
            "wasm",
            "coreutils.wasm",
        );
        fs.mkdirSync(path.join(DIST, "extensions/lo-shell/wasm"), {
            recursive: true,
        });
        fs.copyFileSync(
            src,
            path.join(DIST, "extensions/lo-shell/wasm/coreutils.wasm"),
        );
    }

    // wabt.js for wasm-viewer.
    {
        const tgz = path.join(CACHE, `wabt-${CFG.wabt}.tgz`);
        if (!fs.existsSync(tgz))
            await download(
                `https://registry.npmjs.org/wabt/-/wabt-${CFG.wabt}.tgz`,
                tgz,
            );
        const dir = path.join(CACHE, `wabt-${CFG.wabt}`);
        fs.rmSync(dir, { recursive: true, force: true });
        fs.mkdirSync(dir, { recursive: true });
        sh("tar", [
            "xzf",
            tgz,
            "-C",
            dir,
            "--strip-components=1",
            "package/index.js",
            "package/LICENSE",
        ]);
        const target = path.join(DIST, "extensions/wasm-viewer/vendor/wabt");
        fs.mkdirSync(target, { recursive: true });
        fs.copyFileSync(
            path.join(dir, "index.js"),
            path.join(target, "index.js"),
        );
        fs.copyFileSync(
            path.join(dir, "LICENSE"),
            path.join(target, "LICENSE"),
        );
    }

    // isomorphic-git for scratch-git, plus the Buffer it expects (a web worker has none). The buffer package and its two
    // dependencies are CommonJS, so they are wrapped in a tiny module loader instead of using a bundler.
    {
        const fetchPackage = async (name, version, files) => {
            const tgz = path.join(CACHE, `${name}-${version}.tgz`);
            if (!fs.existsSync(tgz))
                await download(
                    `https://registry.npmjs.org/${name}/-/${name}-${version}.tgz`,
                    tgz,
                );
            const dir = path.join(CACHE, `${name}-${version}`);
            fs.rmSync(dir, { recursive: true, force: true });
            fs.mkdirSync(dir, { recursive: true });
            sh("tar", [
                "xzf",
                tgz,
                "-C",
                dir,
                "--strip-components=1",
                ...files.map((f) => `package/${f}`),
            ]);
            return dir;
        };
        const target = path.join(
            DIST,
            "extensions/scratch-git/vendor/isomorphic-git",
        );
        fs.mkdirSync(target, { recursive: true });

        const git = await fetchPackage("isomorphic-git", CFG.isomorphicGit, [
            "index.umd.min.js",
            "LICENSE.md",
        ]);
        for (const name of ["index.umd.min.js", "LICENSE.md"])
            fs.copyFileSync(path.join(git, name), path.join(target, name));

        const modules = [];
        const licenses = [];
        for (const [name, version] of Object.entries(CFG.bufferPackages)) {
            const dir = await fetchPackage(name, version, [
                "index.js",
                "LICENSE",
            ]);
            modules.push(
                `${JSON.stringify(name)}: function (module, exports, require) {\n${fs.readFileSync(path.join(dir, "index.js"), "utf8")}\n}`,
            );
            licenses.push(
                `${name} ${version}\n\n${fs.readFileSync(path.join(dir, "LICENSE"), "utf8").trim()}\n`,
            );
        }
        fs.writeFileSync(
            path.join(target, "buffer.js"),
            `(function () {
const defs = {
${modules.join(",\n")}
};
const cache = {};
function load(name) {
    if (!cache[name]) {
        const module = (cache[name] = { exports: {} });
        defs[name](module, module.exports, load);
    }
    return cache[name].exports;
}
return load("buffer");
})()
`,
        );
        fs.writeFileSync(
            path.join(target, "LICENSE-buffer.txt"),
            licenses.join("\n---\n\n"),
        );
    }

    // WASM-4 runtime for the wasm4 extension: the developer build for the editor tab, the slim one for exported pages.
    {
        const tgz = path.join(CACHE, `wasm4-${CFG.wasm4}.tgz`);
        if (!fs.existsSync(tgz))
            await download(
                `https://registry.npmjs.org/wasm4/-/wasm4-${CFG.wasm4}.tgz`,
                tgz,
            );
        const dir = path.join(CACHE, `wasm4-${CFG.wasm4}`);
        fs.rmSync(dir, { recursive: true, force: true });
        fs.mkdirSync(dir, { recursive: true });
        sh("tar", [
            "xzf",
            tgz,
            "-C",
            dir,
            "--strip-components=1",
            "package/LICENSE.txt",
            "package/assets/runtime/developer-build/wasm4.js",
            "package/assets/runtime/developer-build/wasm4.css",
            "package/assets/runtime/slim/wasm4.js",
            "package/assets/runtime/slim/wasm4.css",
        ]);
        const target = path.join(DIST, "extensions/wasm4/vendor/wasm4");
        for (const [from, to] of [
            ["LICENSE.txt", "LICENSE.txt"],
            ["assets/runtime/developer-build/wasm4.js", "dev/wasm4.js"],
            ["assets/runtime/developer-build/wasm4.css", "dev/wasm4.css"],
            ["assets/runtime/slim/wasm4.js", "slim/wasm4.js"],
            ["assets/runtime/slim/wasm4.css", "slim/wasm4.css"],
        ]) {
            fs.mkdirSync(path.dirname(path.join(target, to)), { recursive: true });
            fs.copyFileSync(path.join(dir, from), path.join(target, to));
        }
        // The runtime also listens for keys on `top`. That throws and is ignored for a cross-origin webview, but ours is
        // same-origin, so the game would get (and swallow) every key typed in the editor.
        const devRuntime = path.join(target, "dev/wasm4.js");
        const keyListenersOnTop = "top&&top!=window";
        const devSource = fs.readFileSync(devRuntime, "utf8");
        if (!devSource.includes(keyListenersOnTop))
            throw new Error("wasm4 runtime: key listeners on `top` not found, update the patch in scripts.js");
        fs.writeFileSync(devRuntime, devSource.replace(keyListenersOnTop, "false"));
        fs.writeFileSync(path.join(target, "version.txt"), `${CFG.wasm4}\n`);
    }

    // VS Code requests package.nls.json of every extension, avoid the 404s.
    for (const name of extDirs) {
        const nls = path.join(DIST, "extensions", name, "package.nls.json");
        if (!fs.existsSync(nls)) fs.writeFileSync(nls, "{}\n");
    }

    fs.cpSync(path.join(ROOT, "public"), DIST, { recursive: true });
    // Default profile, see README.
    fs.copyFileSync(
        path.join(ROOT, "src", "playground.code-profile"),
        path.join(DIST, "playground.code-profile"),
    );
    fs.cpSync(path.join(ROOT, "src", "main.js"), path.join(DIST, "main.js"));
    fs.cpSync(path.join(ROOT, "src", "about.js"), path.join(DIST, "about.js"));
    fs.copyFileSync(
        path.join(REPO_ROOT, "extras/vscode-ext/assets/icons/lo.svg"),
        path.join(DIST, "lo.svg"),
    );
    const html = fs.readFileSync(path.join(ROOT, "src", "index.html"), "utf8");
    fs.writeFileSync(path.join(DIST, "index.html"), html);
    fs.writeFileSync(
        path.join(DIST, "playground-config.json"),
        JSON.stringify({
            extensions: extDirs,
            defaultTheme: CFG.defaultTheme,
            vscodeWeb: CFG.vscodeWeb,
        }),
    );
    console.log("built ->", DIST);
}

function serve(port) {
    const TYPES = {
        ".html": "text/html",
        ".js": "text/javascript",
        ".mjs": "text/javascript",
        ".css": "text/css",
        ".json": "application/json",
        ".wasm": "application/wasm",
        ".svg": "image/svg+xml",
        ".png": "image/png",
        ".ttf": "font/ttf",
        ".woff": "font/woff",
        ".woff2": "font/woff2",
    };

    http.createServer((req, res) => {
        let p = path.join(
            DIST,
            decodeURIComponent(new URL(req.url, "http://x").pathname),
        );
        if (!p.startsWith(DIST)) {
            res.writeHead(403).end();
            return;
        }
        if (fs.existsSync(p) && fs.statSync(p).isDirectory())
            p = path.join(p, "index.html");
        res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
        res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
        res.setHeader("Cross-Origin-Resource-Policy", "cross-origin");
        res.setHeader("Access-Control-Allow-Origin", "*");
        if (!fs.existsSync(p)) {
            res.writeHead(404).end("not found");
            return;
        }
        res.setHeader(
            "Content-Type",
            TYPES[path.extname(p)] ?? "application/octet-stream",
        );
        fs.createReadStream(p).pipe(res);
    }).listen(port, () => console.log(`http://localhost:${port}`));
}
