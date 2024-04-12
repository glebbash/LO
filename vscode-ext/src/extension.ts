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
            const useNodeWasi = config.get<boolean>("useNodeWasi") ?? false;
            let compilerPath =
                config.get<string | undefined>("compilerPath") ??
                "${workspaceFolder}/target/wasm32-unknown-unknown/release/lo.wasm";

            compilerPath = compilerPath.replaceAll(
                "${workspaceFolder}",
                workspaceUri.fsPath
            );

            let compilerModule: WebAssembly.Module;
            try {
                compilerModule = await WebAssembly.compile(
                    await vscode.workspace.fs.readFile(
                        vscode.Uri.file(compilerPath)
                    )
                );
            } catch (err) {
                return vscode.window.showErrorMessage(
                    "Compiler loading error",
                    { modal: true, detail: "" + err }
                );
            }

            const currentFile = vscode.window.activeTextEditor?.document;
            if (currentFile === undefined) {
                return vscode.window.showErrorMessage("No files opened");
            }

            const compilerResult = await runProgram({
                useNodeWasi,
                processName: "compiler.wasm",
                cwdUri: workspaceUri,
                args: ["./" + vscode.workspace.asRelativePath(currentFile.uri)],
                module: compilerModule,
            });
            if (compilerResult.exitCode !== 0) {
                return vscode.window.showErrorMessage(
                    `Compiler errored (exit code: ${compilerResult.exitCode})`,
                    {
                        modal: true,
                        detail: new TextDecoder().decode(compilerResult.stderr),
                    }
                );
            }

            let programModule: WebAssembly.Module;
            try {
                programModule = await WebAssembly.compile(
                    compilerResult.stdout
                );
            } catch (err) {
                return vscode.window.showErrorMessage("Program loading error", {
                    modal: true,
                    detail: "" + err,
                });
            }
            const programResult = await runProgram({
                useNodeWasi,
                processName: currentFile.uri.fsPath,
                cwdUri: workspaceUri,
                args: [],
                module: programModule,
            });
            if (programResult.exitCode !== 0) {
                return vscode.window.showErrorMessage(
                    `Program errored (exit code: ${programResult.exitCode})`,
                    {
                        modal: true,
                        detail: new TextDecoder().decode(programResult.stderr),
                    }
                );
            }

            return vscode.window.showInformationMessage("Program output", {
                modal: true,
                detail: new TextDecoder().decode(programResult.stdout),
            });
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

type ProgramOptions = {
    processName: string;
    args: string[];
    cwdUri: vscode.Uri;
    module: WebAssembly.Module;
};

type ProgramResult = {
    exitCode: number;
    stdout: Uint8Array;
    stderr: Uint8Array;
};

async function runProgram(
    options: ProgramOptions & { useNodeWasi: boolean }
): Promise<ProgramResult> {
    return options.useNodeWasi
        ? runProgramNode(options)
        : runProgramCode(options);
}

async function runProgramNode(options: ProgramOptions): Promise<ProgramResult> {
    const tmpDir = options.cwdUri.fsPath + "/tmp";
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
            args: [options.processName, ...options.args],
            preopens: { ".": options.cwdUri.fsPath },
            returnOnExit: true,
        });

        const instance = await WebAssembly.instantiate(options.module, {
            wasi_snapshot_preview1: wasi.wasiImport,
        });
        const exitCode = wasi.start(instance);
        return {
            exitCode,
            stdout: await fs.readFile(stdoutFile),
            stderr: await fs.readFile(stderrFile),
        };
    } finally {
        await stdoutHandle?.close();
        await fs.unlink(stdoutFile);

        await stderrHandle?.close();
        await fs.unlink(stderrFile);
    }
}

async function runProgramCode(options: ProgramOptions): Promise<ProgramResult> {
    const wasm = await Wasm.api();
    const stdin = wasm.createWritable();
    const [stdout, stdoutBytes] = accumulateBytes(wasm.createReadable());
    const [stderr, stderrBytes] = accumulateBytes(wasm.createReadable());

    const process = await wasm.createProcess(
        options.processName,
        options.module,
        {
            stdio: {
                in: { kind: "pipeIn", pipe: stdin },
                out: { kind: "pipeOut", pipe: stdout },
                err: { kind: "pipeOut", pipe: stderr },
            },
            args: options.args,
            mountPoints: [
                {
                    kind: "vscodeFileSystem",
                    uri: options.cwdUri,
                    mountPoint: "/",
                },
            ],
        }
    );
    const exitCode = await process.run();
    return {
        exitCode,
        stdout: stdoutBytes.get(),
        stderr: stderrBytes.get(),
    };
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
