import * as vscode from "vscode";
import fs from "node:fs/promises";
import crypto from "node:crypto";
import { Wasm, Readable } from "@vscode/wasm-wasi";

type DiagnisticItem =
    | { type: "file"; index: number; path: string }
    | { type: "hover"; source: number; range: string; content: string }
    | {
          type: "link";
          source: number;
          sourceRange: string;
          target: number;
          targetRange: string;
      }
    | { type: "end" };

type FileAnalysis = {
    uri: vscode.Uri;
    hovers: vscode.Hover[];
    links: vscode.LocationLink[];
};

export async function activate(context: vscode.ExtensionContext) {
    const workspaceUri = vscode.workspace.workspaceFolders?.[0]?.uri!;
    const diagnosticCollection =
        vscode.languages.createDiagnosticCollection("lo");

    const analysisPerIndex = new Map<number, FileAnalysis>();
    const analysisPerUri = new Map<string, FileAnalysis>();

    const parseRange = (raw: string) => {
        const [startPos, endPos] = raw.split("-");
        const [startLine, startCol] = startPos.split(":").map(Number);
        const [endLine, endCol] = endPos.split(":").map(Number);

        return new vscode.Range(
            new vscode.Position(startLine - 1, startCol - 1),
            new vscode.Position(endLine - 1, endCol - 1)
        );
    };

    const processDocument = async (document: vscode.TextDocument) => {
        const ctx = await loadCompilerCtx();
        if (!ctx) {
            return;
        }

        const compilerResult = await runProgram({
            useNodeWasi: ctx.useNodeWasi,
            processName: "lo",
            cwdUri: workspaceUri,
            args: [vscode.workspace.asRelativePath(document.uri), "--inspect"],
            module: ctx.compilerModule,
        });
        analysisPerUri.clear();
        analysisPerIndex.clear();
        diagnosticCollection.clear();
        if (compilerResult.exitCode !== 0) {
            return showCompilerError(
                workspaceUri,
                diagnosticCollection,
                new TextDecoder().decode(compilerResult.stderr),
                compilerResult.exitCode
            );
        }

        const diagnostics: DiagnisticItem[] = JSON.parse(
            new TextDecoder().decode(compilerResult.stdout)
        );

        for (const d of diagnostics) {
            if (d.type === "file") {
                const uri = vscode.Uri.joinPath(workspaceUri, d.path);
                const diag = { uri, hovers: [], links: [] };
                analysisPerIndex.set(d.index, diag);
                analysisPerUri.set(uri.toString(true), diag);
            }

            if (d.type === "hover") {
                const fileDiagnostic = analysisPerIndex.get(d.source)!;
                fileDiagnostic.hovers.push(
                    new vscode.Hover(
                        new vscode.MarkdownString().appendCodeblock(
                            d.content,
                            "lo"
                        ),
                        parseRange(d.range)
                    )
                );
            }

            if (d.type === "link") {
                const fileDiagnostic = analysisPerIndex.get(d.source)!;
                fileDiagnostic.links.push({
                    originSelectionRange: parseRange(d.sourceRange),
                    targetUri: analysisPerIndex.get(d.target)!.uri,
                    targetRange: parseRange(d.targetRange),
                });
            }
        }
    };

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.runFile", async () => {
            const ctx = await loadCompilerCtx();
            if (!ctx) {
                return;
            }

            const currentFile = vscode.window.activeTextEditor?.document;
            if (currentFile === undefined) {
                return vscode.window.showErrorMessage("No files opened");
            }

            const compilerResult = await runProgram({
                useNodeWasi: ctx.useNodeWasi,
                processName: "lo",
                cwdUri: workspaceUri,
                args: [vscode.workspace.asRelativePath(currentFile.uri)],
                module: ctx.compilerModule,
            });
            diagnosticCollection.clear();
            if (compilerResult.exitCode !== 0) {
                return showCompilerError(
                    workspaceUri,
                    diagnosticCollection,
                    new TextDecoder().decode(compilerResult.stderr),
                    compilerResult.exitCode
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
                useNodeWasi: ctx.useNodeWasi,
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
        vscode.workspace.onDidSaveTextDocument(async (doc) => {
            if (doc.languageId === "lo") {
                await processDocument(doc);
            }
        })
    );

    context.subscriptions.push(
        vscode.languages.registerDefinitionProvider("lo", {
            provideDefinition(document, position, _token) {
                const links =
                    analysisPerUri.get(document.uri.toString(true))?.links ??
                    [];
                for (const ref of links) {
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
                const hovers =
                    analysisPerUri.get(document.uri.toString(true))?.hovers ??
                    [];
                for (const hover of hovers) {
                    if (hover.range!.contains(position)) {
                        return hover;
                    }
                }

                return null;
            },
        })
    );
}

async function loadCompilerCtx() {
    const workspaceUri = vscode.workspace.workspaceFolders?.[0]?.uri!;

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
            await vscode.workspace.fs.readFile(vscode.Uri.file(compilerPath))
        );
    } catch (err) {
        vscode.window.showErrorMessage("Compiler loading error", {
            modal: true,
            detail: "" + err,
        });
        return undefined;
    }

    return { workspaceUri, useNodeWasi, compilerModule };
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
    const { WASI } = await import("node:wasi");

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

async function showCompilerError(
    workspaceUri: vscode.Uri,
    diagnosticCollection: vscode.DiagnosticCollection,
    errorMessage: string,
    exitCode: number
) {
    const match = errorMessage.match(/^(.+):(\d+):(\d+) - (.+)\n$/);
    if (match === null) {
        return vscode.window.showErrorMessage(
            `Compiler errored (exit code: ${exitCode})`,
            { modal: true, detail: errorMessage }
        );
    }

    const filePath = match[1];
    const lineNumber = Number(match[2]);
    const columnNumber = Number(match[3]);
    const message = match[4];

    const fileUri = vscode.Uri.joinPath(workspaceUri, filePath);

    const range = new vscode.Range(
        new vscode.Position(lineNumber - 1, columnNumber - 1),
        new vscode.Position(lineNumber - 1, columnNumber - 1)
    );

    const diagnostic = new vscode.Diagnostic(
        range,
        message,
        vscode.DiagnosticSeverity.Error
    );

    diagnosticCollection.set(fileUri, [
        ...(diagnosticCollection.get(fileUri) ?? []),
        diagnostic,
    ]);
}
