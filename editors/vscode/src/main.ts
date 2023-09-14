import { type ChildProcess, spawn } from "child_process";
import { type Socket, connect } from "net";
import { isAbsolute } from "path";
import { TextDecoder, promisify } from "util";
import {
	ExtensionContext,
	OutputChannel,
	TextEditor,
	Uri,
	languages,
	window,
	workspace,
} from "vscode";
import {
	DocumentFilter,
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	StreamInfo,
} from "vscode-languageclient/node";
import { Commands } from "./commands";
import { syntaxTree } from "./commands/syntaxTree";
import { Session } from "./session";
import { StatusBar } from "./statusBar";
import { setContextValue } from "./utils";

import resolveImpl = require("resolve/async");
import type * as resolve from "resolve";

const resolveAsync = promisify<string, resolve.AsyncOpts, string | undefined>(
	resolveImpl,
);

let client: LanguageClient;

const IN_BIOME_PROJECT = "inBiomeProject";

export async function activate(context: ExtensionContext) {
	const outputChannel = window.createOutputChannel("Biome");
	const traceOutputChannel = window.createOutputChannel("Biome Trace");

	const requiresConfiguration = workspace
		.getConfiguration("biome")
		.get<boolean>("requireConfiguration");

	// If the extension requires a configuration file to be present, we attempt to
	// locate it. If a config file cannot be found, we do not go any further.
	if (requiresConfiguration) {
		outputChannel.appendLine("Configuration file required, looking for one.");
		// TODO: Stop looking for rome.json when we reach biome v2.0
		const configFiles = await workspace.findFiles("**/{biome,rome}.json");
		if (configFiles.length === 0) {
			outputChannel.appendLine(
				"No config file found, disabling Biome extension",
			);
			return;
		}
		outputChannel.appendLine(
			`Config file found at ${configFiles[0].fsPath}, enabling Biome extension`,
		);
	}

	const command = await getServerPath(context, outputChannel);

	if (!command) {
		await window.showErrorMessage(
			"The Biome extensions doesn't ship with prebuilt binaries for your platform yet. " +
				"You can still use it by cloning the biomejs/biome repo from GitHub to build the LSP " +
				"yourself and use it with this extension with the biome.lspBin setting",
		);
		return;
	}

	outputChannel.appendLine(`Using Biome from ${command}`);

	const statusBar = new StatusBar();

	const serverOptions: ServerOptions = createMessageTransports.bind(
		undefined,
		outputChannel,
		command,
	);

	const documentSelector: DocumentFilter[] = [
		{ language: "javascript", scheme: "file" },
		{ language: "typescript", scheme: "file" },
		{ language: "javascriptreact", scheme: "file" },
		{ language: "typescriptreact", scheme: "file" },
		{ language: "json", scheme: "file" },
		{ language: "jsonc", scheme: "file" },
	];

	const clientOptions: LanguageClientOptions = {
		documentSelector,
		outputChannel,
		traceOutputChannel,
	};

	client = new LanguageClient(
		"biome_lsp",
		"Biome",
		serverOptions,
		clientOptions,
	);

	const session = new Session(context, client);

	const codeDocumentSelector =
		client.protocol2CodeConverter.asDocumentSelector(documentSelector);

	// we are now in a biome project
	setContextValue(IN_BIOME_PROJECT, true);

	session.registerCommand(Commands.SyntaxTree, syntaxTree(session));
	session.registerCommand(Commands.ServerStatus, () => {
		traceOutputChannel.show();
	});
	session.registerCommand(Commands.RestartLspServer, async () => {
		try {
			if (client.isRunning()) {
				await client.restart();
			} else {
				await client.start();
			}
		} catch (error) {
			client.error("Restarting client failed", error, "force");
		}
	});

	context.subscriptions.push(
		client.onDidChangeState((evt) => {
			statusBar.setServerState(client, evt.newState);
		}),
	);

	const handleActiveTextEditorChanged = (textEditor?: TextEditor) => {
		if (!textEditor) {
			statusBar.setActive(false);
			return;
		}

		const { document } = textEditor;
		statusBar.setActive(languages.match(codeDocumentSelector, document) > 0);
	};

	context.subscriptions.push(
		window.onDidChangeActiveTextEditor(handleActiveTextEditorChanged),
	);

	handleActiveTextEditorChanged(window.activeTextEditor);
	await client.start();
}

type Architecture = "x64" | "arm64";

type PlatformTriplets = {
	[P in NodeJS.Platform]?: {
		[A in Architecture]: {
			triplet: string;
			package: string;
		};
	};
};

const PLATFORMS: PlatformTriplets = {
	win32: {
		x64: {
			triplet: "x86_64-pc-windows-msvc",
			package: "@biomejs/cli-win32-x64",
		},
		arm64: {
			triplet: "aarch64-pc-windows-msvc",
			package: "@biomejs/cli-win32-arm64",
		},
	},
	darwin: {
		x64: {
			triplet: "x86_64-apple-darwin",
			package: "@biomejs/cli-darwin-x64",
		},
		arm64: {
			triplet: "aarch64-apple-darwin",
			package: "@biomejs/cli-darwin-arm64",
		},
	},
	linux: {
		x64: {
			triplet: "x86_64-unknown-linux-gnu",
			package: "@biomejs/cli-linux-x64",
		},
		arm64: {
			triplet: "aarch64-unknown-linux-gnu",
			package: "@biomejs/cli-linux-arm64",
		},
	},
};

async function getServerPath(
	context: ExtensionContext,
	outputChannel: OutputChannel,
): Promise<string | undefined> {
	// Only allow the bundled Biome binary in untrusted workspaces
	if (!workspace.isTrusted) {
		return getBundledBinary(context, outputChannel);
	}

	if (process.env.DEBUG_SERVER_PATH) {
		outputChannel.appendLine(
			`Biome DEBUG_SERVER_PATH detected: ${process.env.DEBUG_SERVER_PATH}`,
		);
		return process.env.DEBUG_SERVER_PATH;
	}

	const config = workspace.getConfiguration();
	const explicitPath = config.get("biome.lspBin");
	if (typeof explicitPath === "string" && explicitPath !== "") {
		return getWorkspaceRelativePath(explicitPath);
	}

	return (
		(await getWorkspaceDependency(outputChannel)) ??
		(await getBundledBinary(context, outputChannel))
	);
}

// Resolve `path` as relative to the workspace root
async function getWorkspaceRelativePath(path: string) {
	if (isAbsolute(path)) {
		return path;
	} else {
		for (let i = 0; i < workspace.workspaceFolders.length; i++) {
			const workspaceFolder = workspace.workspaceFolders[i];
			const possiblePath = Uri.joinPath(workspaceFolder.uri, path);
			if (await fileExists(possiblePath)) {
				return possiblePath.fsPath;
			}
		}
		return undefined;
	}
}

// Tries to resolve a path to `@biomejs/cli-*` binary package from the root of the workspace
async function getWorkspaceDependency(
	outputChannel: OutputChannel,
): Promise<string | undefined> {
	const packageName = PLATFORMS[process.platform]?.[process.arch]?.package;

	const manifestName = `${packageName}/package.json`;
	const binaryName =
		process.platform === "win32"
			? `${packageName}/biome.exe`
			: `${packageName}/biome`;

	for (const workspaceFolder of workspace.workspaceFolders) {
		const path = Uri.joinPath(
			workspaceFolder.uri,
			"node_modules",
			".bin",
			"biome",
		);

		if (await fileExists(path)) {
			return path.fsPath;
		}
	}

	window.showWarningMessage(
		"Unable to resolve the biome server from your dependencies. Make sure it's correctly installed, or untick the `requireConfiguration` setting to use the bundled binary.",
	);

	return undefined;
}

// Returns the path of the binary distribution of Biome included in the bundle of the extension
async function getBundledBinary(
	context: ExtensionContext,
	outputChannel: OutputChannel,
) {
	const triplet = PLATFORMS[process.platform]?.[process.arch]?.triplet;
	if (!triplet) {
		outputChannel.appendLine(
			`Unsupported platform ${process.platform} ${process.arch}`,
		);
		return undefined;
	}

	const binaryExt = triplet.includes("windows") ? ".exe" : "";
	const binaryName = `biome${binaryExt}`;

	const bundlePath = Uri.joinPath(context.extensionUri, "server", binaryName);
	const bundleExists = await fileExists(bundlePath);
	if (!bundleExists) {
		outputChannel.appendLine(
			"Extension bundle does not include the prebuilt binary",
		);
		return undefined;
	}

	return bundlePath.fsPath;
}

async function fileExists(path: Uri) {
	try {
		await workspace.fs.stat(path);
		return true;
	} catch (err) {
		if (err.code === "ENOENT" || err.code === "FileNotFound") {
			return false;
		} else {
			throw err;
		}
	}
}

interface MutableBuffer {
	content: string;
}

function collectStream(
	outputChannel: OutputChannel,
	process: ChildProcess,
	key: "stdout" | "stderr",
	buffer: MutableBuffer,
) {
	return new Promise<void>((resolve, reject) => {
		const stream = process[key];
		stream.setEncoding("utf-8");

		stream.on("error", (err) => {
			outputChannel.appendLine(`[cli-${key}] error`);
			reject(err);
		});
		stream.on("close", () => {
			outputChannel.appendLine(`[cli-${key}] close`);
			resolve();
		});
		stream.on("finish", () => {
			outputChannel.appendLine(`[cli-${key}] finish`);
			resolve();
		});
		stream.on("end", () => {
			outputChannel.appendLine(`[cli-${key}] end`);
			resolve();
		});

		stream.on("data", (data) => {
			outputChannel.appendLine(`[cli-${key}] data ${data.length}`);
			buffer.content += data;
		});
	});
}

function withTimeout(promise: Promise<void>, duration: number) {
	return Promise.race([
		promise,
		new Promise<void>((resolve) => setTimeout(resolve, duration)),
	]);
}

async function getSocket(
	outputChannel: OutputChannel,
	command: string,
): Promise<string> {
	const process = spawn(command, ["__print_socket"], {
		stdio: [null, "pipe", "pipe"],
	});

	const stdout = { content: "" };
	const stderr = { content: "" };

	const stdoutPromise = collectStream(outputChannel, process, "stdout", stdout);
	const stderrPromise = collectStream(outputChannel, process, "stderr", stderr);

	const exitCode = await new Promise<number>((resolve, reject) => {
		process.on("error", reject);
		process.on("exit", (code) => {
			outputChannel.appendLine(`[cli] exit ${code}`);
			resolve(code);
		});
		process.on("close", (code) => {
			outputChannel.appendLine(`[cli] close ${code}`);
			resolve(code);
		});
	});

	await Promise.all([
		withTimeout(stdoutPromise, 1000),
		withTimeout(stderrPromise, 1000),
	]);

	const pipeName = stdout.content.trimEnd();

	if (exitCode !== 0 || pipeName.length === 0) {
		let message = `Command "${command} __print_socket" exited with code ${exitCode}`;
		if (stderr.content.length > 0) {
			message += `\nOutput:\n${stderr.content}`;
		}

		throw new Error(message);
	} else {
		outputChannel.appendLine(`Connecting to "${pipeName}" ...`);
		return pipeName;
	}
}

function wrapConnectionError(err: Error, path: string): Error {
	return Object.assign(
		new Error(
			`Could not connect to the Biome server at "${path}": ${err.message}`,
		),
		{ name: err.name, stack: err.stack },
	);
}

async function createMessageTransports(
	outputChannel: OutputChannel,
	command: string,
): Promise<StreamInfo> {
	const path = await getSocket(outputChannel, command);

	let socket: Socket;
	try {
		socket = connect(path);
	} catch (err) {
		throw wrapConnectionError(err, path);
	}

	await new Promise((resolve, reject) => {
		socket.once("ready", resolve);
		socket.once("error", (err) => {
			reject(wrapConnectionError(err, path));
		});
	});

	return { writer: socket, reader: socket };
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
