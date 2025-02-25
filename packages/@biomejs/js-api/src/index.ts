import type {
	BiomePath,
	Configuration,
	Diagnostic,
	FixFileMode,
	ProjectKey,
	Workspace,
} from "@biomejs/wasm-nodejs";
import { Distribution, type WasmModule, loadModule, wrapError } from "./wasm";

// Re-export of some useful types for users
export type { Diagnostic, Configuration };
export { Distribution };

export interface FormatContentDebugOptions extends FormatContentOptions {
	/**
	 * If `true`, you'll be able to inspect the IR of the formatter
	 */
	debug: boolean;
}

export interface FormatContentOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Biome knows how to parse the content
	 */
	filePath: string;
	/**
	 * The range where to format the content
	 */
	range?: [number, number];
}

export interface FormatResult {
	/**
	 * The new formatted content
	 */
	content: string;
	/**
	 * A series of errors encountered while executing an operation
	 */
	diagnostics: Diagnostic[];
}

export interface FormatDebugResult extends FormatResult {
	/**
	 * The IR emitted by the formatter
	 */
	ir: string;
}

export interface LintContentOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Biome knows how to parse the content
	 */
	filePath: string;
	fixFileMode?: FixFileMode;
}

export interface LintResult {
	content: string;
	diagnostics: Diagnostic[];
}

function isFormatContentDebug(
	options: FormatContentOptions | FormatContentDebugOptions,
): options is FormatContentDebugOptions {
	return "debug" in options && options.debug !== undefined;
}

export interface BiomeCreate {
	distribution: Distribution;
}

export interface PrintDiagnosticsOptions {
	/**
	 * The name of the file to print diagnostics for
	 */
	filePath: string;
	/**
	 * The content of the file the diagnostics were emitted for
	 */
	fileSource: string;
	/**
	 * Whether to print the diagnostics in verbose mode
	 */
	verbose?: boolean;
}

export class Biome {
	private constructor(
		private readonly module: WasmModule,
		private readonly workspace: Workspace,
	) {}

	/**
	 * It creates a new instance of the class {Biome}.
	 */
	static async create(options: BiomeCreate): Promise<Biome> {
		const module = await loadModule(options.distribution);
		const workspace = new module.Workspace();
		const biome = new Biome(module, workspace);
		biome.openProject();
		return biome;
	}

	/**
	 * Stop this instance of Biome
	 *
	 * After calling `shutdown()` on this object, it should be considered
	 * unusable as calling any method on it will fail
	 */
	shutdown() {
		this.workspace.free();
	}

	/**
	 * Allows to apply a custom configuration.
	 *
	 * If fails when the configuration is incorrect.
	 *
	 * @param configuration
	 */
	applyConfiguration(
		projectKey: ProjectKey,
		configuration: Configuration,
	): void {
		try {
			this.workspace.updateSettings({
				projectKey,
				configuration,
				gitignoreMatches: [],
				workspaceDirectory: "./",
			});
		} catch (e) {
			throw wrapError(e);
		}
	}

	/**
	 * Open a possible workspace project folder. Returns the key of said project. Use this key when you want to switch to different projects.
	 *
	 * @param {string} [path]
	 */
	openProject(path?: string): ProjectKey {
		return this.workspace.openProject({
			path: path || "",
			openUninitialized: true,
		});
	}

	private tryCatchWrapper<T>(func: () => T): T {
		try {
			return func();
		} catch (err) {
			throw wrapError(err);
		}
	}

	private withFile<T>(
		projectKey: ProjectKey,
		path: string,
		content: string,
		func: (path: BiomePath) => T,
	): T {
		return this.tryCatchWrapper(() => {
			this.workspace.openFile({
				projectKey,
				content: { type: "fromClient", content },
				version: 0,
				path,
			});

			try {
				return func(path);
			} finally {
				this.workspace.closeFile({
					projectKey,
					path,
				});
			}
		});
	}

	formatContent(
		projectKey: ProjectKey,
		content: string,
		options: FormatContentOptions,
	): FormatResult;
	formatContent(
		projectKey: ProjectKey,
		content: string,
		options: FormatContentDebugOptions,
	): FormatDebugResult;

	/**
	 * If formats some content.
	 *
	 * @param {String} content The content to format
	 * @param {FormatContentOptions | FormatContentDebugOptions} options Options needed when formatting some content
	 */
	formatContent(
		projectKey: ProjectKey,
		content: string,
		options: FormatContentOptions | FormatContentDebugOptions,
	): FormatResult | FormatDebugResult {
		return this.withFile(projectKey, options.filePath, content, (path) => {
			let code = content;

			const { diagnostics } = this.workspace.pullDiagnostics({
				projectKey,
				path,
				categories: ["syntax"],
				maxDiagnostics: Number.MAX_SAFE_INTEGER,
				only: [],
				skip: [],
			});

			const hasErrors = diagnostics.some(
				(diag) => diag.severity === "fatal" || diag.severity === "error",
			);
			if (!hasErrors) {
				if (options.range) {
					const result = this.workspace.formatRange({
						projectKey,
						path,
						range: options.range,
					});
					code = result.code;
				} else {
					const result = this.workspace.formatFile({
						projectKey,
						path,
					});
					code = result.code;
				}

				if (isFormatContentDebug(options)) {
					const ir = this.workspace.getFormatterIr({
						projectKey,
						path,
					});

					return {
						content: code,
						diagnostics,
						ir,
					};
				}
			}

			return {
				content: code,
				diagnostics,
			};
		});
	}

	/**
	 * Lint the content of a file.
	 *
	 * @param {String} content The content to lint
	 * @param {LintContentOptions} options Options needed when linting some content
	 */
	lintContent(
		projectKey: ProjectKey,
		content: string,
		{ filePath, fixFileMode }: LintContentOptions,
	): LintResult {
		const maybeFixedContent = fixFileMode
			? this.withFile(projectKey, filePath, content, (path) => {
					let code = content;

					const result = this.workspace.fixFile({
						projectKey,
						path,
						fixFileMode: fixFileMode,
						shouldFormat: false,
						only: [],
						skip: [],
						ruleCategories: ["syntax", "lint"],
					});

					code = result.code;

					return code;
				})
			: content;

		return this.withFile(projectKey, filePath, maybeFixedContent, (path) => {
			const { diagnostics } = this.workspace.pullDiagnostics({
				projectKey,
				path,
				categories: ["syntax", "lint"],
				maxDiagnostics: Number.MAX_SAFE_INTEGER,
				only: [],
				skip: [],
			});

			return {
				content: maybeFixedContent,
				diagnostics,
			};
		});
	}

	/**
	 * Print a list of diagnostics to an HTML string.
	 *
	 * @param {Diagnostic[]} diagnostics The list of diagnostics to print
	 * @param {PrintDiagnosticsOptions} options Options needed for printing the diagnostics
	 */
	printDiagnostics(
		diagnostics: Diagnostic[],
		options: PrintDiagnosticsOptions,
	): string {
		return this.tryCatchWrapper(() => {
			const printer = new this.module.DiagnosticPrinter(
				options.filePath,
				options.fileSource,
			);

			try {
				for (const diag of diagnostics) {
					if (options.verbose) {
						printer.print_verbose(diag);
					} else {
						printer.print_simple(diag);
					}
				}
				return printer.finish();
			} catch (err) {
				// Only call `free` if the `print` method throws, `finish` will
				// take care of deallocating the printer even if it fails
				printer.free();
				throw err;
			}
		});
	}
}
