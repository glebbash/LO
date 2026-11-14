import { create, commands, URI } from "/vscode/out/vs/workbench/workbench.web.main.internal.js";
import { showAbout } from "/about.js";

const cfg = await (await fetch("/playground-config.json")).json();
const userDefaults = await (await fetch("/settings.json")).json(); // src/settings.json, the exported default settings
const origin = { scheme: location.protocol.slice(0, -1), authority: location.host };

if (!self.crossOriginIsolated) {
    console.warn("page is not cross-origin isolated: wasm-wasi-core (SharedArrayBuffer) will not work. Check COOP/COEP headers.");
}

// By default the extension host / webview iframes load from https://{{uuid}}.vscode-cdn.net, which then fetches our files
// from localhost: Chrome blocks that (Local Network Access). When running locally, serve the iframes from *.localhost instead.
const isLocal = ["localhost", "127.0.0.1"].includes(location.hostname);
const localEndpoints = isLocal ? {
    webEndpointUrlTemplate: `${location.protocol}//{{uuid}}.localhost:${location.port}/vscode`,
    webviewContentExternalBaseUrlTemplate: `${location.protocol}//{{uuid}}.localhost:${location.port}/vscode/out/vs/workbench/contrib/webview/browser/pre/`,
} : {};

// settings.json overrides the hardcoded defaults below; object values (colorCustomizations) are merged one level deep
// so exporting your own color tweaks doesn't drop the orange remote indicator.
function mergeDefaults(hardcoded) {
    const out = { ...hardcoded };
    for (const [key, value] of Object.entries(userDefaults)) {
        const base = out[key];
        out[key] = value && typeof value === "object" && !Array.isArray(value) && base && typeof base === "object" ? { ...base, ...value } : value;
    }
    return out;
}

create(document.body, {
    productConfiguration: { enableTelemetry: false, nameShort: "LO Playground", nameLong: "LO Playground",
        // merged (deep) into product.json: lets scratch-fs register file/text search providers (Quick Open, Search view)
        // The web build ships with Microsoft Marketplace as the gallery, which its terms only allow for Microsoft's own products.
        // Use Open VSX instead (merged deep into product.json, so the leftover Microsoft endpoints are blanked).
        extensionsGallery: {
            serviceUrl: "https://open-vsx.org/vscode/gallery",
            itemUrl: "https://open-vsx.org/vscode/item",
            resourceUrlTemplate: "https://open-vsx.org/vscode/asset/{publisher}/{name}/{version}/Microsoft.VisualStudio.Code.WebResources/{path}",
            publisherUrl: "", controlUrl: "", nlsBaseUrl: "", extensionUrlTemplate: "", servicePublisherUrl: "",
        },
        extensionEnabledApiProposals: { "lo-playground.scratch-fs": ["fileSearchProvider", "textSearchProvider"] },
        ...localEndpoints },
    additionalBuiltinExtensions: cfg.extensions.map((e) => ({ ...origin, path: `/extensions/${e}` })),
    configurationDefaults: mergeDefaults({
        "workbench.colorTheme": cfg.defaultTheme,
        "workbench.startupEditor": "none",
        // default is "...${separator}${rootName}${separator}${appName}", where rootName is "Scratch FS": noise in the tab title
        "window.title": "${dirty}${activeEditorShort}${separator}${appName}",
        // the "remote indicator" (bottom left) is orange like the LO logo (extras/vscode-ext/assets/icons/lo.svg: rgb(99.6%, 72.2%, 48.6%))
        "workbench.colorCustomizations": { "statusBarItem.remoteBackground": "#feb87c", "statusBarItem.remoteForeground": "#000000",
            "statusBarItem.remoteHoverBackground": "#f2a35e", "statusBarItem.remoteHoverForeground": "#000000" },
        "security.workspace.trust.enabled": false,
        "telemetry.telemetryLevel": "off",
        "github.gitAuthentication": false,
        // AI/chat: `chat.disableAIFeatures` alone only hides the Copilot setup UI, so also turn the rest off
        "chat.disableAIFeatures": true,
        "chat.agent.enabled": false,
        "chat.unifiedAgentsBar.enabled": false,
        "chat.viewSessions.enabled": false,
        "chat.restoreLastPanelSession": false,
        "git.autofetch": false,
    }),
    windowIndicator: { label: "LO Playground", tooltip: "About LO Playground", command: "lo-playground.about" },
    commands: [{
        id: "lo-playground.about",
        label: "LO: About LO Playground", // shows up in the command palette too
        handler: () => showAbout(),
    }, {
        // Without a pty host, the terminal "+" button (this command) ends up in a profile picker with a single entry.
        // Host commands are stacked on top of built-in ones, so this opens our shell directly.
        // (Don't override `...terminal.newWithProfile`: VS Code re-registers it on every profile refresh and throws if it exists.)
        id: "workbench.action.terminal.new",
        handler: () => commands.executeCommand("lo-shell.new"),
    }, {
        // lets extensions (running in a worker, which can't trigger downloads) save a file: (name, mime, base64)
        id: "playground.download",
        handler: (name, mime, base64) => {
            const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
            const a = Object.assign(document.createElement("a"), { href: URL.createObjectURL(new Blob([bytes], { type: mime })), download: name });
            a.click();
            setTimeout(() => URL.revokeObjectURL(a.href), 10_000);
        },
    }],
    workspaceProvider: {
        workspace: { folderUri: URI.parse("scratch:/Scratch FS") /* last path segment is the explorer title, see extensions/scratch-fs */ },
        trusted: true,
        async open() { return false; },
    },
});

// If the terminal panel is restored as open from the saved UI state it is blank (no pty host in the web), so create a
// shell in that case only. `.integrated-terminal` is the terminal view body; wait until it is actually visible.
(async () => {
    for (let i = 0; i < 60; i++) {
        await new Promise((r) => setTimeout(r, 250));
        const view = document.querySelector(".integrated-terminal");
        if (view && view.offsetParent !== null && view.clientHeight > 0) {
            try { await commands.executeCommand("lo-shell.ensure"); return; } catch { /* extension host not ready yet, retry */ }
        }
    }
})();
