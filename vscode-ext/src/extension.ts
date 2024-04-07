import * as vscode from "vscode";
import { Wasm } from "@vscode/wasm-wasi";

export async function activate(context: vscode.ExtensionContext) {
    const workspaceUri = vscode.workspace.workspaceFolders?.[0]?.uri!;
    const hovers = new Map<vscode.Uri, vscode.Hover[]>();
    const links = new Map<vscode.Uri, vscode.LocationLink[]>();

    const wasm: Wasm = await Wasm.api();
    let compilerModule: WebAssembly.Module;

    const processDocument = (document: vscode.TextDocument) => {
        const fileUri = document.uri!;

        hovers.set(fileUri, [
            new vscode.Hover(
                new vscode.MarkdownString().appendCodeblock(
                    "fn puts(value: str)",
                    "lo"
                ),
                new vscode.Range(
                    new vscode.Position(3, 4),
                    new vscode.Position(3, 8)
                )
            ),
        ]);

        links.set(fileUri, [
            {
                originSelectionRange: new vscode.Range(
                    new vscode.Position(0, 9),
                    new vscode.Position(0, 21)
                ),
                targetUri: vscode.Uri.joinPath(
                    workspaceUri,
                    "./examples/lib/cli.lo"
                ),
                targetRange: new vscode.Range(
                    new vscode.Position(0, 0),
                    new vscode.Position(0, 0)
                ),
            },
            {
                originSelectionRange: new vscode.Range(
                    new vscode.Position(3, 4),
                    new vscode.Position(3, 8)
                ),
                targetUri: vscode.Uri.joinPath(
                    workspaceUri,
                    "./examples/lib/print.lo"
                ),
                targetRange: new vscode.Range(
                    new vscode.Position(3, 3),
                    new vscode.Position(3, 7)
                ),
            },
        ]);
    };

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.loadCompiler", async () => {
            try {
                const compilerUri = await vscode.window.showOpenDialog({
                    defaultUri: workspaceUri,
                    openLabel: "Load",
                });

                if (compilerUri?.[0] === undefined) {
                    return;
                }

                compilerModule = WebAssembly.compile(
                    await vscode.workspace.fs.readFile(compilerUri[0])
                );

                vscode.window.showInformationMessage("Compiler loaded");
            } catch (error) {
                vscode.window.showErrorMessage(
                    `Error loading compiler: ${error}`
                );
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.runFile", async () => {
            let currentCompilerModule = compilerModule;
            if (currentCompilerModule === undefined) {
                // TODO: use this
                // return vscode.window.showErrorMessage("Compiler not loaded");

                currentCompilerModule = WebAssembly.compile(
                    await vscode.workspace.fs.readFile(
                        vscode.Uri.joinPath(
                            workspaceUri,
                            "target/wasm32-unknown-unknown/release/lo.wasm"
                        )
                    )
                );

                vscode.window.showWarningMessage("Using dev compiler");
            }

            const currentFile = vscode.window.activeTextEditor?.document;
            if (currentFile === undefined) {
                return vscode.window.showErrorMessage("No files opened");
            }

            const pty = wasm.createPseudoterminal();
            const terminal = vscode.window.createTerminal({
                name: "LO Compiler",
                pty,
                isTransient: true,
            });
            terminal.show(true);

            try {
                const process = await wasm.createProcess(
                    "compiler.wasm",
                    currentCompilerModule,
                    {
                        stdio: pty.stdio,
                        args: [
                            "./" +
                                vscode.workspace.asRelativePath(
                                    currentFile!.uri
                                ),
                        ],
                        mountPoints: [
                            // TODO: figure out why reading files throws BADF (error 8)
                            {
                                kind: "vscodeFileSystem",
                                uri: workspaceUri,
                                mountPoint: "/",
                            },
                        ],
                    }
                );

                const exitCode = await process.run();
                if (exitCode !== 0) {
                    vscode.window.showErrorMessage(
                        `Process exited with code: ${exitCode}`
                    );
                }
            } catch (error) {
                vscode.window.showErrorMessage((error as Error).message);
                terminal.dispose();
            }
        })
    );

    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument((doc) => {
            if (doc.languageId === "lo") {
                processDocument(doc);
            }
        })
    );

    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument((doc) => {
            if (doc.languageId === "lo") {
                processDocument(doc);
            }
        })
    );

    context.subscriptions.push(
        vscode.languages.registerDefinitionProvider("lo", {
            provideDefinition(document, position, _token) {
                const documentRefs = links.get(document.uri);
                if (!documentRefs) {
                    return null;
                }

                for (const ref of documentRefs) {
                    if (ref.originSelectionRange!.contains(position)) {
                        return [ref];
                    }
                }

                return null;
            },
        })
    );

    context.subscriptions.push(
        vscode.languages.registerHoverProvider("lo", {
            provideHover(document, position, _token) {
                const documentHovers = hovers.get(document.uri);
                if (!documentHovers) {
                    return null;
                }

                for (const hover of documentHovers) {
                    if (hover.range!.contains(position)) {
                        return hover;
                    }
                }

                return null;
            },
        })
    );

    console.log("LO extension activated");
}

export function deactivate() {}
