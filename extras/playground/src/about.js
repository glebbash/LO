// About dialog with a quick start, opened from the indicator in the bottom left (which pulses until it was opened once).
// Plain HTML, styled with the theme's colors, because VS Code's own dialogs have no links or formatting. The file links go
// through the `scratch.open` command of scratch-fs, which waits for the workspace to be seeded.
import { commands } from "/vscode/out/vs/workbench/workbench.web.main.internal.js";

const REPO = "https://github.com/glebbash/LO";
const SEEN_KEY = "lo-playground.about-seen"; // set once the dialog was opened, the indicator stops pulsing then
const TAB_KEY = "lo-playground.about-tab"; // the last selected tab
const MOD = /Mac|iPhone|iPad/.test(navigator.platform) ? "⌘" : "Ctrl";

// The binary literals walkthrough of the "Change the compiler" tab. The `find` texts are lines of lib/lo/lexer.lo that
// the links put the cursor on, the snippets go above them.
const LEXER_PREFIX = `if self.current_char()? as u8 == 'b' {
    self.next_char()
}`;
const LEXER_PREFIX_FIND = "if self.current_char()? as u8 == 'x' {";
const LEXER_VALUE = `if int_literal.starts_with("0b") {
    int_literal = int_literal.skip_chars(2)
    radix = 2 as i64
}`;
const LEXER_VALUE_FIND = 'if int_literal.starts_with("0x") {';
const REBUILD = "./lo.wasm compile lib/lo/_cli.lo > lo.wasm";
const TEST_PROGRAM = `export fn main(): u32 {
    return 0b101010
}`;

const css = `
.lo-about { padding: 0; border: 1px solid var(--vscode-widget-border, #454545); border-radius: 8px; width: min(620px, calc(100vw - 32px));
    /* anchored near the top, so switching to a longer tab doesn't move the dialog */
    margin: min(6vh, 48px) auto auto; max-height: calc(100vh - 2 * min(6vh, 48px)); background: var(--vscode-editorWidget-background, #252526);
    color: var(--vscode-editorWidget-foreground, var(--vscode-foreground, #ccc));
    font: 13px/1.5 var(--vscode-font-family, system-ui, sans-serif); box-shadow: 0 8px 32px var(--vscode-widget-shadow, rgba(0,0,0,.5)); }
.lo-about[open] { display: flex; flex-direction: column; }
.lo-about::backdrop { background: rgba(0, 0, 0, .45); }
.lo-about * { box-sizing: border-box; }
.lo-about header { display: flex; align-items: center; gap: 14px; padding: 20px 24px 4px; }
.lo-about header img { height: 48px; width: auto; flex: none; }
.lo-about h1 { margin: 0; font-size: 20px; font-weight: 700; line-height: 1.25; }
.lo-about header p { margin: 2px 0 0; opacity: .75; }
.lo-about .body { padding: 4px 24px 8px; overflow-y: auto; min-height: 0; overscroll-behavior: contain; }
.lo-about p { margin: 8px 0; }
.lo-about .muted { opacity: .75; }
.lo-about .chips { display: flex; flex-wrap: wrap; gap: 6px; margin: 12px 0 4px; padding: 0; list-style: none; }
.lo-about .chips li { margin: 0; padding: 1px 8px; border-radius: 10px; font-size: 12px;
    background: var(--vscode-badge-background, #4d4d4d); color: var(--vscode-badge-foreground, #fff); }
.lo-about [role=tablist] { display: flex; gap: 4px; margin: 14px 0 0; border-bottom: 1px solid var(--vscode-widget-border, #454545); }
.lo-about [role=tab] { display: inline-flex; align-items: center; gap: 6px; margin-bottom: -1px; padding: 6px 10px; border: 0;
    border-bottom: 2px solid transparent; background: none; color: inherit; font: inherit; font-weight: 600; cursor: pointer; opacity: .7; }
.lo-about [role=tab]:hover { opacity: 1; }
.lo-about [role=tab][aria-selected=true] { opacity: 1; border-bottom-color: #feb87c; }
.lo-about ol { margin: 8px 0; padding-left: 22px; }
.lo-about ol > li { margin: 8px 0; padding-left: 2px; }
.lo-about code, .lo-about pre, .lo-about kbd { font-family: var(--vscode-editor-font-family, ui-monospace, monospace); font-size: 12px; }
.lo-about code { padding: 1px 4px; border-radius: 3px; background: var(--vscode-textCodeBlock-background, rgba(127,127,127,.15)); }
.lo-about .code { position: relative; margin: 6px 0 2px; }
.lo-about pre { margin: 0; padding: 7px 64px 7px 10px; border-radius: 4px; overflow-x: auto; line-height: 1.45;
    background: var(--vscode-textCodeBlock-background, rgba(127,127,127,.15)); }
.lo-about kbd { padding: 0 4px; border-radius: 3px; border: 1px solid var(--vscode-keybindingLabel-border, rgba(128,128,128,.4));
    border-bottom-color: var(--vscode-keybindingLabel-bottomBorder, rgba(128,128,128,.4));
    background: var(--vscode-keybindingLabel-background, rgba(128,128,128,.17)); color: var(--vscode-keybindingLabel-foreground, inherit); }
.lo-about .ico { display: inline-flex; padding: 2px; border-radius: 4px; vertical-align: -4px;
    background: var(--vscode-toolbar-hoverBackground, rgba(90,93,94,.31)); }
.lo-about .ico .codicon { font-size: 16px; }
.lo-about a, .lo-about .link { color: var(--vscode-textLink-foreground, #3794ff); text-decoration: none; }
.lo-about a:hover, .lo-about .link:hover { text-decoration: underline; color: var(--vscode-textLink-activeForeground, #3794ff); }
.lo-about .link { padding: 0; border: 0; background: none; font: inherit; cursor: pointer; }
.lo-about .indicator { padding: 0 5px; border-radius: 2px; background: #feb87c; color: #000; font-weight: 700; white-space: nowrap; }
.lo-about footer { display: flex; align-items: center; justify-content: space-between; gap: 12px; padding: 12px 24px 18px;
    border-top: 1px solid var(--vscode-widget-border, #454545); }
.lo-about .small { opacity: .65; font-size: 12px; }
.lo-about .buttons { display: flex; gap: 8px; }
.lo-about button.btn, .lo-about a.btn { white-space: nowrap; font: inherit; padding: 5px 14px; border-radius: 2px; border: 1px solid transparent;
    cursor: pointer; text-decoration: none; display: inline-block; }
.lo-about a.btn { background: var(--vscode-button-background, #0e639c); color: var(--vscode-button-foreground, #fff); }
.lo-about a.btn:hover { background: var(--vscode-button-hoverBackground, #1177bb); text-decoration: none; color: var(--vscode-button-foreground, #fff); }
.lo-about button.secondary, .lo-about .copy { background: var(--vscode-button-secondaryBackground, #3a3d41); color: var(--vscode-button-secondaryForeground, #fff); }
.lo-about button.secondary:hover, .lo-about .copy:hover { background: var(--vscode-button-secondaryHoverBackground, #45494e); }
.lo-about .copy { position: absolute; top: 4px; right: 4px; padding: 1px 8px; border: 0; border-radius: 3px; font: inherit; font-size: 12px; cursor: pointer; }
.lo-about :focus-visible { outline: 1px solid var(--vscode-focusBorder, #007fd4); outline-offset: 2px; }
@media (max-width: 520px) { .lo-about footer { flex-direction: column; align-items: stretch; } }
`;

const esc = (s) =>
    String(s)
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;");
const icon = (name, label) =>
    `<span class="ico" role="img" aria-label="${label}" title="${label}"><span class="codicon codicon-${name}"></span></span>`;
const kbd = (...keys) => keys.map((k) => `<kbd>${k}</kbd>`).join("+");
// `line`: the copied text ends with a line break, for snippets that are pasted above a line
const snippet = (text, line = false) =>
    `<div class="code"><pre>${esc(text)}</pre><button type="button" class="copy" data-copy="${esc(line ? text + "\n" : text)}">Copy</button></div>`;
const openLink = (label, path, { find, selectAll } = {}) =>
    `<button type="button" class="link" data-open="${esc(path)}"${find ? ` data-find="${esc(find)}"` : ""}${selectAll ? " data-select-all" : ""}>${label}</button>`;

const storage = {
    get(key) {
        try {
            return localStorage.getItem(key);
        } catch {
            return null;
        }
    },
    set(key, value) {
        try {
            localStorage.setItem(key, value);
        } catch {
            /* storage is blocked, the dialog then opens on every visit */
        }
    },
};

let vscodeWebVersion;
async function getVscodeWebVersion() {
    vscodeWebVersion ??= await fetch("/playground-config.json")
        .then((r) => r.json())
        .then((j) => j.vscodeWeb)
        .catch(() => "");
    return vscodeWebVersion;
}

function template(version) {
    const run = icon("debug-start", "Run");
    return `
        <style>${css}</style>
        <header>
            <img src="/lo.svg" alt="">
            <div>
                <h1>LO Playground</h1>
                <p>A small self-hosted language that compiles to WebAssembly</p>
            </div>
        </header>
        <div class="body">
            <ul class="chips" aria-label="What's inside">
                <li>VS Code</li><li>LO compiler</li><li>WASM-4 with hot reload</li><li>Shell</li><li>Git</li><li>.wasm viewer</li>
            </ul>
            <p class="muted">It all runs in this tab: nothing to install, no code is sent to a server. Your files are saved in
            this browser, <code>Scratch: Download workspace as zip</code> keeps a copy.</p>

            <div role="tablist" aria-label="Quick start">
                <button type="button" role="tab" id="lo-about-tab-game" aria-controls="lo-about-game" data-tab="game">
                    <span class="codicon codicon-game"></span>Make a game</button>
                <button type="button" role="tab" id="lo-about-tab-compiler" aria-controls="lo-about-compiler" data-tab="compiler">
                    <span class="codicon codicon-tools"></span>Change the compiler</button>
            </div>

            <section role="tabpanel" id="lo-about-game" aria-labelledby="lo-about-tab-game">
                <p><a href="https://wasm4.org" target="_blank" rel="noopener noreferrer">WASM-4</a> is a fantasy console for tiny
                games: 160x160 pixels, 4 colors, up to 64 KB.</p>
                <ol>
                    <li>Open ${openLink("dark-maze.lo", "test/demos/wasm4/src/dark-maze.lo")}.</li>
                    <li>Press ${icon("game", "Run game")} in the editor's title bar, top right. The game opens on the left.</li>
                    <li>Click the game, press ${kbd("X")} to start and move with the arrow keys.</li>
                    <li>Edit the code and save (${kbd(MOD, "S")}): the game reloads with your change. Hot swap
                        ${icon("beaker", "Hot swap")} keeps its state, ${icon("debug-rerun", "Restart")} restarts it.</li>
                </ol>
                <p class="muted">More smaller game starters are in <code>test/demos/wasm4/src/</code>. <code>WASM-4: Export HTML</code> saves a game
                as a single page you can share.</p>
            </section>

            <section role="tabpanel" id="lo-about-compiler" aria-labelledby="lo-about-tab-compiler" hidden>
                <p>LO is self-hosted: the compiler is written in LO, about 15k lines in <code>lib/lo/</code>. <code>lo.wasm</code>
                in the workspace root is that compiler built by itself, and ${run}, the errors in the editor and the games all
                use it. Here is how to add binary literals like <code>0b101010</code>:</p>
                <ol>
                    <li>Teach the lexer the <code>0b</code> prefix: in <code>lib/lo/lexer.lo</code>, paste this above the
                        <code>0x</code> check in ${openLink("lex_int_literal", "lib/lo/lexer.lo", { find: LEXER_PREFIX_FIND })}
                        (the link puts the cursor there):
                        ${snippet(LEXER_PREFIX, true)}</li>
                    <li>Then the digits: paste this above the <code>0x</code> check in
                        ${openLink("parse_int_literal_value", "lib/lo/lexer.lo", { find: LEXER_VALUE_FIND })} and save:
                        ${snippet(LEXER_VALUE, true)}</li>
                    <li>Rebuild the compiler with itself in a <button type="button" class="link" data-act="terminal">terminal</button>:
                        ${snippet(REBUILD)}</li>
                    <li>Replace the code in ${openLink("main.lo", "main.lo", { selectAll: true })} with this, save and press ${run}.
                        It shows <code>main() returned 42</code>.
                        ${snippet(TEST_PROGRAM, true)}</li>
                </ol>
                <p class="muted">Broke the compiler? <code>git restore lo.wasm</code> in the terminal brings the original back.</p>
            </section>
        </div>
        <footer>
            <span>
                Reopen this with <span class="indicator">LO Playground</span> in the bottom left<br>
                <span class="small">Code - OSS web build${version ? " (@github1s/vscode-web " + esc(version) + ")" : ""}</span>
            </span>
            <span class="buttons">
                <a class="btn" href="${REPO}" target="_blank" rel="noopener noreferrer">GitHub repo</a>
                <button type="button" class="btn secondary" value="close" autofocus>Close</button>
            </span>
        </footer>`;
}

function run(action) {
    Promise.resolve()
        .then(action)
        .catch((e) => console.error("LO Playground:", e));
}

async function copy(button) {
    const flash = (text) => {
        button.textContent = text;
        clearTimeout(button._timer);
        button._timer = setTimeout(() => (button.textContent = "Copy"), 1500);
    };
    try {
        await navigator.clipboard.writeText(button.dataset.copy);
        flash("Copied");
    } catch {
        // no clipboard access: select the code, so the usual shortcut copies it
        getSelection().selectAllChildren(
            button.parentElement.querySelector("pre"),
        );
        flash(`${MOD}+C`);
    }
}

export async function showAbout() {
    const version = await getVscodeWebVersion();
    if (document.querySelector("dialog.lo-about")) return;
    storage.set(SEEN_KEY, "1");
    stopHint?.();

    const host = document.querySelector(".monaco-workbench") ?? document.body; // theme variables live on the workbench
    const dialog = Object.assign(document.createElement("dialog"), {
        className: "lo-about",
    });
    dialog.setAttribute("aria-label", "LO Playground");
    dialog.innerHTML = template(version);

    const tabs = [...dialog.querySelectorAll("[role=tab]")];
    const select = (tab, focus = false) => {
        for (const t of tabs) {
            const on = t === tab;
            t.setAttribute("aria-selected", String(on));
            t.tabIndex = on ? 0 : -1;
            dialog.querySelector(`#${t.getAttribute("aria-controls")}`).hidden =
                !on;
        }
        if (focus) tab.focus();
        storage.set(TAB_KEY, tab.dataset.tab);
    };
    select(tabs.find((t) => t.dataset.tab === storage.get(TAB_KEY)) ?? tabs[0]);

    dialog.addEventListener("keydown", (e) => {
        e.stopPropagation(); // keep the workbench keybindings out of the dialog
        const i = tabs.indexOf(document.activeElement);
        const next =
            i < 0
                ? undefined
                : {
                      ArrowRight: i + 1,
                      ArrowLeft: i - 1,
                      Home: 0,
                      End: tabs.length - 1,
                  }[e.key];
        if (next === undefined) return;
        e.preventDefault();
        select(tabs[(next + tabs.length) % tabs.length], true);
    });
    // The workbench cancels every wheel event under it (against the macOS swipe navigation, see BrowserWindow in
    // vs/workbench/browser/window.ts), which also keeps the body from scrolling. Wheel events in the body skip that
    // listener, its `overscroll-behavior: contain` keeps the swipe navigation off. Ctrl+wheel (zoom) still goes to VS Code.
    const body = dialog.querySelector(".body");
    dialog.addEventListener(
        "wheel",
        (e) => {
            if (!e.ctrlKey && body.contains(e.target)) e.stopPropagation();
        },
        { passive: true },
    );
    dialog.addEventListener("click", (e) => {
        if (e.target === dialog) return dialog.close(); // click on the backdrop
        const el = e.target.closest("button");
        if (!el) return;
        if (el.dataset.tab) return select(el);
        if (el.dataset.copy !== undefined) return copy(el);
        if (el.dataset.open) {
            dialog.close();
            const options = {
                find: el.dataset.find,
                selectAll: el.dataset.selectAll !== undefined,
            };
            return run(() =>
                commands.executeCommand(
                    "scratch.open",
                    el.dataset.open,
                    options,
                ),
            );
        }
        if (el.dataset.act === "terminal") {
            dialog.close();
            // creates a shell if there is none (lo-shell.new shows it), then shows the active one
            return run(async () => {
                await commands.executeCommand("lo-shell.ensure");
                await commands.executeCommand("lo-shell.focus");
            });
        }
        if (el.value === "close") dialog.close();
    });
    dialog.addEventListener("close", () => dialog.remove());
    host.append(dialog);
    dialog.showModal();
}

// Until the dialog was opened once, rings ripple out of the indicator, a glow around it grows and shrinks and the indicator
// itself pulses to a lighter color. The status bar clips its items to its 22px, so the rings and the glow are on their own
// layer, which is kept over the indicator. With reduced motion only the color pulse and a steady glow are left.
const INDICATOR = ".monaco-workbench .part.statusbar > .items-container > .statusbar-item.remote-kind";
const HINT_CSS = `
.lo-hint { position: fixed; z-index: 100; pointer-events: none;
    --lo-hint-color: var(--vscode-statusBarItem-remoteBackground, #feb87c);
    animation: lo-hint-glow .8s ease-in-out infinite alternate; }
.lo-hint::before, .lo-hint::after { content: ""; position: absolute; inset: 0; border: 2px solid var(--lo-hint-color);
    opacity: 0; animation: lo-hint-ripple 1.6s cubic-bezier(.25, .6, .45, 1) infinite; }
.lo-hint::after { animation-delay: .8s; }
@keyframes lo-hint-ripple {
    0% { inset: -1px; opacity: 1; }
    100% { inset: -30px; opacity: 0; }
}
@keyframes lo-hint-glow {
    from { box-shadow: 0 0 4px 0 color-mix(in srgb, var(--lo-hint-color) 35%, transparent); }
    to { box-shadow: 0 0 18px 5px color-mix(in srgb, var(--lo-hint-color) 75%, transparent); }
}
html[data-lo-hint] ${INDICATOR} { animation: lo-hint-pulse .8s ease-in-out infinite alternate; }
html[data-lo-hint] ${INDICATOR}:hover { animation: none; } /* the hover color, also on the 2px the label leaves uncovered */
@keyframes lo-hint-pulse { to { background-color: color-mix(in srgb, var(--vscode-statusBarItem-remoteBackground) 30%, white); } }
@media (prefers-reduced-motion: reduce) {
    .lo-hint, .lo-hint::before, .lo-hint::after { animation: none; }
    .lo-hint { box-shadow: 0 0 14px 4px color-mix(in srgb, var(--lo-hint-color) 60%, transparent); }
    html[data-lo-hint] ${INDICATOR} { animation-duration: 1.6s; }
}`;
let stopHint;

export function hintUntilOpened() {
    if (storage.get(SEEN_KEY) || stopHint) return;
    const style = Object.assign(document.createElement("style"), { textContent: HINT_CSS });
    const layer = Object.assign(document.createElement("div"), { className: "lo-hint" });
    // The indicator is rendered later and can move (window size, zoom) or go away (status bar hidden).
    const place = () => {
        const r = document.querySelector(INDICATOR)?.getBoundingClientRect();
        layer.hidden = !r?.width || !r?.height;
        if (layer.hidden) return;
        if (!layer.isConnected) document.querySelector(".monaco-workbench").append(layer); // theme variables live there
        Object.assign(layer.style, { left: `${r.left}px`, top: `${r.top}px`, width: `${r.width}px`, height: `${r.height}px` });
    };
    document.head.append(style);
    document.documentElement.dataset.loHint = "";
    const timer = setInterval(place, 500);
    addEventListener("resize", place);
    place();
    stopHint = () => {
        clearInterval(timer);
        removeEventListener("resize", place);
        layer.remove();
        style.remove();
        delete document.documentElement.dataset.loHint;
        stopHint = undefined;
    };
}
