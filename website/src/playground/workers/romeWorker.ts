import init, {
	Configuration,
	DiagnosticPrinter,
	RomePath,
	RuleCategories,
	Workspace,
} from "@biomejs/wasm-web";
import {
	ArrowParentheses,
	IndentStyle,
	LintRules,
	LoadingState,
	PlaygroundSettings,
	QuoteProperties,
	QuoteStyle,
	RomeOutput,
	Semicolons,
} from "../types";
import { isJsonFilename } from "../utils";

let workspace: Workspace | null = null;
let fileCounter = 0;

type File = {
	filename: string;
	id: number;
	content: string;
	version: number;
};

const files: Map<string, File> = new Map();

let configuration: undefined | Configuration;

function getPathForFile(file: File): RomePath {
	return {
		path: file.filename,
	};
}

self.addEventListener("message", async (e) => {
	switch (e.data.type) {
		case "init": {
			try {
				await init();

				workspace = new Workspace();

				self.postMessage({ type: "init", loadingState: LoadingState.Success });
			} catch (err) {
				console.error(err);
				self.postMessage({ type: "init", loadingState: LoadingState.Error });
			}

			break;
		}

		case "updateSettings": {
			if (!workspace) {
				console.error("Workspace was not initialized");
				break;
			}

			const {
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				jsxQuoteStyle,
				quoteProperties,
				lintRules,
				enabledLinting,
				trailingComma,
				semicolons,
				arrowParentheses,
				importSortingEnabled,
				unsafeParameterDecoratorsEnabled,
				allowComments,
			} = e.data.settings as PlaygroundSettings;

			configuration = {
				formatter: {
					enabled: true,
					formatWithErrors: true,
					lineWidth: lineWidth,
					indentStyle: indentStyle === IndentStyle.Tab ? "tab" : "space",
					indentSize: indentWidth,
				},

				linter: {
					enabled: enabledLinting,
				},

				organizeImports: {
					enabled: importSortingEnabled,
				},

				javascript: {
					formatter: {
						quoteStyle: quoteStyle === QuoteStyle.Double ? "double" : "single",
						jsxQuoteStyle:
							jsxQuoteStyle === QuoteStyle.Double ? "double" : "single",
						quoteProperties:
							quoteProperties === QuoteProperties.Preserve
								? "preserve"
								: "asNeeded",
						trailingComma,
						semicolons:
							semicolons === Semicolons.Always ? "always" : "asNeeded",
						arrowParentheses:
							arrowParentheses === ArrowParentheses.Always
								? "always"
								: "asNeeded",
					},
					parser: {
						unsafeParameterDecoratorsEnabled,
					},
				},
				json: {
					parser: {
						allowComments,
					},
				},
			};

			switch (lintRules) {
				case LintRules.Recommended: {
					configuration.linter!.rules = {
						nursery: {
							recommended: false,
						},
					};
					break;
				}
				case LintRules.All: {
					configuration.linter!.rules = {
						all: true,
					};
					break;
				}
			}

			workspace.updateSettings({
				configuration,
			});
			break;
		}

		case "update": {
			if (!workspace) {
				console.error("Workspace was not initialized");
				break;
			}

			const { filename, code, cursorPosition } = e.data;

			let file = files.get(filename);
			if (file === undefined) {
				file = {
					filename,
					version: 0,
					content: code,
					id: fileCounter++,
				};

				workspace.openFile({
					path: getPathForFile(file),
					version: 0,
					content: code,
				});
			} else {
				file = {
					filename,
					id: file.id,
					version: file.version + 1,
					content: code,
				};

				workspace.openFile({
					path: getPathForFile(file),
					version: file.version,
					content: code,
				});
			}
			files.set(filename, file);
			const path = getPathForFile(file);

			const syntaxTree = workspace.getSyntaxTree({
				path,
			});

			const controlFlowGraph = !isJsonFilename(filename)
				? workspace.getControlFlowGraph({
						path,
						cursor: cursorPosition,
				  })
				: "";

			const formatterIr = workspace.getFormatterIr({
				path,
			});

			const importSorting = workspace.organizeImports({
				path,
			});

			const categories: RuleCategories = [];
			if (configuration?.formatter?.enabled) {
				categories.push("Syntax");
			}
			if (configuration?.linter?.enabled) {
				categories.push("Lint");
			}
			const diagnosticsResult = workspace.pullDiagnostics({
				path,
				categories: categories,
				max_diagnostics: Number.MAX_SAFE_INTEGER,
			});

			const printer = new DiagnosticPrinter(path.path, code);
			for (const diag of diagnosticsResult.diagnostics) {
				printer.print_verbose(diag);
			}

			const printed = workspace.formatFile({
				path,
			});

			const romeOutput: RomeOutput = {
				syntax: {
					// Replace 4 spaced indentation with 2
					// TODO replace this in Rome itself
					ast: syntaxTree.ast.replace(/ {4}/g, "  "),
					cst: syntaxTree.cst,
				},
				diagnostics: {
					console: printer.finish(),
					list: diagnosticsResult.diagnostics,
				},
				formatter: {
					code: printed.code,
					ir: formatterIr,
				},
				analysis: {
					controlFlowGraph,
				},
				importSorting: {
					code: importSorting.code,
				},
			};

			self.postMessage({
				type: "updated",
				filename,
				romeOutput,
			});
			break;
		}

		default:
			console.error(`Unknown message '${e.data.type}'.`);
	}
});
