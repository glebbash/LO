import * as vscode from "vscode";
import { WASI } from "node:wasi";
import fs from "node:fs/promises";
import crypto from "node:crypto";
import { Wasm, Readable } from "@vscode/wasm-wasi";

export async function activate(context: vscode.ExtensionContext) {
    const workspaceUri = vscode.workspace.workspaceFolders?.[0]?.uri!;

    const hovers = new Map<vscode.Uri, vscode.Hover[]>();
    const links = new Map<vscode.Uri, vscode.LocationLink[]>();

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
        vscode.commands.registerCommand("lo.runFile", async () => {
            const config = vscode.workspace.getConfiguration("lo");
            const useNodeWasi = config.get<boolean>("useNodeWasi");
            let compilerPath =
                config.get<string | undefined>("compilerPath") ??
                "${workspaceFolder}/target/wasm32-unknown-unknown/release/lo.wasm";

            compilerPath = compilerPath.replaceAll(
                "${workspaceFolder}",
                workspaceUri.fsPath
            );

            const compiler = await WebAssembly.compile(
                await vscode.workspace.fs.readFile(
                    vscode.Uri.file(compilerPath)
                )
            );

            const currentFile = vscode.window.activeTextEditor?.document;
            if (currentFile === undefined) {
                return vscode.window.showErrorMessage("No files opened");
            }

            if (useNodeWasi) {
                await runCompilerNode(workspaceUri, currentFile.uri, compiler);
            } else {
                await runCompilerCode(workspaceUri, currentFile.uri, compiler);
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

async function runCompilerNode(
    workspaceUri: vscode.Uri,
    sourceUri: vscode.Uri,
    compilerModule: WebAssembly.Module
) {
    const tmpDir = workspaceUri.fsPath + "/tmp";
    const stdoutFile = `${tmpDir}/${crypto.randomUUID()}.tmp`;
    const stderrFile = `${tmpDir}/${crypto.randomUUID()}.tmp`;
    let stdoutHandle: fs.FileHandle | undefined;
    let stderrHandle: fs.FileHandle | undefined;

    try {
        stdoutHandle = await fs.open(stdoutFile, "w+");
        stderrHandle = await fs.open(stderrFile, "w+");

        const wasi = new WASI({
            stdout: stdoutHandle.fd,
            stderr: stderrHandle.fd,
            args: [
                "compiler.wasm",
                "./" + vscode.workspace.asRelativePath(sourceUri),
            ],
            preopens: { ".": workspaceUri.fsPath },
            returnOnExit: true,
        });

        const instance = await WebAssembly.instantiate(compilerModule, {
            wasi_snapshot_preview1: {
                ...wasi.wasiImport,
                path_open: (...args: any[]) => {
                    console.log({ path_open: { args: fixBigNums(args) } });
                    return wasi.wasiImport.path_open(...args);
                },
            },
        });
        const exitCode = wasi.start(instance);
        if (exitCode !== 0) {
            const output = await fs.readFile(stderrFile, "utf-8");
            return vscode.window.showErrorMessage(
                `exit_code=${exitCode}\n` + output
            );
        }

        const output = await fs.readFile(stdoutFile, "utf-8");
        return vscode.window.showInformationMessage(output);
    } catch (error) {
        return vscode.window.showErrorMessage((error as Error).message);
    } finally {
        await stdoutHandle?.close();
        await fs.unlink(stdoutFile);

        await stderrHandle?.close();
        await fs.unlink(stderrFile);
    }
}

async function runCompilerCode(
    workspaceUri: vscode.Uri,
    sourceUri: vscode.Uri,
    compilerModule: WebAssembly.Module
) {
    const wasm = await Wasm.api();
    const stdin = wasm.createWritable();
    const [stdout, stdoutBytes] = accumulateBytes(wasm.createReadable());
    const [stderr, stderrBytes] = accumulateBytes(wasm.createReadable());

    try {
        const process = await wasm.createProcess(
            "compiler.wasm",
            compilerModule,
            {
                stdio: {
                    in: { kind: "pipeIn", pipe: stdin },
                    out: { kind: "pipeOut", pipe: stdout },
                    err: { kind: "pipeOut", pipe: stderr },
                },
                args: ["./" + vscode.workspace.asRelativePath(sourceUri)],
                mountPoints: [
                    {
                        kind: "vscodeFileSystem",
                        uri: workspaceUri,
                        mountPoint: "/",
                    },
                ],
                trace: true,
            }
        );
        const exitCode = await process.run();
        if (exitCode !== 0) {
            const output = new TextDecoder().decode(stderrBytes.get());
            return vscode.window.showErrorMessage(
                `exit_code=${exitCode}\n` + output
            );
        }

        const output = new TextDecoder().decode(stdoutBytes.get());
        return vscode.window.showInformationMessage(output);
    } catch (error) {
        return vscode.window.showErrorMessage((error as Error).message);
    }
}

function accumulateBytes(
    readable: Readable
): [Readable, { get: () => Uint8Array }] {
    let data = new Uint8Array();
    readable.onData((chunk) => {
        const newData = new Uint8Array(data.length + chunk.length);
        newData.set(data);
        newData.set(chunk, data.length);
        data = newData;
    });
    return [readable, { get: () => data }];
}

function fixBigNums(x: any) {
    return JSON.parse(
        JSON.stringify(x, (_key, value) =>
            typeof value === "bigint" ? Number(value) : value
        )
    );
}
