import { create, commands, URI } from "/vscode/out/vs/workbench/workbench.web.main.internal.js";
import { showAbout, hintUntilOpened } from "/about.js";

const cfg = await (await fetch("/playground-config.json", { cache: "no-cache" })).json();
const profileText = await (await fetch("/playground.code-profile", { cache: "no-cache" })).text();
const origin = { scheme: location.protocol.slice(0, -1), authority: location.host };

if (!self.crossOriginIsolated) {
    console.warn("Page is not cross-origin isolated, wasm-wasi-core will not work. Check the COOP/COEP headers.");
}

create(document.body, {
    productConfiguration: {
        enableTelemetry: false,
        nameShort: "LO Playground",
        nameLong: "LO Playground",
        // Merged deeply into product.json, so the Microsoft endpoints are blanked instead of left in place.
        extensionsGallery: {
            serviceUrl: "https://open-vsx.org/vscode/gallery",
            itemUrl: "https://open-vsx.org/vscode/item",
            resourceUrlTemplate: "https://open-vsx.org/vscode/asset/{publisher}/{name}/{version}/Microsoft.VisualStudio.Code.WebResources/{path}",
            publisherUrl: "",
            controlUrl: "",
            nlsBaseUrl: "",
            extensionUrlTemplate: "",
            servicePublisherUrl: "",
        },
        extensionEnabledApiProposals: {
            "lo-playground.scratch-fs": ["fileSearchProvider", "textSearchProvider"],
        },
    },
    additionalBuiltinExtensions: cfg.extensions.map((e) => ({ ...origin, path: `/extensions/${e}` })),
    configurationDefaults: {
        "workbench.colorTheme": cfg.defaultTheme,
        "workbench.startupEditor": "none",
        "window.title": "${dirty}${activeEditorShort}${separator}${appName}",
        // Same orange as the logo (extras/vscode-ext/assets/icons/lo.svg).
        "workbench.colorCustomizations": {
            "statusBarItem.remoteBackground": "#feb87c",
            "statusBarItem.remoteForeground": "#000000",
            "statusBarItem.remoteHoverBackground": "#f2a35e",
            "statusBarItem.remoteHoverForeground": "#000000",
        },
        "security.workspace.trust.enabled": false,
        "telemetry.telemetryLevel": "off",
        "github.gitAuthentication": false,
        "git.autofetch": false,
        "chat.disableAIFeatures": true,
        "chat.agent.enabled": false,
        "chat.unifiedAgentsBar.enabled": false,
        "chat.viewSessions.enabled": false,
        "chat.restoreLastPanelSession": false,
    },
    profile: { name: "LO Playground", contents: profileText },
    windowIndicator: { label: "LO Playground", tooltip: "About LO Playground", command: "lo-playground.about" },
    commands: [
        {
            id: "lo-playground.about",
            label: "LO: About LO Playground",
            handler: () => showAbout(),
        },
        {
            // Replaces the terminal profile picker with the only profile there is, see README.
            id: "workbench.action.terminal.new",
            handler: () => commands.executeCommand("lo-shell.new"),
        },
        {
            // Called by extensions, a web worker cannot start a download itself.
            id: "playground.download",
            handler: (name, mime, base64) => {
                const bytes = Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));
                const blob = new Blob([bytes], { type: mime });
                const a = Object.assign(document.createElement("a"), { href: URL.createObjectURL(blob), download: name });
                a.click();
                setTimeout(() => URL.revokeObjectURL(a.href), 10_000);
            },
        },
    ],
    webviewEndpoint: `${location.origin}/vscode/out/vs/workbench/contrib/webview/browser/pre`,
    workspaceProvider: {
        // The last path segment is shown as the Explorer title.
        workspace: { folderUri: URI.parse("scratch:/Scratch FS") },
        trusted: true,
        async open() {
            return false;
        },
    },
});

hintUntilOpened();

// A terminal panel restored as open from the saved UI state is empty, so create a shell for it.
(async () => {
    for (let i = 0; i < 60; i++) {
        await new Promise((r) => setTimeout(r, 250));
        const view = document.querySelector(".integrated-terminal");
        if (view && view.offsetParent !== null && view.clientHeight > 0) {
            try {
                await commands.executeCommand("lo-shell.ensure");
                return;
            } catch {
                // the extension host is not ready yet
            }
        }
    }
})();
