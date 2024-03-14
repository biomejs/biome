import {
	ArrowParentheses,
	AttributePosition,
	IndentStyle,
	type PlaygroundSettings,
	type PrettierOptions,
	type PrettierOutput,
	type QuoteProperties,
	QuoteStyle,
	Semicolons,
	type TrailingComma,
	defaultPlaygroundState,
} from "@/playground/types";
import {
	isCssFilename,
	isJsonFilename,
	isTypeScriptFilename,
} from "@/playground/utils";
import * as prettier from "prettier";
// @ts-expect-error
import parserBabel from "prettier/esm/parser-babel.mjs";
// @ts-expect-error
import pluginEstree from "prettier/plugins/estree.mjs";
// @ts-expect-error
import pluginCss from "prettier/plugins/postcss.mjs";

let settings = defaultPlaygroundState.settings;

self.addEventListener("message", async (e) => {
	switch (e.data.type) {
		case "updateSettings": {
			settings = e.data.settings as PlaygroundSettings;
			break;
		}

		case "format": {
			const {
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				jsxQuoteStyle,
				quoteProperties,
				trailingComma,
				semicolons,
				arrowParentheses,
				bracketSpacing,
				bracketSameLine,
				attributePosition,
			} = settings;
			const code = e.data.code as string;
			const filename = e.data.filename as string;

			const prettierOutput = await formatWithPrettier(code, {
				lineWidth,
				indentStyle,
				indentWidth,
				filepath: filename,
				quoteStyle,
				jsxQuoteStyle,
				quoteProperties,
				trailingComma,
				semicolons,
				arrowParentheses,
				bracketSpacing,
				bracketSameLine,
				singleAttributePerLine:
					attributePosition === AttributePosition.Multiline,
			});

			self.postMessage({
				type: "formatted",
				filename,
				prettierOutput,
			});

			break;
		}

		default:
			console.error(`Unknown message ${e.data.type}.`);
	}
});

async function formatWithPrettier(
	code: string,
	options: {
		lineWidth: number;
		indentStyle: IndentStyle;
		indentWidth: number;
		filepath: string;
		quoteStyle: QuoteStyle;
		jsxQuoteStyle: QuoteStyle;
		quoteProperties: QuoteProperties;
		trailingComma: TrailingComma;
		semicolons: Semicolons;
		arrowParentheses: ArrowParentheses;
		bracketSpacing: boolean;
		bracketSameLine: boolean;
		singleAttributePerLine?: boolean;
	},
): Promise<PrettierOutput> {
	try {
		const prettierOptions: PrettierOptions = {
			useTabs: options.indentStyle === IndentStyle.Tab,
			tabWidth: options.indentWidth,
			printWidth: options.lineWidth,
			filepath: options.filepath,
			plugins: [parserBabel, pluginEstree, pluginCss],
			parser: getPrettierParser(options.filepath),
			singleQuote: options.quoteStyle === QuoteStyle.Single,
			jsxSingleQuote: options.jsxQuoteStyle === QuoteStyle.Single,
			quoteProps: options.quoteProperties,
			trailingComma: options.trailingComma,
			semi: options.semicolons === Semicolons.Always,
			arrowParens:
				options.arrowParentheses === ArrowParentheses.Always
					? "always"
					: "avoid",
			bracketSpacing: options.bracketSpacing,
			bracketSameLine: options.bracketSameLine,
			singleAttributePerLine: options.singleAttributePerLine ?? false,
		};

		// @ts-expect-error
		const debug = prettier.__debug;
		const document = await debug.printToDoc(code, prettierOptions);

		// formatDoc must be before prettier.format because prettier.format mutates the document and breaks the ir
		const ir = await debug.formatDoc(document, {
			parser: "babel",
			plugins: [parserBabel, pluginEstree],
		});

		const formattedCode = await prettier.format(code, prettierOptions);

		return {
			type: "SUCCESS",
			code: formattedCode,
			ir,
		};
	} catch (err: unknown) {
		if (err instanceof SyntaxError) {
			return {
				type: "ERROR",
				stack: err.message,
			};
		}
		return {
			type: "ERROR",
			stack: (err as Error).stack ?? "",
		};
	}
}

function getPrettierParser(filename: string): string {
	if (isTypeScriptFilename(filename)) {
		return "babel-ts";
	}
	if (isJsonFilename(filename)) {
		return "json5";
	}
	if (isCssFilename(filename)) {
		return "css";
	}
	return "babel";
}
