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

/** JSONC (comments, trailing commas) -> value; strings are respected */
function parseJsonc(text) {
    let out = "",
        i = 0;
    while (i < text.length) {
        const c = text[i],
            n = text[i + 1];
        if (c === '"') {
            // copy the whole string literal
            let j = i + 1;
            while (text[j] !== '"') j += text[j] === "\\" ? 2 : 1;
            out += text.slice(i, j + 1);
            i = j + 1;
        } else if (c === "/" && n === "/") {
            while (i < text.length && text[i] !== "\n") i++;
        } else if (c === "/" && n === "*") {
            i = text.indexOf("*/", i + 2) + 2;
            if (i < 2) throw new Error("unterminated /* comment");
        } else {
            out += c;
            i++;
        }
    }
    return JSON.parse(out.replace(/,(\s*[}\]])/g, "$1")); // trailing commas (a "," followed by } or ] can't be inside a string here)
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

async function build() {
    fs.rmSync(DIST, { recursive: true, force: true });
    fs.mkdirSync(DIST, { recursive: true });

    // 1. VS Code web build
    const vscodeDir = path.join(CACHE, `vscode-web-${CFG.vscodeCommit}`);
    if (!fs.existsSync(path.join(vscodeDir, "out"))) {
        const tarball = path.join(
            CACHE,
            `vscode-web-${CFG.vscodeCommit}.tar.gz`,
        );
        const url = `https://update.code.visualstudio.com/commit:${CFG.vscodeCommit}/web-standalone/${CFG.vscodeQuality}`;
        console.log("downloading VS Code web", url);
        await download(url, tarball);
        fs.mkdirSync(vscodeDir, { recursive: true });
        sh("tar", ["xzf", tarball, "-C", vscodeDir, "--strip-components=1"]);
    }
    // only the static client is needed, not the node server
    for (const entry of [
        "out",
        "extensions",
        "node_modules",
        "favicon.ico",
        "code-192.png",
        "code-512.png",
        "manifest.json",
        "package.json",
    ]) {
        const src = path.join(vscodeDir, entry);
        if (fs.existsSync(src))
            fs.cpSync(src, path.join(DIST, "vscode", entry), {
                recursive: true,
            });
    }
    // built-in extensions we don't want (keep github-authentication: the workbench still tries to activate it):
    // chat/AI contributions, and the JS/TS, HTML and CSS language servers (syntax highlighting is in the *-basics/html/css packs, which stay)
    for (const ext of [
        "TypeScriptTeam.jsts-chat-features",
        "mermaid-markdown-features",
        "prompt-basics",
        "typescript-language-features",
        "html-language-features",
        "css-language-features",
    ]) {
        fs.rmSync(path.join(DIST, "vscode/extensions", ext), {
            recursive: true,
            force: true,
        });
    }
    // Agents-window bundle, unused, and 24MB is too close to Cloudflare's 25MiB per-file limit
    fs.rmSync(path.join(DIST, "vscode/out/vs/sessions"), {
        recursive: true,
        force: true,
    });

    // 2. extensions from Open VSX
    /** downloads the latest `publisher.name` vsix from Open VSX and unpacks it, returns the `extension/` dir */
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

    // 3. our own extensions (./extensions/*), page
    for (const name of fs.readdirSync(path.join(ROOT, "extensions"))) {
        fs.cpSync(
            path.join(ROOT, "extensions", name),
            path.join(DIST, "extensions", name),
            { recursive: true },
        );
        extDirs.push(name);
    }
    // lo-shell runs coreutils.wasm, which ships inside ms-vscode.webshell (MIT, microsoft/vscode-wasm)
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

    // wabt.js (npm `wabt`, Apache-2.0): used by wasm-viewer for wasm2wat / wat2wasm
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

    // VS Code always requests <ext>/package.nls.json; extensions without one cause a 404 (console noise) each time
    for (const name of extDirs) {
        const nls = path.join(DIST, "extensions", name, "package.nls.json");
        if (!fs.existsSync(nls)) fs.writeFileSync(nls, "{}\n");
    }

    fs.cpSync(path.join(ROOT, "public"), DIST, { recursive: true });
    fs.cpSync(path.join(ROOT, "src", "main.js"), path.join(DIST, "main.js"));
    // default settings (src/settings.json is JSONC; main.js fetches the plain JSON version)
    fs.writeFileSync(
        path.join(DIST, "settings.json"),
        JSON.stringify(
            parseJsonc(
                fs.readFileSync(
                    path.join(ROOT, "src", "settings.json"),
                    "utf8",
                ),
            ),
        ),
    );
    fs.cpSync(path.join(ROOT, "src", "about.js"), path.join(DIST, "about.js"));
    fs.copyFileSync(
        path.join(REPO_ROOT, "extras/vscode-ext/assets/icons/lo.svg"),
        path.join(DIST, "lo.svg"),
    ); // logo for the About dialog
    const html = fs.readFileSync(path.join(ROOT, "src", "index.html"), "utf8");
    fs.writeFileSync(path.join(DIST, "index.html"), html);
    fs.writeFileSync(
        path.join(DIST, "playground-config.json"),
        JSON.stringify({
            extensions: extDirs,
            defaultTheme: CFG.defaultTheme,
            commit: CFG.vscodeCommit,
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
        res.setHeader("Cross-Origin-Embedder-Policy", "credentialless");
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
