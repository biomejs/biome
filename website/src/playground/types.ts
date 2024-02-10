import type { Diagnostic } from "@biomejs/wasm-web";
import type { parser } from "codemirror-lang-rome-ast";
import type { Dispatch, SetStateAction } from "react";

export type { Options as PrettierOptions } from "prettier";

export enum IndentStyle {
	Tab = "tab",
	Space = "space",
}

export enum SourceType {
	Module = "module",
	Script = "script",
}

export enum QuoteStyle {
	Double = "double",
	Single = "single",
}

export enum LintRules {
	Recommended = "recommended",
	All = "all",
}

export enum QuoteProperties {
	AsNeeded = "as-needed",
	Preserve = "preserve",
}

export enum TrailingComma {
	All = "all",
	Es5 = "es5",
	None = "none",
}

export enum LoadingState {
	Loading = 0,
	Success = 1,
	Error = 2,
}

export enum Semicolons {
	Always = "always",
	AsNeeded = "as-needed",
}

export enum ArrowParentheses {
	Always = "always",
	AsNeeded = "as-needed",
}
export enum AttributePosition {
	Auto = "auto",
	Multiline = "multiline",
}

export type PrettierOutput =
	| { type: "SUCCESS"; code: string; ir: string }
	| { type: "ERROR"; stack: string };

export const emptyPrettierOutput: PrettierOutput = {
	type: "SUCCESS",
	code: "",
	ir: "",
};

export interface BiomeOutput {
	syntax: {
		ast: string;
		cst: string;
	};
	diagnostics: {
		console: string;
		list: Diagnostic[];
	};
	formatter: {
		code: string;
		ir: string;
	};
	analysis: {
		controlFlowGraph: string;
	};
	importSorting: {
		code: string;
	};
}

export const emptyBiomeOutput: BiomeOutput = {
	syntax: {
		ast: "",
		cst: "",
	},
	diagnostics: {
		console: "",
		list: [],
	},
	formatter: {
		code: "",
		ir: "",
	},
	analysis: {
		controlFlowGraph: "",
	},
	importSorting: {
		code: "",
	},
};

export interface PlaygroundSettings {
	lineWidth: number;
	indentStyle: IndentStyle;
	indentWidth: number;
	quoteStyle: QuoteStyle;
	jsxQuoteStyle: QuoteStyle;
	quoteProperties: QuoteProperties;
	trailingComma: TrailingComma;
	semicolons: Semicolons;
	arrowParentheses: ArrowParentheses;
	attributePosition: AttributePosition;
	bracketSpacing: boolean;
	bracketSameLine: boolean;
	lintRules: LintRules;
	enabledLinting: boolean;
	importSortingEnabled: boolean;
	unsafeParameterDecoratorsEnabled: boolean;
	allowComments: boolean;
}

export interface PlaygroundFileState {
	content: string;
	prettier: PrettierOutput;
	biome: BiomeOutput;
}

export interface PlaygroundState {
	currentFile: string;
	singleFileMode: boolean;
	tab: string;
	cursorPosition: number;
	files: Record<string, undefined | PlaygroundFileState>;
	settings: PlaygroundSettings;
}

export const defaultPlaygroundState: PlaygroundState = {
	cursorPosition: 0,
	tab: "formatter",
	currentFile: "main.tsx",
	singleFileMode: false,
	files: {
		"main.tsx": {
			content: "",
			prettier: emptyPrettierOutput,
			biome: emptyBiomeOutput,
		},
	},
	settings: {
		lineWidth: 80,
		indentWidth: 2,
		indentStyle: IndentStyle.Tab,
		quoteStyle: QuoteStyle.Double,
		jsxQuoteStyle: QuoteStyle.Double,
		quoteProperties: QuoteProperties.AsNeeded,
		trailingComma: TrailingComma.All,
		semicolons: Semicolons.Always,
		arrowParentheses: ArrowParentheses.Always,
		attributePosition: AttributePosition.Auto,
		bracketSpacing: true,
		bracketSameLine: false,
		lintRules: LintRules.Recommended,
		enabledLinting: true,
		importSortingEnabled: true,
		unsafeParameterDecoratorsEnabled: true,
		allowComments: true,
	},
};

export interface PlaygroundProps {
	setPlaygroundState: Dispatch<SetStateAction<PlaygroundState>>;
	resetPlaygroundState: () => void;
	playgroundState: PlaygroundState;
}

export type Tree = ReturnType<typeof parser.parse>;
type RangeMapKey = [number, number];
type RangeMapValue = [number, number];
export interface BiomeAstSyntacticData {
	ast: Tree;
	// key is range of original `SyntaxToken`, value is the range string, like `20..20` corresponding range in
	// `biome_xx_ast` `Display` string.
	rangeMap: Map<RangeMapKey, RangeMapValue>;
}
