import type {
	BiomePath,
	FixFileMode,
	Module,
	OpenProjectResult,
	ProjectKey,
	Workspace,
} from "./wasm";
import { tryCatchWrapper } from "./wasm";

/**
 * Get the UTF-8 byte length of a UTF-16 code unit.
 *
 * @param codeUnit A single character (UTF-16 code unit)
 * @returns The number of bytes this code unit takes in UTF-8 encoding
 */
function getUtf8ByteLength(codeUnit: string): number {
	const code = codeUnit.charCodeAt(0);
	if (code < 128) {
		return 1;
	}
	if (code < 2048) {
		return 2;
	}
	// UTF-16 high surrogate
	if (55296 <= code && code <= 56319) {
		return 4;
	}
	// UTF-16 low surrogate
	if (56320 <= code && code <= 57343) {
		return 0;
	}
	if (code < 65536) {
		return 3;
	}
	throw new Error(`Bad UTF-16 code unit "${codeUnit}" with code ${code}`);
}

/**
 * Convert a byte-based span to a UTF-16 code unit span.
 *
 * Biome internally uses UTF-8 byte offsets for spans, but JavaScript strings
 * use UTF-16 code units. This function converts a span from byte offsets to
 * code unit offsets, allowing you to use `string.slice(start, end)` correctly
 * with non-ASCII content.
 *
 * @param span A tuple of [startInBytes, endInBytes] from Biome diagnostics
 * @param str The original string that the span refers to
 * @returns A tuple of [startInCodeUnits, endInCodeUnits] for use with string.slice()
 *
 * @example
 * ```ts
 * const content = "/** Franççççais *\/ let a = 123";
 * const result = biome.lintContent(projectKey, content, { filePath: "example.js" });
 *
 * for (const diagnostic of result.diagnostics) {
 *     const [start, end] = spanInBytesToSpanInCodeUnits(
 *         diagnostic.location.span,
 *         content
 *     );
 *     const text = content.slice(start, end);
 *     console.log(text); // Correctly extracts the text
 * }
 * ```
 */
export function spanInBytesToSpanInCodeUnits(
	span: [number, number],
	str: string,
): [number, number] {
	const [startInBytes, endInBytes] = span;
	const spanInCodeUnits: [number, number] = [startInBytes, endInBytes];

	let currCodeUnitIndex = 0;

	// Scan through the string, looking for the start of the substring
	let bytePos = 0;
	while (bytePos < startInBytes && currCodeUnitIndex < str.length) {
		const byteLength = getUtf8ByteLength(str.charAt(currCodeUnitIndex));
		bytePos += byteLength;
		++currCodeUnitIndex;

		// Make sure to include low surrogate
		if (byteLength === 4 && bytePos === startInBytes) {
			++currCodeUnitIndex;
		}
	}

	// We've found the start, we update the start of spanInCodeUnits
	spanInCodeUnits[0] = currCodeUnitIndex;

	// Now scan through the following string to find the end
	while (bytePos < endInBytes && currCodeUnitIndex < str.length) {
		const byteLength = getUtf8ByteLength(str.charAt(currCodeUnitIndex));
		bytePos += byteLength;
		++currCodeUnitIndex;

		// Make sure to include low surrogate
		if (byteLength === 4 && bytePos === endInBytes) {
			++currCodeUnitIndex;
		}
	}

	// We've found the end, we update the end of spanInCodeUnits
	spanInCodeUnits[1] = currCodeUnitIndex;

	return spanInCodeUnits;
}

export interface FormatContentDebugOptions extends FormatContentOptions {
	/**
	 * If `true`, you'll be able to inspect the IR of the formatter
	 */
	debug: boolean;
}

export interface FormatContentOptions {
	/**
	 * A virtual path of the file. You should add the extension, so Biome knows
	 * how to parse the content
	 */
	filePath: string;
	/**
	 * The range where to format the content
	 */
	range?: [number, number];
}

export interface FormatResult<Diagnostic> {
	/**
	 * The new formatted content
	 */
	content: string;
	/**
	 * A series of errors encountered while executing an operation
	 */
	diagnostics: Diagnostic[];
}

export interface FormatDebugResult<Diagnostic>
	extends FormatResult<Diagnostic> {
	/**
	 * The IR emitted by the formatter
	 */
	ir: string;
}

export interface LintContentOptions {
	/**
	 * A virtual path of the file. You should add the extension, so Biome knows
	 * how to parse the content
	 */
	filePath: string;
	fixFileMode?: FixFileMode;
}

export interface LintResult<Diagnostic> {
	content: string;
	diagnostics: Diagnostic[];
}

function isFormatContentDebug(
	options: FormatContentOptions | FormatContentDebugOptions,
): options is FormatContentDebugOptions {
	return "debug" in options && options.debug !== undefined;
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

/**
 * List of modules that have been initialized.
 */
const initialized = new WeakSet();

export class BiomeCommon<Configuration, Diagnostic> {
	private readonly workspace: Workspace<Configuration, Diagnostic>;

	constructor(private readonly module: Module<Configuration, Diagnostic>) {
		if (!initialized.has(module)) {
			module.main();
			initialized.add(module);
		}

		this.workspace = new module.Workspace();
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
	 * @param projectKey The identifier of the project
	 * @param configuration
	 */
	applyConfiguration(
		projectKey: ProjectKey,
		configuration: Configuration,
	): void {
		tryCatchWrapper(() => {
			this.workspace.updateSettings({
				projectKey,
				configuration,
				workspaceDirectory: "./",
			});
		});
	}

	/**
	 * Open a possible workspace project folder. Returns the key of said project. Use this key when you want to switch to different projects.
	 *
	 * @param [path]
	 */
	openProject(path?: string): OpenProjectResult {
		return this.workspace.openProject({
			path: path || "",
			openUninitialized: true,
		});
	}

	private withFile<T>(
		projectKey: ProjectKey,
		path: string,
		content: string,
		func: (path: BiomePath) => T,
	): T {
		return tryCatchWrapper(() => {
			this.workspace.openFile({
				projectKey,
				content: { type: "fromClient", content, version: 0 },
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
	): FormatResult<Diagnostic>;
	formatContent(
		projectKey: ProjectKey,
		content: string,
		options: FormatContentDebugOptions,
	): FormatDebugResult<Diagnostic>;

	/**
	 * If formats some content.
	 *
	 * @param projectKey The identifier of the project
	 * @param content The content to format
	 * @param options Options needed when formatting some content
	 */
	formatContent(
		projectKey: ProjectKey,
		content: string,
		options: FormatContentOptions | FormatContentDebugOptions,
	): FormatResult<Diagnostic> | FormatDebugResult<Diagnostic> {
		return this.withFile(projectKey, options.filePath, content, (path) => {
			let code = content;

			const result = this.workspace.pullDiagnostics({
				projectKey,
				path,
				categories: ["syntax"],
				pullCodeActions: false,
			});

			if (0 === result.errors) {
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
						diagnostics: result.diagnostics,
						ir,
					};
				}
			}

			return {
				content: code,
				diagnostics: result.diagnostics,
			};
		});
	}

	/**
	 * Lint the content of a file.
	 *
	 * @param projectKey The identifier of the project
	 * @param content The content to lint
	 * @param options Options needed when linting some content
	 */
	lintContent(
		projectKey: ProjectKey,
		content: string,
		{ filePath, fixFileMode }: LintContentOptions,
	): LintResult<Diagnostic> {
		const maybeFixedContent = fixFileMode
			? this.withFile(projectKey, filePath, content, (path) => {
					const { code } = this.workspace.fixFile({
						projectKey,
						path,
						fixFileMode: fixFileMode,
						shouldFormat: false,
						ruleCategories: ["syntax", "lint", "action"],
					});
					return code;
				})
			: content;

		return this.withFile(projectKey, filePath, maybeFixedContent, (path) => {
			const { diagnostics } = this.workspace.pullDiagnostics({
				projectKey,
				path,
				categories: ["syntax", "lint", "action"],
				pullCodeActions: false,
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
	 * @param diagnostics The list of diagnostics to print
	 * @param options Options needed for printing the diagnostics
	 */
	printDiagnostics(
		diagnostics: Diagnostic[],
		options: PrintDiagnosticsOptions,
	): string {
		return tryCatchWrapper(() => {
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
