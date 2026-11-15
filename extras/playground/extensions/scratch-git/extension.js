// Local git for the scratch: workspace, on top of isomorphic-git (vendor/isomorphic-git/, added by scripts.js).
// The repository lives in the workspace as .git. There is no staging area, a commit takes every change.
const vscode = require("vscode");

const AUTHOR = { name: "LO Playground", email: "playground@localhost" };
const SCHEME = "scratch-git";

let gitPromise;
function loadGit(context) {
    return (gitPromise ??= (async () => {
        const vendor = vscode.Uri.joinPath(context.extensionUri, "vendor", "isomorphic-git");
        const text = async (name) => {
            const res = await fetch(vscode.Uri.joinPath(vendor, name).toString());
            if (!res.ok) throw new Error(`${name}: ${res.status}`);
            return res.text();
        };
        // isomorphic-git expects a global Buffer, which a worker lacks. It is passed in as a parameter so that other
        // extensions in the same host do not see it.
        const { Buffer } = new Function(`return ${await text("buffer.js")}`)();
        const module = { exports: {} };
        new Function("module", "exports", "Buffer", await text("index.umd.min.js"))(module, module.exports, Buffer);
        return module.exports;
    })());
}

const fsError = (code, path) => Object.assign(new Error(`${code}: ${path}`), { code });

/** The subset of the node `fs.promises` API that isomorphic-git uses, backed by the entries of ScratchFS. */
function createFs(store) {
    const DIR = vscode.FileType.Directory;
    // isomorphic-git passes paths like "/Scratch FS/.", so "." and ".." segments are resolved.
    const norm = (p) => {
        const out = [];
        for (const part of p.split("/")) {
            if (part === "..") out.pop();
            else if (part && part !== ".") out.push(part);
        }
        return "/" + out.join("/");
    };
    const parentOf = (p) => p.slice(0, p.lastIndexOf("/")) || "/";
    const entry = (p) => {
        const e = store.entries.get(norm(p));
        if (!e) throw fsError("ENOENT", p);
        return e;
    };
    const stat = async (p) => {
        const e = entry(p);
        const isDir = e.type === DIR;
        return {
            type: isDir ? "dir" : "file", mode: isDir ? 0o40000 : 0o100644, size: e.data.byteLength, ino: 0,
            mtimeMs: e.mtime, ctimeMs: e.ctime, uid: 1, gid: 1, dev: 1,
            isFile: () => !isDir, isDirectory: () => isDir, isSymbolicLink: () => false,
        };
    };
    return {
        promises: {
            async readFile(p, options) {
                const e = entry(p);
                if (e.type === DIR) throw fsError("EISDIR", p);
                const encoding = typeof options === "string" ? options : options?.encoding;
                return encoding ? new TextDecoder().decode(e.data) : e.data;
            },
            async writeFile(p, data) {
                p = norm(p);
                if (entry(parentOf(p)).type !== DIR) throw fsError("ENOTDIR", p);
                if (store.entries.get(p)?.type === DIR) throw fsError("EISDIR", p);
                store._set(p, typeof data === "string" ? new TextEncoder().encode(data) : new Uint8Array(data));
                store._persist();
            },
            async unlink(p) {
                if (entry(p).type === DIR) throw fsError("EISDIR", p);
                store.entries.delete(norm(p));
                store._persist();
            },
            async readdir(p) {
                p = norm(p);
                if (entry(p).type !== DIR) throw fsError("ENOTDIR", p);
                const prefix = p === "/" ? "/" : p + "/";
                const names = [];
                for (const k of store.entries.keys()) if (k !== p && k.startsWith(prefix) && !k.slice(prefix.length).includes("/")) names.push(k.slice(prefix.length));
                return names;
            },
            async mkdir(p) {
                p = norm(p);
                if (store.entries.has(p)) throw fsError("EEXIST", p);
                if (!store.entries.has(parentOf(p))) throw fsError("ENOENT", p);
                store._mkdir(p);
                store._persist();
            },
            async rmdir(p) {
                p = norm(p);
                if (entry(p).type !== DIR) throw fsError("ENOTDIR", p);
                if ((await this.readdir(p)).length) throw fsError("ENOTEMPTY", p);
                store.entries.delete(p);
                store._persist();
            },
            stat,
            lstat: stat,
            async readlink(p) { throw fsError("ENOENT", p); },
            async symlink(_target, p) { throw fsError("EPERM", p); },
        },
    };
}

const STATUS = {
    // Working tree against the index
    modified: { badge: "M", tooltip: "Modified", color: "gitDecoration.modifiedResourceForeground" },
    untracked: { badge: "U", tooltip: "Untracked", color: "gitDecoration.untrackedResourceForeground" },
    deleted: { badge: "D", tooltip: "Deleted", color: "gitDecoration.deletedResourceForeground" },
};
const STAGED = {
    // Index against HEAD
    modified: { badge: "M", tooltip: "Staged: modified", color: "gitDecoration.stageModifiedResourceForeground" },
    added: { badge: "A", tooltip: "Staged: added", color: "gitDecoration.addedResourceForeground" },
    deleted: { badge: "D", tooltip: "Staged: deleted", color: "gitDecoration.stageDeletedResourceForeground" },
};
const isBinary = (data) => data.subarray(0, 8192).includes(0);
const sameBytes = (a, b) => a.byteLength === b.byteLength && a.every((v, i) => v === b[i]);

exports.activate = async (context) => {
    const scratch = vscode.extensions.getExtension("lo-playground.scratch-fs");
    const { fs: store, root: ROOT, ready } = await scratch.activate();
    const git = await loadGit(context);
    const gitFs = createFs(store);
    const dir = ROOT;
    const base = { fs: gitFs, dir };
    const rootUri = vscode.Uri.from({ scheme: "scratch", path: ROOT });
    const fileUri = (rel) => vscode.Uri.from({ scheme: "scratch", path: `${ROOT}/${rel}` });
    const relPath = (uri) => uri.path.slice(ROOT.length + 1);
    const showUri = (rel, ref = "HEAD") => vscode.Uri.from({ scheme: SCHEME, path: `${ROOT}/${rel}`, query: ref });

    // Operations that write to the repository run one at a time.
    let chain = Promise.resolve();
    const serial = (fn) => {
        const result = chain.then(fn);
        chain = result.catch(() => {});
        return result;
    };
    const report = (e) => vscode.window.showErrorMessage(`Git: ${e?.message ?? e}`);

    const readAt = async (rel, ref = "HEAD") => {
        const oid = await git.resolveRef({ ...base, ref });
        return (await git.readBlob({ ...base, oid, filepath: rel })).blob;
    };
    const readWork = (rel) => store.readFile(fileUri(rel));
    const ensureDir = (uri) => {
        const parent = uri.with({ path: uri.path.slice(0, uri.path.lastIndexOf("/")) });
        if (parent.path.length <= ROOT.length) return;
        try {
            store.stat(parent);
        } catch {
            ensureDir(parent);
            store.createDirectory(parent);
        }
    };
    const writeWork = (rel, data) => {
        const uri = fileUri(rel);
        ensureDir(uri);
        store.writeFile(uri, data, { create: true, overwrite: true });
    };
    const removeWork = (rel) => {
        try { store.delete(fileUri(rel)); } catch { /* already gone */ }
    };

    // Source control view
    const scm = vscode.scm.createSourceControl("scratch-git", "Scratch Git", rootUri);
    scm.inputBox.placeholder = "Message (Ctrl+Enter to commit)";
    scm.acceptInputCommand = { command: "scratchGit.commit", title: "Commit" };
    const stagedGroup = scm.createResourceGroup("staged", "Staged Changes");
    const group = scm.createResourceGroup("changes", "Changes");
    stagedGroup.hideWhenEmpty = group.hideWhenEmpty = true;
    context.subscriptions.push(scm);

    // The index entries (path -> blob id) as of the last refresh.
    let indexOids = new Map();
    const readIndex = async (rel) => {
        const oid = indexOids.get(rel);
        if (!oid) throw new Error(`${rel} is not in the index`);
        return (await git.readBlob({ ...base, oid })).blob;
    };
    const readIndexOids = async () => {
        const out = new Map();
        await git.walk({
            ...base,
            trees: [git.STAGE()],
            map: async (rel, [entry]) => {
                if (rel !== "." && entry && (await entry.type()) === "blob") out.set(rel, await entry.oid());
            },
        });
        return out;
    };

    // Original side of the editor gutter markers and of the diffs. The query is HEAD, INDEX or a commit id.
    const originals = new vscode.EventEmitter();
    context.subscriptions.push(vscode.workspace.registerTextDocumentContentProvider(SCHEME, {
        onDidChange: originals.event,
        async provideTextDocumentContent(uri) {
            try {
                const rel = relPath(uri);
                const blob = uri.query === "INDEX" ? await readIndex(rel) : await readAt(rel, uri.query || "HEAD");
                return isBinary(blob) ? `Binary file, ${blob.byteLength} bytes\n` : new TextDecoder().decode(blob);
            } catch {
                return "";
            }
        },
    }));
    scm.quickDiffProvider = {
        provideOriginalResource: (uri) => (uri.scheme === "scratch" && indexOids.has(relPath(uri)) ? showUri(relPath(uri), "INDEX") : undefined),
    };

    const decorationsChanged = new vscode.EventEmitter();
    let decorations = new Map();
    context.subscriptions.push(vscode.window.registerFileDecorationProvider({
        onDidChangeFileDecorations: decorationsChanged.event,
        provideFileDecoration(uri) {
            const s = decorations.get(uri.toString());
            return s && new vscode.FileDecoration(s.badge, s.tooltip, new vscode.ThemeColor(s.color));
        },
    }));

    /** @type {{rel: string, status: string}[]} */
    let staged = [], unstaged = [];
    const refresh = () => serial(async () => {
        const [rows, index] = await Promise.all([git.statusMatrix(base), readIndexOids()]);
        indexOids = index;
        staged = [];
        unstaged = [];
        // Rows are [path, HEAD, workdir, stage]. HEAD: 0 absent, 1 present. Workdir: 0 absent, 1 same as HEAD,
        // 2 different. Stage: 0 absent, 1 same as HEAD, 2 same as workdir, 3 differs from both.
        for (const [rel, head, work, stage] of rows) {
            if (stage === 0 ? head === 1 : stage !== 1) staged.push({ rel, status: stage === 0 ? "deleted" : head === 0 ? "added" : "modified" });
            if (work === 0 ? stage !== 0 : stage === 0 || stage === 3 || (stage === 1 && work === 2)) {
                unstaged.push({ rel, status: work === 0 ? "deleted" : stage === 0 ? "untracked" : "modified" });
            }
        }
        const byPath = (a, b) => (a.rel < b.rel ? -1 : 1);
        staged.sort(byPath);
        unstaged.sort(byPath);
        const previous = decorations;
        decorations = new Map([
            ...staged.map((c) => [fileUri(c.rel).toString(), STAGED[c.status]]),
            ...unstaged.map((c) => [fileUri(c.rel).toString(), STATUS[c.status]]),
        ]);
        stagedGroup.resourceStates = staged.map(({ rel, status }) => ({
            resourceUri: fileUri(rel),
            decorations: { strikeThrough: status === "deleted", tooltip: STAGED[status].tooltip },
            command: { command: "scratchGit.openChange", title: "Open", arguments: [rel, true, status] },
        }));
        group.resourceStates = unstaged.map(({ rel, status }) => ({
            resourceUri: fileUri(rel),
            decorations: { strikeThrough: status === "deleted", tooltip: STATUS[status].tooltip },
            command: { command: "scratchGit.openChange", title: "Open", arguments: [rel, false, status] },
        }));
        scm.count = staged.length + unstaged.length;
        decorationsChanged.fire([...new Set([...previous.keys(), ...decorations.keys()])].map((u) => vscode.Uri.parse(u)));
        for (const doc of vscode.workspace.textDocuments) if (doc.uri.scheme === SCHEME) originals.fire(doc.uri);
    });

    const stage = (changes) => serial(async () => {
        const add = changes.filter((c) => c.status !== "deleted").map((c) => c.rel);
        if (add.length) await git.add({ ...base, filepath: add });
        for (const c of changes) if (c.status === "deleted") await git.remove({ ...base, filepath: c.rel });
    });
    const unstage = (changes) => serial(async () => {
        for (const c of changes) await git.resetIndex({ ...base, filepath: c.rel });
    });
    const commitStaged = (message) => serial(() => git.commit({ ...base, message, author: AUTHOR }));

    const createRepository = () => vscode.window.withProgress({ location: vscode.ProgressLocation.Notification, title: "Creating the git repository" }, async () => {
        await git.init({ ...base, defaultBranch: "main" });
        await git.add({ ...base, filepath: "." });
        await git.commit({ ...base, message: "initial commit", author: AUTHOR });
    });

    // Working tree operations
    const restoreFile = async (rel, status) => {
        if (status === "untracked") removeWork(rel);
        else writeWork(rel, await readIndex(rel));
    };
    const confirm = async (message, action) => (await vscode.window.showWarningMessage(message, { modal: true }, action)) === action;
    const statesOf = (args, list) => [...new Set(args.flat().filter((s) => s?.resourceUri).map((s) => list.find((c) => fileUri(c.rel).toString() === s.resourceUri.toString())).filter(Boolean))];

    const restoreCommit = async (oid) => {
        for (const c of staged) await git.resetIndex({ ...base, filepath: c.rel });
        const files = await git.listFiles({ ...base, ref: oid });
        const wanted = new Set(files);
        for (const rel of files) {
            const blob = await readAt(rel, oid);
            let current;
            try { current = readWork(rel); } catch { /* missing */ }
            if (!current || !sameBytes(current, blob)) writeWork(rel, blob);
        }
        const rows = await git.statusMatrix(base);
        for (const [rel, , work] of rows) if (work !== 0 && !wanted.has(rel)) removeWork(rel);
    };

    const changedInCommit = async (oid) => {
        const { commit } = await git.readCommit({ ...base, oid });
        const parent = commit.parent[0];
        const trees = parent ? [git.TREE({ ref: oid }), git.TREE({ ref: parent })] : [git.TREE({ ref: oid })];
        const out = [];
        await git.walk({
            ...base,
            trees,
            map: async (rel, [now, before]) => {
                if (rel === ".") return;
                if ((now && (await now.type()) === "tree") || (before && (await before.type()) === "tree")) return;
                const a = now && (await now.oid()), b = before && (await before.oid());
                if (a !== b) out.push({ rel, status: !before ? "added" : !now ? "deleted" : "modified", parent });
            },
        });
        return out.sort((x, y) => (x.rel < y.rel ? -1 : 1));
    };

    const showHistory = async () => {
        const log = await git.log({ ...base, depth: 200 });
        const picked = await vscode.window.showQuickPick(
            log.map((e) => ({
                label: e.commit.message.split("\n")[0],
                description: e.oid.slice(0, 7),
                detail: new Date(e.commit.committer.timestamp * 1000).toLocaleString(),
                oid: e.oid,
            })),
            { title: "Scratch Git: history", placeHolder: "Pick a commit" },
        );
        if (!picked) return;
        const action = await vscode.window.showQuickPick(
            [
                { label: "$(diff) Show changed files", id: "show" },
                { label: "$(history) Restore the workspace to this commit", description: "history is kept, commit the result to make it permanent", id: "restore" },
            ],
            { title: `${picked.description} ${picked.label}` },
        );
        if (action?.id === "restore") {
            if (!(await confirm(`Restore all files to commit ${picked.description}? Uncommitted changes are lost.`, "Restore"))) return;
            await serial(() => restoreCommit(picked.oid));
            await refresh();
        } else if (action?.id === "show") {
            const files = await changedInCommit(picked.oid);
            if (!files.length) return void vscode.window.showInformationMessage("No file changes in this commit.");
            const file = await vscode.window.showQuickPick(files.map((f) => ({ label: f.rel, description: f.status, file: f })), { title: `${picked.description} ${picked.label}` });
            if (!file) return;
            const { rel, status, parent } = file.file;
            const title = `${rel} (${picked.description})`;
            if (status === "modified") await vscode.commands.executeCommand("vscode.diff", showUri(rel, parent), showUri(rel, picked.oid), title);
            else await vscode.commands.executeCommand("vscode.open", showUri(rel, status === "deleted" ? parent : picked.oid), undefined, title);
        }
    };

    const guard = (fn) => async (...args) => {
        try { return await fn(...args); } catch (e) { console.error(e); report(e); }
    };
    const register = (id, fn) => context.subscriptions.push(vscode.commands.registerCommand(id, guard(fn)));

    register("scratchGit.refresh", refresh);
    register("scratchGit.history", showHistory);
    register("scratchGit.commit", async () => {
        if (!staged.length) {
            if (!unstaged.length) return void vscode.window.showInformationMessage("Nothing to commit.");
            if (!(await confirm("There are no staged changes to commit. Stage all changes and commit them directly?", "Commit All"))) return;
            await stage(unstaged);
        }
        let message = scm.inputBox.value.trim();
        if (!message) message = (await vscode.window.showInputBox({ prompt: "Commit message" }))?.trim();
        if (!message) return void (await refresh());
        await commitStaged(message);
        scm.inputBox.value = "";
        await refresh();
    });
    register("scratchGit.openChange", async (rel, isStaged, status) => {
        const open = (uri, title) => vscode.commands.executeCommand("vscode.open", uri, undefined, title);
        if (isStaged) {
            if (status === "deleted") return open(showUri(rel), `${rel} (deleted)`);
            if (status === "added") return open(showUri(rel, "INDEX"), `${rel} (Index)`);
            if (isBinary(await readIndex(rel))) return open(fileUri(rel));
            return vscode.commands.executeCommand("vscode.diff", showUri(rel), showUri(rel, "INDEX"), `${rel} (Index)`);
        }
        if (status === "untracked") return open(fileUri(rel));
        if (status === "deleted") return open(showUri(rel, "INDEX"), `${rel} (deleted)`);
        if (isBinary(await readIndex(rel))) return open(fileUri(rel));
        return vscode.commands.executeCommand("vscode.diff", showUri(rel, "INDEX"), fileUri(rel), `${rel} (Working Tree)`);
    });
    const stageCommand = (id, list, action, all) => register(id, async (...args) => {
        const targets = all ? list() : statesOf(args, list());
        if (!targets.length) return;
        await action(targets);
        await refresh();
    });
    stageCommand("scratchGit.stage", () => unstaged, stage);
    stageCommand("scratchGit.stageAll", () => unstaged, stage, true);
    stageCommand("scratchGit.unstage", () => staged, unstage);
    stageCommand("scratchGit.unstageAll", () => staged, unstage, true);
    const discard = async (targets, label) => {
        if (!targets.length) return;
        if (!(await confirm(`Discard the changes in ${label ?? (targets.length === 1 ? `"${targets[0].rel}"` : `${targets.length} files`)}? This cannot be undone.`, "Discard"))) return;
        for (const t of targets) await restoreFile(t.rel, t.status);
        await refresh();
    };
    register("scratchGit.discard", (...args) => discard(statesOf(args, unstaged)));
    register("scratchGit.discardAll", () => discard(unstaged, `${unstaged.length} files`));

    // Refresh when files change, git's own files are not part of that.
    let timer;
    context.subscriptions.push(store.onDidChangeFile((events) => {
        if (events.every((e) => e.uri.path.startsWith(`${ROOT}/.git/`))) return;
        clearTimeout(timer);
        timer = setTimeout(() => refresh().catch(console.error), 400);
    }));
    context.subscriptions.push({ dispose: () => clearTimeout(timer) });

    // The repository is created in the background, the API below waits for it.
    const repositoryReady = (async () => {
        if (!(await ready)) return false;
        if (!store.entries.has(`${ROOT}/.git`)) await serial(createRepository);
        await refresh();
        return true;
    })();
    repositoryReady.catch(report);

    // Used by the shell (lo-shell). Paths are relative to the workspace root, "" is everything.
    const select = (list, specs) => {
        const picked = new Set();
        for (const spec of specs) {
            const hits = list.filter((c) => spec === "" || c.rel === spec || c.rel.startsWith(`${spec}/`));
            if (!hits.length) throw new Error(`pathspec '${spec}' did not match any files`);
            hits.forEach((c) => picked.add(c));
        }
        return [...picked];
    };
    const api = async (fn) => {
        if (!(await repositoryReady)) throw new Error("not a git repository");
        await refresh();
        try {
            return await fn();
        } finally {
            await refresh();
        }
    };
    return {
        status: () => api(() => ({ staged, unstaged })),
        add: (specs) => api(() => stage(select(unstaged, specs))),
        unstage: (specs) => api(() => unstage(select(staged, specs))),
        restore: (specs) => api(async () => {
            for (const c of select(unstaged.filter((u) => u.status !== "untracked"), specs)) await restoreFile(c.rel, c.status);
        }),
        commit: (message, all) => api(async () => {
            if (all) {
                await stage(unstaged.filter((c) => c.status !== "untracked"));
                await refresh();
            }
            if (!staged.length) throw new Error("nothing to commit");
            return { oid: await commitStaged(message) };
        }),
        log: () => api(() => git.log({ ...base, depth: 200 })),
    };
};
