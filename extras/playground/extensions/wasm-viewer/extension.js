// Opens .wasm files as WAT text (read-only virtual document), WAT language support, and wasm2wat / wat2wasm for other
// extensions (lo-shell). Conversion is done by wabt.js (vendor/wabt/index.js, added by scripts.js).
const vscode = require("vscode");

const FEATURES = {
    exceptions: true, mutable_globals: true, sat_float_to_int: true, sign_extension: true, simd: true, threads: true,
    function_references: true, multi_value: true, tail_call: true, bulk_memory: true, reference_types: true,
    annotations: true, gc: true, memory64: true, extended_const: true, relaxed_simd: true, multi_memory: true,
};

let wabtPromise;
function loadWabt(context) {
    return (wabtPromise ??= (async () => {
        const res = await fetch(vscode.Uri.joinPath(context.extensionUri, "vendor", "wabt", "index.js").toString());
        if (!res.ok) throw new Error(`wabt.js: ${res.status}`);
        const module = { exports: {} };
        new Function("module", "exports", await res.text())(module, module.exports); // UMD/CommonJS bundle
        return module.exports();
    })());
}

const SCHEME = "wasm-wat";

exports.activate = (context) => {
    const wabt = () => loadWabt(context);

    async function wasm2wat(bytes) {
        const m = (await wabt()).readWasm(bytes, { readDebugNames: true, ...FEATURES });
        try {
            try { m.applyNames(); } catch { /* keep generated names */ }
            return m.toText({ foldExprs: false, inlineExport: false });
        } finally { m.destroy(); }
    }

    async function wat2wasm(text, filename = "input.wat") {
        const m = (await wabt()).parseWat(filename, text, FEATURES);
        try {
            m.resolveNames();
            m.validate();
            return m.toBinary({ log: false, write_debug_names: true }).buffer;
        } finally { m.destroy(); }
    }

    // wasm-wat:/path/to/file.wasm.wat?<original uri> -> WAT of the original
    const changed = new vscode.EventEmitter();
    context.subscriptions.push(vscode.workspace.registerTextDocumentContentProvider(SCHEME, {
        onDidChange: changed.event,
        async provideTextDocumentContent(uri) {
            const bytes = await vscode.workspace.fs.readFile(vscode.Uri.parse(uri.query));
            return wasm2wat(bytes);
        },
    }));

    async function openAsWat(uri, viewColumn) {
        const virtual = vscode.Uri.from({ scheme: SCHEME, path: uri.path + ".wat", query: uri.toString() });
        const doc = await vscode.workspace.openTextDocument(virtual);
        await vscode.languages.setTextDocumentLanguage(doc, "wat");
        await vscode.window.showTextDocument(doc, { viewColumn, preview: false });
    }

    // Double clicking a .wasm file: show the WAT in the same tab position (custom editors can't host a text editor)
    context.subscriptions.push(vscode.window.registerCustomEditorProvider("wasmViewer.wat", {
        openCustomDocument: (uri) => ({ uri, dispose() {} }),
        async resolveCustomEditor(document, panel) {
            panel.webview.html = "<body style=\"font-family:sans-serif;padding:1em\">Converting to WAT...</body>";
            try {
                await openAsWat(document.uri, panel.viewColumn);
                panel.dispose();
            } catch (e) {
                const msg = String(e?.message ?? e).replace(/[<&]/g, (c) => (c === "<" ? "&lt;" : "&amp;"));
                panel.webview.html = `<body style="font-family:sans-serif;padding:1em"><h3>Can't convert to WAT</h3><pre>${msg}</pre></body>`;
            }
        },
    }, { supportsMultipleEditorsPerDocument: true }));

    return { wasm2wat, wat2wasm };
};
