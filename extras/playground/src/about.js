// "About LO Playground" dialog, opened by clicking the remote indicator in the bottom left (windowIndicator in main.js).
// A plain HTML <dialog> (VS Code's own dialogs can't show links or formatting) styled with the current theme's colors.
const REPO = "https://github.com/glebbash/LO";

const css = `
.lo-about { padding: 0; border: 1px solid var(--vscode-widget-border, #454545); border-radius: 8px; width: min(520px, calc(100vw - 32px));
    background: var(--vscode-editorWidget-background, #252526); color: var(--vscode-editorWidget-foreground, var(--vscode-foreground, #ccc));
    font: 13px/1.5 var(--vscode-font-family, system-ui, sans-serif); box-shadow: 0 8px 32px var(--vscode-widget-shadow, rgba(0,0,0,.5)); }
.lo-about::backdrop { background: rgba(0, 0, 0, .45); }
.lo-about * { box-sizing: border-box; }
.lo-about header { display: flex; align-items: center; gap: 14px; padding: 20px 24px 8px; }
.lo-about header img { width: 56px; height: auto; flex: none; border-radius: 6px; background: #feb87c; padding: 6px; }
.lo-about h1 { margin: 0; font-size: 20px; font-weight: 700; }
.lo-about header p { margin: 2px 0 0; opacity: .75; }
.lo-about .body { padding: 8px 24px 4px; }
.lo-about .body p { margin: 10px 0; }
.lo-about ul { margin: 10px 0; padding-left: 20px; }
.lo-about li { margin: 4px 0; }
.lo-about a { color: var(--vscode-textLink-foreground, #3794ff); text-decoration: none; }
.lo-about a:hover { text-decoration: underline; color: var(--vscode-textLink-activeForeground, #3794ff); }
.lo-about footer { display: flex; align-items: center; justify-content: space-between; gap: 12px; padding: 12px 24px 20px; }
.lo-about .small { opacity: .65; font-size: 12px; }
.lo-about .buttons { display: flex; gap: 8px; }
.lo-about button, .lo-about a.btn { font: inherit; padding: 5px 14px; border-radius: 2px; border: 1px solid transparent; cursor: pointer; text-decoration: none; display: inline-block; }
.lo-about a.btn, .lo-about button.primary { background: var(--vscode-button-background, #0e639c); color: var(--vscode-button-foreground, #fff); }
.lo-about a.btn:hover, .lo-about button.primary:hover { background: var(--vscode-button-hoverBackground, #1177bb); text-decoration: none; color: var(--vscode-button-foreground, #fff); }
.lo-about button.secondary { background: var(--vscode-button-secondaryBackground, #3a3d41); color: var(--vscode-button-secondaryForeground, #fff); }
.lo-about button.secondary:hover { background: var(--vscode-button-secondaryHoverBackground, #45494e); }
.lo-about :focus-visible { outline: 1px solid var(--vscode-focusBorder, #007fd4); outline-offset: 2px; }
`;

let vscodeVersion;
async function getVscodeVersion() {
    vscodeVersion ??= await fetch("/vscode/package.json").then((r) => r.json()).then((j) => j.version).catch(() => "");
    return vscodeVersion;
}

export async function showAbout() {
    if (document.querySelector("dialog.lo-about")) return;
    const version = await getVscodeVersion();

    const host = document.querySelector(".monaco-workbench") ?? document.body; // theme variables live on the workbench
    const dialog = Object.assign(document.createElement("dialog"), { className: "lo-about" });
    dialog.setAttribute("aria-label", "About LO Playground");
    dialog.innerHTML = `
        <style>${css}</style>
        <header>
            <img src="/lo.svg" alt="">
            <div>
                <h1>LO Playground</h1>
                <p>Try the LO programming language in your browser</p>
            </div>
        </header>
        <div class="body">
            <p><strong>LO</strong> is a small programming language that compiles to WebAssembly. Here the compiler
            (<code>lo.wasm</code>) runs as WebAssembly inside a VS Code web build, so there is nothing to install.</p>
            <ul>
                <li>Your files live in the <em>Scratch FS</em> workspace, saved in this browser (IndexedDB), and can be
                    downloaded as a zip with <code>Scratch: Download workspace as zip</code>.</li>
                <li>Run a file with <kbd>▶</kbd> in the editor toolbar, or from the terminal: <code>./lo.wasm compile main.lo &gt; main.wasm</code>.</li>
                <li>Everything runs locally, no code is sent to a server.</li>
            </ul>
            <p>Source, docs and issues: <a href="${REPO}" target="_blank" rel="noopener noreferrer">github.com/glebbash/LO</a></p>
        </div>
        <footer>
            <span class="small">VS Code for the Web${version ? " " + version : ""}</span>
            <span class="buttons">
                <a class="btn" href="${REPO}" target="_blank" rel="noopener noreferrer">Open GitHub repo</a>
                <button class="secondary" value="close" autofocus>Close</button>
            </span>
        </footer>`;
    dialog.addEventListener("click", (e) => { if (e.target === dialog) dialog.close(); }); // click on the backdrop
    dialog.querySelector("button[value=close]").addEventListener("click", () => dialog.close());
    dialog.addEventListener("close", () => dialog.remove());
    host.append(dialog);
    dialog.showModal();
}
