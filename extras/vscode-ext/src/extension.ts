import * as vscode from "vscode";
import * as wasi from "./run-wasi";
import { FileAnalysis, FileAnalysisCollection } from "./analysis";
import { Wasm, type RootFileSystem, type Stdio } from "@vscode/wasm-wasi/v1";

// TODO: clear inspections on file close

// WebShell 0.13.0-pre.1 interface
export type WebShellCommandHandler = (
    command: string,
    args: string[],
    cwd: string,
    stdio: Stdio,
    rootFileSystem: RootFileSystem,
) => Promise<number>;

type DiagnisticItem =
    | { type: "file"; index: number; path: string }
    | { type: "info"; loc: string; link?: string; hover?: string }
    | { type: "message"; loc: string; content: string; severity?: string }
    | { type: "end" };

const ID = "lo";

export async function activate(context: vscode.ExtensionContext) {
    const logChannel = vscode.window.createOutputChannel("LO extension");
    const analysisPerWorkspace = new Map<string, FileAnalysisCollection>();

    const getDefaultWorkspaceUri = () => {
        return vscode.workspace.workspaceFolders?.[0]?.uri;
    };

    const getWorkspaceFolderUri = (uri: vscode.Uri) => {
        return vscode.workspace.getWorkspaceFolder(uri)?.uri;
    };

    const getAnalysisForWorkspace = (workspaceUri: vscode.Uri) => {
        const key = workspaceUri.toString(true);

        let analysis = analysisPerWorkspace.get(key);

        if (!analysis) {
            analysis = new FileAnalysisCollection(ID);
            analysisPerWorkspace.set(key, analysis);
        }

        return analysis;
    };

    const parseRange = (raw: string) => {
        const [startPos, endPos] = raw.split("-");
        const [startLine, startCol] = startPos.split(":").map(Number);
        const [endLine, endCol] = endPos.split(":").map(Number);

        return new vscode.Range(
            new vscode.Position(startLine - 1, startCol - 1),
            new vscode.Position(endLine - 1, endCol - 1),
        );
    };

    const inspectDocument = async (document: vscode.TextDocument) => {
        const workspaceUri = getWorkspaceFolderUri(document.uri);
        if (!workspaceUri) {
            return;
        }

        const analysis = getAnalysisForWorkspace(workspaceUri);
        const ctx = await loadCompilerCtx(workspaceUri);
        if (!ctx) {
            return;
        }

        const inspectLatency = new Latency("Inspect " + document.fileName);
        const compilerResult = await wasi.runWasiProgram({
            processName: ID,
            cwdUri: workspaceUri,
            args: [
                "inspect",
                vscode.workspace.asRelativePath(document.uri, false),
            ],
            module: ctx.compilerModule,
        });
        inspectLatency.measureAndLog(logChannel);

        analysis.clear();
        if (compilerResult.exitCode !== 0) {
            const stderr = new TextDecoder().decode(compilerResult.stderr);
            // when stderr is empty stdout contains error diagnostics in json format
            if (stderr !== "") {
                return showCompilerError(
                    workspaceUri,
                    analysis,
                    stderr,
                    compilerResult.exitCode,
                );
            }
        }

        const diagnostics: DiagnisticItem[] = [];
        try {
            const items = JSON.parse(
                new TextDecoder().decode(compilerResult.stdout),
            );
            diagnostics.push(...items);
        } catch {
            return showCompilerError(
                workspaceUri,
                analysis,
                "Inspect output is not a valid JSON array: " +
                    new TextDecoder().decode(compilerResult.stdout),
                compilerResult.exitCode,
            );
        }

        const analysisPerIndex = new Map<number, FileAnalysis>();
        for (const diagnostic of diagnostics) {
            if (diagnostic.type === "file") {
                const uri = vscode.Uri.joinPath(workspaceUri, diagnostic.path);
                const analysisItem = {
                    uri,
                    hovers: [],
                    links: [],
                    messages: [],
                } satisfies FileAnalysis;
                analysis.push(analysisItem);
                analysisPerIndex.set(diagnostic.index, analysisItem);
                continue;
            }

            if (diagnostic.type === "info") {
                const sourceIndex = Number(diagnostic.loc.split("/")[0]);
                const sourceRange = parseRange(diagnostic.loc.split("/")[1]);
                const fileDiagnostic = analysisPerIndex.get(sourceIndex)!;

                if (diagnostic.link) {
                    const targetIndex = Number(diagnostic.link.split("/")[0]);
                    const targetRange = parseRange(
                        diagnostic.link.split("/")[1],
                    );

                    fileDiagnostic.links.push({
                        originSelectionRange: sourceRange,
                        targetUri: analysisPerIndex.get(targetIndex)!.uri,
                        targetRange: targetRange,
                        pointsTo: "definition",
                    });

                    analysisPerIndex.get(targetIndex)!.links.push({
                        originSelectionRange: targetRange,
                        targetUri: fileDiagnostic.uri,
                        targetRange: sourceRange,
                        pointsTo: "usage",
                    });
                }

                if (diagnostic.hover) {
                    fileDiagnostic.hovers.push(
                        new vscode.Hover(
                            new vscode.MarkdownString().appendCodeblock(
                                diagnostic.hover,
                                ID,
                            ),
                            sourceRange,
                        ),
                    );
                }
                continue;
            }

            if (diagnostic.type === "message") {
                const sourceIndex = Number(diagnostic.loc.split("/")[0]);
                const sourceRange = parseRange(diagnostic.loc.split("/")[1]);
                const fileDiagnostic = analysisPerIndex.get(sourceIndex)!;

                let severity = vscode.DiagnosticSeverity.Information;
                if (diagnostic.severity === "error") {
                    severity = vscode.DiagnosticSeverity.Error;
                } else if (diagnostic.severity === "warning") {
                    severity = vscode.DiagnosticSeverity.Warning;
                }

                fileDiagnostic.messages.push(
                    new vscode.Diagnostic(
                        sourceRange,
                        diagnostic.content,
                        severity,
                    ),
                );

                continue;
            }

            if (diagnostic.type === "end") {
                continue;
            }

            diagnostic satisfies never;
        }

        analysis.showMessages();
    };

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.initProject", async () => {
            let workspaceUri = getDefaultWorkspaceUri();
            if (!workspaceUri) {
                return vscode.window.showErrorMessage("No folder open");
            }

            const initDir = vscode.Uri.joinPath(
                context.extensionUri,
                "assets/initial-project",
            );

            const initFiles = ["lo.wasm", "main.lo", "lib"];

            for (const file of initFiles) {
                await vscode.workspace.fs.copy(
                    vscode.Uri.joinPath(initDir, file),
                    vscode.Uri.joinPath(workspaceUri, file),
                );
            }

            await vscode.window.showInformationMessage("Project initialized");
        }),
    );

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.runFile", async () => {
            const currentFile = vscode.window.activeTextEditor?.document;
            if (currentFile === undefined) {
                return vscode.window.showErrorMessage("No files opened");
            }

            const workspaceUri = getWorkspaceFolderUri(currentFile.uri);
            if (!workspaceUri) {
                return vscode.window.showErrorMessage(
                    "Document is not in any workspace folder",
                );
            }

            const analysis = getAnalysisForWorkspace(workspaceUri);

            const ctx = await loadCompilerCtx(workspaceUri);
            if (!ctx) {
                return;
            }

            const compileLatency = new Latency(
                "Compile " + currentFile.fileName,
            );
            const compilerResult = await wasi.runWasiProgram({
                processName: ID,
                cwdUri: workspaceUri,
                args: [
                    "compile",
                    vscode.workspace.asRelativePath(currentFile.uri, false),
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
                    compilerResult.exitCode,
                );
            }

            let programModule: WebAssembly.Module;
            try {
                programModule = await WebAssembly.compile(
                    compilerResult.stdout as never,
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
                    },
                );
            }

            return vscode.window.showInformationMessage("Program output", {
                modal: true,
                detail: new TextDecoder().decode(programResult.stdout),
            });
        }),
    );

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.webshell.clear", (async (
            _command,
            _args,
            _cwd,
            stdio,
            _rootFileSystem,
        ) => {
            if (stdio.out?.kind === "terminal") {
                await stdio.out.terminal.write(
                    new TextEncoder().encode("\x1b[2J\x1b[H"),
                );
            }
            return 0;
        }) satisfies WebShellCommandHandler),
    );

    context.subscriptions.push(
        vscode.commands.registerCommand("lo.webshell.run", (async (
            command,
            args,
            cwd,
            stdio,
            _rootFileSystem,
        ) => {
            const workspaceUri = getDefaultWorkspaceUri();
            if (!workspaceUri) {
                await logError("No workspace folder found");
                return 1;
            }

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
                        programPath,
                    ),
                );
                programModule = await WebAssembly.compile(
                    programBytes as never,
                );
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
                    },
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
                                stdoutRedirect.fileName,
                            ),
                            stdoutRedirect.pipe.get(),
                        );
                    } catch (err) {
                        console.error(
                            `error redirecting stdout to ${stdoutRedirect.fileName}`,
                            err,
                        );
                    }
                }
            }

            async function logError(errorMessage: string) {
                if (stdio.err?.kind === "terminal") {
                    await stdio.err.terminal.write(errorMessage + "\n");
                }
            }
        }) satisfies WebShellCommandHandler),
    );

    const config = vscode.workspace.getConfiguration(ID);
    if (config.get<boolean>("enableFormatting") ?? true) {
        context.subscriptions.push(
            vscode.languages.registerDocumentFormattingEditProvider(ID, {
                async provideDocumentFormattingEdits(document) {
                    const workspaceUri = getWorkspaceFolderUri(document.uri);
                    if (!workspaceUri) {
                        return null;
                    }

                    const analysis = getAnalysisForWorkspace(workspaceUri);
                    const formatLatency = new Latency(
                        "Format " + document.fileName,
                    );
                    const edits = await formatFile(
                        workspaceUri,
                        document,
                        analysis,
                    );
                    formatLatency.measureAndLog(logChannel);
                    return edits;
                },
            }),
        );
    }

    context.subscriptions.push(
        vscode.languages.registerDefinitionProvider(ID, {
            provideDefinition(document, position, token) {
                const workspaceUri = getWorkspaceFolderUri(document.uri);
                if (!workspaceUri) {
                    return null;
                }

                const analysis = getAnalysisForWorkspace(workspaceUri);
                return analysis.provideDefinition(document, position, token);
            },
        }),
        vscode.languages.registerHoverProvider(ID, {
            provideHover(document, position, token) {
                const workspaceUri = getWorkspaceFolderUri(document.uri);
                if (!workspaceUri) {
                    return null;
                }

                const analysis = getAnalysisForWorkspace(workspaceUri);
                return analysis.provideHover(document, position, token);
            },
        }),
        vscode.languages.registerRenameProvider(ID, {
            prepareRename(document, position, token) {
                const workspaceUri = getWorkspaceFolderUri(document.uri);
                if (!workspaceUri) {
                    return null;
                }

                const analysis = getAnalysisForWorkspace(workspaceUri);
                return analysis.prepareRename(document, position, token);
            },
            provideRenameEdits(document, position, newName) {
                const workspaceUri = getWorkspaceFolderUri(document.uri);
                if (!workspaceUri) {
                    return null;
                }

                const analysis = getAnalysisForWorkspace(workspaceUri);
                return analysis.provideRenameEdits(document, position, newName);
            },
        }),
    );

    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(async (doc) => {
            if (doc.languageId !== ID) {
                return;
            }

            // NOTE: when there is a pinned `.lo` file it acts as a project root
            //   and is inspected instead of the currently opened file
            let pinnedDoc: vscode.TextDocument | undefined;

            pinSearchLoop: for (const group of [
                vscode.window.tabGroups.activeTabGroup,
                ...vscode.window.tabGroups.all,
            ]) {
                for (const tab of group.tabs) {
                    if (
                        tab.isPinned &&
                        tab.input instanceof vscode.TabInputText
                    ) {
                        pinnedDoc = await vscode.workspace.openTextDocument(
                            tab.input.uri,
                        );
                        break pinSearchLoop;
                    }
                }
            }

            await inspectDocument(pinnedDoc ?? doc);
        }),
    );
}

async function formatFile(
    workspaceUri: vscode.Uri,
    currentFile: vscode.TextDocument,
    analysis: FileAnalysisCollection,
) {
    const ctx = await loadCompilerCtx(workspaceUri);
    if (!ctx) {
        return [];
    }

    const tmpFileName = crypto.randomUUID() + ".lo";

    const compilerResult = await wasi.runWasiProgram({
        processName: ID,
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
            compilerResult.exitCode,
        );
        return [];
    }

    const formattedFile = new TextDecoder().decode(compilerResult.stdout);
    return [
        new vscode.TextEdit(
            new vscode.Range(
                new vscode.Position(0, 0),
                currentFile.lineAt(currentFile.lineCount - 1).range.end,
            ),
            formattedFile,
        ),
    ];
}

async function loadCompilerCtx(workspaceUri: vscode.Uri) {
    const config = vscode.workspace.getConfiguration(ID);
    const compilerPath = config.get<string>("compilerPath")!;

    let compilerModule: WebAssembly.Module;
    try {
        compilerModule = await WebAssembly.compile(
            (await vscode.workspace.fs.readFile(
                vscode.Uri.joinPath(workspaceUri, compilerPath),
            )) as never,
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
    exitCode: number,
) {
    const match = errorMessage.match(/^(?:ERROR: )?(.+):(\d+):(\d+) - (.+)\n$/);
    if (match === null) {
        return vscode.window.showErrorMessage(
            `Compiler errored (exit code: ${exitCode})`,
            { modal: true, detail: errorMessage },
        );
    }

    const filePath = match[1];
    const lineNumber = Number(match[2]);
    const columnNumber = Number(match[3]);
    const message = match[4];

    const fileUri = vscode.Uri.joinPath(workspaceUri, filePath);

    const range = new vscode.Range(
        new vscode.Position(lineNumber - 1, columnNumber - 1),
        new vscode.Position(lineNumber - 1, columnNumber - 1),
    );

    const diagnostic = new vscode.Diagnostic(
        range,
        message,
        vscode.DiagnosticSeverity.Error,
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
