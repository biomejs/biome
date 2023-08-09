import type {
	Configuration,
	Diagnostic,
	PullDiagnosticsResult,
	RomePath,
	Workspace,
} from "@rometools/wasm-nodejs";
import { Distribution, WasmModule, loadModule, wrapError } from "./wasm";

// Re-export of some useful types for users
export type { Configuration, Diagnostic };
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
	 * so Rome knows how to parse the content
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

export interface FormatDebugResult {
	/**
	 * The new formatted content
	 */
	content: string;
	/**
	 * A series of errors encountered while executing an operation
	 */
	diagnostics: Diagnostic[];
	/**
	 * The IR emitted by the formatter
	 */
	ir: string;
}

export interface LintContentOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Rome knows how to parse the content
	 */
	filePath: string;
}

function isFormatContentDebug(
	options: FormatContentOptions | FormatContentDebugOptions,
): options is FormatContentDebugOptions {
	return "debug" in options && options.debug !== undefined;
}

export interface RomeCreate {
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

export class Rome {
	private constructor(
		private readonly module: WasmModule,
		private readonly workspace: Workspace,
	) {}

	/**
	 * It creates a new instance of the class {Rome}.
	 */
	public static async create(options: RomeCreate): Promise<Rome> {
		const module = await loadModule(options.distribution);
		const workspace = new module.Workspace();
		return new Rome(module, workspace);
	}

	/**
	 * Stop this instance of Rome
	 *
	 * After calling `shutdown()` on this object, it should be considered
	 * unusable as calling any method on it will fail
	 */
	public shutdown() {
		this.workspace.free();
	}

	/**
	 * Allows to apply a custom configuration.
	 *
	 * If fails when the configuration is incorrect.
	 *
	 * @param configuration
	 */
	public applyConfiguration(configuration: Configuration): void {
		try {
			this.workspace.updateSettings({
				configuration,
			});
		} catch (e) {
			throw wrapError(e);
		}
	}

	private withFile<T>(
		path: string,
		content: string,
		func: (path: RomePath) => T,
	): T {
		try {
			const romePath: RomePath = {
				path,
			};

			this.workspace.openFile({
				content,
				version: 0,
				path: romePath,
			});

			try {
				return func(romePath);
			} finally {
				this.workspace.closeFile({
					path: romePath,
				});
			}
		} catch (err) {
			throw wrapError(err);
		}
	}

	formatContent(content: string, options: FormatContentOptions): FormatResult;
	formatContent(
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
		content: string,
		options: FormatContentOptions | FormatContentDebugOptions,
	): FormatResult | FormatDebugResult {
		return this.withFile(options.filePath, content, (path) => {
			let code = content;

			const { diagnostics } = this.workspace.pullDiagnostics({
				path,
				categories: ["Syntax"],
				max_diagnostics: Number.MAX_SAFE_INTEGER,
			});

			const hasErrors = diagnostics.some(
				(diag) => diag.severity === "fatal" || diag.severity === "error",
			);
			if (!hasErrors) {
				if (options.range) {
					const result = this.workspace.formatRange({
						path,
						range: options.range,
					});
					code = result.code;
				} else {
					const result = this.workspace.formatFile({
						path,
					});
					code = result.code;
				}

				if (isFormatContentDebug(options)) {
					const ir = this.workspace.getFormatterIr({
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
		content: string,
		options: LintContentOptions,
	): PullDiagnosticsResult {
		return this.withFile(options.filePath, content, (path) => {
			return this.workspace.pullDiagnostics({
				path,
				categories: ["Syntax", "Lint"],
				max_diagnostics: Number.MAX_SAFE_INTEGER,
			});
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
		try {
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
			} catch (err) {
				// Only call `free` if the `print` method throws, `finish` will
				// take care of deallocating the printer even if it fails
				printer.free();
				throw err;
			}

			return printer.finish();
		} catch (err) {
			throw wrapError(err);
		}
	}
}
