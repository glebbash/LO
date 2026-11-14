// In-memory FileSystemProvider for scheme `scratch:`, persisted to IndexedDB. Seeded on first start: downloads lo.wasm from GitHub and runs `lo.wasm eject`.
const vscode = require("vscode");

const LO_WASM_URL = "https://raw.githubusercontent.com/glebbash/LO/main/lo.wasm";
const MAIN = '@include("./lib/cli.lo")\n\nfn main() {\n    println("Hello World!")\n}\n';

// The workspace folder is scratch:/Scratch FS, so the explorer shows "Scratch FS" instead of "/" (keep in sync with src/main.js)
const ROOT = "/Scratch FS";
const DB = "lo-playground-scratch", STORE = "files", KEY = "tree";
const idb = () => new Promise((res, rej) => {
    const r = indexedDB.open(DB, 1);
    r.onupgradeneeded = () => r.result.createObjectStore(STORE);
    r.onsuccess = () => res(r.result);
    r.onerror = () => rej(r.error);
});
const idbGet = async () => { const db = await idb(); return new Promise((res, rej) => { const q = db.transaction(STORE).objectStore(STORE).get(KEY); q.onsuccess = () => res(q.result); q.onerror = () => rej(q.error); }); };
const idbSet = async (v) => { const db = await idb(); return new Promise((res, rej) => { const t = db.transaction(STORE, "readwrite"); t.objectStore(STORE).put(v, KEY); t.oncomplete = () => res(); t.onerror = () => rej(t.error); }); };

class ScratchFS {
    constructor() {
        /** @type {Map<string, {type: number, data: Uint8Array, ctime: number, mtime: number}>} path -> entry; "/" always present */
        this.entries = new Map([["/", { type: vscode.FileType.Directory, data: new Uint8Array(), ctime: Date.now(), mtime: Date.now() }]]);
        this._em = new vscode.EventEmitter();
        this.onDidChangeFile = this._em.event;
        this._timer = undefined;
    }
    async load() {
        const saved = await idbGet().catch(() => undefined);
        if (saved && saved.some(([k]) => k === ROOT)) { this.entries = new Map(saved); return; }
        this._mkdir(ROOT);
        this.needsSeed = true; // see seed(); nothing is persisted until seeding succeeded
    }
    _persist() {
        if (this.needsSeed) return;
        clearTimeout(this._timer);
        this._timer = setTimeout(() => idbSet([...this.entries]).catch((e) => console.error("scratch-fs persist failed", e)), 300);
    }
    _mkdir(p) { if (!this.entries.has(p)) this.entries.set(p, { type: vscode.FileType.Directory, data: new Uint8Array(), ctime: Date.now(), mtime: Date.now() }); }
    _set(p, data) { const now = Date.now(); const old = this.entries.get(p); this.entries.set(p, { type: vscode.FileType.File, data, ctime: old?.ctime ?? now, mtime: now }); }
    _get(uri) { const e = this.entries.get(uri.path.replace(/\/+$/, "") || "/"); if (!e) throw vscode.FileSystemError.FileNotFound(uri); return e; }
    _p(uri) { return uri.path.replace(/\/+$/, "") || "/"; }
    _parent(p) { return p.slice(0, p.lastIndexOf("/")) || "/"; }

    watch() { return new vscode.Disposable(() => {}); }
    stat(uri) { const e = this._get(uri); return { type: e.type, ctime: e.ctime, mtime: e.mtime, size: e.data.byteLength }; }
    readDirectory(uri) {
        const p = this._p(uri); this._get(uri);
        const prefix = p === "/" ? "/" : p + "/";
        const out = [];
        for (const [k, e] of this.entries) if (k !== p && k.startsWith(prefix) && !k.slice(prefix.length).includes("/")) out.push([k.slice(prefix.length), e.type]);
        return out;
    }
    createDirectory(uri) {
        const p = this._p(uri); this._get(uri.with({ path: this._parent(p) }));
        this._mkdir(p); this._persist();
        this._em.fire([{ type: vscode.FileChangeType.Created, uri }]);
    }
    readFile(uri) { const e = this._get(uri); if (e.type === vscode.FileType.Directory) throw vscode.FileSystemError.FileIsADirectory(uri); return e.data; }
    writeFile(uri, content, { create, overwrite }) {
        const p = this._p(uri); const exists = this.entries.has(p);
        if (!exists && !create) throw vscode.FileSystemError.FileNotFound(uri);
        if (exists && create && !overwrite) throw vscode.FileSystemError.FileExists(uri);
        this._get(uri.with({ path: this._parent(p) }));
        this._set(p, content); this._persist();
        this._em.fire([{ type: exists ? vscode.FileChangeType.Changed : vscode.FileChangeType.Created, uri }]);
    }
    delete(uri) {
        const p = this._p(uri); this._get(uri);
        for (const k of [...this.entries.keys()]) if (k === p || k.startsWith(p + "/")) this.entries.delete(k);
        this._persist();
        this._em.fire([{ type: vscode.FileChangeType.Deleted, uri }]);
    }
    rename(from, to, { overwrite }) {
        const a = this._p(from), b = this._p(to); this._get(from);
        if (this.entries.has(b) && !overwrite) throw vscode.FileSystemError.FileExists(to);
        for (const k of [...this.entries.keys()]) if (k === a || k.startsWith(a + "/")) { this.entries.set(b + k.slice(a.length), this.entries.get(k)); this.entries.delete(k); }
        this._persist();
        this._em.fire([{ type: vscode.FileChangeType.Deleted, uri: from }, { type: vscode.FileChangeType.Created, uri: to }]);
    }
}

// --- search (Quick Open / Search view) --- proposed APIs, enabled for this extension in src/main.js
const isBinary = (data) => data.subarray(0, 4096).includes(0);
const subsequence = (needle, hay) => { let i = 0; for (const ch of hay) if (ch === needle[i] && ++i === needle.length) return true; return i === needle.length; };
const globToRegExp = (g) => new RegExp("^" + g.replace(/[.+^${}()|[\]\\]/g, "\\$&").replace(/\*\*\/?/g, "\0").replace(/\*/g, "[^/]*").replace(/\?/g, "[^/]").replace(/\0/g, ".*") + "$");

function* filesUnder(fs, folder) {
    const base = folder.path.replace(/\/+$/, "");
    for (const [k, e] of fs.entries) if (e.type === vscode.FileType.File && k.startsWith(base + "/")) yield [k, e];
}
const toUri = (path) => vscode.Uri.from({ scheme: "scratch", path });

function registerSearch(fs) {
    return [
        vscode.workspace.registerFileSearchProvider("scratch", {
            provideFileSearchResults(query, options, token) {
                const needle = query.pattern.toLowerCase().replace(/\s+/g, "");
                const out = [];
                for (const [k] of filesUnder(fs, options.folder)) {
                    if (token.isCancellationRequested || out.length >= (options.maxResults ?? 1000)) break;
                    const rel = k.slice(options.folder.path.length).toLowerCase();
                    if (subsequence(needle, rel)) out.push(toUri(k));
                }
                return out;
            },
        }),
        vscode.workspace.registerTextSearchProvider("scratch", {
            provideTextSearchResults(query, options, progress, token) {
                let re;
                try {
                    const src = query.isRegExp ? query.pattern : query.pattern.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
                    re = new RegExp(query.isWordMatch ? `\\b(?:${src})\\b` : src, query.isCaseSensitive ? "g" : "gi");
                } catch (e) {
                    return { message: { text: String(e.message), type: vscode.TextSearchCompleteMessageType.Warning } };
                }
                const includes = (options.includes ?? []).map(globToRegExp), excludes = (options.excludes ?? []).map(globToRegExp);
                const dec = new TextDecoder();
                let count = 0;
                for (const [k, e] of filesUnder(fs, options.folder)) {
                    const rel = k.slice(options.folder.path.replace(/\/+$/, "").length + 1);
                    if (includes.length && !includes.some((r) => r.test(rel))) continue;
                    if (excludes.some((r) => r.test(rel))) continue;
                    if (token.isCancellationRequested || isBinary(e.data)) continue;
                    const lines = dec.decode(e.data).split("\n");
                    for (let i = 0; i < lines.length; i++) {
                        const ranges = [];
                        for (const m of lines[i].matchAll(re)) { if (!m[0]) break; ranges.push(new vscode.Range(i, m.index, i, m.index + m[0].length)); }
                        if (!ranges.length) continue;
                        progress.report({ uri: toUri(k), ranges, preview: { text: lines[i], matches: ranges.map((r) => new vscode.Range(0, r.start.character, 0, r.end.character)) } });
                        if (++count >= options.maxResults) return { limitHit: true };
                    }
                }
                return { limitHit: false };
            },
        }),
    ];
}

// --- zip (store only, no compression) ---
const CRC = (() => { const t = new Uint32Array(256); for (let n = 0; n < 256; n++) { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; t[n] = c >>> 0; } return t; })();
const crc32 = (d) => { let c = 0xffffffff; for (let i = 0; i < d.length; i++) c = CRC[(c ^ d[i]) & 255] ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
/** @param {[string, Uint8Array][]} files path (no leading slash, dirs end with "/") -> content */
function zip(files) {
    const enc = new TextEncoder(), chunks = [], central = [];
    let offset = 0;
    const d = new Date(), time = (d.getHours() << 11) | (d.getMinutes() << 5) | (d.getSeconds() >> 1), date = ((d.getFullYear() - 1980) << 9) | ((d.getMonth() + 1) << 5) | d.getDate();
    for (const [name, data] of files) {
        const n = enc.encode(name), crc = crc32(data);
        const h = new DataView(new ArrayBuffer(30));
        h.setUint32(0, 0x04034b50, true); h.setUint16(4, 20, true); h.setUint16(6, 0x0800, true); h.setUint16(8, 0, true);
        h.setUint16(10, time, true); h.setUint16(12, date, true); h.setUint32(14, crc, true);
        h.setUint32(18, data.length, true); h.setUint32(22, data.length, true); h.setUint16(26, n.length, true);
        chunks.push(new Uint8Array(h.buffer), n, data);
        const c = new DataView(new ArrayBuffer(46));
        c.setUint32(0, 0x02014b50, true); c.setUint16(4, 20, true); c.setUint16(6, 20, true); c.setUint16(8, 0x0800, true);
        c.setUint16(12, time, true); c.setUint16(14, date, true); c.setUint32(16, crc, true);
        c.setUint32(20, data.length, true); c.setUint32(24, data.length, true); c.setUint16(28, n.length, true);
        c.setUint32(38, name.endsWith("/") ? 0x10 : 0, true); c.setUint32(42, offset, true);
        central.push(new Uint8Array(c.buffer), n);
        offset += 30 + n.length + data.length;
    }
    const size = central.reduce((a, b) => a + b.length, 0);
    const e = new DataView(new ArrayBuffer(22));
    e.setUint32(0, 0x06054b50, true); e.setUint16(8, files.length, true); e.setUint16(10, files.length, true);
    e.setUint32(12, size, true); e.setUint32(16, offset, true);
    const all = [...chunks, ...central, new Uint8Array(e.buffer)];
    const out = new Uint8Array(offset + size + 22);
    let o = 0; for (const a of all) { out.set(a, o); o += a.length; }
    return out;
}
const toBase64 = (b) => { let s = ""; for (let i = 0; i < b.length; i += 0x8000) s += String.fromCharCode(...b.subarray(i, i + 0x8000)); return btoa(s); };

/** First start: download lo.wasm, then let it unpack its bundled src/, lib/ and test/ (`lo.wasm eject`) into the workspace. */
async function seed(fs) {
    const root = vscode.Uri.from({ scheme: "scratch", path: ROOT });
    await vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: "Setting up Scratch FS" }, async (progress) => {
        progress.report({ message: "downloading lo.wasm" });
        const res = await fetch(LO_WASM_URL);
        if (!res.ok) throw new Error(`GET ${LO_WASM_URL}: ${res.status}`);
        const bytes = new Uint8Array(await res.arrayBuffer());
        fs._set(`${ROOT}/lo.wasm`, bytes);
        fs._set(`${ROOT}/main.lo`, new TextEncoder().encode(MAIN));

        progress.report({ message: "lo.wasm eject" });
        const loader = await vscode.extensions.getExtension("ms-vscode.wasm-wasi-core").activate();
        const wasm = typeof loader.load === "function" ? loader.load(1) : loader;
        const stderr = [];
        const err = wasm.createReadable();
        err.onData((c) => stderr.push(new TextDecoder().decode(c)));
        const proc = await wasm.createProcess("lo", await WebAssembly.compile(bytes), {
            args: ["eject"], // argv[0] is the process name, set by wasm-wasi-core
            mountPoints: [{ kind: "vscodeFileSystem", uri: root, mountPoint: "/" }],
            stdio: { in: { kind: "pipeIn", pipe: wasm.createWritable() }, out: { kind: "pipeOut", pipe: wasm.createReadable() }, err: { kind: "pipeOut", pipe: err } },
        });
        const code = await proc.run();
        if (code !== 0) throw new Error(`lo.wasm eject exited with ${code}: ${stderr.join("")}`);
    });
    fs.needsSeed = false;
    fs._persist();
    void vscode.window.showTextDocument(vscode.Uri.from({ scheme: "scratch", path: `${ROOT}/main.lo` }));
}

exports.activate = async (ctx) => {
    const fs = new ScratchFS();
    await fs.load();
    ctx.subscriptions.push(vscode.workspace.registerFileSystemProvider("scratch", fs, { isCaseSensitive: true }));
    ctx.subscriptions.push(...registerSearch(fs));
    if (fs.needsSeed) {
        seed(fs).catch((e) => vscode.window.showErrorMessage(`Scratch FS setup failed (reload to retry): ${e?.message ?? e}`));
    }
    ctx.subscriptions.push(vscode.commands.registerCommand("scratch.downloadZip", async () => {
        const files = [];
        for (const [k, e] of [...fs.entries].sort(([a], [b]) => (a < b ? -1 : 1))) {
            if (k === ROOT || !k.startsWith(ROOT + "/")) continue;
            const name = k.slice(ROOT.length + 1);
            files.push(e.type === vscode.FileType.Directory ? [name + "/", new Uint8Array()] : [name, e.data]);
        }
        // `playground.download` is registered by the page itself (src/main.js): a web worker can't start a download
        await vscode.commands.executeCommand("playground.download", "scratch-fs.zip", "application/zip", toBase64(zip(files)));
    }));
    ctx.subscriptions.push(vscode.commands.registerCommand("scratch.reset", async () => {
        await idbSet([]);
        await vscode.commands.executeCommand("workbench.action.reloadWindow"); // (the extension host is a worker: no `location.reload`)
    }));
};
