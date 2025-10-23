// Generated file, do not edit by hand, see `xtask/codegen`
import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	features: FeatureName;
	path: BiomePath;
	projectKey: ProjectKey;
}
export type FeatureName = FeatureKind[];
export type BiomePath = string;
export type ProjectKey = number;
export type FeatureKind =
	| "format"
	| "lint"
	| "search"
	| "assist"
	| "debug"
	| "htmlFullSupport";
export interface FileFeaturesResult {
	featuresSupported: FeaturesSupported;
}
export type FeaturesSupported = { [K in FeatureKind]?: SupportKind };
export type SupportKind =
	| "supported"
	| "ignored"
	| "protected"
	| "featureNotEnabled"
	| "fileNotSupported";
export interface UpdateSettingsParams {
	configuration: Configuration;
	projectKey: ProjectKey;
	workspaceDirectory?: BiomePath;
}
/**
 * The configuration that is contained inside the file `biome.json`
 */
export interface Configuration {
	/**
	 * A field for the [JSON schema](https://json-schema.org/) specification
	 */
	$schema?: Schema;
	/**
	 * Specific configuration for assists
	 */
	assist?: AssistConfiguration;
	/**
	 * Specific configuration for the Css language
	 */
	css?: CssConfiguration;
	/**
	 * A list of paths to other JSON files, used to extends the current configuration.
	 */
	extends?: Extends;
	/**
	 * The configuration of the filesystem
	 */
	files?: FilesConfiguration;
	/**
	 * The configuration of the formatter
	 */
	formatter?: FormatterConfiguration;
	/**
	 * Specific configuration for the GraphQL language
	 */
	graphql?: GraphqlConfiguration;
	/**
	 * Specific configuration for the GraphQL language
	 */
	grit?: GritConfiguration;
	/**
	 * Specific configuration for the HTML language
	 */
	html?: HtmlConfiguration;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript?: JsConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	json?: JsonConfiguration;
	/**
	 * The configuration for the linter
	 */
	linter?: LinterConfiguration;
	/**
	 * A list of granular patterns that should be applied only to a sub set of files
	 */
	overrides?: Overrides;
	/**
	 * List of plugins to load.
	 */
	plugins?: Plugins;
	/**
	 * Indicates whether this configuration file is at the root of a Biome project. By default, this is `true`.
	 */
	root?: Bool;
	/**
	 * The configuration of the VCS integration
	 */
	vcs?: VcsConfiguration;
}
export type Schema = string;
export interface AssistConfiguration {
	/**
	 * Whether Biome should fail in CLI if the assist were not applied to the code.
	 */
	actions?: Actions;
	/**
	 * Whether Biome should enable assist via LSP and CLI.
	 */
	enabled?: Bool;
	/**
	 * A list of glob patterns. Biome will include files/folders that will match these patterns.
	 */
	includes?: NormalizedGlob[];
}
/**
 * Options applied to CSS files
 */
export interface CssConfiguration {
	/**
	 * CSS assist options
	 */
	assist?: CssAssistConfiguration;
	/**
	 * CSS formatter options
	 */
	formatter?: CssFormatterConfiguration;
	/**
	 * CSS globals
	 */
	globals?: string[];
	/**
	 * CSS linter options
	 */
	linter?: CssLinterConfiguration;
	/**
	 * CSS parsing options
	 */
	parser?: CssParserConfiguration;
}
export type Extends = string[] | string;
/**
 * The configuration of the filesystem
 */
export interface FilesConfiguration {
	/**
	* **Deprecated:** Please use _force-ignore syntax_ in `files.includes` instead: https://biomejs.dev/reference/configuration/#filesincludes

Set of file and folder names that should be unconditionally ignored by Biome's scanner. 
	 */
	experimentalScannerIgnores?: string[];
	/**
	 * Tells Biome to not emit diagnostics when handling files that it doesn't know
	 */
	ignoreUnknown?: Bool;
	/**
	 * A list of glob patterns. Biome will handle only those files/folders that will match these patterns.
	 */
	includes?: NormalizedGlob[];
	/**
	 * The maximum allowed size for source code files in bytes. Files above this limit will be ignored for performance reasons. Defaults to 1 MiB
	 */
	maxSize?: MaxSize;
}
/**
 * Generic options applied to all files
 */
export interface FormatterConfiguration {
	/**
	 * The attribute position style in HTML-ish languages. Defaults to auto.
	 */
	attributePosition?: AttributePosition;
	/**
	 * Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
	 */
	bracketSameLine?: BracketSameLine;
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing?: BracketSpacing;
	enabled?: Bool;
	/**
	 * Whether to expand arrays and objects on multiple lines. When set to `auto`, object literals are formatted on multiple lines if the first property has a newline, and array literals are formatted on a single line if it fits in the line. When set to `always`, these literals are formatted on multiple lines, regardless of length of the list. When set to `never`, these literals are formatted on a single line if it fits in the line. When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto".
	 */
	expand?: Expand;
	/**
	 * Whether formatting should be allowed to proceed if a given file has syntax errors
	 */
	formatWithErrors?: Bool;
	/**
	 * A list of glob patterns. The formatter will include files/folders that will match these patterns.
	 */
	includes?: NormalizedGlob[];
	/**
	 * The indent style.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation, 2 by default
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of line ending.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	* Use any `.editorconfig` files to configure the formatter. Configuration in `biome.json` will override `.editorconfig` configuration.

Default: `true`. 
	 */
	useEditorconfig?: Bool;
}
/**
 * Options applied to GraphQL files
 */
export interface GraphqlConfiguration {
	/**
	 * Assist options
	 */
	assist?: GraphqlAssistConfiguration;
	/**
	 * GraphQL formatter options
	 */
	formatter?: GraphqlFormatterConfiguration;
	linter?: GraphqlLinterConfiguration;
}
/**
 * Options applied to GritQL files
 */
export interface GritConfiguration {
	/**
	 * Assist options
	 */
	assist?: GritAssistConfiguration;
	/**
	 * Formatting options
	 */
	formatter?: GritFormatterConfiguration;
	/**
	 * Formatting options
	 */
	linter?: GritLinterConfiguration;
}
/**
 * Options applied to HTML files
 */
export interface HtmlConfiguration {
	assist?: HtmlAssistConfiguration;
	/**
	 * Enables full support for HTML, Vue, Svelte and Astro files.
	 */
	experimentalFullSupportEnabled?: Bool;
	/**
	 * HTML formatter options
	 */
	formatter?: HtmlFormatterConfiguration;
	/**
	 * HTML linter options
	 */
	linter?: HtmlLinterConfiguration;
	/**
	 * HTML parsing options
	 */
	parser?: HtmlParserConfiguration;
}
/**
 * A set of options applied to the JavaScript files
 */
export interface JsConfiguration {
	/**
	 * Assist options
	 */
	assist?: JsAssistConfiguration;
	/**
	 * Formatting options
	 */
	formatter?: JsFormatterConfiguration;
	/**
	* A list of global bindings that should be ignored by the analyzers

If defined here, they should not emit diagnostics. 
	 */
	globals?: string[];
	/**
	 * Indicates the type of runtime or transformation used for interpreting JSX.
	 */
	jsxRuntime?: JsxRuntime;
	/**
	 * Linter options
	 */
	linter?: JsLinterConfiguration;
	/**
	 * Parsing options
	 */
	parser?: JsParserConfiguration;
}
/**
 * Options applied to JSON files
 */
export interface JsonConfiguration {
	/**
	 * Assist options
	 */
	assist?: JsonAssistConfiguration;
	/**
	 * Formatting options
	 */
	formatter?: JsonFormatterConfiguration;
	/**
	 * Linting options
	 */
	linter?: JsonLinterConfiguration;
	/**
	 * Parsing options
	 */
	parser?: JsonParserConfiguration;
}
export interface LinterConfiguration {
	/**
	 * An object where the keys are the names of the domains, and the values are `all`, `recommended`, or `none`.
	 */
	domains?: RuleDomains;
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: Bool;
	/**
	 * A list of glob patterns. The analyzer will handle only those files/folders that will match these patterns.
	 */
	includes?: NormalizedGlob[];
	/**
	 * List of rules
	 */
	rules?: Rules;
}
export type Overrides = OverridePattern[];
export type Plugins = PluginConfiguration[];
export type Bool = boolean;
/**
 * Set of properties to integrate Biome with a VCS software.
 */
export interface VcsConfiguration {
	/**
	 * The kind of client.
	 */
	clientKind?: VcsClientKind;
	/**
	 * The main branch of the project
	 */
	defaultBranch?: string;
	/**
	 * Whether Biome should integrate itself with the VCS client
	 */
	enabled?: Bool;
	/**
	* The folder where Biome should check for VCS files. By default, Biome will use the same folder where `biome.json` was found.

If Biome can't find the configuration, it will attempt to use the current working directory. If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic will be emitted 
	 */
	root?: string;
	/**
	 * Whether Biome should use the VCS ignore file. When [true], Biome will ignore the files specified in the ignore file.
	 */
	useIgnoreFile?: Bool;
}
export interface Actions {
	/**
	 * It enables the assist actions recommended by Biome. `true` by default.
	 */
	recommended?: boolean;
	source?: Source;
}
/**
 * Normalized Biome glob pattern that strips `./` from the pattern.
 */
export type NormalizedGlob = Glob;
/**
 * Options that changes how the CSS assist behaves
 */
export interface CssAssistConfiguration {
	/**
	 * Control the assist for CSS files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the CSS formatter behaves
 */
export interface CssFormatterConfiguration {
	/**
	 * Control the formatter for CSS (and its super languages) files.
	 */
	enabled?: Bool;
	/**
	 * The indent style applied to CSS (and its super languages) files.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation applied to CSS (and its super languages) files. Default to 2.
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of line ending applied to CSS (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * The type of quotes used in CSS code. Defaults to double.
	 */
	quoteStyle?: QuoteStyle;
}
/**
 * Options that changes how the CSS linter behaves
 */
export interface CssLinterConfiguration {
	/**
	 * Control the linter for CSS files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the CSS parser behaves
 */
export interface CssParserConfiguration {
	/**
	 * Allow comments to appear on incorrect lines in `.css` files
	 */
	allowWrongLineComments?: Bool;
	/**
	 * Enables parsing of CSS Modules specific features.
	 */
	cssModules?: Bool;
	/**
	 * Enables parsing of Tailwind CSS 4.0 directives and functions.
	 */
	tailwindDirectives?: Bool;
}
export type MaxSize = number;
export type AttributePosition = "auto" | "multiline";
/**
 * Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
 */
export type BracketSameLine = boolean;
export type BracketSpacing = boolean;
export type Expand = "auto" | "always" | "never";
export type IndentStyle = "tab" | "space";
export type IndentWidth = number;
export type LineEnding = "lf" | "crlf" | "cr" | "auto";
/**
	* Validated value for the `line_width` formatter options

The allowed range of values is 1..=320 
	 */
export type LineWidth = number;
/**
 * Options that changes how the GraphQL linter behaves
 */
export interface GraphqlAssistConfiguration {
	/**
	 * Control the formatter for GraphQL files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the GraphQL formatter behaves
 */
export interface GraphqlFormatterConfiguration {
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing?: BracketSpacing;
	/**
	 * Control the formatter for GraphQL files.
	 */
	enabled?: Bool;
	/**
	 * The indent style applied to GraphQL files.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation applied to GraphQL files. Default to 2.
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of line ending applied to GraphQL files. `auto` uses CRLF on Windows and LF on other platforms.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to GraphQL files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * The type of quotes used in GraphQL code. Defaults to double.
	 */
	quoteStyle?: QuoteStyle;
}
/**
 * Options that change how the GraphQL linter behaves.
 */
export interface GraphqlLinterConfiguration {
	/**
	 * Control the formatter for GraphQL files.
	 */
	enabled?: Bool;
}
export interface GritAssistConfiguration {
	/**
	 * Control the assist functionality for Grit files.
	 */
	enabled?: Bool;
}
export interface GritFormatterConfiguration {
	/**
	 * Control the formatter for Grit files.
	 */
	enabled?: Bool;
	/**
	 * The indent style applied to Grit files.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation applied to Grit files. Default to 2.
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of line ending applied to Grit files.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to Grit files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
}
export interface GritLinterConfiguration {
	/**
	 * Control the linter for Grit files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the HTML assist behaves
 */
export interface HtmlAssistConfiguration {
	/**
	 * Control the assist for HTML (and its super languages) files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the HTML formatter behaves
 */
export interface HtmlFormatterConfiguration {
	/**
	 * The attribute position style in HTML elements. Defaults to auto.
	 */
	attributePosition?: AttributePosition;
	/**
	 * Whether to hug the closing bracket of multiline HTML tags to the end of the last line, rather than being alone on the following line. Defaults to false.
	 */
	bracketSameLine?: BracketSameLine;
	/**
	 * Control the formatter for HTML (and its super languages) files.
	 */
	enabled?: Bool;
	/**
	 * Whether to indent the `<script>` and `<style>` tags for HTML (and its super languages). Defaults to false.
	 */
	indentScriptAndStyle?: IndentScriptAndStyle;
	/**
	 * The indent style applied to HTML (and its super languages) files.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation applied to HTML (and its super languages) files. Default to 2.
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of line ending applied to HTML (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to HTML (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * Whether void elements should be self-closed. Defaults to never.
	 */
	selfCloseVoidElements?: SelfCloseVoidElements;
	/**
	 * Whether to account for whitespace sensitivity when formatting HTML (and its super languages). Defaults to "css".
	 */
	whitespaceSensitivity?: WhitespaceSensitivity;
}
/**
 * Options that changes how the HTML linter behaves
 */
export interface HtmlLinterConfiguration {
	/**
	 * Control the linter for HTML (and its super languages) files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the HTML parser behaves
 */
export interface HtmlParserConfiguration {
	/**
	 * Enables the parsing of double text expressions such as `{{ expression }}` inside `.html` files
	 */
	interpolation?: Bool;
}
/**
 * Assist options specific to the JavaScript assist
 */
export interface JsAssistConfiguration {
	/**
	 * Control the assist for JavaScript (and its super languages) files.
	 */
	enabled?: Bool;
}
/**
 * Formatting options specific to the JavaScript files
 */
export interface JsFormatterConfiguration {
	/**
	 * Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
	 */
	arrowParentheses?: ArrowParentheses;
	/**
	 * The attribute position style in JSX elements. Defaults to auto.
	 */
	attributePosition?: AttributePosition;
	/**
	 * Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
	 */
	bracketSameLine?: BracketSameLine;
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing?: BracketSpacing;
	/**
	 * Control the formatter for JavaScript (and its super languages) files.
	 */
	enabled?: Bool;
	/**
	 * Whether to expand arrays and objects on multiple lines. When set to `auto`, object literals are formatted on multiple lines if the first property has a newline, and array literals are formatted on a single line if it fits in the line. When set to `always`, these literals are formatted on multiple lines, regardless of length of the list. When set to `never`, these literals are formatted on a single line if it fits in the line. When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto".
	 */
	expand?: Expand;
	/**
	 * The indent style applied to JavaScript (and its super languages) files.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of quotes used in JSX. Defaults to double.
	 */
	jsxQuoteStyle?: QuoteStyle;
	/**
	 * The type of line ending applied to JavaScript (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to JavaScript (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * When breaking binary expressions into multiple lines, whether to break them before or after the binary operator. Defaults to "after".
	 */
	operatorLinebreak?: OperatorLinebreak;
	/**
	 * When properties in objects are quoted. Defaults to asNeeded.
	 */
	quoteProperties?: QuoteProperties;
	/**
	 * The type of quotes used in JavaScript code. Defaults to double.
	 */
	quoteStyle?: QuoteStyle;
	/**
	 * Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
	 */
	semicolons?: Semicolons;
	/**
	 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
	 */
	trailingCommas?: TrailingCommas;
}
/**
 * Indicates the type of runtime or transformation used for interpreting JSX.
 */
export type JsxRuntime = "transparent" | "reactClassic";
/**
 * Linter options specific to the JavaScript linter
 */
export interface JsLinterConfiguration {
	/**
	 * Control the linter for JavaScript (and its super languages) files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the JavaScript parser behaves
 */
export interface JsParserConfiguration {
	/**
	 * Enables parsing of Grit metavariables. Defaults to `false`.
	 */
	gritMetavariables?: Bool;
	/**
	* When enabled, files like `.js`/`.mjs`/`.cjs` may contain JSX syntax.

Defaults to `true`. 
	 */
	jsxEverywhere?: Bool;
	/**
	* It enables the experimental and unsafe parsing of parameter decorators

These decorators belong to an old proposal, and they are subject to change. 
	 */
	unsafeParameterDecoratorsEnabled?: Bool;
}
/**
 * Assist options specific to the JSON linter
 */
export interface JsonAssistConfiguration {
	/**
	 * Control the assist for JSON (and its super languages) files.
	 */
	enabled?: Bool;
}
export interface JsonFormatterConfiguration {
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing?: BracketSpacing;
	/**
	 * Control the formatter for JSON (and its super languages) files.
	 */
	enabled?: Bool;
	/**
	 * Whether to expand arrays and objects on multiple lines. When set to `auto`, object literals are formatted on multiple lines if the first property has a newline, and array literals are formatted on a single line if it fits in the line. When set to `always`, these literals are formatted on multiple lines, regardless of length of the list. When set to `never`, these literals are formatted on a single line if it fits in the line. When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto".
	 */
	expand?: Expand;
	/**
	 * The indent style applied to JSON (and its super languages) files.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation applied to JSON (and its super languages) files. Default to 2.
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of line ending applied to JSON (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
	 */
	trailingCommas?: TrailingCommas2;
}
/**
 * Linter options specific to the JSON linter
 */
export interface JsonLinterConfiguration {
	/**
	 * Control the linter for JSON (and its super languages) files.
	 */
	enabled?: Bool;
}
/**
 * Options that changes how the JSON parser behaves
 */
export interface JsonParserConfiguration {
	/**
	 * Allow parsing comments in `.json` files
	 */
	allowComments?: Bool;
	/**
	 * Allow parsing trailing commas in `.json` files
	 */
	allowTrailingCommas?: Bool;
}
export type RuleDomains = { [K in RuleDomain]?: RuleDomainValue };
export interface Rules {
	a11y?: SeverityOrGroup_for_A11y;
	complexity?: SeverityOrGroup_for_Complexity;
	correctness?: SeverityOrGroup_for_Correctness;
	nursery?: SeverityOrGroup_for_Nursery;
	performance?: SeverityOrGroup_for_Performance;
	/**
	 * It enables the lint rules recommended by Biome. `true` by default.
	 */
	recommended?: boolean;
	security?: SeverityOrGroup_for_Security;
	style?: SeverityOrGroup_for_Style;
	suspicious?: SeverityOrGroup_for_Suspicious;
}
export interface OverridePattern {
	/**
	 * Specific configuration for the Json language
	 */
	assist?: OverrideAssistConfiguration;
	/**
	 * Specific configuration for the CSS language
	 */
	css?: CssConfiguration;
	/**
	 * Specific configuration for the filesystem
	 */
	files?: OverrideFilesConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	formatter?: OverrideFormatterConfiguration;
	/**
	 * Specific configuration for the Graphql language
	 */
	graphql?: GraphqlConfiguration;
	/**
	 * Specific configuration for the GritQL language
	 */
	grit?: GritConfiguration;
	/**
	 * Specific configuration for the GritQL language
	 */
	html?: HtmlConfiguration;
	/**
	 * A list of glob patterns. Biome will include files/folders that will match these patterns.
	 */
	includes?: OverrideGlobs;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript?: JsConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	json?: JsonConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	linter?: OverrideLinterConfiguration;
	/**
	 * Specific configuration for additional plugins
	 */
	plugins?: Plugins;
}
export type PluginConfiguration = string;
export type VcsClientKind = "git";
/**
 * A list of rules that belong to this group
 */
export interface Source {
	/**
	 * Provides a code action to sort the imports and exports in the file using a built-in or custom order.
	 */
	organizeImports?: RuleAssistConfiguration_for_OrganizeImportsOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce attribute sorting in JSX elements.
	 */
	useSortedAttributes?: RuleAssistConfiguration_for_UseSortedAttributesOptions;
	/**
	 * Sort the keys of a JSON object in natural order.
	 */
	useSortedKeys?: RuleAssistConfiguration_for_UseSortedKeysOptions;
	/**
	 * Enforce ordering of CSS properties and nested rules.
	 */
	useSortedProperties?: RuleAssistConfiguration_for_UseSortedPropertiesOptions;
}
export type Glob = string;
export type QuoteStyle = "double" | "single";
/**
	* Whether to indent the content of `<script>` and `<style>` tags for HTML-ish templating languages (Vue, Svelte, etc.).

When true, the content of `<script>` and `<style>` tags will be indented one level. 
	 */
export type IndentScriptAndStyle = boolean;
/**
 * Controls whether void-elements should be self closed
 */
export type SelfCloseVoidElements = "never" | "always";
/**
	* Whitespace sensitivity for HTML formatting.

The following two cases won't produce the same output:

|                |      html      |    output    | | -------------- | :------------: | :----------: | | with spaces    | `1<b> 2 </b>3` | 1<b> 2 </b>3 | | without spaces |  `1<b>2</b>3`  |  1<b>2</b>3  |

This happens because whitespace is significant in inline elements.

As a consequence of this, the formatter must format blocks that look like this (assume a small line width, <20): ```html <span>really long content</span> ``` as this, where the content hugs the tags: ```html <span >really long content</span > ```

Note that this is only necessary for inline elements. Block elements do not have this restriction. 
	 */
export type WhitespaceSensitivity = "css" | "strict" | "ignore";
export type ArrowParentheses = "always" | "asNeeded";
export type OperatorLinebreak = "after" | "before";
export type QuoteProperties = "asNeeded" | "preserve";
export type Semicolons = "always" | "asNeeded";
/**
 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures.
 */
export type TrailingCommas = "all" | "es5" | "none";
export type TrailingCommas2 = "none" | "all";
/**
 * Rule domains
 */
export type RuleDomain =
	| "react"
	| "test"
	| "solid"
	| "next"
	| "qwik"
	| "vue"
	| "project"
	| "tailwind";
export type RuleDomainValue = "all" | "none" | "recommended";
export type SeverityOrGroup_for_A11y = GroupPlainConfiguration | A11y;
export type SeverityOrGroup_for_Complexity =
	| GroupPlainConfiguration
	| Complexity;
export type SeverityOrGroup_for_Correctness =
	| GroupPlainConfiguration
	| Correctness;
export type SeverityOrGroup_for_Nursery = GroupPlainConfiguration | Nursery;
export type SeverityOrGroup_for_Performance =
	| GroupPlainConfiguration
	| Performance;
export type SeverityOrGroup_for_Security = GroupPlainConfiguration | Security;
export type SeverityOrGroup_for_Style = GroupPlainConfiguration | Style;
export type SeverityOrGroup_for_Suspicious =
	| GroupPlainConfiguration
	| Suspicious;
export interface OverrideAssistConfiguration {
	/**
	 * List of actions
	 */
	actions?: Actions;
	/**
	 * if `false`, it disables the feature and the assist won't be executed. `true` by default
	 */
	enabled?: Bool;
}
export interface OverrideFilesConfiguration {
	/**
	 * File size limit in bytes
	 */
	maxSize?: MaxSize;
}
export interface OverrideFormatterConfiguration {
	/**
	 * The attribute position style.
	 */
	attributePosition?: AttributePosition;
	/**
	 * Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
	 */
	bracketSameLine?: BracketSameLine;
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing?: BracketSpacing;
	enabled?: Bool;
	/**
	 * Whether to expand arrays and objects on multiple lines. When set to `auto`, object literals are formatted on multiple lines if the first property has a newline, and array literals are formatted on a single line if it fits in the line. When set to `always`, these literals are formatted on multiple lines, regardless of length of the list. When set to `never`, these literals are formatted on a single line if it fits in the line. When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto".
	 */
	expand?: Expand;
	/**
	 * Stores whether formatting should be allowed to proceed if a given file has syntax errors
	 */
	formatWithErrors?: Bool;
	/**
	 * The size of the indentation, 2 by default (deprecated, use `indent-width`)
	 */
	indentSize?: IndentWidth;
	/**
	 * The indent style.
	 */
	indentStyle?: IndentStyle;
	/**
	 * The size of the indentation, 2 by default
	 */
	indentWidth?: IndentWidth;
	/**
	 * The type of line ending.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth?: LineWidth;
}
export type OverrideGlobs = Glob[];
export interface OverrideLinterConfiguration {
	/**
	 * List of rules
	 */
	domains?: RuleDomains;
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: Bool;
	/**
	 * List of rules
	 */
	rules?: Rules;
}
export type RuleAssistConfiguration_for_OrganizeImportsOptions =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOptions_for_OrganizeImportsOptions;
export type RuleAssistConfiguration_for_UseSortedAttributesOptions =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOptions_for_UseSortedAttributesOptions;
export type RuleAssistConfiguration_for_UseSortedKeysOptions =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOptions_for_UseSortedKeysOptions;
export type RuleAssistConfiguration_for_UseSortedPropertiesOptions =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOptions_for_UseSortedPropertiesOptions;
export type GroupPlainConfiguration = "off" | "on" | "info" | "warn" | "error";
/**
 * A list of rules that belong to this group
 */
export interface A11y {
	/**
	 * Enforce that the accessKey attribute is not used on any HTML element.
	 */
	noAccessKey?: RuleFixConfiguration_for_NoAccessKeyOptions;
	/**
	 * Enforce that aria-hidden="true" is not set on focusable elements.
	 */
	noAriaHiddenOnFocusable?: RuleFixConfiguration_for_NoAriaHiddenOnFocusableOptions;
	/**
	 * Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
	 */
	noAriaUnsupportedElements?: RuleFixConfiguration_for_NoAriaUnsupportedElementsOptions;
	/**
	 * Enforce that autoFocus prop is not used on elements.
	 */
	noAutofocus?: RuleFixConfiguration_for_NoAutofocusOptions;
	/**
	 * Enforces that no distracting elements are used.
	 */
	noDistractingElements?: RuleFixConfiguration_for_NoDistractingElementsOptions;
	/**
	 * The scope prop should be used only on \<th> elements.
	 */
	noHeaderScope?: RuleFixConfiguration_for_NoHeaderScopeOptions;
	/**
	 * Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
	 */
	noInteractiveElementToNoninteractiveRole?: RuleFixConfiguration_for_NoInteractiveElementToNoninteractiveRoleOptions;
	/**
	 * Enforce that a label element or component has a text label and an associated input.
	 */
	noLabelWithoutControl?: RuleConfiguration_for_NoLabelWithoutControlOptions;
	/**
	 * Disallow use event handlers on non-interactive elements.
	 */
	noNoninteractiveElementInteractions?: RuleConfiguration_for_NoNoninteractiveElementInteractionsOptions;
	/**
	 * Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveElementToInteractiveRole?: RuleFixConfiguration_for_NoNoninteractiveElementToInteractiveRoleOptions;
	/**
	 * Enforce that tabIndex is not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveTabindex?: RuleFixConfiguration_for_NoNoninteractiveTabindexOptions;
	/**
	 * Prevent the usage of positive integers on tabIndex property
	 */
	noPositiveTabindex?: RuleFixConfiguration_for_NoPositiveTabindexOptions;
	/**
	 * Enforce img alt prop does not contain the word "image", "picture", or "photo".
	 */
	noRedundantAlt?: RuleConfiguration_for_NoRedundantAltOptions;
	/**
	 * Enforce explicit role property is not the same as implicit/default role property on an element.
	 */
	noRedundantRoles?: RuleFixConfiguration_for_NoRedundantRolesOptions;
	/**
	 * Enforce that static, visible elements (such as \<div>) that have click handlers use the valid role attribute.
	 */
	noStaticElementInteractions?: RuleConfiguration_for_NoStaticElementInteractionsOptions;
	/**
	 * Enforces the usage of the title element for the svg element.
	 */
	noSvgWithoutTitle?: RuleConfiguration_for_NoSvgWithoutTitleOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce that all elements that require alternative text have meaningful information to relay back to the end user.
	 */
	useAltText?: RuleConfiguration_for_UseAltTextOptions;
	/**
	 * Enforce that anchors have content and that the content is accessible to screen readers.
	 */
	useAnchorContent?: RuleFixConfiguration_for_UseAnchorContentOptions;
	/**
	 * Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant.
	 */
	useAriaActivedescendantWithTabindex?: RuleFixConfiguration_for_UseAriaActivedescendantWithTabindexOptions;
	/**
	 * Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
	 */
	useAriaPropsForRole?: RuleConfiguration_for_UseAriaPropsForRoleOptions;
	/**
	 * Enforce that ARIA properties are valid for the roles that are supported by the element.
	 */
	useAriaPropsSupportedByRole?: RuleConfiguration_for_UseAriaPropsSupportedByRoleOptions;
	/**
	 * Enforces the usage of the attribute type for the element button
	 */
	useButtonType?: RuleConfiguration_for_UseButtonTypeOptions;
	/**
	 * Elements with an interactive role and interaction handlers must be focusable.
	 */
	useFocusableInteractive?: RuleConfiguration_for_UseFocusableInteractiveOptions;
	/**
	 * Disallow a missing generic family keyword within font families.
	 */
	useGenericFontNames?: RuleConfiguration_for_UseGenericFontNamesOptions;
	/**
	 * Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.
	 */
	useHeadingContent?: RuleConfiguration_for_UseHeadingContentOptions;
	/**
	 * Enforce that html element has lang attribute.
	 */
	useHtmlLang?: RuleConfiguration_for_UseHtmlLangOptions;
	/**
	 * Enforces the usage of the attribute title for the element iframe.
	 */
	useIframeTitle?: RuleConfiguration_for_UseIframeTitleOptions;
	/**
	 * Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress.
	 */
	useKeyWithClickEvents?: RuleConfiguration_for_UseKeyWithClickEventsOptions;
	/**
	 * Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur.
	 */
	useKeyWithMouseEvents?: RuleConfiguration_for_UseKeyWithMouseEventsOptions;
	/**
	 * Enforces that audio and video elements must have a track for captions.
	 */
	useMediaCaption?: RuleConfiguration_for_UseMediaCaptionOptions;
	/**
	 * It detects the use of role attributes in JSX elements and suggests using semantic elements instead.
	 */
	useSemanticElements?: RuleConfiguration_for_UseSemanticElementsOptions;
	/**
	 * Enforce that all anchors are valid, and they are navigable elements.
	 */
	useValidAnchor?: RuleConfiguration_for_UseValidAnchorOptions;
	/**
	 * Ensures that ARIA properties aria-* are all valid.
	 */
	useValidAriaProps?: RuleFixConfiguration_for_UseValidAriaPropsOptions;
	/**
	 * Elements with ARIA roles must use a valid, non-abstract ARIA role.
	 */
	useValidAriaRole?: RuleFixConfiguration_for_UseValidAriaRoleOptions;
	/**
	 * Enforce that ARIA state and property values are valid.
	 */
	useValidAriaValues?: RuleConfiguration_for_UseValidAriaValuesOptions;
	/**
	 * Use valid values for the autocomplete attribute on input elements.
	 */
	useValidAutocomplete?: RuleConfiguration_for_UseValidAutocompleteOptions;
	/**
	 * Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country.
	 */
	useValidLang?: RuleConfiguration_for_UseValidLangOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Complexity {
	/**
	 * Disallow unclear usage of consecutive space characters in regular expression literals
	 */
	noAdjacentSpacesInRegex?: RuleFixConfiguration_for_NoAdjacentSpacesInRegexOptions;
	/**
	 * Disallow the use of arguments.
	 */
	noArguments?: RuleConfiguration_for_NoArgumentsOptions;
	/**
	 * Disallow primitive type aliases and misleading types.
	 */
	noBannedTypes?: RuleFixConfiguration_for_NoBannedTypesOptions;
	/**
	 * Disallow comma operator.
	 */
	noCommaOperator?: RuleConfiguration_for_NoCommaOperatorOptions;
	/**
	 * Disallow empty type parameters in type aliases and interfaces.
	 */
	noEmptyTypeParameters?: RuleConfiguration_for_NoEmptyTypeParametersOptions;
	/**
	 * Disallow functions that exceed a given Cognitive Complexity score.
	 */
	noExcessiveCognitiveComplexity?: RuleConfiguration_for_NoExcessiveCognitiveComplexityOptions;
	/**
	 * Restrict the number of lines of code in a function.
	 */
	noExcessiveLinesPerFunction?: RuleConfiguration_for_NoExcessiveLinesPerFunctionOptions;
	/**
	 * This rule enforces a maximum depth to nested describe() in test files.
	 */
	noExcessiveNestedTestSuites?: RuleConfiguration_for_NoExcessiveNestedTestSuitesOptions;
	/**
	 * Disallow unnecessary boolean casts
	 */
	noExtraBooleanCast?: RuleFixConfiguration_for_NoExtraBooleanCastOptions;
	/**
	 * Disallow to use unnecessary callback on flatMap.
	 */
	noFlatMapIdentity?: RuleFixConfiguration_for_NoFlatMapIdentityOptions;
	/**
	 * Prefer for...of statement instead of Array.forEach.
	 */
	noForEach?: RuleConfiguration_for_NoForEachOptions;
	/**
	 * Disallow shorthand type conversions.
	 */
	noImplicitCoercions?: RuleFixConfiguration_for_NoImplicitCoercionsOptions;
	/**
	 * Disallow the use of the !important style.
	 */
	noImportantStyles?: RuleFixConfiguration_for_NoImportantStylesOptions;
	/**
	 * This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
	 */
	noStaticOnlyClass?: RuleConfiguration_for_NoStaticOnlyClassOptions;
	/**
	 * Disallow this and super in static contexts.
	 */
	noThisInStatic?: RuleFixConfiguration_for_NoThisInStaticOptions;
	/**
	 * Disallow unnecessary catch clauses.
	 */
	noUselessCatch?: RuleFixConfiguration_for_NoUselessCatchOptions;
	/**
	 * Disallow unnecessary constructors.
	 */
	noUselessConstructor?: RuleFixConfiguration_for_NoUselessConstructorOptions;
	/**
	 * Avoid using unnecessary continue.
	 */
	noUselessContinue?: RuleFixConfiguration_for_NoUselessContinueOptions;
	/**
	 * Disallow empty exports that don't change anything in a module file.
	 */
	noUselessEmptyExport?: RuleFixConfiguration_for_NoUselessEmptyExportOptions;
	/**
	 * Disallow unnecessary escape sequence in regular expression literals.
	 */
	noUselessEscapeInRegex?: RuleFixConfiguration_for_NoUselessEscapeInRegexOptions;
	/**
	 * Disallow unnecessary fragments
	 */
	noUselessFragments?: RuleFixConfiguration_for_NoUselessFragmentsOptions;
	/**
	 * Disallow unnecessary labels.
	 */
	noUselessLabel?: RuleFixConfiguration_for_NoUselessLabelOptions;
	/**
	 * Disallow unnecessary nested block statements.
	 */
	noUselessLoneBlockStatements?: RuleFixConfiguration_for_NoUselessLoneBlockStatementsOptions;
	/**
	 * Disallow renaming import, export, and destructured assignments to the same name.
	 */
	noUselessRename?: RuleFixConfiguration_for_NoUselessRenameOptions;
	/**
	 * Disallow unnecessary concatenation of string or template literals.
	 */
	noUselessStringConcat?: RuleFixConfiguration_for_NoUselessStringConcatOptions;
	/**
	 * Disallow unnecessary String.raw function in template string literals without any escape sequence.
	 */
	noUselessStringRaw?: RuleConfiguration_for_NoUselessStringRawOptions;
	/**
	 * Disallow useless case in switch statements.
	 */
	noUselessSwitchCase?: RuleFixConfiguration_for_NoUselessSwitchCaseOptions;
	/**
	 * Disallow ternary operators when simpler alternatives exist.
	 */
	noUselessTernary?: RuleFixConfiguration_for_NoUselessTernaryOptions;
	/**
	 * Disallow useless this aliasing.
	 */
	noUselessThisAlias?: RuleFixConfiguration_for_NoUselessThisAliasOptions;
	/**
	 * Disallow using any or unknown as type constraint.
	 */
	noUselessTypeConstraint?: RuleFixConfiguration_for_NoUselessTypeConstraintOptions;
	/**
	 * Disallow initializing variables to undefined.
	 */
	noUselessUndefinedInitialization?: RuleFixConfiguration_for_NoUselessUndefinedInitializationOptions;
	/**
	 * Disallow the use of void operators, which is not a familiar operator.
	 */
	noVoid?: RuleConfiguration_for_NoVoidOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Use arrow functions over function expressions.
	 */
	useArrowFunction?: RuleFixConfiguration_for_UseArrowFunctionOptions;
	/**
	 * Use Date.now() to get the number of milliseconds since the Unix Epoch.
	 */
	useDateNow?: RuleFixConfiguration_for_UseDateNowOptions;
	/**
	 * Promotes the use of .flatMap() when map().flat() are used together.
	 */
	useFlatMap?: RuleFixConfiguration_for_UseFlatMapOptions;
	/**
	 * Prefer Array#{indexOf,lastIndexOf}() over Array#{findIndex,findLastIndex}() when looking for the index of an item.
	 */
	useIndexOf?: RuleFixConfiguration_for_UseIndexOfOptions;
	/**
	 * Enforce the usage of a literal access to properties over computed property access.
	 */
	useLiteralKeys?: RuleFixConfiguration_for_UseLiteralKeysOptions;
	/**
	 * Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals
	 */
	useNumericLiterals?: RuleFixConfiguration_for_UseNumericLiteralsOptions;
	/**
	 * Enforce using concise optional chain instead of chained logical expressions.
	 */
	useOptionalChain?: RuleFixConfiguration_for_UseOptionalChainOptions;
	/**
	 * Enforce the use of the regular expression literals instead of the RegExp constructor if possible.
	 */
	useRegexLiterals?: RuleFixConfiguration_for_UseRegexLiteralsOptions;
	/**
	 * Disallow number literal object member names which are not base 10 or use underscore as separator.
	 */
	useSimpleNumberKeys?: RuleFixConfiguration_for_UseSimpleNumberKeysOptions;
	/**
	 * Discard redundant terms from logical expressions.
	 */
	useSimplifiedLogicExpression?: RuleFixConfiguration_for_UseSimplifiedLogicExpressionOptions;
	/**
	 * Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed.
	 */
	useWhile?: RuleFixConfiguration_for_UseWhileOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	/**
	 * Prevent passing of children as props.
	 */
	noChildrenProp?: RuleConfiguration_for_NoChildrenPropOptions;
	/**
	 * Prevents from having const variables being re-assigned.
	 */
	noConstAssign?: RuleFixConfiguration_for_NoConstAssignOptions;
	/**
	 * Disallow constant expressions in conditions
	 */
	noConstantCondition?: RuleConfiguration_for_NoConstantConditionOptions;
	/**
	 * Disallow the use of Math.min and Math.max to clamp a value where the result itself is constant.
	 */
	noConstantMathMinMaxClamp?: RuleFixConfiguration_for_NoConstantMathMinMaxClampOptions;
	/**
	 * Disallow returning a value from a constructor.
	 */
	noConstructorReturn?: RuleConfiguration_for_NoConstructorReturnOptions;
	/**
	 * Disallow empty character classes in regular expression literals.
	 */
	noEmptyCharacterClassInRegex?: RuleConfiguration_for_NoEmptyCharacterClassInRegexOptions;
	/**
	 * Disallows empty destructuring patterns.
	 */
	noEmptyPattern?: RuleConfiguration_for_NoEmptyPatternOptions;
	/**
	 * Disallow the use of __dirname and __filename in the global scope.
	 */
	noGlobalDirnameFilename?: RuleFixConfiguration_for_NoGlobalDirnameFilenameOptions;
	/**
	 * Disallow calling global object properties as functions
	 */
	noGlobalObjectCalls?: RuleConfiguration_for_NoGlobalObjectCallsOptions;
	/**
	 * Disallow function and var declarations that are accessible outside their block.
	 */
	noInnerDeclarations?: RuleConfiguration_for_NoInnerDeclarationsOptions;
	/**
	 * Ensure that builtins are correctly instantiated.
	 */
	noInvalidBuiltinInstantiation?: RuleFixConfiguration_for_NoInvalidBuiltinInstantiationOptions;
	/**
	 * Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors.
	 */
	noInvalidConstructorSuper?: RuleConfiguration_for_NoInvalidConstructorSuperOptions;
	/**
	 * Disallow non-standard direction values for linear gradient functions.
	 */
	noInvalidDirectionInLinearGradient?: RuleConfiguration_for_NoInvalidDirectionInLinearGradientOptions;
	/**
	 * Disallows invalid named grid areas in CSS Grid Layouts.
	 */
	noInvalidGridAreas?: RuleConfiguration_for_NoInvalidGridAreasOptions;
	/**
	 * Disallow the use of @import at-rules in invalid positions.
	 */
	noInvalidPositionAtImportRule?: RuleConfiguration_for_NoInvalidPositionAtImportRuleOptions;
	/**
	 * Disallow the use of variables and function parameters before their declaration
	 */
	noInvalidUseBeforeDeclaration?: RuleConfiguration_for_NoInvalidUseBeforeDeclarationOptions;
	/**
	 * Disallow missing var function for css variables.
	 */
	noMissingVarFunction?: RuleConfiguration_for_NoMissingVarFunctionOptions;
	/**
	 * Disallows defining React components inside other components.
	 */
	noNestedComponentDefinitions?: RuleConfiguration_for_NoNestedComponentDefinitionsOptions;
	/**
	 * Forbid the use of Node.js builtin modules.
	 */
	noNodejsModules?: RuleConfiguration_for_NoNodejsModulesOptions;
	/**
	 * Disallow \8 and \9 escape sequences in string literals.
	 */
	noNonoctalDecimalEscape?: RuleFixConfiguration_for_NoNonoctalDecimalEscapeOptions;
	/**
	 * Disallow literal numbers that lose precision
	 */
	noPrecisionLoss?: RuleConfiguration_for_NoPrecisionLossOptions;
	/**
	 * Restrict imports of private exports.
	 */
	noPrivateImports?: RuleConfiguration_for_NoPrivateImportsOptions;
	/**
	 * Disallow the use of process global.
	 */
	noProcessGlobal?: RuleFixConfiguration_for_NoProcessGlobalOptions;
	/**
	 * Disallow useVisibleTask$() functions in Qwik components.
	 */
	noQwikUseVisibleTask?: RuleConfiguration_for_NoQwikUseVisibleTaskOptions;
	/**
	 * Disallow assigning to React component props.
	 */
	noReactPropAssignments?: RuleConfiguration_for_NoReactPropAssignmentsOptions;
	/**
	 * Prevent the usage of the return value of React.render.
	 */
	noRenderReturnValue?: RuleConfiguration_for_NoRenderReturnValueOptions;
	/**
	 * Disallow the use of configured elements.
	 */
	noRestrictedElements?: RuleConfiguration_for_NoRestrictedElementsOptions;
	/**
	 * Disallow assignments where both sides are exactly the same.
	 */
	noSelfAssign?: RuleConfiguration_for_NoSelfAssignOptions;
	/**
	 * Disallow returning a value from a setter
	 */
	noSetterReturn?: RuleConfiguration_for_NoSetterReturnOptions;
	/**
	 * Disallow destructuring props inside JSX components in Solid projects.
	 */
	noSolidDestructuredProps?: RuleConfiguration_for_NoSolidDestructuredPropsOptions;
	/**
	 * Disallow comparison of expressions modifying the string case with non-compliant value.
	 */
	noStringCaseMismatch?: RuleFixConfiguration_for_NoStringCaseMismatchOptions;
	/**
	 * Disallow lexical declarations in switch clauses.
	 */
	noSwitchDeclarations?: RuleFixConfiguration_for_NoSwitchDeclarationsOptions;
	/**
	 * Disallow the use of dependencies that aren't specified in the package.json.
	 */
	noUndeclaredDependencies?: RuleConfiguration_for_NoUndeclaredDependenciesOptions;
	/**
	 * Prevents the usage of variables that haven't been declared inside the document.
	 */
	noUndeclaredVariables?: RuleConfiguration_for_NoUndeclaredVariablesOptions;
	/**
	 * Disallow unknown CSS value functions.
	 */
	noUnknownFunction?: RuleConfiguration_for_NoUnknownFunctionOptions;
	/**
	 * Disallow unknown media feature names.
	 */
	noUnknownMediaFeatureName?: RuleConfiguration_for_NoUnknownMediaFeatureNameOptions;
	/**
	 * Disallow unknown properties.
	 */
	noUnknownProperty?: RuleConfiguration_for_NoUnknownPropertyOptions;
	/**
	 * Disallow unknown pseudo-class selectors.
	 */
	noUnknownPseudoClass?: RuleConfiguration_for_NoUnknownPseudoClassOptions;
	/**
	 * Disallow unknown pseudo-element selectors.
	 */
	noUnknownPseudoElement?: RuleConfiguration_for_NoUnknownPseudoElementOptions;
	/**
	 * Disallow unknown type selectors.
	 */
	noUnknownTypeSelector?: RuleConfiguration_for_NoUnknownTypeSelectorOptions;
	/**
	 * Disallow unknown CSS units.
	 */
	noUnknownUnit?: RuleConfiguration_for_NoUnknownUnitOptions;
	/**
	 * Disallow unmatchable An+B selectors.
	 */
	noUnmatchableAnbSelector?: RuleConfiguration_for_NoUnmatchableAnbSelectorOptions;
	/**
	 * Disallow unreachable code
	 */
	noUnreachable?: RuleConfiguration_for_NoUnreachableOptions;
	/**
	 * Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass
	 */
	noUnreachableSuper?: RuleConfiguration_for_NoUnreachableSuperOptions;
	/**
	 * Disallow control flow statements in finally blocks.
	 */
	noUnsafeFinally?: RuleConfiguration_for_NoUnsafeFinallyOptions;
	/**
	 * Disallow the use of optional chaining in contexts where the undefined value is not allowed.
	 */
	noUnsafeOptionalChaining?: RuleConfiguration_for_NoUnsafeOptionalChainingOptions;
	/**
	 * Disallow unused function parameters.
	 */
	noUnusedFunctionParameters?: RuleFixConfiguration_for_NoUnusedFunctionParametersOptions;
	/**
	 * Disallow unused imports.
	 */
	noUnusedImports?: RuleFixConfiguration_for_NoUnusedImportsOptions;
	/**
	 * Disallow unused labels.
	 */
	noUnusedLabels?: RuleFixConfiguration_for_NoUnusedLabelsOptions;
	/**
	 * Disallow unused private class members
	 */
	noUnusedPrivateClassMembers?: RuleFixConfiguration_for_NoUnusedPrivateClassMembersOptions;
	/**
	 * Disallow unused variables.
	 */
	noUnusedVariables?: RuleFixConfiguration_for_NoUnusedVariablesOptions;
	/**
	 * This rules prevents void elements (AKA self-closing elements) from having children.
	 */
	noVoidElementsWithChildren?: RuleFixConfiguration_for_NoVoidElementsWithChildrenOptions;
	/**
	 * Disallow returning a value from a function with the return type 'void'
	 */
	noVoidTypeReturn?: RuleConfiguration_for_NoVoidTypeReturnOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce all dependencies are correctly specified in a React hook.
	 */
	useExhaustiveDependencies?: RuleFixConfiguration_for_UseExhaustiveDependenciesOptions;
	/**
	 * Enforce specifying the name of GraphQL operations.
	 */
	useGraphqlNamedOperations?: RuleFixConfiguration_for_UseGraphqlNamedOperationsOptions;
	/**
	 * Enforce that all React hooks are being called from the Top Level component functions.
	 */
	useHookAtTopLevel?: RuleConfiguration_for_UseHookAtTopLevelOptions;
	/**
	 * Enforces that \<img> elements have both width and height attributes.
	 */
	useImageSize?: RuleConfiguration_for_UseImageSizeOptions;
	/**
	 * Enforce file extensions for relative imports.
	 */
	useImportExtensions?: RuleFixConfiguration_for_UseImportExtensionsOptions;
	/**
	 * Require calls to isNaN() when checking for NaN.
	 */
	useIsNan?: RuleFixConfiguration_for_UseIsNanOptions;
	/**
	 * Enforces the use of with { type: "json" } for JSON module imports.
	 */
	useJsonImportAttributes?: RuleFixConfiguration_for_UseJsonImportAttributesOptions;
	/**
	 * Disallow missing key props in iterators/collection literals.
	 */
	useJsxKeyInIterable?: RuleConfiguration_for_UseJsxKeyInIterableOptions;
	/**
	 * Enforce the consistent use of the radix argument when using parseInt().
	 */
	useParseIntRadix?: RuleFixConfiguration_for_UseParseIntRadixOptions;
	/**
	 * Prefer using the class prop as a classlist over the classnames helper.
	 */
	useQwikClasslist?: RuleConfiguration_for_UseQwikClasslistOptions;
	/**
	 * Enforce JSDoc comment lines to start with a single asterisk, except for the first one.
	 */
	useSingleJsDocAsterisk?: RuleFixConfiguration_for_UseSingleJsDocAsteriskOptions;
	/**
	 * Prevent the usage of static string literal id attribute on elements.
	 */
	useUniqueElementIds?: RuleConfiguration_for_UseUniqueElementIdsOptions;
	/**
	 * Enforce "for" loop update clause moving the counter in the right direction.
	 */
	useValidForDirection?: RuleConfiguration_for_UseValidForDirectionOptions;
	/**
	 * This rule checks that the result of a typeof expression is compared to a valid value.
	 */
	useValidTypeof?: RuleFixConfiguration_for_UseValidTypeofOptions;
	/**
	 * Require generator functions to contain yield.
	 */
	useYield?: RuleConfiguration_for_UseYieldOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Nursery {
	/**
	 * Restrict imports of deprecated exports.
	 */
	noDeprecatedImports?: RuleConfiguration_for_NoDeprecatedImportsOptions;
	/**
	 * Prevent the listing of duplicate dependencies. The rule supports the following dependency groups: "bundledDependencies", "bundleDependencies", "dependencies", "devDependencies", "overrides", "optionalDependencies", and "peerDependencies".
	 */
	noDuplicateDependencies?: RuleConfiguration_for_NoDuplicateDependenciesOptions;
	/**
	 * Disallow empty sources.
	 */
	noEmptySource?: RuleConfiguration_for_NoEmptySourceOptions;
	/**
	 * Require Promise-like statements to be handled appropriately.
	 */
	noFloatingPromises?: RuleFixConfiguration_for_NoFloatingPromisesOptions;
	/**
	 * Prevent import cycles.
	 */
	noImportCycles?: RuleConfiguration_for_NoImportCyclesOptions;
	/**
	 * Disallow string literals inside JSX elements.
	 */
	noJsxLiterals?: RuleConfiguration_for_NoJsxLiteralsOptions;
	/**
	 * Disallow Promises to be used in places where they are almost certainly a mistake.
	 */
	noMisusedPromises?: RuleFixConfiguration_for_NoMisusedPromisesOptions;
	/**
	 * Prevent client components from being async functions.
	 */
	noNextAsyncClientComponent?: RuleConfiguration_for_NoNextAsyncClientComponentOptions;
	/**
	 * Replaces usages of forwardRef with passing ref as a prop.
	 */
	noReactForwardRef?: RuleFixConfiguration_for_NoReactForwardRefOptions;
	/**
	 * Disallow variable declarations from shadowing variables declared in the outer scope.
	 */
	noShadow?: RuleConfiguration_for_NoShadowOptions;
	/**
	 * Disallow unnecessary type-based conditions that can be statically determined as redundant.
	 */
	noUnnecessaryConditions?: RuleConfiguration_for_NoUnnecessaryConditionsOptions;
	/**
	 * Warn when importing non-existing exports.
	 */
	noUnresolvedImports?: RuleConfiguration_for_NoUnresolvedImportsOptions;
	/**
	 * Disallow expression statements that are neither a function call nor an assignment.
	 */
	noUnusedExpressions?: RuleConfiguration_for_NoUnusedExpressionsOptions;
	/**
	 * Disallow unused catch bindings.
	 */
	noUselessCatchBinding?: RuleFixConfiguration_for_NoUselessCatchBindingOptions;
	/**
	 * Disallow the use of useless undefined.
	 */
	noUselessUndefined?: RuleFixConfiguration_for_NoUselessUndefinedOptions;
	/**
	 * Enforce that Vue component data options are declared as functions.
	 */
	noVueDataObjectDeclaration?: RuleFixConfiguration_for_NoVueDataObjectDeclarationOptions;
	/**
	 * Disallow duplicate keys in Vue component data, methods, computed properties, and other options.
	 */
	noVueDuplicateKeys?: RuleConfiguration_for_NoVueDuplicateKeysOptions;
	/**
	 * Disallow reserved keys in Vue component data and computed properties.
	 */
	noVueReservedKeys?: RuleConfiguration_for_NoVueReservedKeysOptions;
	/**
	 * Disallow reserved names to be used as props.
	 */
	noVueReservedProps?: RuleConfiguration_for_NoVueReservedPropsOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce consistent arrow function bodies.
	 */
	useConsistentArrowReturn?: RuleFixConfiguration_for_UseConsistentArrowReturnOptions;
	/**
	 * Require the @deprecated directive to specify a deletion date.
	 */
	useDeprecatedDate?: RuleConfiguration_for_UseDeprecatedDateOptions;
	/**
	 * Require switch-case statements to be exhaustive.
	 */
	useExhaustiveSwitchCases?: RuleFixConfiguration_for_UseExhaustiveSwitchCasesOptions;
	/**
	 * Enforce types in functions, methods, variables, and parameters.
	 */
	useExplicitType?: RuleConfiguration_for_UseExplicitTypeOptions;
	/**
	 * Enforce a maximum number of parameters in function definitions.
	 */
	useMaxParams?: RuleConfiguration_for_UseMaxParamsOptions;
	/**
	 * Disallow use* hooks outside of component$ or other use* hooks in Qwik applications.
	 */
	useQwikMethodUsage?: RuleConfiguration_for_UseQwikMethodUsageOptions;
	/**
	 * Disallow unserializable expressions in Qwik dollar ($) scopes.
	 */
	useQwikValidLexicalScope?: RuleConfiguration_for_UseQwikValidLexicalScopeOptions;
	/**
	 * Enforce the sorting of CSS utility classes.
	 */
	useSortedClasses?: RuleFixConfiguration_for_UseSortedClassesOptions;
	/**
	 * Enforce specific order of Vue compiler macros.
	 */
	useVueDefineMacrosOrder?: RuleFixConfiguration_for_UseVueDefineMacrosOrderOptions;
	/**
	 * Enforce multi-word component names in Vue components.
	 */
	useVueMultiWordComponentNames?: RuleConfiguration_for_UseVueMultiWordComponentNamesOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Performance {
	/**
	 * Disallow the use of spread (...) syntax on accumulators.
	 */
	noAccumulatingSpread?: RuleConfiguration_for_NoAccumulatingSpreadOptions;
	/**
	 * Disallow await inside loops.
	 */
	noAwaitInLoops?: RuleConfiguration_for_NoAwaitInLoopsOptions;
	/**
	 * Disallow the use of barrel file.
	 */
	noBarrelFile?: RuleConfiguration_for_NoBarrelFileOptions;
	/**
	 * Disallow the use of the delete operator.
	 */
	noDelete?: RuleFixConfiguration_for_NoDeleteOptions;
	/**
	 * Disallow accessing namespace imports dynamically.
	 */
	noDynamicNamespaceImportAccess?: RuleConfiguration_for_NoDynamicNamespaceImportAccessOptions;
	/**
	 * Prevent usage of \<img> element in a Next.js project.
	 */
	noImgElement?: RuleConfiguration_for_NoImgElementOptions;
	/**
	 * Disallow the use of namespace imports.
	 */
	noNamespaceImport?: RuleConfiguration_for_NoNamespaceImportOptions;
	/**
	 * Avoid re-export all.
	 */
	noReExportAll?: RuleConfiguration_for_NoReExportAllOptions;
	/**
	 * Prevent duplicate polyfills from Polyfill.io.
	 */
	noUnwantedPolyfillio?: RuleConfiguration_for_NoUnwantedPolyfillioOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Ensure the preconnect attribute is used when using Google Fonts.
	 */
	useGoogleFontPreconnect?: RuleFixConfiguration_for_UseGoogleFontPreconnectOptions;
	/**
	 * Enforce using Solid's \<For /> component for mapping an array to JSX elements.
	 */
	useSolidForComponent?: RuleConfiguration_for_UseSolidForComponentOptions;
	/**
	 * Require regex literals to be declared at the top level.
	 */
	useTopLevelRegex?: RuleConfiguration_for_UseTopLevelRegexOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Security {
	/**
	 * Disallow target="_blank" attribute without rel="noopener".
	 */
	noBlankTarget?: RuleFixConfiguration_for_NoBlankTargetOptions;
	/**
	 * Prevent the usage of dangerous JSX props
	 */
	noDangerouslySetInnerHtml?: RuleConfiguration_for_NoDangerouslySetInnerHtmlOptions;
	/**
	 * Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop.
	 */
	noDangerouslySetInnerHtmlWithChildren?: RuleConfiguration_for_NoDangerouslySetInnerHtmlWithChildrenOptions;
	/**
	 * Disallow the use of global eval().
	 */
	noGlobalEval?: RuleConfiguration_for_NoGlobalEvalOptions;
	/**
	 * Disallow usage of sensitive data such as API keys and tokens.
	 */
	noSecrets?: RuleConfiguration_for_NoSecretsOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
}
/**
 * A list of rules that belong to this group
 */
export interface Style {
	/**
	 * Disallow use of CommonJs module system in favor of ESM style imports.
	 */
	noCommonJs?: RuleConfiguration_for_NoCommonJsOptions;
	/**
	 * Disallow default exports.
	 */
	noDefaultExport?: RuleConfiguration_for_NoDefaultExportOptions;
	/**
	 * Disallow a lower specificity selector from coming after a higher specificity selector.
	 */
	noDescendingSpecificity?: RuleConfiguration_for_NoDescendingSpecificityOptions;
	/**
	 * Disallow using a callback in asynchronous tests and hooks.
	 */
	noDoneCallback?: RuleConfiguration_for_NoDoneCallbackOptions;
	/**
	 * Disallow TypeScript enum.
	 */
	noEnum?: RuleConfiguration_for_NoEnumOptions;
	/**
	 * Disallow exporting an imported variable.
	 */
	noExportedImports?: RuleConfiguration_for_NoExportedImportsOptions;
	/**
	 * Prevent usage of \<head> element in a Next.js project.
	 */
	noHeadElement?: RuleConfiguration_for_NoHeadElementOptions;
	/**
	 * Disallow implicit true values on JSX boolean attributes
	 */
	noImplicitBoolean?: RuleFixConfiguration_for_NoImplicitBooleanOptions;
	/**
	 * Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
	 */
	noInferrableTypes?: RuleFixConfiguration_for_NoInferrableTypesOptions;
	/**
	 * Reports usage of "magic numbers" — numbers used directly instead of being assigned to named constants.
	 */
	noMagicNumbers?: RuleConfiguration_for_NoMagicNumbersOptions;
	/**
	 * Disallow the use of TypeScript's namespaces.
	 */
	noNamespace?: RuleConfiguration_for_NoNamespaceOptions;
	/**
	 * Disallow negation in the condition of an if statement if it has an else clause.
	 */
	noNegationElse?: RuleFixConfiguration_for_NoNegationElseOptions;
	/**
	 * Disallow nested ternary expressions.
	 */
	noNestedTernary?: RuleConfiguration_for_NoNestedTernaryOptions;
	/**
	 * Disallow non-null assertions using the ! postfix operator.
	 */
	noNonNullAssertion?: RuleFixConfiguration_for_NoNonNullAssertionOptions;
	/**
	 * Disallow reassigning function parameters.
	 */
	noParameterAssign?: RuleConfiguration_for_NoParameterAssignOptions;
	/**
	 * Disallow the use of parameter properties in class constructors.
	 */
	noParameterProperties?: RuleConfiguration_for_NoParameterPropertiesOptions;
	/**
	 * Disallow the use of process.env.
	 */
	noProcessEnv?: RuleConfiguration_for_NoProcessEnvOptions;
	/**
	 * This rule allows you to specify global variable names that you don’t want to use in your application.
	 */
	noRestrictedGlobals?: RuleConfiguration_for_NoRestrictedGlobalsOptions;
	/**
	 * Disallow specified modules when loaded by import or require.
	 */
	noRestrictedImports?: RuleConfiguration_for_NoRestrictedImportsOptions;
	/**
	 * Disallow user defined types.
	 */
	noRestrictedTypes?: RuleFixConfiguration_for_NoRestrictedTypesOptions;
	/**
	 * Disallow the use of constants which its value is the upper-case version of its name.
	 */
	noShoutyConstants?: RuleFixConfiguration_for_NoShoutyConstantsOptions;
	/**
	 * Enforce the use of String.slice() over String.substr() and String.substring().
	 */
	noSubstr?: RuleFixConfiguration_for_NoSubstrOptions;
	/**
	 * Disallow template literals if interpolation and special-character handling are not needed
	 */
	noUnusedTemplateLiteral?: RuleFixConfiguration_for_NoUnusedTemplateLiteralOptions;
	/**
	 * Disallow else block when the if block breaks early.
	 */
	noUselessElse?: RuleFixConfiguration_for_NoUselessElseOptions;
	/**
	 * Disallow use of @value rule in css modules.
	 */
	noValueAtRule?: RuleConfiguration_for_NoValueAtRuleOptions;
	/**
	 * Disallow the use of yoda expressions.
	 */
	noYodaExpression?: RuleFixConfiguration_for_NoYodaExpressionOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Disallow Array constructors.
	 */
	useArrayLiterals?: RuleFixConfiguration_for_UseArrayLiteralsOptions;
	/**
	 * Enforce the use of as const over literal type and type annotation.
	 */
	useAsConstAssertion?: RuleFixConfiguration_for_UseAsConstAssertionOptions;
	/**
	 * Use at() instead of integer index access.
	 */
	useAtIndex?: RuleFixConfiguration_for_UseAtIndexOptions;
	/**
	 * Requires following curly brace conventions.
	 */
	useBlockStatements?: RuleFixConfiguration_for_UseBlockStatementsOptions;
	/**
	 * Enforce using else if instead of nested if in else clauses.
	 */
	useCollapsedElseIf?: RuleFixConfiguration_for_UseCollapsedElseIfOptions;
	/**
	 * Enforce using single if instead of nested if clauses.
	 */
	useCollapsedIf?: RuleFixConfiguration_for_UseCollapsedIfOptions;
	/**
	 * Enforce declaring components only within modules that export React Components exclusively.
	 */
	useComponentExportOnlyModules?: RuleConfiguration_for_UseComponentExportOnlyModulesOptions;
	/**
	 * Require consistently using either T\[] or Array\<T>
	 */
	useConsistentArrayType?: RuleFixConfiguration_for_UseConsistentArrayTypeOptions;
	/**
	 * Enforce the use of new for all builtins, except String, Number and Boolean.
	 */
	useConsistentBuiltinInstantiation?: RuleFixConfiguration_for_UseConsistentBuiltinInstantiationOptions;
	/**
	 * This rule enforces consistent use of curly braces inside JSX attributes and JSX children.
	 */
	useConsistentCurlyBraces?: RuleFixConfiguration_for_UseConsistentCurlyBracesOptions;
	/**
	 * Require consistent accessibility modifiers on class properties and methods.
	 */
	useConsistentMemberAccessibility?: RuleConfiguration_for_UseConsistentMemberAccessibilityOptions;
	/**
	 * Require the consistent declaration of object literals. Defaults to explicit definitions.
	 */
	useConsistentObjectDefinitions?: RuleFixConfiguration_for_UseConsistentObjectDefinitionsOptions;
	/**
	 * Enforce type definitions to consistently use either interface or type.
	 */
	useConsistentTypeDefinitions?: RuleFixConfiguration_for_UseConsistentTypeDefinitionsOptions;
	/**
	 * Require const declarations for variables that are only assigned once.
	 */
	useConst?: RuleFixConfiguration_for_UseConstOptions;
	/**
	 * Enforce default function parameters and optional function parameters to be last.
	 */
	useDefaultParameterLast?: RuleFixConfiguration_for_UseDefaultParameterLastOptions;
	/**
	 * Require the default clause in switch statements.
	 */
	useDefaultSwitchClause?: RuleConfiguration_for_UseDefaultSwitchClauseOptions;
	/**
	 * Require specifying the reason argument when using @deprecated directive
	 */
	useDeprecatedReason?: RuleConfiguration_for_UseDeprecatedReasonOptions;
	/**
	 * Require that each enum member value be explicitly initialized.
	 */
	useEnumInitializers?: RuleFixConfiguration_for_UseEnumInitializersOptions;
	/**
	 * Enforce explicitly comparing the length, size, byteLength or byteOffset property of a value.
	 */
	useExplicitLengthCheck?: RuleFixConfiguration_for_UseExplicitLengthCheckOptions;
	/**
	 * Disallow the use of Math.pow in favor of the ** operator.
	 */
	useExponentiationOperator?: RuleFixConfiguration_for_UseExponentiationOperatorOptions;
	/**
	 * Promotes the use of export type for types.
	 */
	useExportType?: RuleFixConfiguration_for_UseExportTypeOptions;
	/**
	 * Require that all exports are declared after all non-export statements.
	 */
	useExportsLast?: RuleConfiguration_for_UseExportsLastOptions;
	/**
	 * Enforce naming conventions for JavaScript and TypeScript filenames.
	 */
	useFilenamingConvention?: RuleConfiguration_for_UseFilenamingConventionOptions;
	/**
	 * Prefer using for...of loops over standard for loops where possible.
	 */
	useForOf?: RuleConfiguration_for_UseForOfOptions;
	/**
	 * This rule enforces the use of \<>...\</> over \<Fragment>...\</Fragment>.
	 */
	useFragmentSyntax?: RuleFixConfiguration_for_UseFragmentSyntaxOptions;
	/**
	 * Validates that all enum values are capitalized.
	 */
	useGraphqlNamingConvention?: RuleConfiguration_for_UseGraphqlNamingConventionOptions;
	/**
	 * Enforce that getters and setters for the same property are adjacent in class and object definitions.
	 */
	useGroupedAccessorPairs?: RuleConfiguration_for_UseGroupedAccessorPairsOptions;
	/**
	 * Promotes the use of import type for types.
	 */
	useImportType?: RuleFixConfiguration_for_UseImportTypeOptions;
	/**
	 * Require all enum members to be literal values.
	 */
	useLiteralEnumMembers?: RuleConfiguration_for_UseLiteralEnumMembersOptions;
	/**
	 * Enforce naming conventions for everything across a codebase.
	 */
	useNamingConvention?: RuleFixConfiguration_for_UseNamingConventionOptions;
	/**
	 * Promotes the usage of node:assert/strict over node:assert.
	 */
	useNodeAssertStrict?: RuleFixConfiguration_for_UseNodeAssertStrictOptions;
	/**
	 * Enforces using the node: protocol for Node.js builtin modules.
	 */
	useNodejsImportProtocol?: RuleFixConfiguration_for_UseNodejsImportProtocolOptions;
	/**
	 * Use the Number properties instead of global ones.
	 */
	useNumberNamespace?: RuleFixConfiguration_for_UseNumberNamespaceOptions;
	/**
	 * Enforce the use of numeric separators in numeric literals.
	 */
	useNumericSeparators?: RuleFixConfiguration_for_UseNumericSeparatorsOptions;
	/**
	 * Prefer object spread over Object.assign() when constructing new objects.
	 */
	useObjectSpread?: RuleFixConfiguration_for_UseObjectSpreadOptions;
	/**
	 * Enforce that components are defined as functions and never as classes.
	 */
	useReactFunctionComponents?: RuleConfiguration_for_UseReactFunctionComponentsOptions;
	/**
	 * Enforce marking members as readonly if they are never modified outside the constructor.
	 */
	useReadonlyClassProperties?: RuleFixConfiguration_for_UseReadonlyClassPropertiesOptions;
	/**
	 * Prevent extra closing tags for components without children.
	 */
	useSelfClosingElements?: RuleFixConfiguration_for_UseSelfClosingElementsOptions;
	/**
	 * Require assignment operator shorthand where possible.
	 */
	useShorthandAssign?: RuleFixConfiguration_for_UseShorthandAssignOptions;
	/**
	 * Enforce using function types instead of object type with call signatures.
	 */
	useShorthandFunctionType?: RuleFixConfiguration_for_UseShorthandFunctionTypeOptions;
	/**
	 * Disallow multiple variable declarations in the same variable statement
	 */
	useSingleVarDeclarator?: RuleFixConfiguration_for_UseSingleVarDeclaratorOptions;
	/**
	 * Require a description parameter for the Symbol().
	 */
	useSymbolDescription?: RuleConfiguration_for_UseSymbolDescriptionOptions;
	/**
	 * Prefer template literals over string concatenation.
	 */
	useTemplate?: RuleFixConfiguration_for_UseTemplateOptions;
	/**
	 * Require new when throwing an error.
	 */
	useThrowNewError?: RuleFixConfiguration_for_UseThrowNewErrorOptions;
	/**
	 * Disallow throwing non-Error values.
	 */
	useThrowOnlyError?: RuleConfiguration_for_UseThrowOnlyErrorOptions;
	/**
	 * Enforce the use of String.trimStart() and String.trimEnd() over String.trimLeft() and String.trimRight().
	 */
	useTrimStartEnd?: RuleFixConfiguration_for_UseTrimStartEndOptions;
	/**
	 * Disallow overload signatures that can be unified into a single signature.
	 */
	useUnifiedTypeSignatures?: RuleFixConfiguration_for_UseUnifiedTypeSignaturesOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Suspicious {
	/**
	 * Disallow the use of alert, confirm, and prompt.
	 */
	noAlert?: RuleConfiguration_for_NoAlertOptions;
	/**
	 * Use standard constants instead of approximated literals.
	 */
	noApproximativeNumericConstant?: RuleFixConfiguration_for_NoApproximativeNumericConstantOptions;
	/**
	 * Discourage the usage of Array index in keys.
	 */
	noArrayIndexKey?: RuleConfiguration_for_NoArrayIndexKeyOptions;
	/**
	 * Disallow assignments in expressions.
	 */
	noAssignInExpressions?: RuleConfiguration_for_NoAssignInExpressionsOptions;
	/**
	 * Disallows using an async function as a Promise executor.
	 */
	noAsyncPromiseExecutor?: RuleConfiguration_for_NoAsyncPromiseExecutorOptions;
	/**
	 * Prevents the use of the ! pattern in the first position of files.includes in the configuration file.
	 */
	noBiomeFirstException?: RuleFixConfiguration_for_NoBiomeFirstExceptionOptions;
	/**
	 * Disallow bitwise operators.
	 */
	noBitwiseOperators?: RuleConfiguration_for_NoBitwiseOperatorsOptions;
	/**
	 * Disallow reassigning exceptions in catch clauses.
	 */
	noCatchAssign?: RuleConfiguration_for_NoCatchAssignOptions;
	/**
	 * Disallow reassigning class members.
	 */
	noClassAssign?: RuleConfiguration_for_NoClassAssignOptions;
	/**
	 * Prevent comments from being inserted as text nodes
	 */
	noCommentText?: RuleFixConfiguration_for_NoCommentTextOptions;
	/**
	 * Disallow comparing against -0
	 */
	noCompareNegZero?: RuleFixConfiguration_for_NoCompareNegZeroOptions;
	/**
	 * Disallow labeled statements that are not loops.
	 */
	noConfusingLabels?: RuleConfiguration_for_NoConfusingLabelsOptions;
	/**
	 * Disallow void type outside of generic or return types.
	 */
	noConfusingVoidType?: RuleFixConfiguration_for_NoConfusingVoidTypeOptions;
	/**
	 * Disallow the use of console.
	 */
	noConsole?: RuleFixConfiguration_for_NoConsoleOptions;
	/**
	 * Disallow TypeScript const enum
	 */
	noConstEnum?: RuleFixConfiguration_for_NoConstEnumOptions;
	/**
	 * Disallow expressions where the operation doesn't affect the value
	 */
	noConstantBinaryExpressions?: RuleConfiguration_for_NoConstantBinaryExpressionsOptions;
	/**
	 * Prevents from having control characters and some escape sequences that match control characters in regular expression literals.
	 */
	noControlCharactersInRegex?: RuleConfiguration_for_NoControlCharactersInRegexOptions;
	/**
	 * Disallow the use of debugger
	 */
	noDebugger?: RuleFixConfiguration_for_NoDebuggerOptions;
	/**
	 * Disallow direct assignments to document.cookie.
	 */
	noDocumentCookie?: RuleConfiguration_for_NoDocumentCookieOptions;
	/**
	 * Prevents importing next/document outside of pages/_document.jsx in Next.js projects.
	 */
	noDocumentImportInPage?: RuleConfiguration_for_NoDocumentImportInPageOptions;
	/**
	 * Require the use of === and !==.
	 */
	noDoubleEquals?: RuleFixConfiguration_for_NoDoubleEqualsOptions;
	/**
	 * Disallow duplicate @import rules.
	 */
	noDuplicateAtImportRules?: RuleConfiguration_for_NoDuplicateAtImportRulesOptions;
	/**
	 * Disallow duplicate case labels.
	 */
	noDuplicateCase?: RuleConfiguration_for_NoDuplicateCaseOptions;
	/**
	 * Disallow duplicate class members.
	 */
	noDuplicateClassMembers?: RuleConfiguration_for_NoDuplicateClassMembersOptions;
	/**
	 * Disallow duplicate custom properties within declaration blocks.
	 */
	noDuplicateCustomProperties?: RuleConfiguration_for_NoDuplicateCustomPropertiesOptions;
	/**
	 * Disallow duplicate conditions in if-else-if chains
	 */
	noDuplicateElseIf?: RuleConfiguration_for_NoDuplicateElseIfOptions;
	/**
	 * No duplicated fields in GraphQL operations.
	 */
	noDuplicateFields?: RuleConfiguration_for_NoDuplicateFieldsOptions;
	/**
	 * Disallow duplicate names within font families.
	 */
	noDuplicateFontNames?: RuleConfiguration_for_NoDuplicateFontNamesOptions;
	/**
	 * Prevents JSX properties to be assigned multiple times.
	 */
	noDuplicateJsxProps?: RuleConfiguration_for_NoDuplicateJsxPropsOptions;
	/**
	 * Disallow two keys with the same name inside objects.
	 */
	noDuplicateObjectKeys?: RuleConfiguration_for_NoDuplicateObjectKeysOptions;
	/**
	 * Disallow duplicate function parameter name.
	 */
	noDuplicateParameters?: RuleConfiguration_for_NoDuplicateParametersOptions;
	/**
	 * Disallow duplicate properties within declaration blocks.
	 */
	noDuplicateProperties?: RuleConfiguration_for_NoDuplicatePropertiesOptions;
	/**
	 * Disallow duplicate selectors within keyframe blocks.
	 */
	noDuplicateSelectorsKeyframeBlock?: RuleConfiguration_for_NoDuplicateSelectorsKeyframeBlockOptions;
	/**
	 * A describe block should not contain duplicate hooks.
	 */
	noDuplicateTestHooks?: RuleConfiguration_for_NoDuplicateTestHooksOptions;
	/**
	 * Disallow CSS empty blocks.
	 */
	noEmptyBlock?: RuleConfiguration_for_NoEmptyBlockOptions;
	/**
	 * Disallow empty block statements and static blocks.
	 */
	noEmptyBlockStatements?: RuleConfiguration_for_NoEmptyBlockStatementsOptions;
	/**
	 * Disallow the declaration of empty interfaces.
	 */
	noEmptyInterface?: RuleFixConfiguration_for_NoEmptyInterfaceOptions;
	/**
	 * Disallow variables from evolving into any type through reassignments.
	 */
	noEvolvingTypes?: RuleConfiguration_for_NoEvolvingTypesOptions;
	/**
	 * Disallow the any type usage.
	 */
	noExplicitAny?: RuleConfiguration_for_NoExplicitAnyOptions;
	/**
	 * Disallow using export or module.exports in files containing tests
	 */
	noExportsInTest?: RuleConfiguration_for_NoExportsInTestOptions;
	/**
	 * Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files.
	 */
	noExtraNonNullAssertion?: RuleFixConfiguration_for_NoExtraNonNullAssertionOptions;
	/**
	 * Disallow fallthrough of switch clauses.
	 */
	noFallthroughSwitchClause?: RuleConfiguration_for_NoFallthroughSwitchClauseOptions;
	/**
	 * Disallow focused tests.
	 */
	noFocusedTests?: RuleFixConfiguration_for_NoFocusedTestsOptions;
	/**
	 * Disallow reassigning function declarations.
	 */
	noFunctionAssign?: RuleConfiguration_for_NoFunctionAssignOptions;
	/**
	 * Disallow assignments to native objects and read-only global variables.
	 */
	noGlobalAssign?: RuleConfiguration_for_NoGlobalAssignOptions;
	/**
	 * Use Number.isFinite instead of global isFinite.
	 */
	noGlobalIsFinite?: RuleFixConfiguration_for_NoGlobalIsFiniteOptions;
	/**
	 * Use Number.isNaN instead of global isNaN.
	 */
	noGlobalIsNan?: RuleFixConfiguration_for_NoGlobalIsNanOptions;
	/**
	 * Prevent using the next/head module in pages/_document.js on Next.js projects.
	 */
	noHeadImportInDocument?: RuleConfiguration_for_NoHeadImportInDocumentOptions;
	/**
	 * Disallow use of implicit any type on variable declarations.
	 */
	noImplicitAnyLet?: RuleConfiguration_for_NoImplicitAnyLetOptions;
	/**
	 * Disallow assigning to imported bindings
	 */
	noImportAssign?: RuleConfiguration_for_NoImportAssignOptions;
	/**
	 * Disallow invalid !important within keyframe declarations
	 */
	noImportantInKeyframe?: RuleConfiguration_for_NoImportantInKeyframeOptions;
	/**
	 * Disallows the use of irregular whitespace characters.
	 */
	noIrregularWhitespace?: RuleConfiguration_for_NoIrregularWhitespaceOptions;
	/**
	 * Disallow labels that share a name with a variable
	 */
	noLabelVar?: RuleConfiguration_for_NoLabelVarOptions;
	/**
	 * Disallow characters made with multiple code points in character class syntax.
	 */
	noMisleadingCharacterClass?: RuleFixConfiguration_for_NoMisleadingCharacterClassOptions;
	/**
	 * Enforce proper usage of new and constructor.
	 */
	noMisleadingInstantiator?: RuleConfiguration_for_NoMisleadingInstantiatorOptions;
	/**
	 * Checks that the assertion function, for example expect, is placed inside an it() function call.
	 */
	noMisplacedAssertion?: RuleConfiguration_for_NoMisplacedAssertionOptions;
	/**
	 * Disallow shorthand assign when variable appears on both sides.
	 */
	noMisrefactoredShorthandAssign?: RuleFixConfiguration_for_NoMisrefactoredShorthandAssignOptions;
	/**
	 * Disallow non-null assertions after optional chaining expressions.
	 */
	noNonNullAssertedOptionalChain?: RuleConfiguration_for_NoNonNullAssertedOptionalChainOptions;
	/**
	 * Disallow octal escape sequences in string literals
	 */
	noOctalEscape?: RuleFixConfiguration_for_NoOctalEscapeOptions;
	/**
	 * Disallow direct use of Object.prototype builtins.
	 */
	noPrototypeBuiltins?: RuleFixConfiguration_for_NoPrototypeBuiltinsOptions;
	/**
	 * Disallow the use if quickfix.biome inside editor settings file.
	 */
	noQuickfixBiome?: RuleFixConfiguration_for_NoQuickfixBiomeOptions;
	/**
	 * Prevents React-specific JSX properties from being used.
	 */
	noReactSpecificProps?: RuleFixConfiguration_for_NoReactSpecificPropsOptions;
	/**
	 * Disallow variable, function, class, and type redeclarations in the same scope.
	 */
	noRedeclare?: RuleConfiguration_for_NoRedeclareOptions;
	/**
	 * Prevents from having redundant "use strict".
	 */
	noRedundantUseStrict?: RuleFixConfiguration_for_NoRedundantUseStrictOptions;
	/**
	 * Disallow comparisons where both sides are exactly the same.
	 */
	noSelfCompare?: RuleConfiguration_for_NoSelfCompareOptions;
	/**
	 * Disallow identifiers from shadowing restricted names.
	 */
	noShadowRestrictedNames?: RuleConfiguration_for_NoShadowRestrictedNamesOptions;
	/**
	 * Disallow shorthand properties that override related longhand properties.
	 */
	noShorthandPropertyOverrides?: RuleConfiguration_for_NoShorthandPropertyOverridesOptions;
	/**
	 * Disallow disabled tests.
	 */
	noSkippedTests?: RuleFixConfiguration_for_NoSkippedTestsOptions;
	/**
	 * Prevents the use of sparse arrays (arrays with holes).
	 */
	noSparseArray?: RuleFixConfiguration_for_NoSparseArrayOptions;
	/**
	 * It detects possible "wrong" semicolons inside JSX elements.
	 */
	noSuspiciousSemicolonInJsx?: RuleConfiguration_for_NoSuspiciousSemicolonInJsxOptions;
	/**
	 * Disallow template literal placeholder syntax in regular strings.
	 */
	noTemplateCurlyInString?: RuleConfiguration_for_NoTemplateCurlyInStringOptions;
	/**
	 * Disallow then property.
	 */
	noThenProperty?: RuleConfiguration_for_NoThenPropertyOptions;
	/**
	 * Prevents the use of the TypeScript directive @ts-ignore.
	 */
	noTsIgnore?: RuleFixConfiguration_for_NoTsIgnoreOptions;
	/**
	 * Disallow let or var variables that are read but never assigned.
	 */
	noUnassignedVariables?: RuleConfiguration_for_NoUnassignedVariablesOptions;
	/**
	 * Disallow unknown at-rules.
	 */
	noUnknownAtRules?: RuleConfiguration_for_NoUnknownAtRulesOptions;
	/**
	 * Disallow unsafe declaration merging between interfaces and classes.
	 */
	noUnsafeDeclarationMerging?: RuleConfiguration_for_NoUnsafeDeclarationMergingOptions;
	/**
	 * Disallow using unsafe negation.
	 */
	noUnsafeNegation?: RuleFixConfiguration_for_NoUnsafeNegationOptions;
	/**
	 * Disallow unnecessary escapes in string literals.
	 */
	noUselessEscapeInString?: RuleFixConfiguration_for_NoUselessEscapeInStringOptions;
	/**
	 * Disallow useless backreferences in regular expression literals that always match an empty string.
	 */
	noUselessRegexBackrefs?: RuleConfiguration_for_NoUselessRegexBackrefsOptions;
	/**
	 * Disallow the use of var
	 */
	noVar?: RuleFixConfiguration_for_NoVarOptions;
	/**
	 * Disallow with statements in non-strict contexts.
	 */
	noWith?: RuleConfiguration_for_NoWithOptions;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Disallow the use of overload signatures that are not next to each other.
	 */
	useAdjacentOverloadSignatures?: RuleConfiguration_for_UseAdjacentOverloadSignaturesOptions;
	/**
	 * Ensure async functions utilize await.
	 */
	useAwait?: RuleConfiguration_for_UseAwaitOptions;
	/**
	 * Promotes the correct usage for ignoring folders in the configuration file.
	 */
	useBiomeIgnoreFolder?: RuleFixConfiguration_for_UseBiomeIgnoreFolderOptions;
	/**
	 * Enforce default clauses in switch statements to be last
	 */
	useDefaultSwitchClauseLast?: RuleConfiguration_for_UseDefaultSwitchClauseLastOptions;
	/**
	 * Enforce passing a message value when creating a built-in error.
	 */
	useErrorMessage?: RuleConfiguration_for_UseErrorMessageOptions;
	/**
	 * Enforce get methods to always return a value.
	 */
	useGetterReturn?: RuleConfiguration_for_UseGetterReturnOptions;
	/**
	 * Enforces the use of a recommended display strategy with Google Fonts.
	 */
	useGoogleFontDisplay?: RuleConfiguration_for_UseGoogleFontDisplayOptions;
	/**
	 * Require for-in loops to include an if statement.
	 */
	useGuardForIn?: RuleConfiguration_for_UseGuardForInOptions;
	/**
	 * Use Array.isArray() instead of instanceof Array.
	 */
	useIsArray?: RuleFixConfiguration_for_UseIsArrayOptions;
	/**
	 * Enforce consistent return values in iterable callbacks.
	 */
	useIterableCallbackReturn?: RuleConfiguration_for_UseIterableCallbackReturnOptions;
	/**
	 * Require using the namespace keyword over the module keyword to declare TypeScript namespaces.
	 */
	useNamespaceKeyword?: RuleFixConfiguration_for_UseNamespaceKeywordOptions;
	/**
	 * Enforce using the digits argument with Number#toFixed().
	 */
	useNumberToFixedDigitsArgument?: RuleFixConfiguration_for_UseNumberToFixedDigitsArgumentOptions;
	/**
	 * Use static Response methods instead of new Response() constructor when possible.
	 */
	useStaticResponseMethods?: RuleFixConfiguration_for_UseStaticResponseMethodsOptions;
	/**
	 * Enforce the use of the directive "use strict" in script files.
	 */
	useStrictMode?: RuleFixConfiguration_for_UseStrictModeOptions;
}
export type RuleAssistPlainConfiguration = "off" | "on";
export interface RuleAssistWithOptions_for_OrganizeImportsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RuleAssistPlainConfiguration;
	/**
	 * Rule's options
	 */
	options: OrganizeImportsOptions;
}
export interface RuleAssistWithOptions_for_UseSortedAttributesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RuleAssistPlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSortedAttributesOptions;
}
export interface RuleAssistWithOptions_for_UseSortedKeysOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RuleAssistPlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSortedKeysOptions;
}
export interface RuleAssistWithOptions_for_UseSortedPropertiesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RuleAssistPlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSortedPropertiesOptions;
}
export type RuleFixConfiguration_for_NoAccessKeyOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoAccessKeyOptions;
export type RuleFixConfiguration_for_NoAriaHiddenOnFocusableOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoAriaHiddenOnFocusableOptions;
export type RuleFixConfiguration_for_NoAriaUnsupportedElementsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoAriaUnsupportedElementsOptions;
export type RuleFixConfiguration_for_NoAutofocusOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoAutofocusOptions;
export type RuleFixConfiguration_for_NoDistractingElementsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoDistractingElementsOptions;
export type RuleFixConfiguration_for_NoHeaderScopeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoHeaderScopeOptions;
export type RuleFixConfiguration_for_NoInteractiveElementToNoninteractiveRoleOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoInteractiveElementToNoninteractiveRoleOptions;
export type RuleConfiguration_for_NoLabelWithoutControlOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoLabelWithoutControlOptions;
export type RuleConfiguration_for_NoNoninteractiveElementInteractionsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNoninteractiveElementInteractionsOptions;
export type RuleFixConfiguration_for_NoNoninteractiveElementToInteractiveRoleOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoNoninteractiveElementToInteractiveRoleOptions;
export type RuleFixConfiguration_for_NoNoninteractiveTabindexOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoNoninteractiveTabindexOptions;
export type RuleFixConfiguration_for_NoPositiveTabindexOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoPositiveTabindexOptions;
export type RuleConfiguration_for_NoRedundantAltOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoRedundantAltOptions;
export type RuleFixConfiguration_for_NoRedundantRolesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoRedundantRolesOptions;
export type RuleConfiguration_for_NoStaticElementInteractionsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoStaticElementInteractionsOptions;
export type RuleConfiguration_for_NoSvgWithoutTitleOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSvgWithoutTitleOptions;
export type RuleConfiguration_for_UseAltTextOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseAltTextOptions;
export type RuleFixConfiguration_for_UseAnchorContentOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseAnchorContentOptions;
export type RuleFixConfiguration_for_UseAriaActivedescendantWithTabindexOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseAriaActivedescendantWithTabindexOptions;
export type RuleConfiguration_for_UseAriaPropsForRoleOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseAriaPropsForRoleOptions;
export type RuleConfiguration_for_UseAriaPropsSupportedByRoleOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseAriaPropsSupportedByRoleOptions;
export type RuleConfiguration_for_UseButtonTypeOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseButtonTypeOptions;
export type RuleConfiguration_for_UseFocusableInteractiveOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseFocusableInteractiveOptions;
export type RuleConfiguration_for_UseGenericFontNamesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseGenericFontNamesOptions;
export type RuleConfiguration_for_UseHeadingContentOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseHeadingContentOptions;
export type RuleConfiguration_for_UseHtmlLangOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseHtmlLangOptions;
export type RuleConfiguration_for_UseIframeTitleOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseIframeTitleOptions;
export type RuleConfiguration_for_UseKeyWithClickEventsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseKeyWithClickEventsOptions;
export type RuleConfiguration_for_UseKeyWithMouseEventsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseKeyWithMouseEventsOptions;
export type RuleConfiguration_for_UseMediaCaptionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseMediaCaptionOptions;
export type RuleConfiguration_for_UseSemanticElementsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseSemanticElementsOptions;
export type RuleConfiguration_for_UseValidAnchorOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseValidAnchorOptions;
export type RuleFixConfiguration_for_UseValidAriaPropsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseValidAriaPropsOptions;
export type RuleFixConfiguration_for_UseValidAriaRoleOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseValidAriaRoleOptions;
export type RuleConfiguration_for_UseValidAriaValuesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseValidAriaValuesOptions;
export type RuleConfiguration_for_UseValidAutocompleteOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseValidAutocompleteOptions;
export type RuleConfiguration_for_UseValidLangOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseValidLangOptions;
export type RuleFixConfiguration_for_NoAdjacentSpacesInRegexOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoAdjacentSpacesInRegexOptions;
export type RuleConfiguration_for_NoArgumentsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoArgumentsOptions;
export type RuleFixConfiguration_for_NoBannedTypesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoBannedTypesOptions;
export type RuleConfiguration_for_NoCommaOperatorOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoCommaOperatorOptions;
export type RuleConfiguration_for_NoEmptyTypeParametersOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEmptyTypeParametersOptions;
export type RuleConfiguration_for_NoExcessiveCognitiveComplexityOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoExcessiveCognitiveComplexityOptions;
export type RuleConfiguration_for_NoExcessiveLinesPerFunctionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoExcessiveLinesPerFunctionOptions;
export type RuleConfiguration_for_NoExcessiveNestedTestSuitesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoExcessiveNestedTestSuitesOptions;
export type RuleFixConfiguration_for_NoExtraBooleanCastOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoExtraBooleanCastOptions;
export type RuleFixConfiguration_for_NoFlatMapIdentityOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoFlatMapIdentityOptions;
export type RuleConfiguration_for_NoForEachOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoForEachOptions;
export type RuleFixConfiguration_for_NoImplicitCoercionsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoImplicitCoercionsOptions;
export type RuleFixConfiguration_for_NoImportantStylesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoImportantStylesOptions;
export type RuleConfiguration_for_NoStaticOnlyClassOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoStaticOnlyClassOptions;
export type RuleFixConfiguration_for_NoThisInStaticOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoThisInStaticOptions;
export type RuleFixConfiguration_for_NoUselessCatchOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessCatchOptions;
export type RuleFixConfiguration_for_NoUselessConstructorOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessConstructorOptions;
export type RuleFixConfiguration_for_NoUselessContinueOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessContinueOptions;
export type RuleFixConfiguration_for_NoUselessEmptyExportOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessEmptyExportOptions;
export type RuleFixConfiguration_for_NoUselessEscapeInRegexOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessEscapeInRegexOptions;
export type RuleFixConfiguration_for_NoUselessFragmentsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessFragmentsOptions;
export type RuleFixConfiguration_for_NoUselessLabelOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessLabelOptions;
export type RuleFixConfiguration_for_NoUselessLoneBlockStatementsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessLoneBlockStatementsOptions;
export type RuleFixConfiguration_for_NoUselessRenameOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessRenameOptions;
export type RuleFixConfiguration_for_NoUselessStringConcatOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessStringConcatOptions;
export type RuleConfiguration_for_NoUselessStringRawOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUselessStringRawOptions;
export type RuleFixConfiguration_for_NoUselessSwitchCaseOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessSwitchCaseOptions;
export type RuleFixConfiguration_for_NoUselessTernaryOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessTernaryOptions;
export type RuleFixConfiguration_for_NoUselessThisAliasOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessThisAliasOptions;
export type RuleFixConfiguration_for_NoUselessTypeConstraintOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessTypeConstraintOptions;
export type RuleFixConfiguration_for_NoUselessUndefinedInitializationOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessUndefinedInitializationOptions;
export type RuleConfiguration_for_NoVoidOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoVoidOptions;
export type RuleFixConfiguration_for_UseArrowFunctionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseArrowFunctionOptions;
export type RuleFixConfiguration_for_UseDateNowOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseDateNowOptions;
export type RuleFixConfiguration_for_UseFlatMapOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseFlatMapOptions;
export type RuleFixConfiguration_for_UseIndexOfOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseIndexOfOptions;
export type RuleFixConfiguration_for_UseLiteralKeysOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseLiteralKeysOptions;
export type RuleFixConfiguration_for_UseNumericLiteralsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNumericLiteralsOptions;
export type RuleFixConfiguration_for_UseOptionalChainOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseOptionalChainOptions;
export type RuleFixConfiguration_for_UseRegexLiteralsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseRegexLiteralsOptions;
export type RuleFixConfiguration_for_UseSimpleNumberKeysOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseSimpleNumberKeysOptions;
export type RuleFixConfiguration_for_UseSimplifiedLogicExpressionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseSimplifiedLogicExpressionOptions;
export type RuleFixConfiguration_for_UseWhileOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseWhileOptions;
export type RuleConfiguration_for_NoChildrenPropOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoChildrenPropOptions;
export type RuleFixConfiguration_for_NoConstAssignOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoConstAssignOptions;
export type RuleConfiguration_for_NoConstantConditionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoConstantConditionOptions;
export type RuleFixConfiguration_for_NoConstantMathMinMaxClampOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoConstantMathMinMaxClampOptions;
export type RuleConfiguration_for_NoConstructorReturnOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoConstructorReturnOptions;
export type RuleConfiguration_for_NoEmptyCharacterClassInRegexOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEmptyCharacterClassInRegexOptions;
export type RuleConfiguration_for_NoEmptyPatternOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEmptyPatternOptions;
export type RuleFixConfiguration_for_NoGlobalDirnameFilenameOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoGlobalDirnameFilenameOptions;
export type RuleConfiguration_for_NoGlobalObjectCallsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoGlobalObjectCallsOptions;
export type RuleConfiguration_for_NoInnerDeclarationsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoInnerDeclarationsOptions;
export type RuleFixConfiguration_for_NoInvalidBuiltinInstantiationOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoInvalidBuiltinInstantiationOptions;
export type RuleConfiguration_for_NoInvalidConstructorSuperOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoInvalidConstructorSuperOptions;
export type RuleConfiguration_for_NoInvalidDirectionInLinearGradientOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoInvalidDirectionInLinearGradientOptions;
export type RuleConfiguration_for_NoInvalidGridAreasOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoInvalidGridAreasOptions;
export type RuleConfiguration_for_NoInvalidPositionAtImportRuleOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoInvalidPositionAtImportRuleOptions;
export type RuleConfiguration_for_NoInvalidUseBeforeDeclarationOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoInvalidUseBeforeDeclarationOptions;
export type RuleConfiguration_for_NoMissingVarFunctionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoMissingVarFunctionOptions;
export type RuleConfiguration_for_NoNestedComponentDefinitionsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNestedComponentDefinitionsOptions;
export type RuleConfiguration_for_NoNodejsModulesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNodejsModulesOptions;
export type RuleFixConfiguration_for_NoNonoctalDecimalEscapeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoNonoctalDecimalEscapeOptions;
export type RuleConfiguration_for_NoPrecisionLossOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoPrecisionLossOptions;
export type RuleConfiguration_for_NoPrivateImportsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoPrivateImportsOptions;
export type RuleFixConfiguration_for_NoProcessGlobalOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoProcessGlobalOptions;
export type RuleConfiguration_for_NoQwikUseVisibleTaskOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoQwikUseVisibleTaskOptions;
export type RuleConfiguration_for_NoReactPropAssignmentsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoReactPropAssignmentsOptions;
export type RuleConfiguration_for_NoRenderReturnValueOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoRenderReturnValueOptions;
export type RuleConfiguration_for_NoRestrictedElementsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoRestrictedElementsOptions;
export type RuleConfiguration_for_NoSelfAssignOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSelfAssignOptions;
export type RuleConfiguration_for_NoSetterReturnOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSetterReturnOptions;
export type RuleConfiguration_for_NoSolidDestructuredPropsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSolidDestructuredPropsOptions;
export type RuleFixConfiguration_for_NoStringCaseMismatchOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoStringCaseMismatchOptions;
export type RuleFixConfiguration_for_NoSwitchDeclarationsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoSwitchDeclarationsOptions;
export type RuleConfiguration_for_NoUndeclaredDependenciesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUndeclaredDependenciesOptions;
export type RuleConfiguration_for_NoUndeclaredVariablesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUndeclaredVariablesOptions;
export type RuleConfiguration_for_NoUnknownFunctionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownFunctionOptions;
export type RuleConfiguration_for_NoUnknownMediaFeatureNameOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownMediaFeatureNameOptions;
export type RuleConfiguration_for_NoUnknownPropertyOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownPropertyOptions;
export type RuleConfiguration_for_NoUnknownPseudoClassOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownPseudoClassOptions;
export type RuleConfiguration_for_NoUnknownPseudoElementOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownPseudoElementOptions;
export type RuleConfiguration_for_NoUnknownTypeSelectorOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownTypeSelectorOptions;
export type RuleConfiguration_for_NoUnknownUnitOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownUnitOptions;
export type RuleConfiguration_for_NoUnmatchableAnbSelectorOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnmatchableAnbSelectorOptions;
export type RuleConfiguration_for_NoUnreachableOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnreachableOptions;
export type RuleConfiguration_for_NoUnreachableSuperOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnreachableSuperOptions;
export type RuleConfiguration_for_NoUnsafeFinallyOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnsafeFinallyOptions;
export type RuleConfiguration_for_NoUnsafeOptionalChainingOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnsafeOptionalChainingOptions;
export type RuleFixConfiguration_for_NoUnusedFunctionParametersOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUnusedFunctionParametersOptions;
export type RuleFixConfiguration_for_NoUnusedImportsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUnusedImportsOptions;
export type RuleFixConfiguration_for_NoUnusedLabelsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUnusedLabelsOptions;
export type RuleFixConfiguration_for_NoUnusedPrivateClassMembersOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUnusedPrivateClassMembersOptions;
export type RuleFixConfiguration_for_NoUnusedVariablesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUnusedVariablesOptions;
export type RuleFixConfiguration_for_NoVoidElementsWithChildrenOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoVoidElementsWithChildrenOptions;
export type RuleConfiguration_for_NoVoidTypeReturnOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoVoidTypeReturnOptions;
export type RuleFixConfiguration_for_UseExhaustiveDependenciesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseExhaustiveDependenciesOptions;
export type RuleFixConfiguration_for_UseGraphqlNamedOperationsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseGraphqlNamedOperationsOptions;
export type RuleConfiguration_for_UseHookAtTopLevelOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseHookAtTopLevelOptions;
export type RuleConfiguration_for_UseImageSizeOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseImageSizeOptions;
export type RuleFixConfiguration_for_UseImportExtensionsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseImportExtensionsOptions;
export type RuleFixConfiguration_for_UseIsNanOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseIsNanOptions;
export type RuleFixConfiguration_for_UseJsonImportAttributesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseJsonImportAttributesOptions;
export type RuleConfiguration_for_UseJsxKeyInIterableOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseJsxKeyInIterableOptions;
export type RuleFixConfiguration_for_UseParseIntRadixOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseParseIntRadixOptions;
export type RuleConfiguration_for_UseQwikClasslistOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseQwikClasslistOptions;
export type RuleFixConfiguration_for_UseSingleJsDocAsteriskOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseSingleJsDocAsteriskOptions;
export type RuleConfiguration_for_UseUniqueElementIdsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseUniqueElementIdsOptions;
export type RuleConfiguration_for_UseValidForDirectionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseValidForDirectionOptions;
export type RuleFixConfiguration_for_UseValidTypeofOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseValidTypeofOptions;
export type RuleConfiguration_for_UseYieldOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseYieldOptions;
export type RuleConfiguration_for_NoDeprecatedImportsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDeprecatedImportsOptions;
export type RuleConfiguration_for_NoDuplicateDependenciesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateDependenciesOptions;
export type RuleConfiguration_for_NoEmptySourceOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEmptySourceOptions;
export type RuleFixConfiguration_for_NoFloatingPromisesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoFloatingPromisesOptions;
export type RuleConfiguration_for_NoImportCyclesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoImportCyclesOptions;
export type RuleConfiguration_for_NoJsxLiteralsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoJsxLiteralsOptions;
export type RuleFixConfiguration_for_NoMisusedPromisesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoMisusedPromisesOptions;
export type RuleConfiguration_for_NoNextAsyncClientComponentOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNextAsyncClientComponentOptions;
export type RuleFixConfiguration_for_NoReactForwardRefOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoReactForwardRefOptions;
export type RuleConfiguration_for_NoShadowOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoShadowOptions;
export type RuleConfiguration_for_NoUnnecessaryConditionsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnnecessaryConditionsOptions;
export type RuleConfiguration_for_NoUnresolvedImportsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnresolvedImportsOptions;
export type RuleConfiguration_for_NoUnusedExpressionsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnusedExpressionsOptions;
export type RuleFixConfiguration_for_NoUselessCatchBindingOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessCatchBindingOptions;
export type RuleFixConfiguration_for_NoUselessUndefinedOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessUndefinedOptions;
export type RuleFixConfiguration_for_NoVueDataObjectDeclarationOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoVueDataObjectDeclarationOptions;
export type RuleConfiguration_for_NoVueDuplicateKeysOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoVueDuplicateKeysOptions;
export type RuleConfiguration_for_NoVueReservedKeysOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoVueReservedKeysOptions;
export type RuleConfiguration_for_NoVueReservedPropsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoVueReservedPropsOptions;
export type RuleFixConfiguration_for_UseConsistentArrowReturnOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseConsistentArrowReturnOptions;
export type RuleConfiguration_for_UseDeprecatedDateOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseDeprecatedDateOptions;
export type RuleFixConfiguration_for_UseExhaustiveSwitchCasesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseExhaustiveSwitchCasesOptions;
export type RuleConfiguration_for_UseExplicitTypeOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseExplicitTypeOptions;
export type RuleConfiguration_for_UseMaxParamsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseMaxParamsOptions;
export type RuleConfiguration_for_UseQwikMethodUsageOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseQwikMethodUsageOptions;
export type RuleConfiguration_for_UseQwikValidLexicalScopeOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseQwikValidLexicalScopeOptions;
export type RuleFixConfiguration_for_UseSortedClassesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseSortedClassesOptions;
export type RuleFixConfiguration_for_UseVueDefineMacrosOrderOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseVueDefineMacrosOrderOptions;
export type RuleConfiguration_for_UseVueMultiWordComponentNamesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseVueMultiWordComponentNamesOptions;
export type RuleConfiguration_for_NoAccumulatingSpreadOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoAccumulatingSpreadOptions;
export type RuleConfiguration_for_NoAwaitInLoopsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoAwaitInLoopsOptions;
export type RuleConfiguration_for_NoBarrelFileOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoBarrelFileOptions;
export type RuleFixConfiguration_for_NoDeleteOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoDeleteOptions;
export type RuleConfiguration_for_NoDynamicNamespaceImportAccessOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDynamicNamespaceImportAccessOptions;
export type RuleConfiguration_for_NoImgElementOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoImgElementOptions;
export type RuleConfiguration_for_NoNamespaceImportOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNamespaceImportOptions;
export type RuleConfiguration_for_NoReExportAllOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoReExportAllOptions;
export type RuleConfiguration_for_NoUnwantedPolyfillioOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnwantedPolyfillioOptions;
export type RuleFixConfiguration_for_UseGoogleFontPreconnectOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseGoogleFontPreconnectOptions;
export type RuleConfiguration_for_UseSolidForComponentOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseSolidForComponentOptions;
export type RuleConfiguration_for_UseTopLevelRegexOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseTopLevelRegexOptions;
export type RuleFixConfiguration_for_NoBlankTargetOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoBlankTargetOptions;
export type RuleConfiguration_for_NoDangerouslySetInnerHtmlOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDangerouslySetInnerHtmlOptions;
export type RuleConfiguration_for_NoDangerouslySetInnerHtmlWithChildrenOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDangerouslySetInnerHtmlWithChildrenOptions;
export type RuleConfiguration_for_NoGlobalEvalOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoGlobalEvalOptions;
export type RuleConfiguration_for_NoSecretsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSecretsOptions;
export type RuleConfiguration_for_NoCommonJsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoCommonJsOptions;
export type RuleConfiguration_for_NoDefaultExportOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDefaultExportOptions;
export type RuleConfiguration_for_NoDescendingSpecificityOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDescendingSpecificityOptions;
export type RuleConfiguration_for_NoDoneCallbackOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDoneCallbackOptions;
export type RuleConfiguration_for_NoEnumOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEnumOptions;
export type RuleConfiguration_for_NoExportedImportsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoExportedImportsOptions;
export type RuleConfiguration_for_NoHeadElementOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoHeadElementOptions;
export type RuleFixConfiguration_for_NoImplicitBooleanOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoImplicitBooleanOptions;
export type RuleFixConfiguration_for_NoInferrableTypesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoInferrableTypesOptions;
export type RuleConfiguration_for_NoMagicNumbersOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoMagicNumbersOptions;
export type RuleConfiguration_for_NoNamespaceOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNamespaceOptions;
export type RuleFixConfiguration_for_NoNegationElseOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoNegationElseOptions;
export type RuleConfiguration_for_NoNestedTernaryOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNestedTernaryOptions;
export type RuleFixConfiguration_for_NoNonNullAssertionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoNonNullAssertionOptions;
export type RuleConfiguration_for_NoParameterAssignOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoParameterAssignOptions;
export type RuleConfiguration_for_NoParameterPropertiesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoParameterPropertiesOptions;
export type RuleConfiguration_for_NoProcessEnvOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoProcessEnvOptions;
export type RuleConfiguration_for_NoRestrictedGlobalsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoRestrictedGlobalsOptions;
export type RuleConfiguration_for_NoRestrictedImportsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoRestrictedImportsOptions;
export type RuleFixConfiguration_for_NoRestrictedTypesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoRestrictedTypesOptions;
export type RuleFixConfiguration_for_NoShoutyConstantsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoShoutyConstantsOptions;
export type RuleFixConfiguration_for_NoSubstrOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoSubstrOptions;
export type RuleFixConfiguration_for_NoUnusedTemplateLiteralOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUnusedTemplateLiteralOptions;
export type RuleFixConfiguration_for_NoUselessElseOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessElseOptions;
export type RuleConfiguration_for_NoValueAtRuleOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoValueAtRuleOptions;
export type RuleFixConfiguration_for_NoYodaExpressionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoYodaExpressionOptions;
export type RuleFixConfiguration_for_UseArrayLiteralsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseArrayLiteralsOptions;
export type RuleFixConfiguration_for_UseAsConstAssertionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseAsConstAssertionOptions;
export type RuleFixConfiguration_for_UseAtIndexOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseAtIndexOptions;
export type RuleFixConfiguration_for_UseBlockStatementsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseBlockStatementsOptions;
export type RuleFixConfiguration_for_UseCollapsedElseIfOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseCollapsedElseIfOptions;
export type RuleFixConfiguration_for_UseCollapsedIfOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseCollapsedIfOptions;
export type RuleConfiguration_for_UseComponentExportOnlyModulesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseComponentExportOnlyModulesOptions;
export type RuleFixConfiguration_for_UseConsistentArrayTypeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseConsistentArrayTypeOptions;
export type RuleFixConfiguration_for_UseConsistentBuiltinInstantiationOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseConsistentBuiltinInstantiationOptions;
export type RuleFixConfiguration_for_UseConsistentCurlyBracesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseConsistentCurlyBracesOptions;
export type RuleConfiguration_for_UseConsistentMemberAccessibilityOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseConsistentMemberAccessibilityOptions;
export type RuleFixConfiguration_for_UseConsistentObjectDefinitionsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseConsistentObjectDefinitionsOptions;
export type RuleFixConfiguration_for_UseConsistentTypeDefinitionsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseConsistentTypeDefinitionsOptions;
export type RuleFixConfiguration_for_UseConstOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseConstOptions;
export type RuleFixConfiguration_for_UseDefaultParameterLastOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseDefaultParameterLastOptions;
export type RuleConfiguration_for_UseDefaultSwitchClauseOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseDefaultSwitchClauseOptions;
export type RuleConfiguration_for_UseDeprecatedReasonOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseDeprecatedReasonOptions;
export type RuleFixConfiguration_for_UseEnumInitializersOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseEnumInitializersOptions;
export type RuleFixConfiguration_for_UseExplicitLengthCheckOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseExplicitLengthCheckOptions;
export type RuleFixConfiguration_for_UseExponentiationOperatorOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseExponentiationOperatorOptions;
export type RuleFixConfiguration_for_UseExportTypeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseExportTypeOptions;
export type RuleConfiguration_for_UseExportsLastOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseExportsLastOptions;
export type RuleConfiguration_for_UseFilenamingConventionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseFilenamingConventionOptions;
export type RuleConfiguration_for_UseForOfOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseForOfOptions;
export type RuleFixConfiguration_for_UseFragmentSyntaxOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseFragmentSyntaxOptions;
export type RuleConfiguration_for_UseGraphqlNamingConventionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseGraphqlNamingConventionOptions;
export type RuleConfiguration_for_UseGroupedAccessorPairsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseGroupedAccessorPairsOptions;
export type RuleFixConfiguration_for_UseImportTypeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseImportTypeOptions;
export type RuleConfiguration_for_UseLiteralEnumMembersOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseLiteralEnumMembersOptions;
export type RuleFixConfiguration_for_UseNamingConventionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNamingConventionOptions;
export type RuleFixConfiguration_for_UseNodeAssertStrictOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNodeAssertStrictOptions;
export type RuleFixConfiguration_for_UseNodejsImportProtocolOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNodejsImportProtocolOptions;
export type RuleFixConfiguration_for_UseNumberNamespaceOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNumberNamespaceOptions;
export type RuleFixConfiguration_for_UseNumericSeparatorsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNumericSeparatorsOptions;
export type RuleFixConfiguration_for_UseObjectSpreadOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseObjectSpreadOptions;
export type RuleConfiguration_for_UseReactFunctionComponentsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseReactFunctionComponentsOptions;
export type RuleFixConfiguration_for_UseReadonlyClassPropertiesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseReadonlyClassPropertiesOptions;
export type RuleFixConfiguration_for_UseSelfClosingElementsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseSelfClosingElementsOptions;
export type RuleFixConfiguration_for_UseShorthandAssignOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseShorthandAssignOptions;
export type RuleFixConfiguration_for_UseShorthandFunctionTypeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseShorthandFunctionTypeOptions;
export type RuleFixConfiguration_for_UseSingleVarDeclaratorOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseSingleVarDeclaratorOptions;
export type RuleConfiguration_for_UseSymbolDescriptionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseSymbolDescriptionOptions;
export type RuleFixConfiguration_for_UseTemplateOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseTemplateOptions;
export type RuleFixConfiguration_for_UseThrowNewErrorOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseThrowNewErrorOptions;
export type RuleConfiguration_for_UseThrowOnlyErrorOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseThrowOnlyErrorOptions;
export type RuleFixConfiguration_for_UseTrimStartEndOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseTrimStartEndOptions;
export type RuleFixConfiguration_for_UseUnifiedTypeSignaturesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseUnifiedTypeSignaturesOptions;
export type RuleConfiguration_for_NoAlertOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoAlertOptions;
export type RuleFixConfiguration_for_NoApproximativeNumericConstantOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoApproximativeNumericConstantOptions;
export type RuleConfiguration_for_NoArrayIndexKeyOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoArrayIndexKeyOptions;
export type RuleConfiguration_for_NoAssignInExpressionsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoAssignInExpressionsOptions;
export type RuleConfiguration_for_NoAsyncPromiseExecutorOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoAsyncPromiseExecutorOptions;
export type RuleFixConfiguration_for_NoBiomeFirstExceptionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoBiomeFirstExceptionOptions;
export type RuleConfiguration_for_NoBitwiseOperatorsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoBitwiseOperatorsOptions;
export type RuleConfiguration_for_NoCatchAssignOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoCatchAssignOptions;
export type RuleConfiguration_for_NoClassAssignOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoClassAssignOptions;
export type RuleFixConfiguration_for_NoCommentTextOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoCommentTextOptions;
export type RuleFixConfiguration_for_NoCompareNegZeroOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoCompareNegZeroOptions;
export type RuleConfiguration_for_NoConfusingLabelsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoConfusingLabelsOptions;
export type RuleFixConfiguration_for_NoConfusingVoidTypeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoConfusingVoidTypeOptions;
export type RuleFixConfiguration_for_NoConsoleOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoConsoleOptions;
export type RuleFixConfiguration_for_NoConstEnumOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoConstEnumOptions;
export type RuleConfiguration_for_NoConstantBinaryExpressionsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoConstantBinaryExpressionsOptions;
export type RuleConfiguration_for_NoControlCharactersInRegexOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoControlCharactersInRegexOptions;
export type RuleFixConfiguration_for_NoDebuggerOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoDebuggerOptions;
export type RuleConfiguration_for_NoDocumentCookieOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDocumentCookieOptions;
export type RuleConfiguration_for_NoDocumentImportInPageOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDocumentImportInPageOptions;
export type RuleFixConfiguration_for_NoDoubleEqualsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoDoubleEqualsOptions;
export type RuleConfiguration_for_NoDuplicateAtImportRulesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateAtImportRulesOptions;
export type RuleConfiguration_for_NoDuplicateCaseOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateCaseOptions;
export type RuleConfiguration_for_NoDuplicateClassMembersOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateClassMembersOptions;
export type RuleConfiguration_for_NoDuplicateCustomPropertiesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateCustomPropertiesOptions;
export type RuleConfiguration_for_NoDuplicateElseIfOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateElseIfOptions;
export type RuleConfiguration_for_NoDuplicateFieldsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateFieldsOptions;
export type RuleConfiguration_for_NoDuplicateFontNamesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateFontNamesOptions;
export type RuleConfiguration_for_NoDuplicateJsxPropsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateJsxPropsOptions;
export type RuleConfiguration_for_NoDuplicateObjectKeysOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateObjectKeysOptions;
export type RuleConfiguration_for_NoDuplicateParametersOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateParametersOptions;
export type RuleConfiguration_for_NoDuplicatePropertiesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicatePropertiesOptions;
export type RuleConfiguration_for_NoDuplicateSelectorsKeyframeBlockOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateSelectorsKeyframeBlockOptions;
export type RuleConfiguration_for_NoDuplicateTestHooksOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoDuplicateTestHooksOptions;
export type RuleConfiguration_for_NoEmptyBlockOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEmptyBlockOptions;
export type RuleConfiguration_for_NoEmptyBlockStatementsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEmptyBlockStatementsOptions;
export type RuleFixConfiguration_for_NoEmptyInterfaceOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoEmptyInterfaceOptions;
export type RuleConfiguration_for_NoEvolvingTypesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoEvolvingTypesOptions;
export type RuleConfiguration_for_NoExplicitAnyOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoExplicitAnyOptions;
export type RuleConfiguration_for_NoExportsInTestOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoExportsInTestOptions;
export type RuleFixConfiguration_for_NoExtraNonNullAssertionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoExtraNonNullAssertionOptions;
export type RuleConfiguration_for_NoFallthroughSwitchClauseOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoFallthroughSwitchClauseOptions;
export type RuleFixConfiguration_for_NoFocusedTestsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoFocusedTestsOptions;
export type RuleConfiguration_for_NoFunctionAssignOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoFunctionAssignOptions;
export type RuleConfiguration_for_NoGlobalAssignOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoGlobalAssignOptions;
export type RuleFixConfiguration_for_NoGlobalIsFiniteOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoGlobalIsFiniteOptions;
export type RuleFixConfiguration_for_NoGlobalIsNanOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoGlobalIsNanOptions;
export type RuleConfiguration_for_NoHeadImportInDocumentOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoHeadImportInDocumentOptions;
export type RuleConfiguration_for_NoImplicitAnyLetOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoImplicitAnyLetOptions;
export type RuleConfiguration_for_NoImportAssignOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoImportAssignOptions;
export type RuleConfiguration_for_NoImportantInKeyframeOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoImportantInKeyframeOptions;
export type RuleConfiguration_for_NoIrregularWhitespaceOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoIrregularWhitespaceOptions;
export type RuleConfiguration_for_NoLabelVarOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoLabelVarOptions;
export type RuleFixConfiguration_for_NoMisleadingCharacterClassOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoMisleadingCharacterClassOptions;
export type RuleConfiguration_for_NoMisleadingInstantiatorOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoMisleadingInstantiatorOptions;
export type RuleConfiguration_for_NoMisplacedAssertionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoMisplacedAssertionOptions;
export type RuleFixConfiguration_for_NoMisrefactoredShorthandAssignOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoMisrefactoredShorthandAssignOptions;
export type RuleConfiguration_for_NoNonNullAssertedOptionalChainOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoNonNullAssertedOptionalChainOptions;
export type RuleFixConfiguration_for_NoOctalEscapeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoOctalEscapeOptions;
export type RuleFixConfiguration_for_NoPrototypeBuiltinsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoPrototypeBuiltinsOptions;
export type RuleFixConfiguration_for_NoQuickfixBiomeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoQuickfixBiomeOptions;
export type RuleFixConfiguration_for_NoReactSpecificPropsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoReactSpecificPropsOptions;
export type RuleConfiguration_for_NoRedeclareOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoRedeclareOptions;
export type RuleFixConfiguration_for_NoRedundantUseStrictOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoRedundantUseStrictOptions;
export type RuleConfiguration_for_NoSelfCompareOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSelfCompareOptions;
export type RuleConfiguration_for_NoShadowRestrictedNamesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoShadowRestrictedNamesOptions;
export type RuleConfiguration_for_NoShorthandPropertyOverridesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoShorthandPropertyOverridesOptions;
export type RuleFixConfiguration_for_NoSkippedTestsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoSkippedTestsOptions;
export type RuleFixConfiguration_for_NoSparseArrayOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoSparseArrayOptions;
export type RuleConfiguration_for_NoSuspiciousSemicolonInJsxOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSuspiciousSemicolonInJsxOptions;
export type RuleConfiguration_for_NoTemplateCurlyInStringOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoTemplateCurlyInStringOptions;
export type RuleConfiguration_for_NoThenPropertyOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoThenPropertyOptions;
export type RuleFixConfiguration_for_NoTsIgnoreOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoTsIgnoreOptions;
export type RuleConfiguration_for_NoUnassignedVariablesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnassignedVariablesOptions;
export type RuleConfiguration_for_NoUnknownAtRulesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnknownAtRulesOptions;
export type RuleConfiguration_for_NoUnsafeDeclarationMergingOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUnsafeDeclarationMergingOptions;
export type RuleFixConfiguration_for_NoUnsafeNegationOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUnsafeNegationOptions;
export type RuleFixConfiguration_for_NoUselessEscapeInStringOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoUselessEscapeInStringOptions;
export type RuleConfiguration_for_NoUselessRegexBackrefsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUselessRegexBackrefsOptions;
export type RuleFixConfiguration_for_NoVarOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoVarOptions;
export type RuleConfiguration_for_NoWithOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoWithOptions;
export type RuleConfiguration_for_UseAdjacentOverloadSignaturesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseAdjacentOverloadSignaturesOptions;
export type RuleConfiguration_for_UseAwaitOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseAwaitOptions;
export type RuleFixConfiguration_for_UseBiomeIgnoreFolderOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseBiomeIgnoreFolderOptions;
export type RuleConfiguration_for_UseDefaultSwitchClauseLastOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseDefaultSwitchClauseLastOptions;
export type RuleConfiguration_for_UseErrorMessageOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseErrorMessageOptions;
export type RuleConfiguration_for_UseGetterReturnOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseGetterReturnOptions;
export type RuleConfiguration_for_UseGoogleFontDisplayOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseGoogleFontDisplayOptions;
export type RuleConfiguration_for_UseGuardForInOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseGuardForInOptions;
export type RuleFixConfiguration_for_UseIsArrayOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseIsArrayOptions;
export type RuleConfiguration_for_UseIterableCallbackReturnOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseIterableCallbackReturnOptions;
export type RuleFixConfiguration_for_UseNamespaceKeywordOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNamespaceKeywordOptions;
export type RuleFixConfiguration_for_UseNumberToFixedDigitsArgumentOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseNumberToFixedDigitsArgumentOptions;
export type RuleFixConfiguration_for_UseStaticResponseMethodsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseStaticResponseMethodsOptions;
export type RuleFixConfiguration_for_UseStrictModeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseStrictModeOptions;
export interface OrganizeImportsOptions {
	groups?: ImportGroups;
	identifierOrder?: SortOrder;
}
export interface UseSortedAttributesOptions {
	sortOrder?: SortOrder;
}
export interface UseSortedKeysOptions {
	sortOrder?: SortOrder;
}
export interface UseSortedPropertiesOptions {}
export type RulePlainConfiguration = "off" | "on" | "info" | "warn" | "error";
export interface RuleWithFixOptions_for_NoAccessKeyOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAccessKeyOptions;
}
export interface RuleWithFixOptions_for_NoAriaHiddenOnFocusableOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAriaHiddenOnFocusableOptions;
}
export interface RuleWithFixOptions_for_NoAriaUnsupportedElementsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAriaUnsupportedElementsOptions;
}
export interface RuleWithFixOptions_for_NoAutofocusOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAutofocusOptions;
}
export interface RuleWithFixOptions_for_NoDistractingElementsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDistractingElementsOptions;
}
export interface RuleWithFixOptions_for_NoHeaderScopeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoHeaderScopeOptions;
}
export interface RuleWithFixOptions_for_NoInteractiveElementToNoninteractiveRoleOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInteractiveElementToNoninteractiveRoleOptions;
}
export interface RuleWithOptions_for_NoLabelWithoutControlOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoLabelWithoutControlOptions;
}
export interface RuleWithOptions_for_NoNoninteractiveElementInteractionsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNoninteractiveElementInteractionsOptions;
}
export interface RuleWithFixOptions_for_NoNoninteractiveElementToInteractiveRoleOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNoninteractiveElementToInteractiveRoleOptions;
}
export interface RuleWithFixOptions_for_NoNoninteractiveTabindexOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNoninteractiveTabindexOptions;
}
export interface RuleWithFixOptions_for_NoPositiveTabindexOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoPositiveTabindexOptions;
}
export interface RuleWithOptions_for_NoRedundantAltOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRedundantAltOptions;
}
export interface RuleWithFixOptions_for_NoRedundantRolesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRedundantRolesOptions;
}
export interface RuleWithOptions_for_NoStaticElementInteractionsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoStaticElementInteractionsOptions;
}
export interface RuleWithOptions_for_NoSvgWithoutTitleOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSvgWithoutTitleOptions;
}
export interface RuleWithOptions_for_UseAltTextOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAltTextOptions;
}
export interface RuleWithFixOptions_for_UseAnchorContentOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAnchorContentOptions;
}
export interface RuleWithFixOptions_for_UseAriaActivedescendantWithTabindexOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAriaActivedescendantWithTabindexOptions;
}
export interface RuleWithOptions_for_UseAriaPropsForRoleOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAriaPropsForRoleOptions;
}
export interface RuleWithOptions_for_UseAriaPropsSupportedByRoleOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAriaPropsSupportedByRoleOptions;
}
export interface RuleWithOptions_for_UseButtonTypeOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseButtonTypeOptions;
}
export interface RuleWithOptions_for_UseFocusableInteractiveOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseFocusableInteractiveOptions;
}
export interface RuleWithOptions_for_UseGenericFontNamesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGenericFontNamesOptions;
}
export interface RuleWithOptions_for_UseHeadingContentOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseHeadingContentOptions;
}
export interface RuleWithOptions_for_UseHtmlLangOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseHtmlLangOptions;
}
export interface RuleWithOptions_for_UseIframeTitleOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseIframeTitleOptions;
}
export interface RuleWithOptions_for_UseKeyWithClickEventsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseKeyWithClickEventsOptions;
}
export interface RuleWithOptions_for_UseKeyWithMouseEventsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseKeyWithMouseEventsOptions;
}
export interface RuleWithOptions_for_UseMediaCaptionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseMediaCaptionOptions;
}
export interface RuleWithOptions_for_UseSemanticElementsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSemanticElementsOptions;
}
export interface RuleWithOptions_for_UseValidAnchorOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidAnchorOptions;
}
export interface RuleWithFixOptions_for_UseValidAriaPropsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidAriaPropsOptions;
}
export interface RuleWithFixOptions_for_UseValidAriaRoleOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidAriaRoleOptions;
}
export interface RuleWithOptions_for_UseValidAriaValuesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidAriaValuesOptions;
}
export interface RuleWithOptions_for_UseValidAutocompleteOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidAutocompleteOptions;
}
export interface RuleWithOptions_for_UseValidLangOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidLangOptions;
}
export interface RuleWithFixOptions_for_NoAdjacentSpacesInRegexOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAdjacentSpacesInRegexOptions;
}
export interface RuleWithOptions_for_NoArgumentsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoArgumentsOptions;
}
export interface RuleWithFixOptions_for_NoBannedTypesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoBannedTypesOptions;
}
export interface RuleWithOptions_for_NoCommaOperatorOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoCommaOperatorOptions;
}
export interface RuleWithOptions_for_NoEmptyTypeParametersOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEmptyTypeParametersOptions;
}
export interface RuleWithOptions_for_NoExcessiveCognitiveComplexityOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExcessiveCognitiveComplexityOptions;
}
export interface RuleWithOptions_for_NoExcessiveLinesPerFunctionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExcessiveLinesPerFunctionOptions;
}
export interface RuleWithOptions_for_NoExcessiveNestedTestSuitesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExcessiveNestedTestSuitesOptions;
}
export interface RuleWithFixOptions_for_NoExtraBooleanCastOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExtraBooleanCastOptions;
}
export interface RuleWithFixOptions_for_NoFlatMapIdentityOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoFlatMapIdentityOptions;
}
export interface RuleWithOptions_for_NoForEachOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoForEachOptions;
}
export interface RuleWithFixOptions_for_NoImplicitCoercionsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImplicitCoercionsOptions;
}
export interface RuleWithFixOptions_for_NoImportantStylesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImportantStylesOptions;
}
export interface RuleWithOptions_for_NoStaticOnlyClassOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoStaticOnlyClassOptions;
}
export interface RuleWithFixOptions_for_NoThisInStaticOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoThisInStaticOptions;
}
export interface RuleWithFixOptions_for_NoUselessCatchOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessCatchOptions;
}
export interface RuleWithFixOptions_for_NoUselessConstructorOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessConstructorOptions;
}
export interface RuleWithFixOptions_for_NoUselessContinueOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessContinueOptions;
}
export interface RuleWithFixOptions_for_NoUselessEmptyExportOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessEmptyExportOptions;
}
export interface RuleWithFixOptions_for_NoUselessEscapeInRegexOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessEscapeInRegexOptions;
}
export interface RuleWithFixOptions_for_NoUselessFragmentsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessFragmentsOptions;
}
export interface RuleWithFixOptions_for_NoUselessLabelOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessLabelOptions;
}
export interface RuleWithFixOptions_for_NoUselessLoneBlockStatementsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessLoneBlockStatementsOptions;
}
export interface RuleWithFixOptions_for_NoUselessRenameOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessRenameOptions;
}
export interface RuleWithFixOptions_for_NoUselessStringConcatOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessStringConcatOptions;
}
export interface RuleWithOptions_for_NoUselessStringRawOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessStringRawOptions;
}
export interface RuleWithFixOptions_for_NoUselessSwitchCaseOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessSwitchCaseOptions;
}
export interface RuleWithFixOptions_for_NoUselessTernaryOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessTernaryOptions;
}
export interface RuleWithFixOptions_for_NoUselessThisAliasOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessThisAliasOptions;
}
export interface RuleWithFixOptions_for_NoUselessTypeConstraintOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessTypeConstraintOptions;
}
export interface RuleWithFixOptions_for_NoUselessUndefinedInitializationOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessUndefinedInitializationOptions;
}
export interface RuleWithOptions_for_NoVoidOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVoidOptions;
}
export interface RuleWithFixOptions_for_UseArrowFunctionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseArrowFunctionOptions;
}
export interface RuleWithFixOptions_for_UseDateNowOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseDateNowOptions;
}
export interface RuleWithFixOptions_for_UseFlatMapOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseFlatMapOptions;
}
export interface RuleWithFixOptions_for_UseIndexOfOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseIndexOfOptions;
}
export interface RuleWithFixOptions_for_UseLiteralKeysOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseLiteralKeysOptions;
}
export interface RuleWithFixOptions_for_UseNumericLiteralsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNumericLiteralsOptions;
}
export interface RuleWithFixOptions_for_UseOptionalChainOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseOptionalChainOptions;
}
export interface RuleWithFixOptions_for_UseRegexLiteralsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseRegexLiteralsOptions;
}
export interface RuleWithFixOptions_for_UseSimpleNumberKeysOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSimpleNumberKeysOptions;
}
export interface RuleWithFixOptions_for_UseSimplifiedLogicExpressionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSimplifiedLogicExpressionOptions;
}
export interface RuleWithFixOptions_for_UseWhileOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseWhileOptions;
}
export interface RuleWithOptions_for_NoChildrenPropOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoChildrenPropOptions;
}
export interface RuleWithFixOptions_for_NoConstAssignOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConstAssignOptions;
}
export interface RuleWithOptions_for_NoConstantConditionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConstantConditionOptions;
}
export interface RuleWithFixOptions_for_NoConstantMathMinMaxClampOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConstantMathMinMaxClampOptions;
}
export interface RuleWithOptions_for_NoConstructorReturnOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConstructorReturnOptions;
}
export interface RuleWithOptions_for_NoEmptyCharacterClassInRegexOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEmptyCharacterClassInRegexOptions;
}
export interface RuleWithOptions_for_NoEmptyPatternOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEmptyPatternOptions;
}
export interface RuleWithFixOptions_for_NoGlobalDirnameFilenameOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoGlobalDirnameFilenameOptions;
}
export interface RuleWithOptions_for_NoGlobalObjectCallsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoGlobalObjectCallsOptions;
}
export interface RuleWithOptions_for_NoInnerDeclarationsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInnerDeclarationsOptions;
}
export interface RuleWithFixOptions_for_NoInvalidBuiltinInstantiationOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInvalidBuiltinInstantiationOptions;
}
export interface RuleWithOptions_for_NoInvalidConstructorSuperOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInvalidConstructorSuperOptions;
}
export interface RuleWithOptions_for_NoInvalidDirectionInLinearGradientOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInvalidDirectionInLinearGradientOptions;
}
export interface RuleWithOptions_for_NoInvalidGridAreasOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInvalidGridAreasOptions;
}
export interface RuleWithOptions_for_NoInvalidPositionAtImportRuleOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInvalidPositionAtImportRuleOptions;
}
export interface RuleWithOptions_for_NoInvalidUseBeforeDeclarationOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInvalidUseBeforeDeclarationOptions;
}
export interface RuleWithOptions_for_NoMissingVarFunctionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoMissingVarFunctionOptions;
}
export interface RuleWithOptions_for_NoNestedComponentDefinitionsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNestedComponentDefinitionsOptions;
}
export interface RuleWithOptions_for_NoNodejsModulesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNodejsModulesOptions;
}
export interface RuleWithFixOptions_for_NoNonoctalDecimalEscapeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNonoctalDecimalEscapeOptions;
}
export interface RuleWithOptions_for_NoPrecisionLossOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoPrecisionLossOptions;
}
export interface RuleWithOptions_for_NoPrivateImportsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoPrivateImportsOptions;
}
export interface RuleWithFixOptions_for_NoProcessGlobalOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoProcessGlobalOptions;
}
export interface RuleWithOptions_for_NoQwikUseVisibleTaskOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoQwikUseVisibleTaskOptions;
}
export interface RuleWithOptions_for_NoReactPropAssignmentsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoReactPropAssignmentsOptions;
}
export interface RuleWithOptions_for_NoRenderReturnValueOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRenderReturnValueOptions;
}
export interface RuleWithOptions_for_NoRestrictedElementsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRestrictedElementsOptions;
}
export interface RuleWithOptions_for_NoSelfAssignOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSelfAssignOptions;
}
export interface RuleWithOptions_for_NoSetterReturnOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSetterReturnOptions;
}
export interface RuleWithOptions_for_NoSolidDestructuredPropsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSolidDestructuredPropsOptions;
}
export interface RuleWithFixOptions_for_NoStringCaseMismatchOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoStringCaseMismatchOptions;
}
export interface RuleWithFixOptions_for_NoSwitchDeclarationsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSwitchDeclarationsOptions;
}
export interface RuleWithOptions_for_NoUndeclaredDependenciesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUndeclaredDependenciesOptions;
}
export interface RuleWithOptions_for_NoUndeclaredVariablesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUndeclaredVariablesOptions;
}
export interface RuleWithOptions_for_NoUnknownFunctionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownFunctionOptions;
}
export interface RuleWithOptions_for_NoUnknownMediaFeatureNameOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownMediaFeatureNameOptions;
}
export interface RuleWithOptions_for_NoUnknownPropertyOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownPropertyOptions;
}
export interface RuleWithOptions_for_NoUnknownPseudoClassOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownPseudoClassOptions;
}
export interface RuleWithOptions_for_NoUnknownPseudoElementOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownPseudoElementOptions;
}
export interface RuleWithOptions_for_NoUnknownTypeSelectorOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownTypeSelectorOptions;
}
export interface RuleWithOptions_for_NoUnknownUnitOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownUnitOptions;
}
export interface RuleWithOptions_for_NoUnmatchableAnbSelectorOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnmatchableAnbSelectorOptions;
}
export interface RuleWithOptions_for_NoUnreachableOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnreachableOptions;
}
export interface RuleWithOptions_for_NoUnreachableSuperOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnreachableSuperOptions;
}
export interface RuleWithOptions_for_NoUnsafeFinallyOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnsafeFinallyOptions;
}
export interface RuleWithOptions_for_NoUnsafeOptionalChainingOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnsafeOptionalChainingOptions;
}
export interface RuleWithFixOptions_for_NoUnusedFunctionParametersOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnusedFunctionParametersOptions;
}
export interface RuleWithFixOptions_for_NoUnusedImportsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnusedImportsOptions;
}
export interface RuleWithFixOptions_for_NoUnusedLabelsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnusedLabelsOptions;
}
export interface RuleWithFixOptions_for_NoUnusedPrivateClassMembersOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnusedPrivateClassMembersOptions;
}
export interface RuleWithFixOptions_for_NoUnusedVariablesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnusedVariablesOptions;
}
export interface RuleWithFixOptions_for_NoVoidElementsWithChildrenOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVoidElementsWithChildrenOptions;
}
export interface RuleWithOptions_for_NoVoidTypeReturnOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVoidTypeReturnOptions;
}
export interface RuleWithFixOptions_for_UseExhaustiveDependenciesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExhaustiveDependenciesOptions;
}
export interface RuleWithFixOptions_for_UseGraphqlNamedOperationsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGraphqlNamedOperationsOptions;
}
export interface RuleWithOptions_for_UseHookAtTopLevelOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseHookAtTopLevelOptions;
}
export interface RuleWithOptions_for_UseImageSizeOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseImageSizeOptions;
}
export interface RuleWithFixOptions_for_UseImportExtensionsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseImportExtensionsOptions;
}
export interface RuleWithFixOptions_for_UseIsNanOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseIsNanOptions;
}
export interface RuleWithFixOptions_for_UseJsonImportAttributesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseJsonImportAttributesOptions;
}
export interface RuleWithOptions_for_UseJsxKeyInIterableOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseJsxKeyInIterableOptions;
}
export interface RuleWithFixOptions_for_UseParseIntRadixOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseParseIntRadixOptions;
}
export interface RuleWithOptions_for_UseQwikClasslistOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseQwikClasslistOptions;
}
export interface RuleWithFixOptions_for_UseSingleJsDocAsteriskOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSingleJsDocAsteriskOptions;
}
export interface RuleWithOptions_for_UseUniqueElementIdsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseUniqueElementIdsOptions;
}
export interface RuleWithOptions_for_UseValidForDirectionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidForDirectionOptions;
}
export interface RuleWithFixOptions_for_UseValidTypeofOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseValidTypeofOptions;
}
export interface RuleWithOptions_for_UseYieldOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseYieldOptions;
}
export interface RuleWithOptions_for_NoDeprecatedImportsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDeprecatedImportsOptions;
}
export interface RuleWithOptions_for_NoDuplicateDependenciesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateDependenciesOptions;
}
export interface RuleWithOptions_for_NoEmptySourceOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEmptySourceOptions;
}
export interface RuleWithFixOptions_for_NoFloatingPromisesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoFloatingPromisesOptions;
}
export interface RuleWithOptions_for_NoImportCyclesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImportCyclesOptions;
}
export interface RuleWithOptions_for_NoJsxLiteralsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoJsxLiteralsOptions;
}
export interface RuleWithFixOptions_for_NoMisusedPromisesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoMisusedPromisesOptions;
}
export interface RuleWithOptions_for_NoNextAsyncClientComponentOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNextAsyncClientComponentOptions;
}
export interface RuleWithFixOptions_for_NoReactForwardRefOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoReactForwardRefOptions;
}
export interface RuleWithOptions_for_NoShadowOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoShadowOptions;
}
export interface RuleWithOptions_for_NoUnnecessaryConditionsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnnecessaryConditionsOptions;
}
export interface RuleWithOptions_for_NoUnresolvedImportsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnresolvedImportsOptions;
}
export interface RuleWithOptions_for_NoUnusedExpressionsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnusedExpressionsOptions;
}
export interface RuleWithFixOptions_for_NoUselessCatchBindingOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessCatchBindingOptions;
}
export interface RuleWithFixOptions_for_NoUselessUndefinedOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessUndefinedOptions;
}
export interface RuleWithFixOptions_for_NoVueDataObjectDeclarationOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVueDataObjectDeclarationOptions;
}
export interface RuleWithOptions_for_NoVueDuplicateKeysOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVueDuplicateKeysOptions;
}
export interface RuleWithOptions_for_NoVueReservedKeysOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVueReservedKeysOptions;
}
export interface RuleWithOptions_for_NoVueReservedPropsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVueReservedPropsOptions;
}
export interface RuleWithFixOptions_for_UseConsistentArrowReturnOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConsistentArrowReturnOptions;
}
export interface RuleWithOptions_for_UseDeprecatedDateOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseDeprecatedDateOptions;
}
export interface RuleWithFixOptions_for_UseExhaustiveSwitchCasesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExhaustiveSwitchCasesOptions;
}
export interface RuleWithOptions_for_UseExplicitTypeOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExplicitTypeOptions;
}
export interface RuleWithOptions_for_UseMaxParamsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseMaxParamsOptions;
}
export interface RuleWithOptions_for_UseQwikMethodUsageOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseQwikMethodUsageOptions;
}
export interface RuleWithOptions_for_UseQwikValidLexicalScopeOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseQwikValidLexicalScopeOptions;
}
export interface RuleWithFixOptions_for_UseSortedClassesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSortedClassesOptions;
}
export interface RuleWithFixOptions_for_UseVueDefineMacrosOrderOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseVueDefineMacrosOrderOptions;
}
export interface RuleWithOptions_for_UseVueMultiWordComponentNamesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseVueMultiWordComponentNamesOptions;
}
export interface RuleWithOptions_for_NoAccumulatingSpreadOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAccumulatingSpreadOptions;
}
export interface RuleWithOptions_for_NoAwaitInLoopsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAwaitInLoopsOptions;
}
export interface RuleWithOptions_for_NoBarrelFileOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoBarrelFileOptions;
}
export interface RuleWithFixOptions_for_NoDeleteOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDeleteOptions;
}
export interface RuleWithOptions_for_NoDynamicNamespaceImportAccessOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDynamicNamespaceImportAccessOptions;
}
export interface RuleWithOptions_for_NoImgElementOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImgElementOptions;
}
export interface RuleWithOptions_for_NoNamespaceImportOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNamespaceImportOptions;
}
export interface RuleWithOptions_for_NoReExportAllOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoReExportAllOptions;
}
export interface RuleWithOptions_for_NoUnwantedPolyfillioOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnwantedPolyfillioOptions;
}
export interface RuleWithFixOptions_for_UseGoogleFontPreconnectOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGoogleFontPreconnectOptions;
}
export interface RuleWithOptions_for_UseSolidForComponentOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSolidForComponentOptions;
}
export interface RuleWithOptions_for_UseTopLevelRegexOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseTopLevelRegexOptions;
}
export interface RuleWithFixOptions_for_NoBlankTargetOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoBlankTargetOptions;
}
export interface RuleWithOptions_for_NoDangerouslySetInnerHtmlOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDangerouslySetInnerHtmlOptions;
}
export interface RuleWithOptions_for_NoDangerouslySetInnerHtmlWithChildrenOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDangerouslySetInnerHtmlWithChildrenOptions;
}
export interface RuleWithOptions_for_NoGlobalEvalOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoGlobalEvalOptions;
}
export interface RuleWithOptions_for_NoSecretsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSecretsOptions;
}
export interface RuleWithOptions_for_NoCommonJsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoCommonJsOptions;
}
export interface RuleWithOptions_for_NoDefaultExportOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDefaultExportOptions;
}
export interface RuleWithOptions_for_NoDescendingSpecificityOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDescendingSpecificityOptions;
}
export interface RuleWithOptions_for_NoDoneCallbackOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDoneCallbackOptions;
}
export interface RuleWithOptions_for_NoEnumOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEnumOptions;
}
export interface RuleWithOptions_for_NoExportedImportsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExportedImportsOptions;
}
export interface RuleWithOptions_for_NoHeadElementOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoHeadElementOptions;
}
export interface RuleWithFixOptions_for_NoImplicitBooleanOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImplicitBooleanOptions;
}
export interface RuleWithFixOptions_for_NoInferrableTypesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoInferrableTypesOptions;
}
export interface RuleWithOptions_for_NoMagicNumbersOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoMagicNumbersOptions;
}
export interface RuleWithOptions_for_NoNamespaceOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNamespaceOptions;
}
export interface RuleWithFixOptions_for_NoNegationElseOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNegationElseOptions;
}
export interface RuleWithOptions_for_NoNestedTernaryOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNestedTernaryOptions;
}
export interface RuleWithFixOptions_for_NoNonNullAssertionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNonNullAssertionOptions;
}
export interface RuleWithOptions_for_NoParameterAssignOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoParameterAssignOptions;
}
export interface RuleWithOptions_for_NoParameterPropertiesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoParameterPropertiesOptions;
}
export interface RuleWithOptions_for_NoProcessEnvOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoProcessEnvOptions;
}
export interface RuleWithOptions_for_NoRestrictedGlobalsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRestrictedGlobalsOptions;
}
export interface RuleWithOptions_for_NoRestrictedImportsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRestrictedImportsOptions;
}
export interface RuleWithFixOptions_for_NoRestrictedTypesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRestrictedTypesOptions;
}
export interface RuleWithFixOptions_for_NoShoutyConstantsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoShoutyConstantsOptions;
}
export interface RuleWithFixOptions_for_NoSubstrOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSubstrOptions;
}
export interface RuleWithFixOptions_for_NoUnusedTemplateLiteralOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnusedTemplateLiteralOptions;
}
export interface RuleWithFixOptions_for_NoUselessElseOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessElseOptions;
}
export interface RuleWithOptions_for_NoValueAtRuleOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoValueAtRuleOptions;
}
export interface RuleWithFixOptions_for_NoYodaExpressionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoYodaExpressionOptions;
}
export interface RuleWithFixOptions_for_UseArrayLiteralsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseArrayLiteralsOptions;
}
export interface RuleWithFixOptions_for_UseAsConstAssertionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAsConstAssertionOptions;
}
export interface RuleWithFixOptions_for_UseAtIndexOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAtIndexOptions;
}
export interface RuleWithFixOptions_for_UseBlockStatementsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseBlockStatementsOptions;
}
export interface RuleWithFixOptions_for_UseCollapsedElseIfOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseCollapsedElseIfOptions;
}
export interface RuleWithFixOptions_for_UseCollapsedIfOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseCollapsedIfOptions;
}
export interface RuleWithOptions_for_UseComponentExportOnlyModulesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseComponentExportOnlyModulesOptions;
}
export interface RuleWithFixOptions_for_UseConsistentArrayTypeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConsistentArrayTypeOptions;
}
export interface RuleWithFixOptions_for_UseConsistentBuiltinInstantiationOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConsistentBuiltinInstantiationOptions;
}
export interface RuleWithFixOptions_for_UseConsistentCurlyBracesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConsistentCurlyBracesOptions;
}
export interface RuleWithOptions_for_UseConsistentMemberAccessibilityOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConsistentMemberAccessibilityOptions;
}
export interface RuleWithFixOptions_for_UseConsistentObjectDefinitionsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConsistentObjectDefinitionsOptions;
}
export interface RuleWithFixOptions_for_UseConsistentTypeDefinitionsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConsistentTypeDefinitionsOptions;
}
export interface RuleWithFixOptions_for_UseConstOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseConstOptions;
}
export interface RuleWithFixOptions_for_UseDefaultParameterLastOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseDefaultParameterLastOptions;
}
export interface RuleWithOptions_for_UseDefaultSwitchClauseOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseDefaultSwitchClauseOptions;
}
export interface RuleWithOptions_for_UseDeprecatedReasonOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseDeprecatedReasonOptions;
}
export interface RuleWithFixOptions_for_UseEnumInitializersOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseEnumInitializersOptions;
}
export interface RuleWithFixOptions_for_UseExplicitLengthCheckOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExplicitLengthCheckOptions;
}
export interface RuleWithFixOptions_for_UseExponentiationOperatorOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExponentiationOperatorOptions;
}
export interface RuleWithFixOptions_for_UseExportTypeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExportTypeOptions;
}
export interface RuleWithOptions_for_UseExportsLastOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExportsLastOptions;
}
export interface RuleWithOptions_for_UseFilenamingConventionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseFilenamingConventionOptions;
}
export interface RuleWithOptions_for_UseForOfOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseForOfOptions;
}
export interface RuleWithFixOptions_for_UseFragmentSyntaxOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseFragmentSyntaxOptions;
}
export interface RuleWithOptions_for_UseGraphqlNamingConventionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGraphqlNamingConventionOptions;
}
export interface RuleWithOptions_for_UseGroupedAccessorPairsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGroupedAccessorPairsOptions;
}
export interface RuleWithFixOptions_for_UseImportTypeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseImportTypeOptions;
}
export interface RuleWithOptions_for_UseLiteralEnumMembersOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseLiteralEnumMembersOptions;
}
export interface RuleWithFixOptions_for_UseNamingConventionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNamingConventionOptions;
}
export interface RuleWithFixOptions_for_UseNodeAssertStrictOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNodeAssertStrictOptions;
}
export interface RuleWithFixOptions_for_UseNodejsImportProtocolOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNodejsImportProtocolOptions;
}
export interface RuleWithFixOptions_for_UseNumberNamespaceOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNumberNamespaceOptions;
}
export interface RuleWithFixOptions_for_UseNumericSeparatorsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNumericSeparatorsOptions;
}
export interface RuleWithFixOptions_for_UseObjectSpreadOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseObjectSpreadOptions;
}
export interface RuleWithOptions_for_UseReactFunctionComponentsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseReactFunctionComponentsOptions;
}
export interface RuleWithFixOptions_for_UseReadonlyClassPropertiesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseReadonlyClassPropertiesOptions;
}
export interface RuleWithFixOptions_for_UseSelfClosingElementsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSelfClosingElementsOptions;
}
export interface RuleWithFixOptions_for_UseShorthandAssignOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseShorthandAssignOptions;
}
export interface RuleWithFixOptions_for_UseShorthandFunctionTypeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseShorthandFunctionTypeOptions;
}
export interface RuleWithFixOptions_for_UseSingleVarDeclaratorOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSingleVarDeclaratorOptions;
}
export interface RuleWithOptions_for_UseSymbolDescriptionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseSymbolDescriptionOptions;
}
export interface RuleWithFixOptions_for_UseTemplateOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseTemplateOptions;
}
export interface RuleWithFixOptions_for_UseThrowNewErrorOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseThrowNewErrorOptions;
}
export interface RuleWithOptions_for_UseThrowOnlyErrorOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseThrowOnlyErrorOptions;
}
export interface RuleWithFixOptions_for_UseTrimStartEndOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseTrimStartEndOptions;
}
export interface RuleWithFixOptions_for_UseUnifiedTypeSignaturesOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseUnifiedTypeSignaturesOptions;
}
export interface RuleWithOptions_for_NoAlertOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAlertOptions;
}
export interface RuleWithFixOptions_for_NoApproximativeNumericConstantOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoApproximativeNumericConstantOptions;
}
export interface RuleWithOptions_for_NoArrayIndexKeyOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoArrayIndexKeyOptions;
}
export interface RuleWithOptions_for_NoAssignInExpressionsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAssignInExpressionsOptions;
}
export interface RuleWithOptions_for_NoAsyncPromiseExecutorOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoAsyncPromiseExecutorOptions;
}
export interface RuleWithFixOptions_for_NoBiomeFirstExceptionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoBiomeFirstExceptionOptions;
}
export interface RuleWithOptions_for_NoBitwiseOperatorsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoBitwiseOperatorsOptions;
}
export interface RuleWithOptions_for_NoCatchAssignOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoCatchAssignOptions;
}
export interface RuleWithOptions_for_NoClassAssignOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoClassAssignOptions;
}
export interface RuleWithFixOptions_for_NoCommentTextOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoCommentTextOptions;
}
export interface RuleWithFixOptions_for_NoCompareNegZeroOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoCompareNegZeroOptions;
}
export interface RuleWithOptions_for_NoConfusingLabelsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConfusingLabelsOptions;
}
export interface RuleWithFixOptions_for_NoConfusingVoidTypeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConfusingVoidTypeOptions;
}
export interface RuleWithFixOptions_for_NoConsoleOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConsoleOptions;
}
export interface RuleWithFixOptions_for_NoConstEnumOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConstEnumOptions;
}
export interface RuleWithOptions_for_NoConstantBinaryExpressionsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoConstantBinaryExpressionsOptions;
}
export interface RuleWithOptions_for_NoControlCharactersInRegexOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoControlCharactersInRegexOptions;
}
export interface RuleWithFixOptions_for_NoDebuggerOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDebuggerOptions;
}
export interface RuleWithOptions_for_NoDocumentCookieOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDocumentCookieOptions;
}
export interface RuleWithOptions_for_NoDocumentImportInPageOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDocumentImportInPageOptions;
}
export interface RuleWithFixOptions_for_NoDoubleEqualsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDoubleEqualsOptions;
}
export interface RuleWithOptions_for_NoDuplicateAtImportRulesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateAtImportRulesOptions;
}
export interface RuleWithOptions_for_NoDuplicateCaseOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateCaseOptions;
}
export interface RuleWithOptions_for_NoDuplicateClassMembersOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateClassMembersOptions;
}
export interface RuleWithOptions_for_NoDuplicateCustomPropertiesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateCustomPropertiesOptions;
}
export interface RuleWithOptions_for_NoDuplicateElseIfOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateElseIfOptions;
}
export interface RuleWithOptions_for_NoDuplicateFieldsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateFieldsOptions;
}
export interface RuleWithOptions_for_NoDuplicateFontNamesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateFontNamesOptions;
}
export interface RuleWithOptions_for_NoDuplicateJsxPropsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateJsxPropsOptions;
}
export interface RuleWithOptions_for_NoDuplicateObjectKeysOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateObjectKeysOptions;
}
export interface RuleWithOptions_for_NoDuplicateParametersOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateParametersOptions;
}
export interface RuleWithOptions_for_NoDuplicatePropertiesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicatePropertiesOptions;
}
export interface RuleWithOptions_for_NoDuplicateSelectorsKeyframeBlockOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateSelectorsKeyframeBlockOptions;
}
export interface RuleWithOptions_for_NoDuplicateTestHooksOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoDuplicateTestHooksOptions;
}
export interface RuleWithOptions_for_NoEmptyBlockOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEmptyBlockOptions;
}
export interface RuleWithOptions_for_NoEmptyBlockStatementsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEmptyBlockStatementsOptions;
}
export interface RuleWithFixOptions_for_NoEmptyInterfaceOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEmptyInterfaceOptions;
}
export interface RuleWithOptions_for_NoEvolvingTypesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoEvolvingTypesOptions;
}
export interface RuleWithOptions_for_NoExplicitAnyOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExplicitAnyOptions;
}
export interface RuleWithOptions_for_NoExportsInTestOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExportsInTestOptions;
}
export interface RuleWithFixOptions_for_NoExtraNonNullAssertionOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoExtraNonNullAssertionOptions;
}
export interface RuleWithOptions_for_NoFallthroughSwitchClauseOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoFallthroughSwitchClauseOptions;
}
export interface RuleWithFixOptions_for_NoFocusedTestsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoFocusedTestsOptions;
}
export interface RuleWithOptions_for_NoFunctionAssignOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoFunctionAssignOptions;
}
export interface RuleWithOptions_for_NoGlobalAssignOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoGlobalAssignOptions;
}
export interface RuleWithFixOptions_for_NoGlobalIsFiniteOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoGlobalIsFiniteOptions;
}
export interface RuleWithFixOptions_for_NoGlobalIsNanOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoGlobalIsNanOptions;
}
export interface RuleWithOptions_for_NoHeadImportInDocumentOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoHeadImportInDocumentOptions;
}
export interface RuleWithOptions_for_NoImplicitAnyLetOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImplicitAnyLetOptions;
}
export interface RuleWithOptions_for_NoImportAssignOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImportAssignOptions;
}
export interface RuleWithOptions_for_NoImportantInKeyframeOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoImportantInKeyframeOptions;
}
export interface RuleWithOptions_for_NoIrregularWhitespaceOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoIrregularWhitespaceOptions;
}
export interface RuleWithOptions_for_NoLabelVarOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoLabelVarOptions;
}
export interface RuleWithFixOptions_for_NoMisleadingCharacterClassOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoMisleadingCharacterClassOptions;
}
export interface RuleWithOptions_for_NoMisleadingInstantiatorOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoMisleadingInstantiatorOptions;
}
export interface RuleWithOptions_for_NoMisplacedAssertionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoMisplacedAssertionOptions;
}
export interface RuleWithFixOptions_for_NoMisrefactoredShorthandAssignOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoMisrefactoredShorthandAssignOptions;
}
export interface RuleWithOptions_for_NoNonNullAssertedOptionalChainOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoNonNullAssertedOptionalChainOptions;
}
export interface RuleWithFixOptions_for_NoOctalEscapeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoOctalEscapeOptions;
}
export interface RuleWithFixOptions_for_NoPrototypeBuiltinsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoPrototypeBuiltinsOptions;
}
export interface RuleWithFixOptions_for_NoQuickfixBiomeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoQuickfixBiomeOptions;
}
export interface RuleWithFixOptions_for_NoReactSpecificPropsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoReactSpecificPropsOptions;
}
export interface RuleWithOptions_for_NoRedeclareOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRedeclareOptions;
}
export interface RuleWithFixOptions_for_NoRedundantUseStrictOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoRedundantUseStrictOptions;
}
export interface RuleWithOptions_for_NoSelfCompareOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSelfCompareOptions;
}
export interface RuleWithOptions_for_NoShadowRestrictedNamesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoShadowRestrictedNamesOptions;
}
export interface RuleWithOptions_for_NoShorthandPropertyOverridesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoShorthandPropertyOverridesOptions;
}
export interface RuleWithFixOptions_for_NoSkippedTestsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSkippedTestsOptions;
}
export interface RuleWithFixOptions_for_NoSparseArrayOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSparseArrayOptions;
}
export interface RuleWithOptions_for_NoSuspiciousSemicolonInJsxOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoSuspiciousSemicolonInJsxOptions;
}
export interface RuleWithOptions_for_NoTemplateCurlyInStringOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoTemplateCurlyInStringOptions;
}
export interface RuleWithOptions_for_NoThenPropertyOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoThenPropertyOptions;
}
export interface RuleWithFixOptions_for_NoTsIgnoreOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoTsIgnoreOptions;
}
export interface RuleWithOptions_for_NoUnassignedVariablesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnassignedVariablesOptions;
}
export interface RuleWithOptions_for_NoUnknownAtRulesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnknownAtRulesOptions;
}
export interface RuleWithOptions_for_NoUnsafeDeclarationMergingOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnsafeDeclarationMergingOptions;
}
export interface RuleWithFixOptions_for_NoUnsafeNegationOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUnsafeNegationOptions;
}
export interface RuleWithFixOptions_for_NoUselessEscapeInStringOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessEscapeInStringOptions;
}
export interface RuleWithOptions_for_NoUselessRegexBackrefsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoUselessRegexBackrefsOptions;
}
export interface RuleWithFixOptions_for_NoVarOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoVarOptions;
}
export interface RuleWithOptions_for_NoWithOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: NoWithOptions;
}
export interface RuleWithOptions_for_UseAdjacentOverloadSignaturesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAdjacentOverloadSignaturesOptions;
}
export interface RuleWithOptions_for_UseAwaitOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseAwaitOptions;
}
export interface RuleWithFixOptions_for_UseBiomeIgnoreFolderOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseBiomeIgnoreFolderOptions;
}
export interface RuleWithOptions_for_UseDefaultSwitchClauseLastOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseDefaultSwitchClauseLastOptions;
}
export interface RuleWithOptions_for_UseErrorMessageOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseErrorMessageOptions;
}
export interface RuleWithOptions_for_UseGetterReturnOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGetterReturnOptions;
}
export interface RuleWithOptions_for_UseGoogleFontDisplayOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGoogleFontDisplayOptions;
}
export interface RuleWithOptions_for_UseGuardForInOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseGuardForInOptions;
}
export interface RuleWithFixOptions_for_UseIsArrayOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseIsArrayOptions;
}
export interface RuleWithOptions_for_UseIterableCallbackReturnOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseIterableCallbackReturnOptions;
}
export interface RuleWithFixOptions_for_UseNamespaceKeywordOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNamespaceKeywordOptions;
}
export interface RuleWithFixOptions_for_UseNumberToFixedDigitsArgumentOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseNumberToFixedDigitsArgumentOptions;
}
export interface RuleWithFixOptions_for_UseStaticResponseMethodsOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseStaticResponseMethodsOptions;
}
export interface RuleWithFixOptions_for_UseStrictModeOptions {
	/**
	 * The kind of the code actions emitted by the rule
	 */
	fix?: FixKind;
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseStrictModeOptions;
}
export type ImportGroups = ImportGroup[];
export type SortOrder = "natural" | "lexicographic";
/**
 * Used to identify the kind of code action emitted by a rule
 */
export type FixKind = "none" | "safe" | "unsafe";
export interface NoAccessKeyOptions {}
export interface NoAriaHiddenOnFocusableOptions {}
export interface NoAriaUnsupportedElementsOptions {}
export interface NoAutofocusOptions {}
export interface NoDistractingElementsOptions {}
export interface NoHeaderScopeOptions {}
export interface NoInteractiveElementToNoninteractiveRoleOptions {}
export interface NoLabelWithoutControlOptions {
	/**
	 * Array of component names that should be considered the same as an `input` element.
	 */
	inputComponents?: string[];
	/**
	 * Array of attributes that should be treated as the `label` accessible text content.
	 */
	labelAttributes?: string[];
	/**
	 * Array of component names that should be considered the same as a `label` element.
	 */
	labelComponents?: string[];
}
export interface NoNoninteractiveElementInteractionsOptions {}
export interface NoNoninteractiveElementToInteractiveRoleOptions {}
export interface NoNoninteractiveTabindexOptions {}
export interface NoPositiveTabindexOptions {}
export interface NoRedundantAltOptions {}
export interface NoRedundantRolesOptions {}
export interface NoStaticElementInteractionsOptions {}
export interface NoSvgWithoutTitleOptions {}
export interface UseAltTextOptions {}
export interface UseAnchorContentOptions {}
export interface UseAriaActivedescendantWithTabindexOptions {}
export interface UseAriaPropsForRoleOptions {}
export interface UseAriaPropsSupportedByRoleOptions {}
export interface UseButtonTypeOptions {}
export interface UseFocusableInteractiveOptions {}
export interface UseGenericFontNamesOptions {}
export interface UseHeadingContentOptions {}
export interface UseHtmlLangOptions {}
export interface UseIframeTitleOptions {}
export interface UseKeyWithClickEventsOptions {}
export interface UseKeyWithMouseEventsOptions {}
export interface UseMediaCaptionOptions {}
export interface UseSemanticElementsOptions {}
export interface UseValidAnchorOptions {}
export interface UseValidAriaPropsOptions {}
export interface UseValidAriaRoleOptions {
	/**
	 * It allows specifying a list of roles that might be invalid otherwise
	 */
	allowInvalidRoles?: string[];
	/**
	 * Use this option to ignore non-DOM elements, such as custom components
	 */
	ignoreNonDom?: boolean;
}
export interface UseValidAriaValuesOptions {}
export interface UseValidAutocompleteOptions {
	/**
	 * `input` like custom components that should be checked.
	 */
	inputComponents?: string[];
}
export interface UseValidLangOptions {}
export interface NoAdjacentSpacesInRegexOptions {}
export interface NoArgumentsOptions {}
export interface NoBannedTypesOptions {}
export interface NoCommaOperatorOptions {}
export interface NoEmptyTypeParametersOptions {}
export interface NoExcessiveCognitiveComplexityOptions {
	/**
	 * The maximum complexity score that we allow. Anything higher is considered excessive.
	 */
	maxAllowedComplexity?: number;
}
export interface NoExcessiveLinesPerFunctionOptions {
	/**
	 * The maximum number of lines allowed in a function body.
	 */
	maxLines?: number;
	/**
	 * When this options is set to `true`, blank lines in the function body are not counted towards the maximum line limit.
	 */
	skipBlankLines?: boolean;
	/**
	 * When this option is set to `true`, Immediately Invoked Function Expressions (IIFEs) are not checked for the maximum line limit.
	 */
	skipIifes?: boolean;
}
export interface NoExcessiveNestedTestSuitesOptions {}
export interface NoExtraBooleanCastOptions {}
export interface NoFlatMapIdentityOptions {}
export interface NoForEachOptions {
	/**
	 * A list of variable names allowed for `forEach` calls.
	 */
	allowedIdentifiers?: string[];
}
export interface NoImplicitCoercionsOptions {}
export interface NoImportantStylesOptions {}
export interface NoStaticOnlyClassOptions {}
export interface NoThisInStaticOptions {}
export interface NoUselessCatchOptions {}
export interface NoUselessConstructorOptions {}
export interface NoUselessContinueOptions {}
export interface NoUselessEmptyExportOptions {}
export interface NoUselessEscapeInRegexOptions {}
export interface NoUselessFragmentsOptions {}
export interface NoUselessLabelOptions {}
export interface NoUselessLoneBlockStatementsOptions {}
export interface NoUselessRenameOptions {}
export interface NoUselessStringConcatOptions {}
export interface NoUselessStringRawOptions {}
export interface NoUselessSwitchCaseOptions {}
export interface NoUselessTernaryOptions {}
export interface NoUselessThisAliasOptions {}
export interface NoUselessTypeConstraintOptions {}
export interface NoUselessUndefinedInitializationOptions {}
export interface NoVoidOptions {}
export interface UseArrowFunctionOptions {}
export interface UseDateNowOptions {}
export interface UseFlatMapOptions {}
export interface UseIndexOfOptions {}
export interface UseLiteralKeysOptions {}
export interface UseNumericLiteralsOptions {}
export interface UseOptionalChainOptions {}
export interface UseRegexLiteralsOptions {}
export interface UseSimpleNumberKeysOptions {}
export interface UseSimplifiedLogicExpressionOptions {}
export interface UseWhileOptions {}
export interface NoChildrenPropOptions {}
export interface NoConstAssignOptions {}
export interface NoConstantConditionOptions {}
export interface NoConstantMathMinMaxClampOptions {}
export interface NoConstructorReturnOptions {}
export interface NoEmptyCharacterClassInRegexOptions {}
export interface NoEmptyPatternOptions {}
export interface NoGlobalDirnameFilenameOptions {}
export interface NoGlobalObjectCallsOptions {}
export interface NoInnerDeclarationsOptions {}
export interface NoInvalidBuiltinInstantiationOptions {}
export interface NoInvalidConstructorSuperOptions {}
export interface NoInvalidDirectionInLinearGradientOptions {}
export interface NoInvalidGridAreasOptions {}
export interface NoInvalidPositionAtImportRuleOptions {}
export interface NoInvalidUseBeforeDeclarationOptions {}
export interface NoMissingVarFunctionOptions {}
export interface NoNestedComponentDefinitionsOptions {}
export interface NoNodejsModulesOptions {}
export interface NoNonoctalDecimalEscapeOptions {}
export interface NoPrecisionLossOptions {}
export interface NoPrivateImportsOptions {
	/**
	* The default visibility to assume for symbols without visibility tag.

Default: **public**. 
	 */
	defaultVisibility?: Visibility;
}
export interface NoProcessGlobalOptions {}
export interface NoQwikUseVisibleTaskOptions {}
export interface NoReactPropAssignmentsOptions {}
export interface NoRenderReturnValueOptions {}
export interface NoRestrictedElementsOptions {
	/**
	 * Elements to restrict. Each key is the element name, and the value is the message to show when the element is used.
	 */
	elements: CustomRestrictedElements;
}
export interface NoSelfAssignOptions {}
export interface NoSetterReturnOptions {}
export interface NoSolidDestructuredPropsOptions {}
export interface NoStringCaseMismatchOptions {}
export interface NoSwitchDeclarationsOptions {}
export interface NoUndeclaredDependenciesOptions {
	/**
	 * If set to `false`, then the rule will show an error when `devDependencies` are imported. Defaults to `true`.
	 */
	devDependencies?: DependencyAvailability;
	/**
	 * If set to `false`, then the rule will show an error when `optionalDependencies` are imported. Defaults to `true`.
	 */
	optionalDependencies?: DependencyAvailability;
	/**
	 * If set to `false`, then the rule will show an error when `peerDependencies` are imported. Defaults to `true`.
	 */
	peerDependencies?: DependencyAvailability;
}
export interface NoUndeclaredVariablesOptions {
	/**
	 * Check undeclared types.
	 */
	checkTypes?: boolean;
}
export interface NoUnknownFunctionOptions {}
export interface NoUnknownMediaFeatureNameOptions {}
export interface NoUnknownPropertyOptions {}
export interface NoUnknownPseudoClassOptions {}
export interface NoUnknownPseudoElementOptions {}
export interface NoUnknownTypeSelectorOptions {}
export interface NoUnknownUnitOptions {}
export interface NoUnmatchableAnbSelectorOptions {}
export interface NoUnreachableOptions {}
export interface NoUnreachableSuperOptions {}
export interface NoUnsafeFinallyOptions {}
export interface NoUnsafeOptionalChainingOptions {}
export interface NoUnusedFunctionParametersOptions {
	/**
	 * Whether to ignore unused variables from an object destructuring with a spread.
	 */
	ignoreRestSiblings?: boolean;
}
export interface NoUnusedImportsOptions {}
export interface NoUnusedLabelsOptions {}
export interface NoUnusedPrivateClassMembersOptions {}
export interface NoUnusedVariablesOptions {
	/**
	 * Whether to ignore unused variables from an object destructuring with a spread.
	 */
	ignoreRestSiblings?: boolean;
}
export interface NoVoidElementsWithChildrenOptions {}
export interface NoVoidTypeReturnOptions {}
export interface UseExhaustiveDependenciesOptions {
	/**
	 * List of hooks of which the dependencies should be validated.
	 */
	hooks?: Hook[];
	/**
	 * Whether to report an error when a hook has no dependencies array.
	 */
	reportMissingDependenciesArray?: boolean;
	/**
	 * Whether to report an error when a dependency is listed in the dependencies array but isn't used. Defaults to true.
	 */
	reportUnnecessaryDependencies?: boolean;
}
export interface UseGraphqlNamedOperationsOptions {}
export interface UseHookAtTopLevelOptions {}
export type UseImageSizeOptions = null;
export interface UseImportExtensionsOptions {
	/**
	 * If `true`, the suggested extension is always `.js` regardless of what extension the source file has in your project.
	 */
	forceJsExtensions?: boolean;
}
export interface UseIsNanOptions {}
export interface UseJsonImportAttributesOptions {}
export interface UseJsxKeyInIterableOptions {
	/**
	 * Set to `true` to check shorthand fragments (`<></>`)
	 */
	checkShorthandFragments?: boolean;
}
export interface UseParseIntRadixOptions {}
export interface UseQwikClasslistOptions {}
export interface UseSingleJsDocAsteriskOptions {}
export interface UseUniqueElementIdsOptions {
	/**
	 * Component names that accept an `id` prop that does not translate to a DOM element id.
	 */
	excludedComponents?: string[];
}
export interface UseValidForDirectionOptions {}
export interface UseValidTypeofOptions {}
export interface UseYieldOptions {}
export interface NoDeprecatedImportsOptions {}
export interface NoDuplicateDependenciesOptions {}
export interface NoEmptySourceOptions {
	/**
	 * Whether comments are considered meaningful
	 */
	allowComments?: boolean;
}
export interface NoFloatingPromisesOptions {}
export interface NoImportCyclesOptions {
	/**
	 * Ignores type-only imports when finding an import cycle. A type-only import (`import type`) will be removed by the compiler, so it cuts an import cycle at runtime. Note that named type imports (`import { type Foo }`) aren't considered as type-only because it's not removed by the compiler if the `verbatimModuleSyntax` option is enabled. Enabled by default.
	 */
	ignoreTypes?: boolean;
}
export interface NoJsxLiteralsOptions {
	/**
	 * An array of strings that won't trigger the rule. Whitespaces are taken into consideration
	 */
	allowedStrings?: string[];
	/**
	 * When enabled, strings inside props are always ignored
	 */
	ignoreProps?: boolean;
	/**
	 * When enabled, also flag string literals inside JSX expressions and attributes
	 */
	noStrings?: boolean;
}
export interface NoMisusedPromisesOptions {}
export interface NoNextAsyncClientComponentOptions {}
export interface NoReactForwardRefOptions {}
export interface NoShadowOptions {}
export interface NoUnnecessaryConditionsOptions {}
export interface NoUnresolvedImportsOptions {}
export interface NoUnusedExpressionsOptions {}
/**
 * Options for the `noUselessCatchBinding` rule. Currently empty; reserved for future extensions (e.g. allowlist of names).
 */
export interface NoUselessCatchBindingOptions {}
export interface NoUselessUndefinedOptions {}
export interface NoVueDataObjectDeclarationOptions {}
export interface NoVueDuplicateKeysOptions {}
export interface NoVueReservedKeysOptions {}
export interface NoVueReservedPropsOptions {}
/**
 * Options for the `useConsistentArrowReturn` rule.
 */
export interface UseConsistentArrowReturnOptions {
	/**
	* Determines whether the rule enforces a consistent style when the return value is an object literal.

This option is only applicable when used in conjunction with the `asNeeded` option. 
	 */
	requireForObjectLiteral?: boolean;
	/**
	 * The style to enforce for arrow function return statements.
	 */
	style?: UseConsistentArrowReturnStyle;
}
export interface UseDeprecatedDateOptions {
	argumentName?: string;
}
export interface UseExhaustiveSwitchCasesOptions {}
export interface UseExplicitTypeOptions {}
export interface UseMaxParamsOptions {
	/**
	 * Maximum number of parameters allowed (default: 4)
	 */
	max?: number;
}
export interface UseQwikMethodUsageOptions {}
export interface UseQwikValidLexicalScopeOptions {}
export interface UseSortedClassesOptions {
	/**
	 * Additional attributes that will be sorted.
	 */
	attributes?: string[];
	/**
	 * Names of the functions or tagged templates that will be sorted.
	 */
	functions?: string[];
}
export interface UseVueDefineMacrosOrderOptions {
	/**
	 * The order of the Vue define macros.
	 */
	order?: string[];
}
export interface UseVueMultiWordComponentNamesOptions {
	/**
	 * Component names to ignore (allowed to be single-word).
	 */
	ignores: string[];
}
export interface NoAccumulatingSpreadOptions {}
export interface NoAwaitInLoopsOptions {}
export interface NoBarrelFileOptions {}
export interface NoDeleteOptions {}
export interface NoDynamicNamespaceImportAccessOptions {}
export interface NoImgElementOptions {}
export interface NoNamespaceImportOptions {}
export interface NoReExportAllOptions {}
export interface NoUnwantedPolyfillioOptions {}
export interface UseGoogleFontPreconnectOptions {}
export interface UseSolidForComponentOptions {}
export interface UseTopLevelRegexOptions {}
export interface NoBlankTargetOptions {
	/**
	 * List of domains where `target="_blank"` is allowed without `rel="noopener"`.
	 */
	allowDomains: string[];
	/**
	 * Whether `noreferrer` is allowed in addition to `noopener`.
	 */
	allowNoReferrer?: boolean;
}
export interface NoDangerouslySetInnerHtmlOptions {}
export interface NoDangerouslySetInnerHtmlWithChildrenOptions {}
export interface NoGlobalEvalOptions {}
export interface NoSecretsOptions {
	/**
	 * Set entropy threshold (default is 41).
	 */
	entropyThreshold?: number;
}
export interface NoCommonJsOptions {}
export interface NoDefaultExportOptions {}
export interface NoDescendingSpecificityOptions {}
export interface NoDoneCallbackOptions {}
export interface NoEnumOptions {}
export interface NoExportedImportsOptions {}
export interface NoHeadElementOptions {}
export interface NoImplicitBooleanOptions {}
export interface NoInferrableTypesOptions {}
export interface NoMagicNumbersOptions {}
export interface NoNamespaceOptions {}
export interface NoNegationElseOptions {}
export interface NoNestedTernaryOptions {}
export interface NoNonNullAssertionOptions {}
export interface NoParameterAssignOptions {
	/**
	 * Whether to report an error when a dependency is listed in the dependencies array but isn't used. Defaults to `allow`.
	 */
	propertyAssignment?: PropertyAssignmentMode;
}
export interface NoParameterPropertiesOptions {}
export interface NoProcessEnvOptions {}
export interface NoRestrictedGlobalsOptions {
	/**
	 * A list of names that should trigger the rule
	 */
	deniedGlobals: Record<string, string>;
}
export interface NoRestrictedImportsOptions {
	/**
	 * A list of import paths that should trigger the rule.
	 */
	paths: Record<string, Paths>;
	/**
	 * gitignore-style patterns that should trigger the rule.
	 */
	patterns?: Patterns[];
}
export interface NoRestrictedTypesOptions {
	types?: Record<string, CustomRestrictedType>;
}
export interface NoShoutyConstantsOptions {}
export interface NoSubstrOptions {}
export interface NoUnusedTemplateLiteralOptions {}
export interface NoUselessElseOptions {}
export interface NoValueAtRuleOptions {}
export interface NoYodaExpressionOptions {}
export interface UseArrayLiteralsOptions {}
export interface UseAsConstAssertionOptions {}
export interface UseAtIndexOptions {}
export interface UseBlockStatementsOptions {}
export interface UseCollapsedElseIfOptions {}
export interface UseCollapsedIfOptions {}
export interface UseComponentExportOnlyModulesOptions {
	/**
	 * Allows the export of constants. This option is for environments that support it, such as [Vite](https://vitejs.dev/)
	 */
	allowConstantExport?: boolean;
	/**
	 * A list of names that can be additionally exported from the module This option is for exports that do not hinder [React Fast Refresh](https://github.com/facebook/react/tree/main/packages/react-refresh), such as [`meta` in Remix](https://remix.run/docs/en/main/route/meta)
	 */
	allowExportNames: string[];
}
export interface UseConsistentArrayTypeOptions {
	syntax?: ConsistentArrayType;
}
export interface UseConsistentBuiltinInstantiationOptions {}
export interface UseConsistentCurlyBracesOptions {}
export interface UseConsistentMemberAccessibilityOptions {
	/**
	 * The kind of accessibility you want to enforce. Default to "noPublic"
	 */
	accessibility?: Accessibility;
}
export interface UseConsistentObjectDefinitionsOptions {
	/**
	 * The preferred syntax to enforce.
	 */
	syntax?: ObjectPropertySyntax;
}
export interface UseConsistentTypeDefinitionsOptions {
	style?: ConsistentTypeDefinition;
}
export interface UseConstOptions {}
export interface UseDefaultParameterLastOptions {}
export interface UseDefaultSwitchClauseOptions {}
export interface UseDeprecatedReasonOptions {}
export interface UseEnumInitializersOptions {}
export interface UseExplicitLengthCheckOptions {}
export interface UseExponentiationOperatorOptions {}
export interface UseExportTypeOptions {}
export interface UseExportsLastOptions {}
export interface UseFilenamingConventionOptions {
	/**
	 * Allowed cases for file names.
	 */
	filenameCases: FilenameCases;
	/**
	 * Regular expression to enforce
	 */
	match?: Regex;
	/**
	 * If `false`, then non-ASCII characters are allowed.
	 */
	requireAscii: boolean;
	/**
	 * If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases. This does not affect other [Case].
	 */
	strictCase: boolean;
}
export interface UseForOfOptions {}
export interface UseFragmentSyntaxOptions {}
export interface UseGraphqlNamingConventionOptions {}
export interface UseGroupedAccessorPairsOptions {}
export interface UseImportTypeOptions {
	/**
	 * The style to apply when import types. Default to "auto"
	 */
	style?: Style2;
}
export interface UseLiteralEnumMembersOptions {}
/**
 * Rule's options.
 */
export interface UseNamingConventionOptions {
	/**
	 * Custom conventions.
	 */
	conventions: Convention[];
	/**
	 * If `false`, then non-ASCII characters are allowed.
	 */
	requireAscii: boolean;
	/**
	 * If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases. This does not affect other [Case].
	 */
	strictCase: boolean;
}
export interface UseNodeAssertStrictOptions {}
export interface UseNodejsImportProtocolOptions {}
export interface UseNumberNamespaceOptions {}
export interface UseNumericSeparatorsOptions {}
export interface UseObjectSpreadOptions {}
export interface UseReactFunctionComponentsOptions {}
export interface UseReadonlyClassPropertiesOptions {
	/**
	 * When `true`, the keywords `public`, `protected`, and `private` are analyzed by the rule.
	 */
	checkAllProperties: boolean;
}
export interface UseSelfClosingElementsOptions {
	ignoreHtmlElements?: boolean;
}
export interface UseShorthandAssignOptions {}
export interface UseShorthandFunctionTypeOptions {}
export interface UseSingleVarDeclaratorOptions {}
export interface UseSymbolDescriptionOptions {}
export interface UseTemplateOptions {}
export interface UseThrowNewErrorOptions {}
export interface UseThrowOnlyErrorOptions {}
export interface UseTrimStartEndOptions {}
export interface UseUnifiedTypeSignaturesOptions {}
export interface NoAlertOptions {}
export interface NoApproximativeNumericConstantOptions {}
export interface NoArrayIndexKeyOptions {}
export interface NoAssignInExpressionsOptions {}
export interface NoAsyncPromiseExecutorOptions {}
export interface NoBiomeFirstExceptionOptions {}
export interface NoBitwiseOperatorsOptions {
	/**
	 * Allows a list of bitwise operators to be used as exceptions.
	 */
	allow: string[];
}
export interface NoCatchAssignOptions {}
export interface NoClassAssignOptions {}
export interface NoCommentTextOptions {}
export interface NoCompareNegZeroOptions {}
export interface NoConfusingLabelsOptions {
	/**
	 * A list of (non-confusing) labels that should be allowed
	 */
	allowedLabels: string[];
}
export interface NoConfusingVoidTypeOptions {}
export interface NoConsoleOptions {
	/**
	 * Allowed calls on the console object.
	 */
	allow: string[];
}
export interface NoConstEnumOptions {}
export interface NoConstantBinaryExpressionsOptions {}
export interface NoControlCharactersInRegexOptions {}
export interface NoDebuggerOptions {}
export interface NoDocumentCookieOptions {}
export interface NoDocumentImportInPageOptions {}
export interface NoDoubleEqualsOptions {
	/**
	* If `true`, an exception is made when comparing with `null`, as it's often relied on to check both for `null` or `undefined`.

If `false`, no such exception will be made. 
	 */
	ignoreNull: boolean;
}
export interface NoDuplicateAtImportRulesOptions {}
export interface NoDuplicateCaseOptions {}
export interface NoDuplicateClassMembersOptions {}
export interface NoDuplicateCustomPropertiesOptions {}
export interface NoDuplicateElseIfOptions {}
export interface NoDuplicateFieldsOptions {}
export interface NoDuplicateFontNamesOptions {}
export interface NoDuplicateJsxPropsOptions {}
export interface NoDuplicateObjectKeysOptions {}
export interface NoDuplicateParametersOptions {}
export interface NoDuplicatePropertiesOptions {}
export interface NoDuplicateSelectorsKeyframeBlockOptions {}
export interface NoDuplicateTestHooksOptions {}
export interface NoEmptyBlockOptions {}
export interface NoEmptyBlockStatementsOptions {}
export interface NoEmptyInterfaceOptions {}
export interface NoEvolvingTypesOptions {}
export interface NoExplicitAnyOptions {}
export interface NoExportsInTestOptions {}
export interface NoExtraNonNullAssertionOptions {}
export interface NoFallthroughSwitchClauseOptions {}
export interface NoFocusedTestsOptions {}
export interface NoFunctionAssignOptions {}
export interface NoGlobalAssignOptions {}
export interface NoGlobalIsFiniteOptions {}
export interface NoGlobalIsNanOptions {}
export interface NoHeadImportInDocumentOptions {}
export interface NoImplicitAnyLetOptions {}
export interface NoImportAssignOptions {}
export interface NoImportantInKeyframeOptions {}
export interface NoIrregularWhitespaceOptions {}
export interface NoLabelVarOptions {}
export interface NoMisleadingCharacterClassOptions {}
export interface NoMisleadingInstantiatorOptions {}
export interface NoMisplacedAssertionOptions {}
export interface NoMisrefactoredShorthandAssignOptions {}
export interface NoNonNullAssertedOptionalChainOptions {}
export interface NoOctalEscapeOptions {}
export interface NoPrototypeBuiltinsOptions {}
export interface NoQuickfixBiomeOptions {
	/**
	 * A list of additional JSON files that should be checked.
	 */
	additionalPaths?: string[];
}
export interface NoReactSpecificPropsOptions {}
export interface NoRedeclareOptions {}
export interface NoRedundantUseStrictOptions {}
export interface NoSelfCompareOptions {}
export interface NoShadowRestrictedNamesOptions {}
export interface NoShorthandPropertyOverridesOptions {}
export interface NoSkippedTestsOptions {}
export interface NoSparseArrayOptions {}
export interface NoSuspiciousSemicolonInJsxOptions {}
export interface NoTemplateCurlyInStringOptions {}
export interface NoThenPropertyOptions {}
export interface NoTsIgnoreOptions {}
export interface NoUnassignedVariablesOptions {}
export interface NoUnknownAtRulesOptions {
	/**
	 * A list of unknown at-rule names to ignore (case-insensitive).
	 */
	ignore: string[];
}
export interface NoUnsafeDeclarationMergingOptions {}
export interface NoUnsafeNegationOptions {}
export interface NoUselessEscapeInStringOptions {}
export interface NoUselessRegexBackrefsOptions {}
export interface NoVarOptions {}
export interface NoWithOptions {}
export interface UseAdjacentOverloadSignaturesOptions {}
export interface UseAwaitOptions {}
export interface UseBiomeIgnoreFolderOptions {}
export interface UseDefaultSwitchClauseLastOptions {}
export interface UseErrorMessageOptions {}
export interface UseGetterReturnOptions {}
export interface UseGoogleFontDisplayOptions {}
export interface UseGuardForInOptions {}
export interface UseIsArrayOptions {}
export interface UseIterableCallbackReturnOptions {}
export interface UseNamespaceKeywordOptions {}
export interface UseNumberToFixedDigitsArgumentOptions {}
export interface UseStaticResponseMethodsOptions {}
export interface UseStrictModeOptions {}
export type ImportGroup = null | GroupMatcher | GroupMatcher[];
export type Visibility = "public" | "package" | "private";
export type CustomRestrictedElements = Record<string, string>;
export type DependencyAvailability = boolean | string[];
export interface Hook {
	/**
	* The "position" of the closure function, starting from zero.

For example, for React's `useEffect()` hook, the closure index is 0. 
	 */
	closureIndex?: number;
	/**
	* The "position" of the array of dependencies, starting from zero.

For example, for React's `useEffect()` hook, the dependencies index is 1. 
	 */
	dependenciesIndex?: number;
	/**
	 * The name of the hook.
	 */
	name?: string;
	/**
	* Whether the result of the hook is stable.

Set to `true` to mark the identity of the hook's return value as stable, or use a number/an array of numbers to mark the "positions" in the return array as stable.

For example, for React's `useRef()` hook the value would be `true`, while for `useState()` it would be `[1]`. 
	 */
	stableResult?: StableHookResult;
}
export type UseConsistentArrowReturnStyle = "asNeeded" | "always" | "never";
/**
 * Specifies whether property assignments on function parameters are allowed or denied.
 */
export type PropertyAssignmentMode = "allow" | "deny";
export type Paths = string | PathOptions;
export type Patterns = PatternOptions;
export type CustomRestrictedType = string | CustomRestrictedTypeOptions;
export type ConsistentArrayType = "shorthand" | "generic";
export type Accessibility = "noPublic" | "explicit" | "none";
export type ObjectPropertySyntax = "explicit" | "shorthand";
export type ConsistentTypeDefinition = "interface" | "type";
export type FilenameCases = FilenameCase[];
export type Regex = string;
/**
 * Rule's options.
 */
export type Style2 = "auto" | "inlineType" | "separatedType";
export interface Convention {
	/**
	 * String cases to enforce
	 */
	formats: Formats;
	/**
	 * Regular expression to enforce
	 */
	match?: Regex;
	/**
	 * Declarations concerned by this convention
	 */
	selector: Selector;
}
export type GroupMatcher = ImportMatcher | SourceMatcher;
export type StableHookResult = boolean | number[] | string[];
export interface PathOptions {
	/**
	 * Names of the exported members that allowed to be not be used.
	 */
	allowImportNames: string[];
	/**
	 * Names of the exported members that should not be used.
	 */
	importNames: string[];
	/**
	 * The message to display when this module is imported.
	 */
	message: string;
}
export interface PatternOptions {
	/**
	 * An array of gitignore-style patterns.
	 */
	group?: SourcesMatcher;
	/**
	 * A regex pattern for import names to forbid within the matched modules.
	 */
	importNamePattern?: Regex;
	/**
	 * If true, the matched patterns in the importNamePattern will be allowed. Defaults to `false`.
	 */
	invertImportNamePattern?: boolean;
	/**
	 * A custom message for diagnostics related to this pattern.
	 */
	message?: string;
}
export interface CustomRestrictedTypeOptions {
	message?: string;
	use?: string;
}
/**
 * Supported cases for file names.
 */
export type FilenameCase =
	| "camelCase"
	| "export"
	| "kebab-case"
	| "PascalCase"
	| "snake_case";
export type Formats = Format[];
export interface Selector {
	/**
	 * Declaration kind
	 */
	kind: Kind;
	/**
	 * Modifiers used on the declaration
	 */
	modifiers: Modifiers;
	/**
	 * Scope of the declaration
	 */
	scope: Scope;
}
export interface ImportMatcher {
	source?: SourcesMatcher;
	type?: boolean;
}
export type SourceMatcher = NegatablePredefinedSourceMatcher | ImportSourceGlob;
export type SourcesMatcher = SourceMatcher | SourceMatcher[];
/**
 * Supported cases.
 */
export type Format =
	| "camelCase"
	| "CONSTANT_CASE"
	| "PascalCase"
	| "snake_case";
export type Kind =
	| "class"
	| "enum"
	| "interface"
	| "enumMember"
	| "importNamespace"
	| "exportNamespace"
	| "variable"
	| "const"
	| "let"
	| "using"
	| "var"
	| "catchParameter"
	| "indexParameter"
	| "exportAlias"
	| "importAlias"
	| "classGetter"
	| "classSetter"
	| "classMethod"
	| "objectLiteralProperty"
	| "objectLiteralGetter"
	| "objectLiteralSetter"
	| "objectLiteralMethod"
	| "typeAlias"
	| "any"
	| "typeLike"
	| "function"
	| "namespaceLike"
	| "namespace"
	| "functionParameter"
	| "typeParameter"
	| "classMember"
	| "classProperty"
	| "objectLiteralMember"
	| "typeMember"
	| "typeGetter"
	| "typeProperty"
	| "typeSetter"
	| "typeMethod";
export type Modifiers = RestrictedModifier[];
export type Scope = "any" | "global";
export type NegatablePredefinedSourceMatcher =
	| ":ALIAS:"
	| ":BUN:"
	| ":NODE:"
	| ":PACKAGE:"
	| ":PACKAGE_WITH_PROTOCOL:"
	| ":PATH:"
	| ":URL:"
	| "!:ALIAS:"
	| "!:BUN:"
	| "!:NODE:"
	| "!:PACKAGE:"
	| "!:PACKAGE_WITH_PROTOCOL:"
	| "!:PATH:"
	| "!:URL:";
/**
 * Glob to match against import sources.
 */
export type ImportSourceGlob = Glob;
export type RestrictedModifier =
	| "abstract"
	| "private"
	| "protected"
	| "readonly"
	| "static";
export interface UpdateSettingsResult {
	diagnostics: Diagnostic[];
}
/**
 * Serializable representation for a [Diagnostic](super::Diagnostic).
 */
export interface Diagnostic {
	advices: Advices;
	category?: Category;
	description: string;
	location: Location;
	message: MarkupBuf;
	severity: Severity;
	source?: Diagnostic;
	tags: DiagnosticTags;
	verboseAdvices: Advices;
}
/**
 * Implementation of [Visitor] collecting serializable [Advice] into a vector.
 */
export interface Advices {
	advices: Advice[];
}
export type Category =
	| "lint/a11y/noAccessKey"
	| "lint/a11y/noAriaHiddenOnFocusable"
	| "lint/a11y/noAriaUnsupportedElements"
	| "lint/a11y/noAutofocus"
	| "lint/a11y/noDistractingElements"
	| "lint/a11y/noHeaderScope"
	| "lint/a11y/noInteractiveElementToNoninteractiveRole"
	| "lint/a11y/noLabelWithoutControl"
	| "lint/a11y/noNoninteractiveElementInteractions"
	| "lint/a11y/noNoninteractiveElementToInteractiveRole"
	| "lint/a11y/noNoninteractiveTabindex"
	| "lint/a11y/noPositiveTabindex"
	| "lint/a11y/noRedundantAlt"
	| "lint/a11y/noRedundantRoles"
	| "lint/a11y/noStaticElementInteractions"
	| "lint/a11y/noSvgWithoutTitle"
	| "lint/a11y/useAltText"
	| "lint/a11y/useAnchorContent"
	| "lint/a11y/useAriaActivedescendantWithTabindex"
	| "lint/a11y/useAriaPropsForRole"
	| "lint/a11y/useAriaPropsSupportedByRole"
	| "lint/a11y/useButtonType"
	| "lint/a11y/useFocusableInteractive"
	| "lint/a11y/useGenericFontNames"
	| "lint/a11y/useHeadingContent"
	| "lint/a11y/useHtmlLang"
	| "lint/a11y/useIframeTitle"
	| "lint/a11y/useKeyWithClickEvents"
	| "lint/a11y/useKeyWithMouseEvents"
	| "lint/a11y/useMediaCaption"
	| "lint/a11y/useSemanticElements"
	| "lint/a11y/useValidAnchor"
	| "lint/a11y/useValidAriaProps"
	| "lint/a11y/useValidAriaRole"
	| "lint/a11y/useValidAriaValues"
	| "lint/a11y/useValidAutocomplete"
	| "lint/a11y/useValidLang"
	| "lint/complexity/noAdjacentSpacesInRegex"
	| "lint/complexity/noArguments"
	| "lint/complexity/noBannedTypes"
	| "lint/complexity/noCommaOperator"
	| "lint/complexity/noEmptyTypeParameters"
	| "lint/complexity/noExcessiveCognitiveComplexity"
	| "lint/complexity/noExcessiveLinesPerFunction"
	| "lint/complexity/noExcessiveNestedTestSuites"
	| "lint/complexity/noExtraBooleanCast"
	| "lint/complexity/noFlatMapIdentity"
	| "lint/complexity/noForEach"
	| "lint/complexity/noImplicitCoercions"
	| "lint/complexity/noImportantStyles"
	| "lint/complexity/noStaticOnlyClass"
	| "lint/complexity/noThisInStatic"
	| "lint/complexity/noUselessCatch"
	| "lint/complexity/noUselessConstructor"
	| "lint/complexity/noUselessContinue"
	| "lint/complexity/noUselessEmptyExport"
	| "lint/complexity/noUselessEscapeInRegex"
	| "lint/complexity/noUselessFragments"
	| "lint/complexity/noUselessLabel"
	| "lint/complexity/noUselessLoneBlockStatements"
	| "lint/complexity/noUselessRename"
	| "lint/complexity/noUselessStringConcat"
	| "lint/complexity/noUselessStringRaw"
	| "lint/complexity/noUselessSwitchCase"
	| "lint/complexity/noUselessTernary"
	| "lint/complexity/noUselessThisAlias"
	| "lint/complexity/noUselessTypeConstraint"
	| "lint/complexity/noUselessUndefinedInitialization"
	| "lint/complexity/noVoid"
	| "lint/complexity/useArrowFunction"
	| "lint/complexity/useDateNow"
	| "lint/complexity/useFlatMap"
	| "lint/complexity/useIndexOf"
	| "lint/complexity/useLiteralKeys"
	| "lint/complexity/useNumericLiterals"
	| "lint/complexity/useOptionalChain"
	| "lint/complexity/useRegexLiterals"
	| "lint/complexity/useSimpleNumberKeys"
	| "lint/complexity/useSimplifiedLogicExpression"
	| "lint/complexity/useWhile"
	| "lint/correctness/noChildrenProp"
	| "lint/correctness/noConstAssign"
	| "lint/correctness/noConstantCondition"
	| "lint/correctness/noConstantMathMinMaxClamp"
	| "lint/correctness/noConstructorReturn"
	| "lint/correctness/noEmptyCharacterClassInRegex"
	| "lint/correctness/noEmptyPattern"
	| "lint/correctness/noGlobalDirnameFilename"
	| "lint/correctness/noGlobalObjectCalls"
	| "lint/correctness/noInnerDeclarations"
	| "lint/correctness/noInvalidBuiltinInstantiation"
	| "lint/correctness/noInvalidConstructorSuper"
	| "lint/correctness/noInvalidDirectionInLinearGradient"
	| "lint/correctness/noInvalidGridAreas"
	| "lint/correctness/noInvalidNewBuiltin"
	| "lint/correctness/noInvalidPositionAtImportRule"
	| "lint/correctness/noInvalidUseBeforeDeclaration"
	| "lint/correctness/noMissingVarFunction"
	| "lint/correctness/noNestedComponentDefinitions"
	| "lint/correctness/noNewSymbol"
	| "lint/correctness/noNodejsModules"
	| "lint/correctness/noNonoctalDecimalEscape"
	| "lint/correctness/noPrecisionLoss"
	| "lint/correctness/noPrivateImports"
	| "lint/correctness/noProcessGlobal"
	| "lint/correctness/noQwikUseVisibleTask"
	| "lint/correctness/noReactPropAssignments"
	| "lint/correctness/noRenderReturnValue"
	| "lint/correctness/noRestrictedElements"
	| "lint/correctness/noSelfAssign"
	| "lint/correctness/noSetterReturn"
	| "lint/correctness/noSolidDestructuredProps"
	| "lint/correctness/noStringCaseMismatch"
	| "lint/correctness/noSwitchDeclarations"
	| "lint/correctness/noUndeclaredDependencies"
	| "lint/correctness/noUndeclaredVariables"
	| "lint/correctness/noUnknownFunction"
	| "lint/correctness/noUnknownMediaFeatureName"
	| "lint/correctness/noUnknownProperty"
	| "lint/correctness/noUnknownPseudoClass"
	| "lint/correctness/noUnknownPseudoClassSelector"
	| "lint/correctness/noUnknownPseudoElement"
	| "lint/correctness/noUnknownTypeSelector"
	| "lint/correctness/noUnknownUnit"
	| "lint/correctness/noUnmatchableAnbSelector"
	| "lint/correctness/noUnreachable"
	| "lint/correctness/noUnreachableSuper"
	| "lint/correctness/noUnsafeFinally"
	| "lint/correctness/noUnsafeOptionalChaining"
	| "lint/correctness/noUnusedFunctionParameters"
	| "lint/correctness/noUnusedImports"
	| "lint/correctness/noUnusedLabels"
	| "lint/correctness/noUnusedPrivateClassMembers"
	| "lint/correctness/noUnusedVariables"
	| "lint/correctness/noVoidElementsWithChildren"
	| "lint/correctness/noVoidTypeReturn"
	| "lint/correctness/useExhaustiveDependencies"
	| "lint/correctness/useGraphqlNamedOperations"
	| "lint/correctness/useHookAtTopLevel"
	| "lint/correctness/useImageSize"
	| "lint/correctness/useImportExtensions"
	| "lint/correctness/useIsNan"
	| "lint/correctness/useJsonImportAttributes"
	| "lint/correctness/useJsxKeyInIterable"
	| "lint/correctness/useParseIntRadix"
	| "lint/correctness/useQwikClasslist"
	| "lint/correctness/useSingleJsDocAsterisk"
	| "lint/correctness/useUniqueElementIds"
	| "lint/correctness/useValidForDirection"
	| "lint/correctness/useValidTypeof"
	| "lint/correctness/useYield"
	| "lint/nursery/noColorInvalidHex"
	| "lint/nursery/noDeprecatedImports"
	| "lint/nursery/noDuplicateDependencies"
	| "lint/nursery/noEmptySource"
	| "lint/nursery/noFloatingPromises"
	| "lint/nursery/noImplicitCoercion"
	| "lint/nursery/noImportCycles"
	| "lint/nursery/noJsxLiterals"
	| "lint/nursery/noMissingGenericFamilyKeyword"
	| "lint/nursery/noMisusedPromises"
	| "lint/nursery/noNextAsyncClientComponent"
	| "lint/nursery/noReactForwardRef"
	| "lint/nursery/noShadow"
	| "lint/nursery/noUnnecessaryConditions"
	| "lint/nursery/noUnresolvedImports"
	| "lint/nursery/noUnusedExpressions"
	| "lint/nursery/noUnwantedPolyfillio"
	| "lint/nursery/noUselessBackrefInRegex"
	| "lint/nursery/noUselessCatchBinding"
	| "lint/nursery/noUselessUndefined"
	| "lint/nursery/noVueDataObjectDeclaration"
	| "lint/nursery/noVueDuplicateKeys"
	| "lint/nursery/noVueReservedKeys"
	| "lint/nursery/noVueReservedProps"
	| "lint/nursery/useAnchorHref"
	| "lint/nursery/useBiomeSuppressionComment"
	| "lint/nursery/useConsistentArrowReturn"
	| "lint/nursery/useConsistentObjectDefinition"
	| "lint/nursery/useDeprecatedDate"
	| "lint/nursery/useExhaustiveSwitchCases"
	| "lint/nursery/useExplicitFunctionReturnType"
	| "lint/nursery/useExplicitType"
	| "lint/nursery/useImportRestrictions"
	| "lint/nursery/useJsxCurlyBraceConvention"
	| "lint/nursery/useMaxParams"
	| "lint/nursery/useQwikMethodUsage"
	| "lint/nursery/useQwikValidLexicalScope"
	| "lint/nursery/useSortedClasses"
	| "lint/nursery/useVueDefineMacrosOrder"
	| "lint/nursery/useVueMultiWordComponentNames"
	| "lint/performance/noAccumulatingSpread"
	| "lint/performance/noAwaitInLoops"
	| "lint/performance/noBarrelFile"
	| "lint/performance/noDelete"
	| "lint/performance/noDynamicNamespaceImportAccess"
	| "lint/performance/noImgElement"
	| "lint/performance/noNamespaceImport"
	| "lint/performance/noReExportAll"
	| "lint/performance/noUnwantedPolyfillio"
	| "lint/performance/useGoogleFontPreconnect"
	| "lint/performance/useSolidForComponent"
	| "lint/performance/useTopLevelRegex"
	| "lint/security/noBlankTarget"
	| "lint/security/noDangerouslySetInnerHtml"
	| "lint/security/noDangerouslySetInnerHtmlWithChildren"
	| "lint/security/noGlobalEval"
	| "lint/security/noSecrets"
	| "lint/style/noCommonJs"
	| "lint/style/noDefaultExport"
	| "lint/style/noDescendingSpecificity"
	| "lint/style/noDoneCallback"
	| "lint/style/noEnum"
	| "lint/style/noExportedImports"
	| "lint/style/noHeadElement"
	| "lint/style/noImplicitBoolean"
	| "lint/style/noInferrableTypes"
	| "lint/style/noMagicNumbers"
	| "lint/style/noNamespace"
	| "lint/style/noNegationElse"
	| "lint/style/noNestedTernary"
	| "lint/style/noNonNullAssertion"
	| "lint/style/noParameterAssign"
	| "lint/style/noParameterProperties"
	| "lint/style/noProcessEnv"
	| "lint/style/noRestrictedGlobals"
	| "lint/style/noRestrictedImports"
	| "lint/style/noRestrictedTypes"
	| "lint/style/noShoutyConstants"
	| "lint/style/noSubstr"
	| "lint/style/noUnusedTemplateLiteral"
	| "lint/style/noUselessElse"
	| "lint/style/noValueAtRule"
	| "lint/style/noYodaExpression"
	| "lint/style/useArrayLiterals"
	| "lint/style/useAsConstAssertion"
	| "lint/style/useAtIndex"
	| "lint/style/useBlockStatements"
	| "lint/style/useCollapsedElseIf"
	| "lint/style/useCollapsedIf"
	| "lint/style/useComponentExportOnlyModules"
	| "lint/style/useConsistentArrayType"
	| "lint/style/useConsistentBuiltinInstantiation"
	| "lint/style/useConsistentCurlyBraces"
	| "lint/style/useConsistentMemberAccessibility"
	| "lint/style/useConsistentObjectDefinitions"
	| "lint/style/useConsistentTypeDefinitions"
	| "lint/style/useConst"
	| "lint/style/useDefaultParameterLast"
	| "lint/style/useDefaultSwitchClause"
	| "lint/style/useDeprecatedReason"
	| "lint/style/useEnumInitializers"
	| "lint/style/useExplicitLengthCheck"
	| "lint/style/useExponentiationOperator"
	| "lint/style/useExportType"
	| "lint/style/useExportsLast"
	| "lint/style/useFilenamingConvention"
	| "lint/style/useForOf"
	| "lint/style/useFragmentSyntax"
	| "lint/style/useGraphqlNamingConvention"
	| "lint/style/useGroupedAccessorPairs"
	| "lint/style/useImportType"
	| "lint/style/useLiteralEnumMembers"
	| "lint/style/useNamingConvention"
	| "lint/style/useNodeAssertStrict"
	| "lint/style/useNodejsImportProtocol"
	| "lint/style/useNumberNamespace"
	| "lint/style/useNumericSeparators"
	| "lint/style/useObjectSpread"
	| "lint/style/useReactFunctionComponents"
	| "lint/style/useReadonlyClassProperties"
	| "lint/style/useSelfClosingElements"
	| "lint/style/useShorthandArrayType"
	| "lint/style/useShorthandAssign"
	| "lint/style/useShorthandFunctionType"
	| "lint/style/useSingleCaseStatement"
	| "lint/style/useSingleVarDeclarator"
	| "lint/style/useSymbolDescription"
	| "lint/style/useTemplate"
	| "lint/style/useThrowNewError"
	| "lint/style/useThrowOnlyError"
	| "lint/style/useTrimStartEnd"
	| "lint/style/useUnifiedTypeSignatures"
	| "lint/suspicious/noAlert"
	| "lint/suspicious/noApproximativeNumericConstant"
	| "lint/suspicious/noArrayIndexKey"
	| "lint/suspicious/noAssignInExpressions"
	| "lint/suspicious/noAsyncPromiseExecutor"
	| "lint/suspicious/noBiomeFirstException"
	| "lint/suspicious/noBitwiseOperators"
	| "lint/suspicious/noCatchAssign"
	| "lint/suspicious/noClassAssign"
	| "lint/suspicious/noCommentText"
	| "lint/suspicious/noCompareNegZero"
	| "lint/suspicious/noConfusingLabels"
	| "lint/suspicious/noConfusingVoidType"
	| "lint/suspicious/noConsole"
	| "lint/suspicious/noConstEnum"
	| "lint/suspicious/noConstantBinaryExpressions"
	| "lint/suspicious/noControlCharactersInRegex"
	| "lint/suspicious/noDebugger"
	| "lint/suspicious/noDocumentCookie"
	| "lint/suspicious/noDocumentImportInPage"
	| "lint/suspicious/noDoubleEquals"
	| "lint/suspicious/noDuplicateAtImportRules"
	| "lint/suspicious/noDuplicateCase"
	| "lint/suspicious/noDuplicateClassMembers"
	| "lint/suspicious/noDuplicateCustomProperties"
	| "lint/suspicious/noDuplicateElseIf"
	| "lint/suspicious/noDuplicateFields"
	| "lint/suspicious/noDuplicateFontNames"
	| "lint/suspicious/noDuplicateJsxProps"
	| "lint/suspicious/noDuplicateObjectKeys"
	| "lint/suspicious/noDuplicateParameters"
	| "lint/suspicious/noDuplicateProperties"
	| "lint/suspicious/noDuplicateSelectorsKeyframeBlock"
	| "lint/suspicious/noDuplicateTestHooks"
	| "lint/suspicious/noEmptyBlock"
	| "lint/suspicious/noEmptyBlockStatements"
	| "lint/suspicious/noEmptyInterface"
	| "lint/suspicious/noEvolvingTypes"
	| "lint/suspicious/noExplicitAny"
	| "lint/suspicious/noExportsInTest"
	| "lint/suspicious/noExtraNonNullAssertion"
	| "lint/suspicious/noFallthroughSwitchClause"
	| "lint/suspicious/noFocusedTests"
	| "lint/suspicious/noFunctionAssign"
	| "lint/suspicious/noGlobalAssign"
	| "lint/suspicious/noGlobalIsFinite"
	| "lint/suspicious/noGlobalIsNan"
	| "lint/suspicious/noHeadImportInDocument"
	| "lint/suspicious/noImplicitAnyLet"
	| "lint/suspicious/noImportAssign"
	| "lint/suspicious/noImportantInKeyframe"
	| "lint/suspicious/noIrregularWhitespace"
	| "lint/suspicious/noLabelVar"
	| "lint/suspicious/noMisleadingCharacterClass"
	| "lint/suspicious/noMisleadingInstantiator"
	| "lint/suspicious/noMisplacedAssertion"
	| "lint/suspicious/noMisrefactoredShorthandAssign"
	| "lint/suspicious/noNonNullAssertedOptionalChain"
	| "lint/suspicious/noOctalEscape"
	| "lint/suspicious/noPrototypeBuiltins"
	| "lint/suspicious/noQuickfixBiome"
	| "lint/suspicious/noReactSpecificProps"
	| "lint/suspicious/noRedeclare"
	| "lint/suspicious/noRedundantUseStrict"
	| "lint/suspicious/noSelfCompare"
	| "lint/suspicious/noShadowRestrictedNames"
	| "lint/suspicious/noShorthandPropertyOverrides"
	| "lint/suspicious/noSkippedTests"
	| "lint/suspicious/noSparseArray"
	| "lint/suspicious/noSuspiciousSemicolonInJsx"
	| "lint/suspicious/noTemplateCurlyInString"
	| "lint/suspicious/noThenProperty"
	| "lint/suspicious/noTsIgnore"
	| "lint/suspicious/noUnassignedVariables"
	| "lint/suspicious/noUnknownAtRules"
	| "lint/suspicious/noUnsafeDeclarationMerging"
	| "lint/suspicious/noUnsafeNegation"
	| "lint/suspicious/noUselessEscapeInString"
	| "lint/suspicious/noUselessRegexBackrefs"
	| "lint/suspicious/noVar"
	| "lint/suspicious/noWith"
	| "lint/suspicious/useAdjacentOverloadSignatures"
	| "lint/suspicious/useAwait"
	| "lint/suspicious/useBiomeIgnoreFolder"
	| "lint/suspicious/useDefaultSwitchClauseLast"
	| "lint/suspicious/useErrorMessage"
	| "lint/suspicious/useGetterReturn"
	| "lint/suspicious/useGoogleFontDisplay"
	| "lint/suspicious/useGuardForIn"
	| "lint/suspicious/useIsArray"
	| "lint/suspicious/useIterableCallbackReturn"
	| "lint/suspicious/useNamespaceKeyword"
	| "lint/suspicious/useNumberToFixedDigitsArgument"
	| "lint/suspicious/useStaticResponseMethods"
	| "lint/suspicious/useStrictMode"
	| "assist/source/useSortedKeys"
	| "assist/source/useSortedProperties"
	| "assist/source/useSortedAttributes"
	| "assist/source/organizeImports"
	| "syntax/correctness/noTypeOnlyImportAttributes"
	| "syntax/correctness/noSuperWithoutExtends"
	| "syntax/correctness/noInitializerWithDefinite"
	| "syntax/correctness/noDuplicatePrivateClassMembers"
	| "files/missingHandler"
	| "format"
	| "check"
	| "ci"
	| "stdin"
	| "init"
	| "configuration"
	| "assist"
	| "migrate"
	| "deserialize"
	| "plugin"
	| "project"
	| "search"
	| "internalError/io"
	| "internalError/fs"
	| "internalError/panic"
	| "reporter/parse"
	| "reporter/format"
	| "reporter/violations"
	| "parse"
	| "lint"
	| "lint/a11y"
	| "lint/complexity"
	| "lint/correctness"
	| "lint/nursery"
	| "lint/performance"
	| "lint/security"
	| "lint/style"
	| "lint/suspicious"
	| "lint/plugin"
	| "suppressions/parse"
	| "suppressions/unknownGroup"
	| "suppressions/unknownRule"
	| "suppressions/unknownAction"
	| "suppressions/unused"
	| "suppressions/incorrect"
	| "args/fileNotFound"
	| "flags/invalid"
	| "semanticTests";
export interface Location {
	path?: Resource_for_String;
	sourceCode?: string;
	span?: TextRange;
}
export type MarkupBuf = MarkupNodeBuf[];
/**
 * The severity to associate to a diagnostic.
 */
export type Severity = "hint" | "information" | "warning" | "error" | "fatal";
export type DiagnosticTags = DiagnosticTag[];
/**
	* Serializable representation of a [Diagnostic](super::Diagnostic) advice

See the [Visitor] trait for additional documentation on all the supported advice types. 
	 */
export type Advice =
	| { log: [LogCategory, MarkupBuf] }
	| { list: MarkupBuf[] }
	| { frame: Location }
	| { diff: TextEdit }
	| { backtrace: [MarkupBuf, Backtrace] }
	| { command: string }
	| { group: [MarkupBuf, Advices] };
/**
 * Represents the resource a diagnostic is associated with.
 */
export type Resource_for_String = "argv" | "memory" | { file: string };
export type TextRange = [TextSize, TextSize];
export interface MarkupNodeBuf {
	content: string;
	elements: MarkupElement[];
}
/**
 * Internal enum used to automatically generate bit offsets for [DiagnosticTags] and help with the implementation of `serde` and `schemars` for tags.
 */
export type DiagnosticTag =
	| "fixable"
	| "internal"
	| "unnecessaryCode"
	| "deprecatedCode"
	| "verbose";
/**
 * The category for a log advice, defines how the message should be presented to the user.
 */
export type LogCategory = "none" | "info" | "warn" | "error";
export interface TextEdit {
	dictionary: string;
	ops: CompressedOp[];
}
export type Backtrace = BacktraceFrame[];
export type TextSize = number;
/**
 * Enumeration of all the supported markup elements
 */
export type MarkupElement =
	| "Emphasis"
	| "Dim"
	| "Italic"
	| "Underline"
	| "Error"
	| "Success"
	| "Warn"
	| "Info"
	| "Debug"
	| "Trace"
	| "Inverse"
	| { Hyperlink: { href: string } };
export type CompressedOp =
	| { diffOp: DiffOp }
	| { equalLines: { line_count: number } };
/**
 * Serializable representation of a backtrace frame.
 */
export interface BacktraceFrame {
	ip: number;
	symbols: BacktraceSymbol[];
}
export type DiffOp =
	| { equal: { range: TextRange } }
	| { insert: { range: TextRange } }
	| { delete: { range: TextRange } };
/**
 * Serializable representation of a backtrace frame symbol.
 */
export interface BacktraceSymbol {
	colno?: number;
	filename?: string;
	lineno?: number;
	name?: string;
}
export interface OpenProjectParams {
	/**
	 * Whether the folder should be opened as a project, even if no `biome.json` can be found.
	 */
	openUninitialized: boolean;
	/**
	 * The path to open
	 */
	path: BiomePath;
}
export interface OpenProjectResult {
	/**
	 * A unique identifier for this project
	 */
	projectKey: ProjectKey;
}
export interface ScanProjectParams {
	/**
	 * Forces scanning of the folder, even if it is already being watched.
	 */
	force: boolean;
	projectKey: ProjectKey;
	scanKind: ScanKind;
	verbose: boolean;
	/**
	* Whether the watcher should watch this path.

Does nothing if the watcher is already watching this path. 
	 */
	watch: boolean;
}
export type ScanKind =
	| "noScanner"
	| "knownFiles"
	| {
			targetedKnownFiles: {
				/**
				 * Determines whether the file scanner should descend into subdirectories of the target paths.
				 */
				descendFromTargets: boolean;
				/**
	* The paths to target by the scanner.

If a target path indicates a folder, all files within are scanned as well.

Target paths must be absolute. 
	 */
				targetPaths: BiomePath[];
			};
	  }
	| "project";
export interface ScanProjectResult {
	/**
	 * A list of child configuration files found inside the project
	 */
	configurationFiles: BiomePath[];
	/**
	 * Diagnostics reported while scanning the project.
	 */
	diagnostics: Diagnostic[];
	/**
	 * Duration of the scan.
	 */
	duration: Duration;
}
export interface Duration {
	nanos: number;
	secs: number;
}
export interface OpenFileParams {
	content: FileContent;
	documentFileSource?: DocumentFileSource;
	path: BiomePath;
	/**
	* Set to `true` to persist the node cache used during parsing, in order to speed up subsequent reparsing if the document has been edited.

This should only be enabled if reparsing is to be expected, such as when the file is opened through the LSP Proxy. 
	 */
	persistNodeCache?: boolean;
	projectKey: ProjectKey;
}
export type FileContent =
	| { content: string; type: "fromClient"; version: number }
	| { type: "fromServer" };
export type DocumentFileSource =
	| "Ignore"
	| "Unknown"
	| { Js: JsFileSource }
	| { Json: JsonFileSource }
	| { Css: CssFileSource }
	| { Graphql: GraphqlFileSource }
	| { Html: HtmlFileSource }
	| { Grit: GritFileSource };
export interface JsFileSource {
	/**
	 * Used to mark if the JavaScript is embedded inside some particular files. This affects the parsing. For example, if inside an Astro file, a top-level return statement is allowed.
	 */
	embedding_kind: EmbeddingKind;
	language: Language;
	module_kind: ModuleKind;
	variant: LanguageVariant;
	version: LanguageVersion;
}
export interface JsonFileSource {
	allowComments: boolean;
	allowTrailingCommas: boolean;
	variant: JsonFileVariant;
}
export interface CssFileSource {
	variant: CssVariant;
}
export interface GraphqlFileSource {
	variant: GraphqlVariant;
}
export interface HtmlFileSource {
	variant: HtmlVariant;
}
export interface GritFileSource {
	variant: GritVariant;
}
export type EmbeddingKind = "Astro" | "Vue" | "Svelte" | "None";
export type Language =
	| "javaScript"
	| { typeScript: { definition_file: boolean } };
/**
 * Is the source file an ECMAScript Module or Script. Changes the parsing semantic.
 */
export type ModuleKind = "script" | "module";
export type LanguageVariant = "standard" | "standardRestricted" | "jsx";
/**
	* Enum of the different ECMAScript standard versions. The versions are ordered in increasing order; The newest version comes last.

Defaults to the latest stable ECMAScript standard. 
	 */
export type LanguageVersion = "eS2022" | "eSNext";
/**
 * It represents the extension of the file
 */
export type JsonFileVariant = "standard" | "jsonc";
/**
	* The style of CSS contained in the file.

Currently, Biome aims to be compatible with the latest Recommendation level standards.

It also supports Tailwind CSS syntax additions, when the parser option is enabled. 
	 */
export type CssVariant = "standard";
/**
 * The style of GraphQL contained in the file.
 */
export type GraphqlVariant = "standard";
export type HtmlVariant =
	| { Standard: HtmlTextExpressions }
	| "Astro"
	| "Vue"
	| "Svelte";
export type GritVariant = "Standard";
export type HtmlTextExpressions = "None" | "Single" | "Double";
export interface OpenFileResult {
	diagnostics: Diagnostic[];
}
export interface ChangeFileParams {
	content: string;
	path: BiomePath;
	projectKey: ProjectKey;
	version: number;
}
export interface ChangeFileResult {
	diagnostics: Diagnostic[];
}
export interface CloseFileParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface FileExitsParams {
	filePath: BiomePath;
}
export interface PathIsIgnoredParams {
	/**
	 * Whether the path is ignored for specific features e.g. `formatter.includes`. When this field is empty, Biome checks only `files.includes`.
	 */
	features: FeatureName;
	/**
	 * Controls how to ignore check should be done
	 */
	ignoreKind?: IgnoreKind;
	/**
	 * The path to inspect
	 */
	path: BiomePath;
	projectKey: ProjectKey;
}
export type IgnoreKind = "path" | "ancestors";
export interface UpdateModuleGraphParams {
	path: BiomePath;
	/**
	 * The kind of update to apply to the module graph
	 */
	updateKind: UpdateKind;
}
export type UpdateKind = "addOrUpdate" | "remove";
export interface GetSyntaxTreeParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface GetSyntaxTreeResult {
	ast: string;
	cst: string;
}
export interface CheckFileSizeParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface CheckFileSizeResult {
	fileSize: number;
	limit: number;
}
export interface GetFileContentParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface GetControlFlowGraphParams {
	cursor: TextSize;
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface GetFormatterIRParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface GetTypeInfoParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface GetRegisteredTypesParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface GetSemanticModelParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface GetModuleGraphParams {}
export interface GetModuleGraphResult {
	data: Record<string, SerializedJsModuleInfo>;
}
export interface SerializedJsModuleInfo {
	/**
	 * Dynamic imports.
	 */
	dynamicImports: string[];
	/**
	 * Exported symbols.
	 */
	exports: string[];
	/**
	* Map of all the paths from static imports in the module.

Maps from the source specifier name to the absolute path it resolves to. Specifiers that could not be resolved to an absolute will map to the specifier itself.

## Example

```json { "./foo": "/absolute/path/to/foo.js", "react": "react" } ``` 
	 */
	staticImportPaths: Record<string, string>;
	/**
	* Map of all static imports found in the module.

Maps from the local imported name to the absolute path it resolves to. 
	 */
	staticImports: Record<string, string>;
}
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	/**
	 * Rules to apply on top of the configuration
	 */
	enabledRules?: AnalyzerSelector[];
	only?: AnalyzerSelector[];
	path: BiomePath;
	projectKey: ProjectKey;
	/**
	 * When `false` the diagnostics, don't have code frames of the code actions (fixes, suppressions, etc.)
	 */
	pullCodeActions: boolean;
	skip?: AnalyzerSelector[];
}
export type RuleCategories = RuleCategory[];
export type AnalyzerSelector = string;
export type RuleCategory = "syntax" | "lint" | "action" | "transformation";
export interface PullDiagnosticsResult {
	diagnostics: Diagnostic[];
	errors: number;
	skippedDiagnostics: number;
}
export interface PullActionsParams {
	categories?: RuleCategories;
	enabledRules?: AnalyzerSelector[];
	only?: AnalyzerSelector[];
	path: BiomePath;
	projectKey: ProjectKey;
	range?: TextRange;
	skip?: AnalyzerSelector[];
	suppressionReason?: string;
}
export interface PullActionsResult {
	actions: CodeAction[];
}
export interface CodeAction {
	category: ActionCategory;
	offset?: TextSize;
	ruleName?: [string, string];
	suggestion: CodeSuggestion;
}
/**
	* The category of a code action, this type maps directly to the [CodeActionKind] type in the Language Server Protocol specification

[CodeActionKind]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind 
	 */
export type ActionCategory =
	| { quickFix: string }
	| { refactor: RefactorKind }
	| { source: SourceActionKind }
	| { other: OtherActionCategory };
/**
 * A Suggestion that is provided by Biome's linter, and can be reported to the user, and can be automatically applied if it has the right [`Applicability`].
 */
export interface CodeSuggestion {
	applicability: Applicability;
	labels: TextRange[];
	msg: MarkupBuf;
	span: TextRange;
	suggestion: TextEdit;
}
/**
	* The sub-category of a refactor code action.

[Check the LSP spec](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind) for more information: 
	 */
export type RefactorKind =
	| "none"
	| "extract"
	| "inline"
	| "rewrite"
	| { other: string };
/**
 * The sub-category of a source code action
 */
export type SourceActionKind =
	| "fixAll"
	| "none"
	| "organizeImports"
	| { other: string };
export type OtherActionCategory =
	| "inlineSuppression"
	| "toplevelSuppression"
	| { generic: string };
/**
 * Indicates how a tool should manage this suggestion.
 */
export type Applicability = "always" | "maybeIncorrect";
export interface FormatFileParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface Printed {
	code: string;
	range?: TextRange;
	sourcemap: SourceMarker[];
	verbatimRanges: TextRange[];
}
/**
 * Lightweight sourcemap marker between source and output tokens
 */
export interface SourceMarker {
	/**
	 * Position of the marker in the output code
	 */
	dest: TextSize;
	/**
	 * Position of the marker in the original source
	 */
	source: TextSize;
}
export interface FormatRangeParams {
	path: BiomePath;
	projectKey: ProjectKey;
	range: TextRange;
}
export interface FormatOnTypeParams {
	offset: TextSize;
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface FixFileParams {
	/**
	 * Rules to apply to the file
	 */
	enabledRules?: AnalyzerSelector[];
	fixFileMode: FixFileMode;
	only?: AnalyzerSelector[];
	path: BiomePath;
	projectKey: ProjectKey;
	ruleCategories: RuleCategories;
	shouldFormat: boolean;
	skip?: AnalyzerSelector[];
	suppressionReason?: string;
}
/**
 * Which fixes should be applied during the analyzing phase
 */
export type FixFileMode =
	| "safeFixes"
	| "safeAndUnsafeFixes"
	| "applySuppressions";
export interface FixFileResult {
	/**
	 * List of all the code actions applied to the file
	 */
	actions: FixAction[];
	/**
	 * New source code for the file with all fixes applied
	 */
	code: string;
	/**
	 * Number of errors
	 */
	errors: number;
	/**
	 * number of skipped suggested fixes
	 */
	skippedSuggestedFixes: number;
}
export interface FixAction {
	/**
	 * Source range at which this action was applied
	 */
	range: TextRange;
	/**
	 * Name of the rule group and rule that emitted this code action
	 */
	rule_name?: [string, string];
}
export interface RenameParams {
	newName: string;
	path: BiomePath;
	projectKey: ProjectKey;
	symbolAt: TextSize;
}
export interface RenameResult {
	/**
	 * List of text edit operations to apply on the source code
	 */
	indels: TextEdit;
	/**
	 * Range of source code modified by this rename operation
	 */
	range: TextRange;
}
export interface ParsePatternParams {
	defaultLanguage: GritTargetLanguage;
	pattern: string;
}
export type GritTargetLanguage = "CSS" | "JavaScript";
export interface ParsePatternResult {
	patternId: PatternId;
}
export type PatternId = string;
export interface SearchPatternParams {
	path: BiomePath;
	pattern: PatternId;
	projectKey: ProjectKey;
}
export interface SearchResults {
	matches: TextRange[];
	path: BiomePath;
}
export interface DropPatternParams {
	pattern: PatternId;
}
export interface Workspace {
	fileFeatures(params: SupportsFeatureParams): Promise<FileFeaturesResult>;
	updateSettings(params: UpdateSettingsParams): Promise<UpdateSettingsResult>;
	openProject(params: OpenProjectParams): Promise<OpenProjectResult>;
	scanProject(params: ScanProjectParams): Promise<ScanProjectResult>;
	openFile(params: OpenFileParams): Promise<OpenFileResult>;
	changeFile(params: ChangeFileParams): Promise<ChangeFileResult>;
	closeFile(params: CloseFileParams): Promise<void>;
	fileExists(params: FileExitsParams): Promise<boolean>;
	isPathIgnored(params: PathIsIgnoredParams): Promise<boolean>;
	updateModuleGraph(params: UpdateModuleGraphParams): Promise<void>;
	getSyntaxTree(params: GetSyntaxTreeParams): Promise<GetSyntaxTreeResult>;
	checkFileSize(params: CheckFileSizeParams): Promise<CheckFileSizeResult>;
	getFileContent(params: GetFileContentParams): Promise<string>;
	getControlFlowGraph(params: GetControlFlowGraphParams): Promise<string>;
	getFormatterIr(params: GetFormatterIRParams): Promise<string>;
	getTypeInfo(params: GetTypeInfoParams): Promise<string>;
	getRegisteredTypes(params: GetRegisteredTypesParams): Promise<string>;
	getSemanticModel(params: GetSemanticModelParams): Promise<string>;
	getModuleGraph(params: GetModuleGraphParams): Promise<GetModuleGraphResult>;
	pullDiagnostics(
		params: PullDiagnosticsParams,
	): Promise<PullDiagnosticsResult>;
	pullActions(params: PullActionsParams): Promise<PullActionsResult>;
	formatFile(params: FormatFileParams): Promise<Printed>;
	formatRange(params: FormatRangeParams): Promise<Printed>;
	formatOnType(params: FormatOnTypeParams): Promise<Printed>;
	fixFile(params: FixFileParams): Promise<FixFileResult>;
	rename(params: RenameParams): Promise<RenameResult>;
	parsePattern(params: ParsePatternParams): Promise<ParsePatternResult>;
	searchPattern(params: SearchPatternParams): Promise<SearchResults>;
	dropPattern(params: DropPatternParams): Promise<void>;
	destroy(): void;
}
export function createWorkspace(transport: Transport): Workspace {
	return {
		fileFeatures(params) {
			return transport.request("biome/file_features", params);
		},
		updateSettings(params) {
			return transport.request("biome/update_settings", params);
		},
		openProject(params) {
			return transport.request("biome/open_project", params);
		},
		scanProject(params) {
			return transport.request("biome/scan_project", params);
		},
		openFile(params) {
			return transport.request("biome/open_file", params);
		},
		changeFile(params) {
			return transport.request("biome/change_file", params);
		},
		closeFile(params) {
			return transport.request("biome/close_file", params);
		},
		fileExists(params) {
			return transport.request("biome/file_exists", params);
		},
		isPathIgnored(params) {
			return transport.request("biome/is_path_ignored", params);
		},
		updateModuleGraph(params) {
			return transport.request("biome/update_module_graph", params);
		},
		getSyntaxTree(params) {
			return transport.request("biome/get_syntax_tree", params);
		},
		checkFileSize(params) {
			return transport.request("biome/check_file_size", params);
		},
		getFileContent(params) {
			return transport.request("biome/get_file_content", params);
		},
		getControlFlowGraph(params) {
			return transport.request("biome/get_control_flow_graph", params);
		},
		getFormatterIr(params) {
			return transport.request("biome/get_formatter_ir", params);
		},
		getTypeInfo(params) {
			return transport.request("biome/get_type_info", params);
		},
		getRegisteredTypes(params) {
			return transport.request("biome/get_registered_types", params);
		},
		getSemanticModel(params) {
			return transport.request("biome/get_semantic_model", params);
		},
		getModuleGraph(params) {
			return transport.request("biome/get_module_graph", params);
		},
		pullDiagnostics(params) {
			return transport.request("biome/pull_diagnostics", params);
		},
		pullActions(params) {
			return transport.request("biome/pull_actions", params);
		},
		formatFile(params) {
			return transport.request("biome/format_file", params);
		},
		formatRange(params) {
			return transport.request("biome/format_range", params);
		},
		formatOnType(params) {
			return transport.request("biome/format_on_type", params);
		},
		fixFile(params) {
			return transport.request("biome/fix_file", params);
		},
		rename(params) {
			return transport.request("biome/rename", params);
		},
		parsePattern(params) {
			return transport.request("biome/parse_pattern", params);
		},
		searchPattern(params) {
			return transport.request("biome/search_pattern", params);
		},
		dropPattern(params) {
			return transport.request("biome/drop_pattern", params);
		},
		destroy() {
			transport.destroy();
		},
	};
}
