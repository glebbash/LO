import * as vscode from "vscode";
import * as wasi from "./run-wasi";
import { FileAnalysis, FileAnalysisCollection } from "./analysis";
import { Wasm, type RootFileSystem, type Stdio } from "@vscode/wasm-wasi";

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
    | { type: "hover"; source: number; range: string; content: string }
    | {
          type: "link";
          source: number;
          sourceRange: string;
          target: number;
          targetRange: string;
      }
    | { type: "end" };

export async function activate(context: vscode.ExtensionContext) {
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

    const processDocument = async (document: vscode.TextDocument) => {
        const ctx = await loadCompilerCtx();
        if (!ctx) {
            return;
        }
        const compilerResult = await wasi.runWasiProgram({
            processName: "lo",
            cwdUri: workspaceUri,
            args: [vscode.workspace.asRelativePath(document.uri), "--inspect"],
            module: ctx.compilerModule,
        });
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

            const compilerResult = await wasi.runWasiProgram({
                processName: "lo",
                cwdUri: workspaceUri,
                args: [vscode.workspace.asRelativePath(currentFile.uri)],
                module: ctx.compilerModule,
            });
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
                    // TODO: this should use StdioFileDescriptor when workspaceFolder mount is setup
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
                    // TODO: figure out why this doesn't work
                    // { stdio: processStdio, args, rootFileSystem }
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
        vscode.workspace.onDidSaveTextDocument(async (doc) => {
            if (doc.languageId === "lo") {
                await processDocument(doc);
            }
        })
    );
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
