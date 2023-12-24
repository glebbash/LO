import * as lsp from "vscode-languageserver/browser";
import { TextDocuments, MarkupKind } from "vscode-languageserver";
import { TextDocument } from "vscode-languageserver-textdocument";

let lastHoverPostion = lsp.Position.create(0, 0);
const indexedDocuments = new Map();

const connection = lsp.createConnection(
    new lsp.BrowserMessageReader(self),
    new lsp.BrowserMessageWriter(self)
);

const { console } = connection;

main().catch(console.error);

async function main() {
    const documents = new TextDocuments(TextDocument);
    documents.listen(connection);

    connection.listen();
    connection.onInitialize((_params) => ({
        capabilities: {
            hoverProvider: {},
            textDocumentSync: lsp.TextDocumentSyncKind.Incremental,
            executeCommandProvider: {
                commands: ["lo.helloWorld"],
            },
        },
    }));

    // @ts-expect-error
    const source = fetch(await import("./server.wasm"));
    const { instance } = await WebAssembly.instantiateStreaming(source);
    console.info("loaded server.wasm");

    documents.onDidChangeContent(({ document }) => {
        indexDocument(document, instance);
    });

    connection.onExecuteCommand((params) => {
        if (params.command === "lo.helloWorld") {
            const args = [lastHoverPostion.line, lastHoverPostion.character];
            // @ts-expect-error
            const res = instance.exports.addTwo(...args);

            connection.window.showInformationMessage(
                `addTwo(${args.join(", ")} = ${res})`
            );
        }
    });

    connection.onHover((params) => {
        const indexedDoc = indexedDocuments.get(params.textDocument.uri);
        if (indexedDoc === undefined) {
            return null;
        }

        for (const hover of indexedDoc.hovers) {
            if (
                hover.range.start.line === params.position.line &&
                hover.range.end.line === params.position.line &&
                params.position.character >= hover.range.start.character &&
                params.position.character <= hover.range.end.character
            ) {
                lastHoverPostion = params.position;
                return {
                    contents: {
                        kind: MarkupKind.PlainText,
                        value: hover.title,
                    },
                    range: hover.range,
                };
            }
        }
        return null;
    });
}

function indexDocument(
    doc: TextDocument,
    _compilerInstance: WebAssembly.Instance
) {
    if (!indexedDocuments.has(doc.uri)) {
        indexedDocuments.set(doc.uri, { hovers: [] });
    }
    let indexedDoc = indexedDocuments.get(doc.uri);
    const hovers = (indexedDoc.hovers = [] as any[]);

    const textContent = doc.getText();
    const wordMatcher = /\b\w+\b/g;
    let match;
    while ((match = wordMatcher.exec(textContent)) !== null) {
        const startOffset = match.index;
        const endOffset = match.index + match[0].length;

        hovers.push({
            title: match[0],
            range: lsp.Range.create(
                doc.positionAt(startOffset),
                doc.positionAt(endOffset)
            ),
        });
    }
}
