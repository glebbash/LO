// A small shell for the terminal view, modeled after ms-vscode.webshell (MIT, microsoft/vscode-wasm).
// Commands: builtins (cd, pwd, clear, help, exit, git), coreutils (ls, cat, mkdir, ... from wasm/coreutils.wasm), wasm2wat,
// wat2wasm and any .wasm file in the workspace (`./lo.wasm compile in.lo > out.wasm`). `> file` redirects the output.
const vscode = require("vscode");

const COREUTILS_FS = new Set("basenc cat comm cp dir expand head ls mkdir mv realpath rm rmdir tail touch unexpand unlink vdir wc".split(" "));
const COREUTILS_NOFS = new Set(["date", "echo", "factor", "printenv", "sleep"]);
const COREUTILS_ADD_CWD = new Set(["dir", "ls", "vdir"]);
const OPEN_CREATE_TRUNCATE = 1 | 8; // OpenFlags.create | OpenFlags.truncate

const seq = (data) => `\x1b]633;${data}\x1b\\`;

function posixJoin(a, b) {
    const out = [];
    for (const part of (b.startsWith("/") ? b : `${a}/${b}`).split("/")) {
        if (part === "" || part === ".") continue;
        if (part === "..") out.pop(); else out.push(part);
    }
    return "/" + out.join("/");
}

/** Splits a line on spaces, honoring '...' and "..." quotes. */
function tokenize(line) {
    const tokens = [];
    let cur = "", quote = null, has = false;
    for (const ch of line) {
        if (quote) { if (ch === quote) quote = null; else cur += ch; }
        else if (ch === '"' || ch === "'") { quote = ch; has = true; }
        else if (/\s/.test(ch)) { if (cur || has) tokens.push(cur); cur = ""; has = false; }
        else cur += ch;
    }
    if (cur || has) tokens.push(cur);
    return tokens;
}

class Shell {
    constructor(wasm, state) {
        this.wasm = wasm;
        this.state = state; // { rootFs, coreutils: Promise<Module> }
        this.cwd = "/workspace";
        this.pty = wasm.createPseudoterminal({ history: true });
    }

    /** `/workspace/a/b` -> workspace folder uri joined with `a/b` */
    toUri(p) {
        const folder = vscode.workspace.workspaceFolders?.[0];
        if (!folder || !(p === "/workspace" || p.startsWith("/workspace/"))) return undefined;
        return vscode.Uri.joinPath(folder.uri, p.slice("/workspace".length));
    }

    async loop(terminal) {
        const { pty } = this;
        for (;;) {
            void pty.prompt(`${seq("A")}${seq(`P;Cwd=${this.cwd}`)}\x1b[01;34m${this.cwd}\x1b[0m $ ${seq("B")}`);
            const line = await pty.readline();
            await pty.write(seq("C"));
            let code = 0;
            try {
                const tokens = tokenize(line);
                if (tokens.length === 0) continue;
                if (tokens[0] === "exit") { terminal.dispose(); return; }
                code = await this.exec(tokens);
            } catch (e) {
                await pty.write(`lo-shell: ${e?.message ?? e}\r\n`);
                code = 1;
            }
            await pty.write(seq(`E;${line}`));
            await pty.write(seq(`D;${code}`));
        }
    }

    async exec(tokens) {
        let out, redirectPath;
        const gt = tokens.indexOf(">");
        if (gt !== -1) {
            const target = tokens[gt + 1];
            if (!target) { await this.pty.write("lo-shell: missing redirect target\r\n"); return 1; }
            redirectPath = posixJoin(this.cwd, target);
            out = { kind: "file", path: redirectPath, openFlags: OPEN_CREATE_TRUNCATE }; // used by coreutils (rootFs paths)
            tokens = tokens.slice(0, gt);
        }
        const stdio = { ...this.pty.stdio, ...(out ? { out } : {}) };
        let [cmd, ...args] = tokens;
        const { pty } = this;

        switch (cmd) {
            case "pwd": await pty.write(`${this.cwd}\r\n`); return 0;
            case "clear": await pty.write("\x1b[2J\x1b[3J\x1b[H"); return 0;
            case "help":
                await pty.write("builtins: cd pwd clear help exit git\r\ngit: status  add <path>...  restore [--staged] <path>...  reset <path>...  commit [-a] -m <message>  log\r\ncoreutils: " + [...COREUTILS_FS, ...COREUTILS_NOFS].sort().join(" ") + "\r\nwasm tools: wasm2wat <in.wasm> [-o out.wat]  wat2wasm <in.wat> [-o out.wasm]\r\nprograms: ./file.wasm args...  |  run file.wasm args...   (supports > file)\r\n");
                return 0;
            case "cd": return this.cd(args);
            case "git": return this.git(args);
            case "wasm2wat":
            case "wat2wasm": return this.wabtTool(cmd, args, redirectPath);
            case "run": [cmd, ...args] = args; if (!cmd) { await pty.write("usage: run <file.wasm> [args]\r\n"); return 1; }
        }

        if (COREUTILS_FS.has(cmd) || COREUTILS_NOFS.has(cmd)) return this.coreutil(cmd, args, stdio);
        if (cmd.endsWith(".wasm")) return this.runWasm(cmd, args, redirectPath);
        await pty.write(`lo-shell: ${cmd}: command not found\r\n`);
        return 127;
    }

    /** git status, add, restore, reset, commit and log through the scratch-git extension. */
    async git([sub, ...args]) {
        const { pty } = this;
        const say = (text) => pty.write(text.replace(/\r?\n/g, "\r\n"));
        const fatal = async (message) => { await say(`fatal: ${message}\n`); return 128; };
        if (!sub) { await say("usage: git <status|add|restore|reset|commit|log> ...\n"); return 1; }
        if (!["status", "add", "restore", "reset", "commit", "log"].includes(sub)) { await say(`git: '${sub}' is not a git command.\n`); return 1; }

        // Paths become relative to the workspace root, where the repository is.
        const specs = (paths) => paths.map((p) => {
            const abs = posixJoin(this.cwd, p);
            if (abs !== "/workspace" && !abs.startsWith("/workspace/")) throw new Error(`${p}: not inside the workspace`);
            return abs.slice("/workspace/".length);
        });
        const git = await vscode.extensions.getExtension("lo-playground.scratch-git").activate();
        try {
            switch (sub) {
                case "status": {
                    const { staged, unstaged } = await git.status();
                    const label = { added: "new file:   ", modified: "modified:   ", deleted: "deleted:    " };
                    const section = (title, lines) => lines.length ? `${title}\n${lines.map((l) => `        ${l}`).join("\n")}\n\n` : "";
                    const text = section("Changes to be committed:", staged.map((c) => label[c.status] + c.rel))
                        + section("Changes not staged for commit:", unstaged.filter((c) => c.status !== "untracked").map((c) => label[c.status] + c.rel))
                        + section("Untracked files:", unstaged.filter((c) => c.status === "untracked").map((c) => c.rel));
                    await say("On branch main\n" + (text || "nothing to commit, working tree clean\n"));
                    return 0;
                }
                case "add":
                    if (!args.length) { await say("Nothing specified, nothing added.\n"); return 0; }
                    await git.add(specs(args));
                    return 0;
                case "restore": {
                    const staged = args.includes("--staged");
                    const paths = args.filter((a) => a !== "--staged");
                    if (!paths.length) return fatal("you must specify path(s) to restore");
                    await (staged ? git.unstage : git.restore)(specs(paths));
                    return 0;
                }
                case "reset": {
                    const paths = args.filter((a) => a !== "--");
                    if (!paths.length) return fatal("specify the path(s) to unstage");
                    await git.unstage(specs(paths));
                    return 0;
                }
                case "commit": {
                    let all = false, message;
                    for (let i = 0; i < args.length; i++) {
                        if (args[i] === "-a") all = true;
                        else if (args[i] === "-m") message = args[++i];
                        else if (args[i] === "-am") { all = true; message = args[++i]; }
                        else return fatal(`unsupported argument '${args[i]}'`);
                    }
                    if (!message) { await say("Aborting commit due to empty commit message.\n"); return 1; }
                    const { oid } = await git.commit(message, all);
                    await say(`[main ${oid.slice(0, 7)}] ${message}\n`);
                    return 0;
                }
                case "log": {
                    for (const e of await git.log()) {
                        await say(`commit ${e.oid}\nAuthor: ${e.commit.author.name} <${e.commit.author.email}>\nDate:   ${new Date(e.commit.author.timestamp * 1000).toLocaleString()}\n\n    ${e.commit.message.trim().split("\n").join("\n    ")}\n\n`);
                    }
                    return 0;
                }
            }
        } catch (e) {
            if (e?.message === "nothing to commit") { await say("nothing to commit\n"); return 1; }
            return fatal(e?.message ?? String(e));
        }
    }

    /** wasm2wat and wat2wasm through the wasm-viewer extension. Writes to `-o file`, `> file` or stdout (text only). */
    async wabtTool(cmd, args, redirectPath) {
        let input, output;
        for (let i = 0; i < args.length; i++) {
            if (args[i] === "-o") output = args[++i]; else if (!input) input = args[i];
        }
        if (!input) { await this.pty.write(`usage: ${cmd} <input> [-o output]\r\n`); return 1; }
        const inUri = this.toUri(posixJoin(this.cwd, input));
        if (!inUri) { await this.pty.write(`lo-shell: ${input}: not inside the workspace\r\n`); return 1; }
        let bytes;
        try { bytes = await vscode.workspace.fs.readFile(inUri); }
        catch { await this.pty.write(`lo-shell: ${input}: No such file\r\n`); return 1; }

        const api = await vscode.extensions.getExtension("lo-playground.wasm-viewer").activate();
        const isWat = cmd === "wasm2wat";
        const result = isWat ? new TextEncoder().encode(await api.wasm2wat(bytes)) : await api.wat2wasm(new TextDecoder().decode(bytes), input);

        let target = output ? posixJoin(this.cwd, output) : redirectPath;
        if (!target && !isWat) target = posixJoin(this.cwd, input.replace(/\.(wat|wast)$/, "") + ".wasm"); // like the wabt tools
        if (target) {
            const outUri = this.toUri(target);
            if (!outUri) { await this.pty.write(`lo-shell: ${target}: not inside the workspace\r\n`); return 1; }
            await vscode.workspace.fs.writeFile(outUri, result);
        } else {
            await this.pty.write(new TextDecoder().decode(result).replace(/\r?\n/g, "\r\n"));
        }
        return 0;
    }

    async cd(args) {
        const target = posixJoin(this.cwd, args[0] ?? "/workspace");
        try {
            const stat = await this.state.rootFs.stat(target);
            if (stat.filetype === 1 /* Filetype.directory */) { this.cwd = target; return 0; }
        } catch { /* fallthrough */ }
        await this.pty.write(`lo-shell: cd: ${target}: No such file or directory\r\n`);
        return 1;
    }

    async coreutil(cmd, args, stdio) {
        const module = await this.state.coreutils;
        let newArgs = [cmd, ...args], options = { stdio, trace: false };
        if (COREUTILS_FS.has(cmd)) {
            let fileFound = false;
            newArgs = [cmd, ...args.map((a) => a.startsWith("-") ? a : (fileFound = true, posixJoin(this.cwd, a)))];
            if (!fileFound && COREUTILS_ADD_CWD.has(cmd)) newArgs.push(this.cwd);
            options.rootFileSystem = this.state.rootFs;
        }
        options.args = newArgs;
        return (await (await this.wasm.createProcess("coreutils", module, options)).run());
    }

    /** Mounts the current directory at `/` for the program, so relative paths work. `> file` writes the captured stdout. */
    async runWasm(file, args, redirectPath) {
        const uri = this.toUri(posixJoin(this.cwd, file));
        const cwdUri = this.toUri(this.cwd);
        if (!uri || !cwdUri) { await this.pty.write(`lo-shell: ${file}: not inside the workspace\r\n`); return 1; }
        let bytes;
        try { bytes = await vscode.workspace.fs.readFile(uri); }
        catch { await this.pty.write(`lo-shell: ${file}: No such file\r\n`); return 127; }
        const module = await WebAssembly.compile(bytes);

        const chunks = [];
        let out = this.pty.stdio.out;
        if (redirectPath) {
            const pipe = this.wasm.createReadable();
            pipe.onData((c) => chunks.push(c));
            out = { kind: "pipeOut", pipe };
        }
        const proc = await this.wasm.createProcess(file.split("/").pop(), module, {
            args, env: {}, trace: false,
            stdio: { ...this.pty.stdio, out },
            mountPoints: [{ kind: "vscodeFileSystem", uri: cwdUri, mountPoint: "/" }],
        });
        const code = await proc.run();
        if (redirectPath) {
            const target = this.toUri(redirectPath);
            if (!target) { await this.pty.write(`lo-shell: ${redirectPath}: not inside the workspace\r\n`); return 1; }
            const data = new Uint8Array(chunks.reduce((n, c) => n + c.length, 0));
            let o = 0; for (const c of chunks) { data.set(c, o); o += c.length; }
            await vscode.workspace.fs.writeFile(target, data);
        }
        return code;
    }
}

exports.activate = async (context) => {
    const loader = await vscode.extensions.getExtension("ms-vscode.wasm-wasi-core").activate();
    const wasm = typeof loader.load === "function" ? loader.load(1) : loader;
    const state = {
        rootFs: await wasm.createRootFileSystem([{ kind: "workspaceFolder" }]),
        coreutils: (async () => {
            const res = await fetch(vscode.Uri.joinPath(context.extensionUri, "wasm", "coreutils.wasm").toString());
            if (!res.ok) throw new Error(`coreutils.wasm: ${res.status}`);
            return WebAssembly.compile(await res.arrayBuffer());
        })(),
    };
    state.coreutils.catch(() => {}); // surfaced when a coreutil is first used

    const create = () => {
        const shell = new Shell(wasm, state);
        return { shell, pty: shell.pty };
    };

    context.subscriptions.push(
        vscode.window.registerTerminalProfileProvider("lo-shell.shell", {
            provideTerminalProfile() {
                const { shell, pty } = create();
                const profile = new vscode.TerminalProfile({ name: "LO Shell", pty, isTransient: true });
                setTimeout(() => void shell.loop({ dispose() { vscode.window.terminals.find((t) => t.creationOptions.pty === pty)?.dispose(); } }), 0);
                return profile;
            },
        }),
        vscode.commands.registerCommand("lo-shell.new", () => {
            const { shell, pty } = create();
            const terminal = vscode.window.createTerminal({ name: "LO Shell", pty, isTransient: true });
            terminal.show();
            void shell.loop(terminal);
        }),
    );

    // Called by src/main.js for a terminal panel restored as open.
    context.subscriptions.push(vscode.commands.registerCommand("lo-shell.ensure", () => {
        if (vscode.window.terminals.length === 0) vscode.commands.executeCommand("lo-shell.new");
    }));
    context.subscriptions.push(vscode.commands.registerCommand("lo-shell.focus", () => {
        (vscode.window.activeTerminal ?? vscode.window.terminals[0])?.show(false);
    }));
};
