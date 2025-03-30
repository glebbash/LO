import * as vscode from "vscode";

export type FileAnalysis = {
    uri: vscode.Uri;
    hovers: vscode.Hover[];
    links: vscode.LocationLink[];
    messages: vscode.Diagnostic[];
};

export class FileAnalysisCollection
    implements vscode.DefinitionProvider, vscode.HoverProvider
{
    diagnosticCollection: vscode.DiagnosticCollection;
    analysisPerUri = new Map<string, FileAnalysis>();

    constructor(private name: string) {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection(
            this.name
        );
    }

    push(fileAnalysis: FileAnalysis) {
        this.analysisPerUri.set(fileAnalysis.uri.toString(true), fileAnalysis);
    }

    registerProviders(context: vscode.ExtensionContext) {
        context.subscriptions.push(
            vscode.languages.registerDefinitionProvider(this.name, this),
            vscode.languages.registerHoverProvider(this.name, this)
        );
    }

    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        _token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.Hover> {
        const hovers =
            this.analysisPerUri.get(document.uri.toString(true))?.hovers ?? [];
        for (const hover of hovers) {
            if (hover.range!.contains(position)) {
                return hover;
            }
        }

        return null;
    }

    provideDefinition(
        document: vscode.TextDocument,
        position: vscode.Position,
        _token: vscode.CancellationToken
    ): vscode.ProviderResult<vscode.LocationLink[] | vscode.Definition> {
        const links =
            this.analysisPerUri.get(document.uri.toString(true))?.links ?? [];
        for (const ref of links) {
            if (ref.originSelectionRange!.contains(position)) {
                return [ref];
            }
        }

        return null;
    }

    showMessages() {
        for (const analysis of this.analysisPerUri.values()) {
            this.diagnosticCollection.set(analysis.uri, [
                ...(this.diagnosticCollection.get(analysis.uri) ?? []),
                ...analysis.messages,
            ]);
        }
    }

    clear() {
        this.analysisPerUri.clear();
        this.diagnosticCollection.clear();
    }
}
