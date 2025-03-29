import * as vscode from "vscode";
import * as wasi from "./run-wasi";
import { FileAnalysis, FileAnalysisCollection } from "./analysis";
import { Wasm, type RootFileSystem, type Stdio } from "@vscode/wasm-wasi/v1";

// WebShell 0.13.0-pre.1 interface
export type WebShellCommandHandler = (
    command: string,
    args: string[],
    cwd: string,
    stdio: Stdio,
    rootFileSystem: RootFileSystem
) => Promise<number>;

type DiagnisticItem =
    | { type: "file"; index: number; path: string }
    | { type: "info"; loc: string; link?: string; hover?: string }
    | { type: "end" };

export async function activate(context: vscode.ExtensionContext) {
    const logChannel = vscode.window.createOutputChannel("LO extension");
    const workspaceUri = vscode.workspace.workspaceFolders?.[0]?.uri!;
    const analysis = new FileAnalysisCollection("lo");
    analysis.registerProviders(context);

    const parseRange = (raw: string) => {
        const [startPos, endPos] = raw.split("-");
        const [startLine, startCol] = startPos.split(":").map(Number);
        const [endLine, endCol] = endPos.split(":").map(Number);

        return new vscode.Range(
            new vscode.Position(startLine - 1, startCol - 1),
            new vscode.Position(endLine - 1, endCol - 1)
        );
    };

    const inspectDocument = async (document: vscode.TextDocument) => {
        const ctx = await loadCompilerCtx();
        if (!ctx) {
            return;
        }

        const inspectLatency = new Latency("Inspect " + document.fileName);
        const compilerResult = await wasi.runWasiProgram({
            processName: "lo",
            cwdUri: workspaceUri,
            args: ["inspect", vscode.workspace.asRelativePath(document.uri)],
            module: ctx.compilerModule,
        });
        inspectLatency.measureAndLog(logChannel);

        analysis.clear();
        if (compilerResult.exitCode !== 0) {
            return showCompilerError(
                workspaceUri,
                analysis,
                new TextDecoder().decode(compilerResult.stderr),
                compilerResult.exitCode
            );
        }

        const diagnostics: DiagnisticItem[] = JSON.parse(
            new TextDecoder().decode(compilerResult.stdout)
        );
        const analysisPerIndex = new Map<number, FileAnalysis>();
        for (const d of diagnostics) {
            if (d.type === "file") {
                const uri = vscode.Uri.joinPath(workspaceUri, d.path);
                const diag = { uri, hovers: [], links: [] };
                analysis.push(diag);
                analysisPerIndex.set(d.index, diag);
            }

            if (d.type === "info") {
                const sourceIndex = Number(d.loc.split("/")[0]);
                const sourceRange = parseRange(d.loc.split("/")[1]);
                const fileDiagnostic = analysisPerIndex.get(sourceIndex)!;

                if (d.link) {
                    const targetIndex = Number(d.link.split("/")[0]);
                    const targetRange = parseRange(d.link.split("/")[1]);

                    fileDiagnostic.links.push({
                        originSelectionRange: sourceRange,
                        targetUri: analysisPerIndex.get(targetIndex)!.uri,
                        targetRange: targetRange,
                    });
                }

                if (d.hover) {
                    fileDiagnostic.hovers.push(
                        new vscode.Hover(
                            new vscode.MarkdownString().appendCodeblock(
                                d.hover,
                                "lo"
                            ),
                            sourceRange
                        )
                    );
                }
            }
        }
    };

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.initProject", async () => {
            const initDir = vscode.Uri.joinPath(
                context.extensionUri,
                "assets/initial-project"
            );

            const initFiles = ["lo.wasm", "main.lo", "lib"];

            for (const file of initFiles) {
                await vscode.workspace.fs.copy(
                    vscode.Uri.joinPath(initDir, file),
                    vscode.Uri.joinPath(workspaceUri, file)
                );
            }

            await vscode.window.showInformationMessage("Project initialized");
        })
    );

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

            const compileLatency = new Latency(
                "Compile " + currentFile.fileName
            );
            const compilerResult = await wasi.runWasiProgram({
                processName: "lo",
                cwdUri: workspaceUri,
                args: [
                    "compile",
                    vscode.workspace.asRelativePath(currentFile.uri),
                ],
                module: ctx.compilerModule,
            });
            compileLatency.measureAndLog(logChannel);

            analysis.clear();
            if (compilerResult.exitCode !== 0) {
                return showCompilerError(
                    workspaceUri,
                    analysis,
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
            const programResult = await wasi.runWasiProgram({
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
        vscode.commands.registerCommand("lo.webshell.clear", (async (
            _command,
            _args,
            _cwd,
            stdio,
            _rootFileSystem
        ) => {
            if (stdio.out?.kind === "terminal") {
                await stdio.out.terminal.write(
                    new TextEncoder().encode("\x1b[2J\x1b[H")
                );
            }
            return 0;
        }) satisfies WebShellCommandHandler)
    );

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.webshell.run", (async (
            command,
            args,
            cwd,
            stdio,
            _rootFileSystem
        ) => {
            const wasm = await Wasm.load();

            const programPath = args[0];
            args = args.slice(1);
            if (programPath === undefined) {
                await logError("Missing program name");
                return 1;
            }

            const processStdio = { ...stdio };

            let stdoutRedirect:
                | {
                      fileName: string;
                      pipe: ReturnType<typeof wasi.accumulateBytes>;
                  }
                | undefined;

            const redirectIndex = args.indexOf(">");
            if (redirectIndex !== -1) {
                const [_, redirectTo] = args.slice(redirectIndex);
                args = args.slice(0, redirectIndex);

                if (redirectTo !== undefined) {
                    stdoutRedirect = {
                        fileName: redirectTo,
                        pipe: wasi.accumulateBytes(wasm.createReadable()),
                    };
                    processStdio.out = {
                        kind: "pipeOut",
                        pipe: stdoutRedirect.pipe,
                    };
                }
            }

            let programModule: WebAssembly.Module;
            try {
                const programBytes = await vscode.workspace.fs.readFile(
                    vscode.Uri.joinPath(
                        workspaceUri,
                        cwd.slice("/workspace".length),
                        programPath
                    )
                );
                programModule = await WebAssembly.compile(programBytes);
            } catch (err) {
                await logError(`Error loading ${programPath}: ${err}`);
                return 1;
            }

            try {
                const process = await wasm.createProcess(
                    command,
                    programModule,
                    {
                        stdio: processStdio,
                        args,
                        mountPoints: [
                            {
                                kind: "vscodeFileSystem",
                                uri: workspaceUri,
                                mountPoint: "/",
                            },
                        ],
                    }
                );
                return await process.run();
            } catch (err) {
                await logError(`Error running ${programPath}: ${err}`);
                return 1;
            } finally {
                if (stdoutRedirect !== undefined) {
                    try {
                        await vscode.workspace.fs.writeFile(
                            vscode.Uri.joinPath(
                                workspaceUri,
                                stdoutRedirect.fileName
                            ),
                            stdoutRedirect.pipe.get()
                        );
                    } catch (err) {
                        console.error(
                            `error redirecting stdout to ${stdoutRedirect.fileName}`,
                            err
                        );
                    }
                }
            }

            async function logError(errorMessage: string) {
                if (stdio.err?.kind === "terminal") {
                    await stdio.err.terminal.write(errorMessage + "\n");
                }
            }
        }) satisfies WebShellCommandHandler)
    );

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.formatFile", async () => {
            const currentFile = vscode.window.activeTextEditor?.document;
            if (currentFile === undefined) {
                return vscode.window.showErrorMessage("No files opened");
            }

            const formatLatency = new Latency("Format " + currentFile.fileName);
            const edits = await formatFile(workspaceUri, currentFile, analysis);
            formatLatency.measureAndLog(logChannel);

            const workspaceEdit = new vscode.WorkspaceEdit();
            workspaceEdit.set(currentFile.uri, edits);
            await vscode.workspace.applyEdit(workspaceEdit);
        })
    );

    const config = vscode.workspace.getConfiguration("lo");
    if (config.get<boolean>("enableFormatting") ?? true) {
        context.subscriptions.push(
            vscode.languages.registerDocumentFormattingEditProvider("lo", {
                async provideDocumentFormattingEdits(currentFile) {
                    const formatLatency = new Latency(
                        "Format " + currentFile.fileName
                    );
                    const edits = await formatFile(
                        workspaceUri,
                        currentFile,
                        analysis
                    );
                    formatLatency.measureAndLog(logChannel);
                    return edits;
                },
            })
        );
    }

    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(async (doc) => {
            if (doc.languageId === "lo") {
                await inspectDocument(doc);
            }
        })
    );
}

async function formatFile(
    workspaceUri: vscode.Uri,
    currentFile: vscode.TextDocument,
    analysis: FileAnalysisCollection
) {
    const ctx = await loadCompilerCtx();
    if (!ctx) {
        return [];
    }

    const tmpFileName = crypto.randomUUID() + ".lo";

    const compilerResult = await wasi.runWasiProgram({
        processName: "lo",
        cwdUri: workspaceUri,
        args: ["format", tmpFileName],
        module: ctx.compilerModule,
        memoryFs: {
            [tmpFileName]: new TextEncoder().encode(currentFile.getText()),
        },
    });

    analysis.clear();
    if (compilerResult.exitCode !== 0) {
        const realFileName = vscode.workspace.asRelativePath(currentFile.uri);
        const errorMessage = new TextDecoder()
            .decode(compilerResult.stderr)
            .replaceAll(tmpFileName, realFileName);

        await showCompilerError(
            workspaceUri,
            analysis,
            errorMessage,
            compilerResult.exitCode
        );
        return [];
    }

    const formattedFile = new TextDecoder().decode(compilerResult.stdout);
    return [
        new vscode.TextEdit(
            new vscode.Range(
                new vscode.Position(0, 0),
                currentFile.lineAt(currentFile.lineCount - 1).range.end
            ),
            formattedFile
        ),
    ];
}

async function loadCompilerCtx() {
    const workspaceUri = vscode.workspace.workspaceFolders?.[0]?.uri!;

    const config = vscode.workspace.getConfiguration("lo");
    const compilerPath = config.get<string>("compilerPath")!;

    let compilerModule: WebAssembly.Module;
    try {
        compilerModule = await WebAssembly.compile(
            await vscode.workspace.fs.readFile(
                vscode.Uri.joinPath(workspaceUri, compilerPath)
            )
        );
    } catch (err) {
        vscode.window.showErrorMessage("Compiler loading error", {
            modal: true,
            detail: "" + err,
        });
        return undefined;
    }

    return { workspaceUri, compilerModule };
}

async function showCompilerError(
    workspaceUri: vscode.Uri,
    analysis: FileAnalysisCollection,
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

    analysis.diagnosticCollection.set(fileUri, [
        ...(analysis.diagnosticCollection.get(fileUri) ?? []),
        diagnostic,
    ]);
}

class Latency {
    private startTime = performance.now();

    constructor(private label: string) {}

    measureAndLog(logChannel: vscode.OutputChannel) {
        const duration = performance.now() - this.startTime;
        logChannel.appendLine(`${this.label} OK in ${duration.toFixed(2)}ms`);
    }
}
