import {
	CancellationToken,
	Disposable,
	DocumentLink,
	DocumentLinkProvider,
	EventEmitter,
	ProviderResult,
	TextDocument,
	TextDocumentChangeEvent,
	TextDocumentContentProvider,
	TextEditor,
	Uri,
	ViewColumn,
	languages,
	window,
	workspace,
} from "vscode";
import { SyntaxTreeParams, syntaxTreeRequest } from "../lsp_requests";
import { Command, Session } from "../session";
import { isBiomeEditor } from "../utils";
import { SyntaxTreeDocument } from "./syntaxTreeDocument";

type FilePath = string;

class SyntaxTreeProvider
	implements TextDocumentContentProvider, DocumentLinkProvider, Disposable
{
	readonly session: Session;
	static scheme = "biome";
	public readonly uri: Uri = Uri.parse(
		`${SyntaxTreeProvider.scheme}:syntax_tree/tree.rast`,
	);
	readonly eventEmitter = new EventEmitter<Uri>();

	// internal cache of the documents
	// they are stored using their file path "file://path/to/file.*"
	private documents = new Map<FilePath, SyntaxTreeDocument>();

	constructor(session: Session) {
		this.session = session;
		workspace.onDidCloseTextDocument(
			this.onDidCloseTextDocument,
			this,
			session.subscriptions,
		);
		window.onDidChangeActiveTextEditor(
			this.onDidChangeActiveTextEditor,
			this,
			session.subscriptions,
		);

		workspace.onDidChangeTextDocument(
			this.onDidChangeTextDocument,
			this,
			session.subscriptions,
		);
	}

	// if a document changes, we remove it from the cache
	private onDidChangeTextDocument(event: TextDocumentChangeEvent) {
		this.documents.delete(event.document.uri.toString());
		this.eventEmitter.fire(this.uri);
	}

	private onDidChangeActiveTextEditor(editor: TextEditor | undefined) {
		if (editor && isBiomeEditor(editor)) {
			this.eventEmitter.fire(this.uri);
		}
	}

	// we remove the document when it's closed, we don't know if in the meantime can change
	private onDidCloseTextDocument(doc: TextDocument | undefined) {
		if (doc) {
			this.documents.delete(doc.uri.toString());
			this.eventEmitter.fire(this.uri);
		}
	}

	provideTextDocumentContent(
		uri: Uri,
		token: CancellationToken,
	): ProviderResult<string> {
		const documentUri = this.session.editor.document.uri.toString();
		// if the document is already cached, we show it
		const document = this.documents.get(documentUri);
		if (document) {
			return document.value;
		}

		const params: SyntaxTreeParams = {
			textDocument: { uri: this.session.editor.document.uri.toString() },
		};

		// send request to the server and store its content in the cache if successful
		return this.session.client
			.sendRequest(syntaxTreeRequest, params, token)
			.then((result) => {
				const document = new SyntaxTreeDocument(uri, result);
				this.documents.set(documentUri, document);

				return document.value;
			});
	}

	dispose(): void {
		this.documents.clear();
	}

	get onDidChange() {
		return this.eventEmitter.event;
	}

	provideDocumentLinks(document: TextDocument): ProviderResult<DocumentLink[]> {
		const doc = this.documents.get(document.uri.toString());
		if (doc) {
			return [];
		}
	}
}

export function syntaxTree(session: Session): Command {
	// we create the provider of the command
	const provider = new SyntaxTreeProvider(session);

	// we register a text document provider
	session.subscriptions.push(
		workspace.registerTextDocumentContentProvider(
			SyntaxTreeProvider.scheme,
			provider,
		),
	);

	session.subscriptions.push(
		languages.setLanguageConfiguration("biome_syntax_tree", {
			brackets: [["[", ")"]],
		}),
	);

	// we return a function that instructs the command what to do
	// when opening a document
	return async () => {
		const document = await workspace.openTextDocument(provider.uri);
		provider.eventEmitter.fire(provider.uri);
		void (await window.showTextDocument(document, {
			viewColumn: ViewColumn.Two,
			preserveFocus: true,
		}));
	};
}
