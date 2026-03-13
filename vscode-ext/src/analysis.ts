import * as vscode from "vscode";

type DirectionalLink = vscode.LocationLink & {
    pointsTo: "definition" | "usage";
};

export type FileAnalysis = {
    uri: vscode.Uri;
    hovers: vscode.Hover[];
    links: DirectionalLink[];
    messages: vscode.Diagnostic[];
};

export class FileAnalysisCollection
    implements
        vscode.DefinitionProvider,
        vscode.HoverProvider,
        vscode.RenameProvider
{
    diagnosticCollection: vscode.DiagnosticCollection;
    analysisPerUri = new Map<string, FileAnalysis>();

    constructor(private name: string) {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection(
            this.name,
        );
    }

    push(fileAnalysis: FileAnalysis) {
        this.analysisPerUri.set(fileAnalysis.uri.toString(true), fileAnalysis);
    }

    registerProviders(context: vscode.ExtensionContext) {
        context.subscriptions.push(
            vscode.languages.registerDefinitionProvider(this.name, this),
            vscode.languages.registerHoverProvider(this.name, this),
            vscode.languages.registerRenameProvider(this.name, this),
        );
    }

    provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        _token: vscode.CancellationToken,
    ): vscode.ProviderResult<vscode.Hover> {
        const analysis = this.analysisPerUri.get(document.uri.toString(true));
        if (!analysis) {
            return null;
        }

        for (const hover of analysis.hovers) {
            if (hover.range!.contains(position)) {
                return hover;
            }
        }

        return null;
    }

    provideDefinition(
        document: vscode.TextDocument,
        position: vscode.Position,
        _token: vscode.CancellationToken,
    ): vscode.ProviderResult<vscode.LocationLink[] | vscode.Definition> {
        const analysis = this.analysisPerUri.get(document.uri.toString(true));
        if (!analysis) {
            return null;
        }

        const outRefs = [];

        for (const ref of analysis.links) {
            if (ref.originSelectionRange!.contains(position)) {
                outRefs.push(ref);
            }
        }

        return outRefs.length === 0 ? null : outRefs;
    }

    prepareRename(
        document: vscode.TextDocument,
        position: vscode.Position,
        _token: vscode.CancellationToken,
    ): vscode.ProviderResult<
        vscode.Range | { range: vscode.Range; placeholder: string }
    > {
        const analysis = this.analysisPerUri.get(document.uri.toString(true));
        if (!analysis) {
            return null;
        }

        for (const hover of analysis.hovers) {
            if (hover.range!.contains(position)) {
                return hover.range;
            }
        }

        return null;
    }

    provideRenameEdits(
        document: vscode.TextDocument,
        position: vscode.Position,
        newSymbolName: string,
    ): vscode.WorkspaceEdit | null {
        const analysis = this.analysisPerUri.get(document.uri.toString(true));
        if (!analysis) {
            return null;
        }

        let symbolRange: vscode.Range | undefined;
        for (const hover of analysis.hovers) {
            if (hover.range!.contains(position)) {
                symbolRange = hover.range;
                break;
            }
        }

        if (!symbolRange) {
            return null;
        }

        let def = { uri: document.uri, range: symbolRange };

        // go to definition if the rename is at the usage site
        for (const link of analysis.links) {
            if (
                link.pointsTo === "definition" &&
                link.originSelectionRange!.isEqual(symbolRange)
            ) {
                def = { uri: link.targetUri, range: link.targetRange };
                break;
            }
        }

        const edit = new vscode.WorkspaceEdit();

        // rename the definition itself
        edit.replace(def.uri, def.range, newSymbolName);

        const analysisAtDef = this.analysisPerUri.get(def.uri.toString(true));
        if (!analysisAtDef) {
            return edit;
        }

        // rename all usages
        for (const link of analysisAtDef.links) {
            if (
                link.pointsTo === "usage" &&
                link.originSelectionRange!.isEqual(def.range)
            ) {
                edit.replace(link.targetUri, link.targetRange, newSymbolName);
            }
        }

        return edit;
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
