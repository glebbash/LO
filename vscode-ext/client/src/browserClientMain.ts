import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/browser";

export function activate(context: vscode.ExtensionContext) {
    console.log("LO extension activated!");

    const client = new LanguageClient(
        "lo",
        "LO language server",
        {
            documentSelector: [{ language: "lo" }],
            synchronize: {},
            initializationOptions: {},
        },
        new Worker(
            vscode.Uri.joinPath(
                context.extensionUri,
                "server/dist/browserServerMain.js"
            ).toString(true)
        )
    );
    context.subscriptions.push(client);
    client.start().then(() => {
        console.log("running LO language client");
    });
}
