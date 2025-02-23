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
export type FeatureKind = "format" | "lint" | "search" | "assist" | "debug";
export interface FileFeaturesResult {
	featuresSupported: Map<FeatureKind, SupportKind>;
}
export interface UpdateSettingsParams {
	configuration: Configuration;
	gitignoreMatches: string[];
	projectKey: ProjectKey;
	vcsBasePath?: BiomePath;
	workspaceDirectory?: BiomePath;
}
/**
 * The configuration that is contained inside the file `biome.json`
 */
export interface Configuration {
	/**
	 * A field for the [JSON schema](https://json-schema.org/) specification
	 */
	$schema?: string;
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
	extends?: string[];
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
export interface AssistConfiguration {
	/**
	 * Whether Biome should fail in CLI if the assist were not applied to the code.
	 */
	actions?: Actions;
	/**
	 * Whether Biome should enable assist via LSP.
	 */
	enabled?: Bool;
	/**
	 * A list of glob patterns. Biome will include files/folders that will match these patterns.
	 */
	includes?: Glob[];
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
/**
 * The configuration of the filesystem
 */
export interface FilesConfiguration {
	/**
	 * Tells Biome to not emit diagnostics when handling files that doesn't know
	 */
	ignoreUnknown?: Bool;
	/**
	 * A list of glob patterns. Biome will handle only those files/folders that will match these patterns.
	 */
	includes?: Glob[];
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
	 * Stores whether formatting should be allowed to proceed if a given file has syntax errors
	 */
	formatWithErrors?: Bool;
	/**
	 * A list of glob patterns. The formatter will include files/folders that will match these patterns.
	 */
	includes?: Glob[];
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
	 * Whether to enforce collapsing object literals when possible. Defaults to preserve.
	 */
	objectWrap?: ObjectWrap;
	/**
	* Use any `.editorconfig` files to configure the formatter. Configuration in `biome.json` will override `.editorconfig` configuration.

Default: `false`. 
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
	/**
	 * HTML formatter options
	 */
	formatter?: HtmlFormatterConfiguration;
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
	domains?: Record<RuleDomain, RuleDomainValue>;
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: Bool;
	/**
	 * A list of glob patterns. The analyzer will handle only those files/folders that will match these patterns.
	 */
	includes?: Glob[];
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
export type Glob = string;
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
	 * The type of line ending applied to CSS (and its super languages) files.
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
}
export type MaxSize = number;
export type AttributePosition = "auto" | "multiline";
/**
 * Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
 */
export type BracketSameLine = boolean;
export type BracketSpacing = boolean;
export type IndentStyle = "tab" | "space";
export type IndentWidth = number;
export type LineEnding = "lf" | "crlf" | "cr";
/**
	* Validated value for the `line_width` formatter options

The allowed range of values is 1..=320 
	 */
export type LineWidth = number;
export type ObjectWrap = "preserve" | "collapse";
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
	 * The type of line ending applied to GraphQL files.
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
	 * The type of line ending applied to HTML (and its super languages) files.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to HTML (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * Whether to account for whitespace sensitivity when formatting HTML (and its super languages). Defaults to "css".
	 */
	whitespaceSensitivity?: WhitespaceSensitivity;
}
/**
 * Options that changes how the HTML parser behaves
 */
export type HtmlParserConfiguration = null;
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
	 * The type of line ending applied to JavaScript (and its super languages) files.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to JavaScript (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * Whether to enforce collapsing object literals when possible. Defaults to preserve.
	 */
	objectWrap?: ObjectWrap;
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
	 * When enabled, files like `.js`/`.ts` can contain JSX syntax. Defaults to `true`.
	 */
	jsxEverywhere?: Bool;
	/**
	* It enables the experimental and unsafe parsing of parameter decorators

These decorators belong to an old proposal, and they are subject to change. 
	 */
	unsafeParameterDecoratorsEnabled?: Bool;
}
/**
 * Linter options specific to the JSON linter
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
	 * Whether to expand arrays and objects on multiple lines. When set to `always`, these literals are formatted on multiple lines, regardless of length of the list. When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "followSource".
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
	 * The type of line ending applied to JSON (and its super languages) files.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	/**
	 * Whether to enforce collapsing object literals when possible. Defaults to preserve.
	 */
	objectWrap?: ObjectWrap;
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
}
export type PluginConfiguration = string;
export type VcsClientKind = "git";
/**
 * A list of rules that belong to this group
 */
export interface Source {
	/**
	 * Provides a whole-source code action to sort the imports in the file using import groups and natural ordering.
	 */
	organizeImports?: RuleAssistConfiguration_for_Options;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce attribute sorting in JSX elements.
	 */
	useSortedAttributes?: RuleAssistConfiguration_for_Null;
	/**
	 * Sorts the keys of a JSON object in natural order
	 */
	useSortedKeys?: RuleAssistConfiguration_for_Null;
	/**
	 * Enforce ordering of CSS properties and nested rules.
	 */
	useSortedProperties?: RuleAssistConfiguration_for_Null;
}
export type QuoteStyle = "double" | "single";
/**
	* Whether to indent the content of `<script>` and `<style>` tags for HTML-ish templating languages (Vue, Svelte, etc.).

When true, the content of `<script>` and `<style>` tags will be indented one level. 
	 */
export type IndentScriptAndStyle = boolean;
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
export type QuoteProperties = "asNeeded" | "preserve";
export type Semicolons = "always" | "asNeeded";
/**
 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures.
 */
export type TrailingCommas = "all" | "es5" | "none";
export type Expand = "always" | "followSource";
export type TrailingCommas2 = "none" | "all";
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
	/**
	 * Whether to enforce collapsing object literals when possible. Defaults to preserve.
	 */
	objectWrap?: ObjectWrap;
}
export type OverrideGlobs = Glob[];
export interface OverrideLinterConfiguration {
	/**
	 * List of rules
	 */
	domains?: Record<RuleDomain, RuleDomainValue>;
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: Bool;
	/**
	 * List of rules
	 */
	rules?: Rules;
}
export type RuleAssistConfiguration_for_Options =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOptions_for_Options;
export type RuleAssistConfiguration_for_Null =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOptions_for_Null;
export type GroupPlainConfiguration = "off" | "on" | "info" | "warn" | "error";
/**
 * A list of rules that belong to this group
 */
export interface A11y {
	/**
	 * Enforce that the accessKey attribute is not used on any HTML element.
	 */
	noAccessKey?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that aria-hidden="true" is not set on focusable elements.
	 */
	noAriaHiddenOnFocusable?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
	 */
	noAriaUnsupportedElements?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that autoFocus prop is not used on elements.
	 */
	noAutofocus?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow target="_blank" attribute without rel="noreferrer"
	 */
	noBlankTarget?: RuleFixConfiguration_for_AllowDomainOptions;
	/**
	 * Enforces that no distracting elements are used.
	 */
	noDistractingElements?: RuleFixConfiguration_for_Null;
	/**
	 * The scope prop should be used only on \<th> elements.
	 */
	noHeaderScope?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
	 */
	noInteractiveElementToNoninteractiveRole?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that a label element or component has a text label and an associated input.
	 */
	noLabelWithoutControl?: RuleConfiguration_for_NoLabelWithoutControlOptions;
	/**
	 * Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveElementToInteractiveRole?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that tabIndex is not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveTabindex?: RuleFixConfiguration_for_Null;
	/**
	 * Prevent the usage of positive integers on tabIndex property
	 */
	noPositiveTabindex?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce img alt prop does not contain the word "image", "picture", or "photo".
	 */
	noRedundantAlt?: RuleConfiguration_for_Null;
	/**
	 * Enforce explicit role property is not the same as implicit/default role property on an element.
	 */
	noRedundantRoles?: RuleFixConfiguration_for_Null;
	/**
	 * Enforces the usage of the title element for the svg element.
	 */
	noSvgWithoutTitle?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce that all elements that require alternative text have meaningful information to relay back to the end user.
	 */
	useAltText?: RuleConfiguration_for_Null;
	/**
	 * Enforce that anchors have content and that the content is accessible to screen readers.
	 */
	useAnchorContent?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant.
	 */
	useAriaActivedescendantWithTabindex?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
	 */
	useAriaPropsForRole?: RuleConfiguration_for_Null;
	/**
	 * Enforces the usage of the attribute type for the element button
	 */
	useButtonType?: RuleConfiguration_for_Null;
	/**
	 * Elements with an interactive role and interaction handlers must be focusable.
	 */
	useFocusableInteractive?: RuleConfiguration_for_Null;
	/**
	 * Disallow a missing generic family keyword within font families.
	 */
	useGenericFontNames?: RuleConfiguration_for_Null;
	/**
	 * Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.
	 */
	useHeadingContent?: RuleConfiguration_for_Null;
	/**
	 * Enforce that html element has lang attribute.
	 */
	useHtmlLang?: RuleConfiguration_for_Null;
	/**
	 * Enforces the usage of the attribute title for the element iframe.
	 */
	useIframeTitle?: RuleConfiguration_for_Null;
	/**
	 * Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress.
	 */
	useKeyWithClickEvents?: RuleConfiguration_for_Null;
	/**
	 * Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur.
	 */
	useKeyWithMouseEvents?: RuleConfiguration_for_Null;
	/**
	 * Enforces that audio and video elements must have a track for captions.
	 */
	useMediaCaption?: RuleConfiguration_for_Null;
	/**
	 * It detects the use of role attributes in JSX elements and suggests using semantic elements instead.
	 */
	useSemanticElements?: RuleConfiguration_for_Null;
	/**
	 * Enforce that all anchors are valid, and they are navigable elements.
	 */
	useValidAnchor?: RuleConfiguration_for_Null;
	/**
	 * Ensures that ARIA properties aria-* are all valid.
	 */
	useValidAriaProps?: RuleFixConfiguration_for_Null;
	/**
	 * Elements with ARIA roles must use a valid, non-abstract ARIA role.
	 */
	useValidAriaRole?: RuleFixConfiguration_for_ValidAriaRoleOptions;
	/**
	 * Enforce that ARIA state and property values are valid.
	 */
	useValidAriaValues?: RuleConfiguration_for_Null;
	/**
	 * Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country.
	 */
	useValidLang?: RuleConfiguration_for_Null;
}
/**
 * A list of rules that belong to this group
 */
export interface Complexity {
	/**
	 * Disallow primitive type aliases and misleading types.
	 */
	noBannedTypes?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow empty type parameters in type aliases and interfaces.
	 */
	noEmptyTypeParameters?: RuleConfiguration_for_Null;
	/**
	 * Disallow functions that exceed a given Cognitive Complexity score.
	 */
	noExcessiveCognitiveComplexity?: RuleConfiguration_for_ComplexityOptions;
	/**
	 * This rule enforces a maximum depth to nested describe() in test files.
	 */
	noExcessiveNestedTestSuites?: RuleConfiguration_for_Null;
	/**
	 * Disallow unnecessary boolean casts
	 */
	noExtraBooleanCast?: RuleFixConfiguration_for_Null;
	/**
	 * Prefer for...of statement instead of Array.forEach.
	 */
	noForEach?: RuleConfiguration_for_NoForEachOptions;
	/**
	 * Disallow unclear usage of consecutive space characters in regular expression literals
	 */
	noMultipleSpacesInRegularExpressionLiterals?: RuleFixConfiguration_for_Null;
	/**
	 * This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
	 */
	noStaticOnlyClass?: RuleConfiguration_for_Null;
	/**
	 * Disallow this and super in static contexts.
	 */
	noThisInStatic?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unnecessary catch clauses.
	 */
	noUselessCatch?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unnecessary constructors.
	 */
	noUselessConstructor?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow empty exports that don't change anything in a module file.
	 */
	noUselessEmptyExport?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unnecessary fragments
	 */
	noUselessFragments?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unnecessary labels.
	 */
	noUselessLabel?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unnecessary nested block statements.
	 */
	noUselessLoneBlockStatements?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow renaming import, export, and destructured assignments to the same name.
	 */
	noUselessRename?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unnecessary concatenation of string or template literals.
	 */
	noUselessStringConcat?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow useless case in switch statements.
	 */
	noUselessSwitchCase?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow ternary operators when simpler alternatives exist.
	 */
	noUselessTernary?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow useless this aliasing.
	 */
	noUselessThisAlias?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow using any or unknown as type constraint.
	 */
	noUselessTypeConstraint?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow initializing variables to undefined.
	 */
	noUselessUndefinedInitialization?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of void operators, which is not a familiar operator.
	 */
	noVoid?: RuleConfiguration_for_Null;
	/**
	 * Disallow with statements in non-strict contexts.
	 */
	noWith?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Use arrow functions over function expressions.
	 */
	useArrowFunction?: RuleFixConfiguration_for_Null;
	/**
	 * Use Date.now() to get the number of milliseconds since the Unix Epoch.
	 */
	useDateNow?: RuleFixConfiguration_for_Null;
	/**
	 * Promotes the use of .flatMap() when map().flat() are used together.
	 */
	useFlatMap?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce the usage of a literal access to properties over computed property access.
	 */
	useLiteralKeys?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce using concise optional chain instead of chained logical expressions.
	 */
	useOptionalChain?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce the use of the regular expression literals instead of the RegExp constructor if possible.
	 */
	useRegexLiterals?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow number literal object member names which are not base10 or uses underscore as separator
	 */
	useSimpleNumberKeys?: RuleFixConfiguration_for_Null;
	/**
	 * Discard redundant terms from logical expressions.
	 */
	useSimplifiedLogicExpression?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed.
	 */
	useWhile?: RuleFixConfiguration_for_Null;
}
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	/**
	 * Prevent passing of children as props.
	 */
	noChildrenProp?: RuleConfiguration_for_Null;
	/**
	 * Prevents from having const variables being re-assigned.
	 */
	noConstAssign?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow constant expressions in conditions
	 */
	noConstantCondition?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of Math.min and Math.max to clamp a value where the result itself is constant.
	 */
	noConstantMathMinMaxClamp?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow returning a value from a constructor.
	 */
	noConstructorReturn?: RuleConfiguration_for_Null;
	/**
	 * Disallow empty character classes in regular expression literals.
	 */
	noEmptyCharacterClassInRegex?: RuleConfiguration_for_Null;
	/**
	 * Disallows empty destructuring patterns.
	 */
	noEmptyPattern?: RuleConfiguration_for_Null;
	/**
	 * Disallow to use unnecessary callback on flatMap.
	 */
	noFlatMapIdentity?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow calling global object properties as functions
	 */
	noGlobalObjectCalls?: RuleConfiguration_for_Null;
	/**
	 * Disallow function and var declarations that are accessible outside their block.
	 */
	noInnerDeclarations?: RuleConfiguration_for_Null;
	/**
	 * Ensure that builtins are correctly instantiated.
	 */
	noInvalidBuiltinInstantiation?: RuleFixConfiguration_for_Null;
	/**
	 * Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors.
	 */
	noInvalidConstructorSuper?: RuleConfiguration_for_Null;
	/**
	 * Disallow non-standard direction values for linear gradient functions.
	 */
	noInvalidDirectionInLinearGradient?: RuleConfiguration_for_Null;
	/**
	 * Disallows invalid named grid areas in CSS Grid Layouts.
	 */
	noInvalidGridAreas?: RuleConfiguration_for_Null;
	/**
	 * Disallow new operators with global non-constructor functions.
	 */
	noInvalidNewBuiltin?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of @import at-rules in invalid positions.
	 */
	noInvalidPositionAtImportRule?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of variables and function parameters before their declaration
	 */
	noInvalidUseBeforeDeclaration?: RuleConfiguration_for_Null;
	/**
	 * Disallow new operators with the Symbol object.
	 */
	noNewSymbol?: RuleFixConfiguration_for_Null;
	/**
	 * Forbid the use of Node.js builtin modules.
	 */
	noNodejsModules?: RuleConfiguration_for_Null;
	/**
	 * Disallow \8 and \9 escape sequences in string literals.
	 */
	noNonoctalDecimalEscape?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow literal numbers that lose precision
	 */
	noPrecisionLoss?: RuleConfiguration_for_Null;
	/**
	 * Prevent the usage of the return value of React.render.
	 */
	noRenderReturnValue?: RuleConfiguration_for_Null;
	/**
	 * Disallow assignments where both sides are exactly the same.
	 */
	noSelfAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow returning a value from a setter
	 */
	noSetterReturn?: RuleConfiguration_for_Null;
	/**
	 * Disallow comparison of expressions modifying the string case with non-compliant value.
	 */
	noStringCaseMismatch?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow lexical declarations in switch clauses.
	 */
	noSwitchDeclarations?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of dependencies that aren't specified in the package.json.
	 */
	noUndeclaredDependencies?: RuleConfiguration_for_NoUndeclaredDependenciesOptions;
	/**
	 * Prevents the usage of variables that haven't been declared inside the document.
	 */
	noUndeclaredVariables?: RuleConfiguration_for_UndeclaredVariablesOptions;
	/**
	 * Disallow unknown CSS value functions.
	 */
	noUnknownFunction?: RuleConfiguration_for_Null;
	/**
	 * Disallow unknown media feature names.
	 */
	noUnknownMediaFeatureName?: RuleConfiguration_for_Null;
	/**
	 * Disallow unknown properties.
	 */
	noUnknownProperty?: RuleConfiguration_for_Null;
	/**
	 * Disallow unknown CSS units.
	 */
	noUnknownUnit?: RuleConfiguration_for_Null;
	/**
	 * Disallow unmatchable An+B selectors.
	 */
	noUnmatchableAnbSelector?: RuleConfiguration_for_Null;
	/**
	 * Avoid using unnecessary continue.
	 */
	noUnnecessaryContinue?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unreachable code
	 */
	noUnreachable?: RuleConfiguration_for_Null;
	/**
	 * Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass
	 */
	noUnreachableSuper?: RuleConfiguration_for_Null;
	/**
	 * Disallow control flow statements in finally blocks.
	 */
	noUnsafeFinally?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of optional chaining in contexts where the undefined value is not allowed.
	 */
	noUnsafeOptionalChaining?: RuleConfiguration_for_Null;
	/**
	 * Disallow unused function parameters.
	 */
	noUnusedFunctionParameters?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unused imports.
	 */
	noUnusedImports?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unused labels.
	 */
	noUnusedLabels?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unused private class members
	 */
	noUnusedPrivateClassMembers?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unused variables.
	 */
	noUnusedVariables?: RuleFixConfiguration_for_Null;
	/**
	 * This rules prevents void elements (AKA self-closing elements) from having children.
	 */
	noVoidElementsWithChildren?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow returning a value from a function with the return type 'void'
	 */
	noVoidTypeReturn?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Disallow Array constructors.
	 */
	useArrayLiterals?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce all dependencies are correctly specified in a React hook.
	 */
	useExhaustiveDependencies?: RuleConfiguration_for_UseExhaustiveDependenciesOptions;
	/**
	 * Enforce that all React hooks are being called from the Top Level component functions.
	 */
	useHookAtTopLevel?: RuleConfiguration_for_DeprecatedHooksOptions;
	/**
	 * Enforce file extensions for relative imports.
	 */
	useImportExtensions?: RuleFixConfiguration_for_UseImportExtensionsOptions;
	/**
	 * Require calls to isNaN() when checking for NaN.
	 */
	useIsNan?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow missing key props in iterators/collection literals.
	 */
	useJsxKeyInIterable?: RuleConfiguration_for_Null;
	/**
	 * Enforce "for" loop update clause moving the counter in the right direction.
	 */
	useValidForDirection?: RuleConfiguration_for_Null;
	/**
	 * Require generator functions to contain yield.
	 */
	useYield?: RuleConfiguration_for_Null;
}
/**
 * A list of rules that belong to this group
 */
export interface Nursery {
	/**
	 * Disallow await inside loops.
	 */
	noAwaitInLoop?: RuleConfiguration_for_Null;
	/**
	 * Disallow use of CommonJs module system in favor of ESM style imports.
	 */
	noCommonJs?: RuleConfiguration_for_Null;
	/**
	 * Disallow expressions where the operation doesn't affect the value
	 */
	noConstantBinaryExpression?: RuleConfiguration_for_Null;
	/**
	 * Disallow a lower specificity selector from coming after a higher specificity selector.
	 */
	noDescendingSpecificity?: RuleConfiguration_for_Null;
	/**
	 * Disallow direct assignments to document.cookie.
	 */
	noDocumentCookie?: RuleConfiguration_for_Null;
	/**
	 * Prevents importing next/document outside of pages/_document.jsx in Next.js projects.
	 */
	noDocumentImportInPage?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate custom properties within declaration blocks.
	 */
	noDuplicateCustomProperties?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate conditions in if-else-if chains
	 */
	noDuplicateElseIf?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate properties within declaration blocks.
	 */
	noDuplicateProperties?: RuleConfiguration_for_Null;
	/**
	 * No duplicated fields in GraphQL operations.
	 */
	noDuplicatedFields?: RuleConfiguration_for_Null;
	/**
	 * Disallow accessing namespace imports dynamically.
	 */
	noDynamicNamespaceImportAccess?: RuleConfiguration_for_Null;
	/**
	 * Disallow TypeScript enum.
	 */
	noEnum?: RuleConfiguration_for_Null;
	/**
	 * Disallow exporting an imported variable.
	 */
	noExportedImports?: RuleConfiguration_for_Null;
	/**
	 * Require Promise-like statements to be handled appropriately.
	 */
	noFloatingPromises?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of __dirname and __filename in the global scope.
	 */
	noGlobalDirnameFilename?: RuleFixConfiguration_for_Null;
	/**
	 * Prevent usage of \<head> element in a Next.js project.
	 */
	noHeadElement?: RuleConfiguration_for_Null;
	/**
	 * Prevent using the next/head module in pages/_document.js on Next.js projects.
	 */
	noHeadImportInDocument?: RuleConfiguration_for_Null;
	/**
	 * Prevent usage of \<img> element in a Next.js project.
	 */
	noImgElement?: RuleConfiguration_for_Null;
	/**
	 * Prevent import cycles.
	 */
	noImportCycles?: RuleConfiguration_for_Null;
	/**
	 * Disallows the use of irregular whitespace characters.
	 */
	noIrregularWhitespace?: RuleConfiguration_for_Null;
	/**
	 * Disallow missing var function for css variables.
	 */
	noMissingVarFunction?: RuleConfiguration_for_Null;
	/**
	 * Disallow nested ternary expressions.
	 */
	noNestedTernary?: RuleConfiguration_for_Null;
	/**
	 * Disallow use event handlers on non-interactive elements.
	 */
	noNoninteractiveElementInteractions?: RuleConfiguration_for_Null;
	/**
	 * Disallow octal escape sequences in string literals
	 */
	noOctalEscape?: RuleConfiguration_for_Null;
	/**
	 * Restricts imports of "package private" exports.
	 */
	noPackagePrivateImports?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of process.env.
	 */
	noProcessEnv?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of process global.
	 */
	noProcessGlobal?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow specified modules when loaded by import or require.
	 */
	noRestrictedImports?: RuleConfiguration_for_RestrictedImportsOptions;
	/**
	 * Disallow user defined types.
	 */
	noRestrictedTypes?: RuleFixConfiguration_for_NoRestrictedTypesOptions;
	/**
	 * Disallow usage of sensitive data such as API keys and tokens.
	 */
	noSecrets?: RuleConfiguration_for_NoSecretsOptions;
	/**
	 * Enforce that static, visible elements (such as \<div>) that have click handlers use the valid role attribute.
	 */
	noStaticElementInteractions?: RuleConfiguration_for_Null;
	/**
	 * Enforce the use of String.slice() over String.substr() and String.substring().
	 */
	noSubstr?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow template literal placeholder syntax in regular strings.
	 */
	noTemplateCurlyInString?: RuleConfiguration_for_Null;
	/**
	 * Prevents the use of the TypeScript directive @ts-ignore.
	 */
	noTsIgnore?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unknown at-rules.
	 */
	noUnknownAtRule?: RuleConfiguration_for_Null;
	/**
	 * Disallow unknown pseudo-class selectors.
	 */
	noUnknownPseudoClass?: RuleConfiguration_for_Null;
	/**
	 * Disallow unknown pseudo-element selectors.
	 */
	noUnknownPseudoElement?: RuleConfiguration_for_Null;
	/**
	 * Disallow unknown type selectors.
	 */
	noUnknownTypeSelector?: RuleConfiguration_for_Null;
	/**
	 * Prevent duplicate polyfills from Polyfill.io.
	 */
	noUnwantedPolyfillio?: RuleConfiguration_for_Null;
	/**
	 * Disallow unnecessary escape sequence in regular expression literals.
	 */
	noUselessEscapeInRegex?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow unnecessary String.raw function in template string literals without any escape sequence.
	 */
	noUselessStringRaw?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of useless undefined.
	 */
	noUselessUndefined?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow use of @value rule in css modules.
	 */
	noValueAtRule?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Disallow the use of overload signatures that are not next to each other.
	 */
	useAdjacentOverloadSignatures?: RuleConfiguration_for_Null;
	/**
	 * Enforce that ARIA properties are valid for the roles that are supported by the element.
	 */
	useAriaPropsSupportedByRole?: RuleConfiguration_for_Null;
	/**
	 * Use at() instead of integer index access.
	 */
	useAtIndex?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce using single if instead of nested if clauses.
	 */
	useCollapsedIf?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce declaring components only within modules that export React Components exclusively.
	 */
	useComponentExportOnlyModules?: RuleConfiguration_for_UseComponentExportOnlyModulesOptions;
	/**
	 * This rule enforces consistent use of curly braces inside JSX attributes and JSX children.
	 */
	useConsistentCurlyBraces?: RuleFixConfiguration_for_Null;
	/**
	 * Require consistent accessibility modifiers on class properties and methods.
	 */
	useConsistentMemberAccessibility?: RuleConfiguration_for_ConsistentMemberAccessibilityOptions;
	/**
	 * Require specifying the reason argument when using @deprecated directive
	 */
	useDeprecatedReason?: RuleConfiguration_for_Null;
	/**
	 * Require explicit return types on functions and class methods.
	 */
	useExplicitType?: RuleConfiguration_for_Null;
	/**
	 * Require that all exports are declared after all non-export statements.
	 */
	useExportsLast?: RuleConfiguration_for_Null;
	/**
	 * Enforces the use of a recommended display strategy with Google Fonts.
	 */
	useGoogleFontDisplay?: RuleConfiguration_for_Null;
	/**
	 * Ensure the preconnect attribute is used when using Google Fonts.
	 */
	useGoogleFontPreconnect?: RuleFixConfiguration_for_Null;
	/**
	 * Require for-in loops to include an if statement.
	 */
	useGuardForIn?: RuleConfiguration_for_Null;
	/**
	 * Enforce specifying the name of GraphQL operations.
	 */
	useNamedOperation?: RuleFixConfiguration_for_Null;
	/**
	 * Validates that all enum values are capitalized.
	 */
	useNamingConvention?: RuleConfiguration_for_Null;
	/**
	 * Enforce the consistent use of the radix argument when using parseInt().
	 */
	useParseIntRadix?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce the sorting of CSS utility classes.
	 */
	useSortedClasses?: RuleFixConfiguration_for_UtilityClassSortingOptions;
	/**
	 * Enforce the use of the directive "use strict" in script files.
	 */
	useStrictMode?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce the use of String.trimStart() and String.trimEnd() over String.trimLeft() and String.trimRight().
	 */
	useTrimStartEnd?: RuleFixConfiguration_for_Null;
	/**
	 * Use valid values for the autocomplete attribute on input elements.
	 */
	useValidAutocomplete?: RuleConfiguration_for_UseValidAutocompleteOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Performance {
	/**
	 * Disallow the use of spread (...) syntax on accumulators.
	 */
	noAccumulatingSpread?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of barrel file.
	 */
	noBarrelFile?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of the delete operator.
	 */
	noDelete?: RuleFixConfiguration_for_Null;
	/**
	 * Avoid re-export all.
	 */
	noReExportAll?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Require regex literals to be declared at the top level.
	 */
	useTopLevelRegex?: RuleConfiguration_for_Null;
}
/**
 * A list of rules that belong to this group
 */
export interface Security {
	/**
	 * Prevent the usage of dangerous JSX props
	 */
	noDangerouslySetInnerHtml?: RuleConfiguration_for_Null;
	/**
	 * Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop.
	 */
	noDangerouslySetInnerHtmlWithChildren?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of global eval().
	 */
	noGlobalEval?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
}
/**
 * A list of rules that belong to this group
 */
export interface Style {
	/**
	 * Disallow the use of arguments.
	 */
	noArguments?: RuleConfiguration_for_Null;
	/**
	 * Disallow comma operator.
	 */
	noCommaOperator?: RuleConfiguration_for_Null;
	/**
	 * Disallow default exports.
	 */
	noDefaultExport?: RuleConfiguration_for_Null;
	/**
	 * Disallow using a callback in asynchronous tests and hooks.
	 */
	noDoneCallback?: RuleConfiguration_for_Null;
	/**
	 * Disallow implicit true values on JSX boolean attributes
	 */
	noImplicitBoolean?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
	 */
	noInferrableTypes?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of TypeScript's namespaces.
	 */
	noNamespace?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of namespace imports.
	 */
	noNamespaceImport?: RuleConfiguration_for_Null;
	/**
	 * Disallow negation in the condition of an if statement if it has an else clause.
	 */
	noNegationElse?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow non-null assertions using the ! postfix operator.
	 */
	noNonNullAssertion?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow reassigning function parameters.
	 */
	noParameterAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of parameter properties in class constructors.
	 */
	noParameterProperties?: RuleConfiguration_for_Null;
	/**
	 * This rule allows you to specify global variable names that you don’t want to use in your application.
	 */
	noRestrictedGlobals?: RuleConfiguration_for_RestrictedGlobalsOptions;
	/**
	 * Disallow the use of constants which its value is the upper-case version of its name.
	 */
	noShoutyConstants?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow template literals if interpolation and special-character handling are not needed
	 */
	noUnusedTemplateLiteral?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow else block when the if block breaks early.
	 */
	noUselessElse?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of yoda expressions.
	 */
	noYodaExpression?: RuleFixConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce the use of as const over literal type and type annotation.
	 */
	useAsConstAssertion?: RuleFixConfiguration_for_Null;
	/**
	 * Requires following curly brace conventions.
	 */
	useBlockStatements?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce using else if instead of nested if in else clauses.
	 */
	useCollapsedElseIf?: RuleFixConfiguration_for_Null;
	/**
	 * Require consistently using either T\[] or Array\<T>
	 */
	useConsistentArrayType?: RuleFixConfiguration_for_ConsistentArrayTypeOptions;
	/**
	 * Enforce the use of new for all builtins, except String, Number and Boolean.
	 */
	useConsistentBuiltinInstantiation?: RuleFixConfiguration_for_Null;
	/**
	 * Require const declarations for variables that are only assigned once.
	 */
	useConst?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce default function parameters and optional function parameters to be last.
	 */
	useDefaultParameterLast?: RuleFixConfiguration_for_Null;
	/**
	 * Require the default clause in switch statements.
	 */
	useDefaultSwitchClause?: RuleConfiguration_for_Null;
	/**
	 * Require that each enum member value be explicitly initialized.
	 */
	useEnumInitializers?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce explicitly comparing the length, size, byteLength or byteOffset property of a value.
	 */
	useExplicitLengthCheck?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of Math.pow in favor of the ** operator.
	 */
	useExponentiationOperator?: RuleFixConfiguration_for_Null;
	/**
	 * Promotes the use of export type for types.
	 */
	useExportType?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce naming conventions for JavaScript and TypeScript filenames.
	 */
	useFilenamingConvention?: RuleConfiguration_for_FilenamingConventionOptions;
	/**
	 * This rule recommends a for-of loop when in a for loop, the index used to extract an item from the iterated array.
	 */
	useForOf?: RuleConfiguration_for_Null;
	/**
	 * This rule enforces the use of \<>...\</> over \<Fragment>...\</Fragment>.
	 */
	useFragmentSyntax?: RuleFixConfiguration_for_Null;
	/**
	 * Promotes the use of import type for types.
	 */
	useImportType?: RuleFixConfiguration_for_Null;
	/**
	 * Require all enum members to be literal values.
	 */
	useLiteralEnumMembers?: RuleConfiguration_for_Null;
	/**
	 * Enforce naming conventions for everything across a codebase.
	 */
	useNamingConvention?: RuleFixConfiguration_for_NamingConventionOptions;
	/**
	 * Promotes the usage of node:assert/strict over node:assert.
	 */
	useNodeAssertStrict?: RuleFixConfiguration_for_Null;
	/**
	 * Enforces using the node: protocol for Node.js builtin modules.
	 */
	useNodejsImportProtocol?: RuleFixConfiguration_for_Null;
	/**
	 * Use the Number properties instead of global ones.
	 */
	useNumberNamespace?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals
	 */
	useNumericLiterals?: RuleFixConfiguration_for_Null;
	/**
	 * Prevent extra closing tags for components without children
	 */
	useSelfClosingElements?: RuleFixConfiguration_for_UseSelfClosingElementsOptions;
	/**
	 * When expressing array types, this rule promotes the usage of T\[] shorthand instead of Array\<T>.
	 */
	useShorthandArrayType?: RuleFixConfiguration_for_Null;
	/**
	 * Require assignment operator shorthand where possible.
	 */
	useShorthandAssign?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce using function types instead of object type with call signatures.
	 */
	useShorthandFunctionType?: RuleFixConfiguration_for_Null;
	/**
	 * Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block.
	 */
	useSingleCaseStatement?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow multiple variable declarations in the same variable statement
	 */
	useSingleVarDeclarator?: RuleFixConfiguration_for_Null;
	/**
	 * Prefer template literals over string concatenation.
	 */
	useTemplate?: RuleFixConfiguration_for_Null;
	/**
	 * Require new when throwing an error.
	 */
	useThrowNewError?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow throwing non-Error values.
	 */
	useThrowOnlyError?: RuleConfiguration_for_Null;
}
/**
 * A list of rules that belong to this group
 */
export interface Suspicious {
	/**
	 * Use standard constants instead of approximated literals.
	 */
	noApproximativeNumericConstant?: RuleFixConfiguration_for_Null;
	/**
	 * Discourage the usage of Array index in keys.
	 */
	noArrayIndexKey?: RuleConfiguration_for_Null;
	/**
	 * Disallow assignments in expressions.
	 */
	noAssignInExpressions?: RuleConfiguration_for_Null;
	/**
	 * Disallows using an async function as a Promise executor.
	 */
	noAsyncPromiseExecutor?: RuleConfiguration_for_Null;
	/**
	 * Disallow reassigning exceptions in catch clauses.
	 */
	noCatchAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow reassigning class members.
	 */
	noClassAssign?: RuleConfiguration_for_Null;
	/**
	 * Prevent comments from being inserted as text nodes
	 */
	noCommentText?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow comparing against -0
	 */
	noCompareNegZero?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow labeled statements that are not loops.
	 */
	noConfusingLabels?: RuleConfiguration_for_Null;
	/**
	 * Disallow void type outside of generic or return types.
	 */
	noConfusingVoidType?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of console.
	 */
	noConsole?: RuleFixConfiguration_for_NoConsoleOptions;
	/**
	 * Disallow TypeScript const enum
	 */
	noConstEnum?: RuleFixConfiguration_for_Null;
	/**
	 * Prevents from having control characters and some escape sequences that match control characters in regular expressions.
	 */
	noControlCharactersInRegex?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of debugger
	 */
	noDebugger?: RuleFixConfiguration_for_Null;
	/**
	 * Require the use of === and !==.
	 */
	noDoubleEquals?: RuleFixConfiguration_for_NoDoubleEqualsOptions;
	/**
	 * Disallow duplicate @import rules.
	 */
	noDuplicateAtImportRules?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate case labels.
	 */
	noDuplicateCase?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate class members.
	 */
	noDuplicateClassMembers?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate names within font families.
	 */
	noDuplicateFontNames?: RuleConfiguration_for_Null;
	/**
	 * Prevents JSX properties to be assigned multiple times.
	 */
	noDuplicateJsxProps?: RuleConfiguration_for_Null;
	/**
	 * Disallow two keys with the same name inside objects.
	 */
	noDuplicateObjectKeys?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate function parameter name.
	 */
	noDuplicateParameters?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate selectors within keyframe blocks.
	 */
	noDuplicateSelectorsKeyframeBlock?: RuleConfiguration_for_Null;
	/**
	 * A describe block should not contain duplicate hooks.
	 */
	noDuplicateTestHooks?: RuleConfiguration_for_Null;
	/**
	 * Disallow CSS empty blocks.
	 */
	noEmptyBlock?: RuleConfiguration_for_Null;
	/**
	 * Disallow empty block statements and static blocks.
	 */
	noEmptyBlockStatements?: RuleConfiguration_for_Null;
	/**
	 * Disallow the declaration of empty interfaces.
	 */
	noEmptyInterface?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow variables from evolving into any type through reassignments.
	 */
	noEvolvingTypes?: RuleConfiguration_for_Null;
	/**
	 * Disallow the any type usage.
	 */
	noExplicitAny?: RuleConfiguration_for_Null;
	/**
	 * Disallow using export or module.exports in files containing tests
	 */
	noExportsInTest?: RuleConfiguration_for_Null;
	/**
	 * Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files.
	 */
	noExtraNonNullAssertion?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow fallthrough of switch clauses.
	 */
	noFallthroughSwitchClause?: RuleConfiguration_for_Null;
	/**
	 * Disallow focused tests.
	 */
	noFocusedTests?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow reassigning function declarations.
	 */
	noFunctionAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow assignments to native objects and read-only global variables.
	 */
	noGlobalAssign?: RuleConfiguration_for_Null;
	/**
	 * Use Number.isFinite instead of global isFinite.
	 */
	noGlobalIsFinite?: RuleFixConfiguration_for_Null;
	/**
	 * Use Number.isNaN instead of global isNaN.
	 */
	noGlobalIsNan?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow use of implicit any type on variable declarations.
	 */
	noImplicitAnyLet?: RuleConfiguration_for_Null;
	/**
	 * Disallow assigning to imported bindings
	 */
	noImportAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow invalid !important within keyframe declarations
	 */
	noImportantInKeyframe?: RuleConfiguration_for_Null;
	/**
	 * Disallow labels that share a name with a variable
	 */
	noLabelVar?: RuleConfiguration_for_Null;
	/**
	 * Disallow characters made with multiple code points in character class syntax.
	 */
	noMisleadingCharacterClass?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce proper usage of new and constructor.
	 */
	noMisleadingInstantiator?: RuleConfiguration_for_Null;
	/**
	 * Checks that the assertion function, for example expect, is placed inside an it() function call.
	 */
	noMisplacedAssertion?: RuleConfiguration_for_Null;
	/**
	 * Disallow shorthand assign when variable appears on both sides.
	 */
	noMisrefactoredShorthandAssign?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow direct use of Object.prototype builtins.
	 */
	noPrototypeBuiltins?: RuleFixConfiguration_for_Null;
	/**
	 * Prevents React-specific JSX properties from being used.
	 */
	noReactSpecificProps?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow variable, function, class, and type redeclarations in the same scope.
	 */
	noRedeclare?: RuleConfiguration_for_Null;
	/**
	 * Prevents from having redundant "use strict".
	 */
	noRedundantUseStrict?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow comparisons where both sides are exactly the same.
	 */
	noSelfCompare?: RuleConfiguration_for_Null;
	/**
	 * Disallow identifiers from shadowing restricted names.
	 */
	noShadowRestrictedNames?: RuleConfiguration_for_Null;
	/**
	 * Disallow shorthand properties that override related longhand properties.
	 */
	noShorthandPropertyOverrides?: RuleConfiguration_for_Null;
	/**
	 * Disallow disabled tests.
	 */
	noSkippedTests?: RuleFixConfiguration_for_Null;
	/**
	 * Prevents the use of sparse arrays (arrays with holes).
	 */
	noSparseArray?: RuleFixConfiguration_for_Null;
	/**
	 * It detects possible "wrong" semicolons inside JSX elements.
	 */
	noSuspiciousSemicolonInJsx?: RuleConfiguration_for_Null;
	/**
	 * Disallow then property.
	 */
	noThenProperty?: RuleConfiguration_for_Null;
	/**
	 * Disallow unsafe declaration merging between interfaces and classes.
	 */
	noUnsafeDeclarationMerging?: RuleConfiguration_for_Null;
	/**
	 * Disallow using unsafe negation.
	 */
	noUnsafeNegation?: RuleFixConfiguration_for_Null;
	/**
	 * Disallow the use of var
	 */
	noVar?: RuleFixConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Ensure async functions utilize await.
	 */
	useAwait?: RuleConfiguration_for_Null;
	/**
	 * Enforce default clauses in switch statements to be last
	 */
	useDefaultSwitchClauseLast?: RuleConfiguration_for_Null;
	/**
	 * Enforce passing a message value when creating a built-in error.
	 */
	useErrorMessage?: RuleConfiguration_for_Null;
	/**
	 * Enforce get methods to always return a value.
	 */
	useGetterReturn?: RuleConfiguration_for_Null;
	/**
	 * Use Array.isArray() instead of instanceof Array.
	 */
	useIsArray?: RuleFixConfiguration_for_Null;
	/**
	 * Require using the namespace keyword over the module keyword to declare TypeScript namespaces.
	 */
	useNamespaceKeyword?: RuleFixConfiguration_for_Null;
	/**
	 * Enforce using the digits argument with Number#toFixed().
	 */
	useNumberToFixedDigitsArgument?: RuleFixConfiguration_for_Null;
	/**
	 * This rule checks that the result of a typeof expression is compared to a valid value.
	 */
	useValidTypeof?: RuleFixConfiguration_for_Null;
}
export type RuleAssistPlainConfiguration = "off" | "on";
export interface RuleAssistWithOptions_for_Options {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RuleAssistPlainConfiguration;
	/**
	 * Rule's options
	 */
	options: Options;
}
export interface RuleAssistWithOptions_for_Null {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RuleAssistPlainConfiguration;
	/**
	 * Rule's options
	 */
	options: null;
}
export type RuleFixConfiguration_for_Null =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_Null;
export type RuleFixConfiguration_for_AllowDomainOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_AllowDomainOptions;
export type RuleConfiguration_for_NoLabelWithoutControlOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoLabelWithoutControlOptions;
export type RuleConfiguration_for_Null =
	| RulePlainConfiguration
	| RuleWithOptions_for_Null;
export type RuleFixConfiguration_for_ValidAriaRoleOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_ValidAriaRoleOptions;
export type RuleConfiguration_for_ComplexityOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_ComplexityOptions;
export type RuleConfiguration_for_NoForEachOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoForEachOptions;
export type RuleConfiguration_for_NoUndeclaredDependenciesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoUndeclaredDependenciesOptions;
export type RuleConfiguration_for_UndeclaredVariablesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UndeclaredVariablesOptions;
export type RuleConfiguration_for_UseExhaustiveDependenciesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseExhaustiveDependenciesOptions;
export type RuleConfiguration_for_DeprecatedHooksOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_DeprecatedHooksOptions;
export type RuleFixConfiguration_for_UseImportExtensionsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseImportExtensionsOptions;
export type RuleConfiguration_for_RestrictedImportsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_RestrictedImportsOptions;
export type RuleFixConfiguration_for_NoRestrictedTypesOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoRestrictedTypesOptions;
export type RuleConfiguration_for_NoSecretsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NoSecretsOptions;
export type RuleConfiguration_for_UseComponentExportOnlyModulesOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseComponentExportOnlyModulesOptions;
export type RuleConfiguration_for_ConsistentMemberAccessibilityOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_ConsistentMemberAccessibilityOptions;
export type RuleFixConfiguration_for_UtilityClassSortingOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UtilityClassSortingOptions;
export type RuleConfiguration_for_UseValidAutocompleteOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UseValidAutocompleteOptions;
export type RuleConfiguration_for_RestrictedGlobalsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_RestrictedGlobalsOptions;
export type RuleFixConfiguration_for_ConsistentArrayTypeOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_ConsistentArrayTypeOptions;
export type RuleConfiguration_for_FilenamingConventionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_FilenamingConventionOptions;
export type RuleFixConfiguration_for_NamingConventionOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NamingConventionOptions;
export type RuleFixConfiguration_for_UseSelfClosingElementsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_UseSelfClosingElementsOptions;
export type RuleFixConfiguration_for_NoConsoleOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoConsoleOptions;
export type RuleFixConfiguration_for_NoDoubleEqualsOptions =
	| RulePlainConfiguration
	| RuleWithFixOptions_for_NoDoubleEqualsOptions;
export interface Options {
	importGroups?: ImportGroup[];
	legacy?: boolean;
}
export type RulePlainConfiguration = "off" | "on" | "info" | "warn" | "error";
export interface RuleWithFixOptions_for_Null {
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
	options: null;
}
export interface RuleWithFixOptions_for_AllowDomainOptions {
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
	options: AllowDomainOptions;
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
export interface RuleWithOptions_for_Null {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: null;
}
export interface RuleWithFixOptions_for_ValidAriaRoleOptions {
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
	options: ValidAriaRoleOptions;
}
export interface RuleWithOptions_for_ComplexityOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: ComplexityOptions;
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
export interface RuleWithOptions_for_UndeclaredVariablesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UndeclaredVariablesOptions;
}
export interface RuleWithOptions_for_UseExhaustiveDependenciesOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: UseExhaustiveDependenciesOptions;
}
export interface RuleWithOptions_for_DeprecatedHooksOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: DeprecatedHooksOptions;
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
export interface RuleWithOptions_for_RestrictedImportsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: RestrictedImportsOptions;
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
export interface RuleWithOptions_for_ConsistentMemberAccessibilityOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: ConsistentMemberAccessibilityOptions;
}
export interface RuleWithFixOptions_for_UtilityClassSortingOptions {
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
	options: UtilityClassSortingOptions;
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
export interface RuleWithOptions_for_RestrictedGlobalsOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: RestrictedGlobalsOptions;
}
export interface RuleWithFixOptions_for_ConsistentArrayTypeOptions {
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
	options: ConsistentArrayTypeOptions;
}
export interface RuleWithOptions_for_FilenamingConventionOptions {
	/**
	 * The severity of the emitted diagnostics by the rule
	 */
	level: RulePlainConfiguration;
	/**
	 * Rule's options
	 */
	options: FilenamingConventionOptions;
}
export interface RuleWithFixOptions_for_NamingConventionOptions {
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
	options: NamingConventionOptions;
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
export type ImportGroup = PredefinedImportGroup | Glob;
/**
 * Used to identify the kind of code action emitted by a rule
 */
export type FixKind = "none" | "safe" | "unsafe";
export interface AllowDomainOptions {
	/**
	 * List of domains to allow `target="_blank"` without `rel="noreferrer"`
	 */
	allowDomains: string[];
}
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
export interface ValidAriaRoleOptions {
	allowInvalidRoles?: string[];
	ignoreNonDom?: boolean;
}
/**
 * Options for the rule `noExcessiveCognitiveComplexity`.
 */
export interface ComplexityOptions {
	/**
	 * The maximum complexity score that we allow. Anything higher is considered excessive.
	 */
	maxAllowedComplexity?: number;
}
export interface NoForEachOptions {
	/**
	 * A list of variable names allowed for `forEach` calls.
	 */
	allowedIdentifiers: string[];
}
/**
 * Rule's options
 */
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
export interface UndeclaredVariablesOptions {
	/**
	 * Check undeclared types.
	 */
	checkTypes?: boolean;
}
/**
 * Options for the rule `useExhaustiveDependencies`
 */
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
/**
 * Options for the `useHookAtTopLevel` rule have been deprecated, since we now use the React hook naming convention to determine whether a function is a hook.
 */
export interface DeprecatedHooksOptions {}
export interface UseImportExtensionsOptions {
	/**
	 * A map of custom import extension mappings, where the key is the inspected file extension, and the value is a pair of `module` extension and `component` import extension
	 */
	suggestedExtensions?: {};
}
/**
 * Options for the rule `noRestrictedImports`.
 */
export interface RestrictedImportsOptions {
	/**
	 * A list of import paths that should trigger the rule.
	 */
	paths: {};
}
export interface NoRestrictedTypesOptions {
	types?: {};
}
export interface NoSecretsOptions {
	/**
	 * Set entropy threshold (default is 41).
	 */
	entropyThreshold?: number;
}
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
export interface ConsistentMemberAccessibilityOptions {
	accessibility?: Accessibility;
}
export interface UtilityClassSortingOptions {
	/**
	 * Additional attributes that will be sorted.
	 */
	attributes?: string[];
	/**
	 * Names of the functions or tagged templates that will be sorted.
	 */
	functions?: string[];
}
export interface UseValidAutocompleteOptions {
	/**
	 * `input` like custom components that should be checked.
	 */
	inputComponents?: string[];
}
/**
 * Options for the rule `noRestrictedGlobals`.
 */
export interface RestrictedGlobalsOptions {
	/**
	 * A list of names that should trigger the rule
	 */
	deniedGlobals: string[];
}
export interface ConsistentArrayTypeOptions {
	syntax?: ConsistentArrayType;
}
/**
 * Rule's options.
 */
export interface FilenamingConventionOptions {
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
/**
 * Rule's options.
 */
export interface NamingConventionOptions {
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
/**
 * Options for the `useSelfClosingElements` rule.
 */
export interface UseSelfClosingElementsOptions {
	ignoreHtmlElements?: boolean;
}
export interface NoConsoleOptions {
	/**
	 * Allowed calls on the console object.
	 */
	allow: string[];
}
/**
 * Rule's options
 */
export interface NoDoubleEqualsOptions {
	/**
	* If `true`, an exception is made when comparing with `null`, as it's often relied on to check both for `null` or `undefined`.

If `false`, no such exception will be made. 
	 */
	ignoreNull: boolean;
}
export type PredefinedImportGroup =
	| ":blank-line:"
	| ":bun:"
	| ":node:"
	| ":types:";
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
export type Accessibility = "noPublic" | "explicit" | "none";
export type ConsistentArrayType = "shorthand" | "generic";
export type FilenameCases = FilenameCase[];
export type Regex = string;
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
export type StableHookResult = boolean | number[];
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
	| "lint/a11y/noBlankTarget"
	| "lint/a11y/noDistractingElements"
	| "lint/a11y/noHeaderScope"
	| "lint/a11y/noInteractiveElementToNoninteractiveRole"
	| "lint/a11y/noLabelWithoutControl"
	| "lint/a11y/noNoninteractiveElementToInteractiveRole"
	| "lint/a11y/noNoninteractiveTabindex"
	| "lint/a11y/noPositiveTabindex"
	| "lint/a11y/noRedundantAlt"
	| "lint/a11y/noRedundantRoles"
	| "lint/a11y/noSvgWithoutTitle"
	| "lint/a11y/useAltText"
	| "lint/a11y/useAnchorContent"
	| "lint/a11y/useAriaActivedescendantWithTabindex"
	| "lint/a11y/useAriaPropsForRole"
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
	| "lint/a11y/useValidLang"
	| "lint/complexity/noBannedTypes"
	| "lint/complexity/noEmptyTypeParameters"
	| "lint/complexity/noExcessiveCognitiveComplexity"
	| "lint/complexity/noExcessiveNestedTestSuites"
	| "lint/complexity/noExtraBooleanCast"
	| "lint/complexity/noForEach"
	| "lint/complexity/noMultipleSpacesInRegularExpressionLiterals"
	| "lint/complexity/noStaticOnlyClass"
	| "lint/complexity/noThisInStatic"
	| "lint/complexity/noUselessCatch"
	| "lint/complexity/noUselessConstructor"
	| "lint/complexity/noUselessEmptyExport"
	| "lint/complexity/noUselessFragments"
	| "lint/complexity/noUselessLabel"
	| "lint/complexity/noUselessLoneBlockStatements"
	| "lint/complexity/noUselessRename"
	| "lint/complexity/noUselessStringConcat"
	| "lint/complexity/noUselessSwitchCase"
	| "lint/complexity/noUselessTernary"
	| "lint/complexity/noUselessThisAlias"
	| "lint/complexity/noUselessTypeConstraint"
	| "lint/complexity/noUselessUndefinedInitialization"
	| "lint/complexity/noVoid"
	| "lint/complexity/noWith"
	| "lint/complexity/useArrowFunction"
	| "lint/complexity/useDateNow"
	| "lint/complexity/useFlatMap"
	| "lint/complexity/useLiteralKeys"
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
	| "lint/correctness/noFlatMapIdentity"
	| "lint/correctness/noGlobalObjectCalls"
	| "lint/correctness/noInnerDeclarations"
	| "lint/correctness/noInvalidBuiltinInstantiation"
	| "lint/correctness/noInvalidConstructorSuper"
	| "lint/correctness/noInvalidDirectionInLinearGradient"
	| "lint/correctness/noInvalidGridAreas"
	| "lint/correctness/noInvalidNewBuiltin"
	| "lint/correctness/noInvalidPositionAtImportRule"
	| "lint/correctness/noInvalidUseBeforeDeclaration"
	| "lint/correctness/noNewSymbol"
	| "lint/correctness/noNodejsModules"
	| "lint/correctness/noNonoctalDecimalEscape"
	| "lint/correctness/noPrecisionLoss"
	| "lint/correctness/noRenderReturnValue"
	| "lint/correctness/noSelfAssign"
	| "lint/correctness/noSetterReturn"
	| "lint/correctness/noStringCaseMismatch"
	| "lint/correctness/noSwitchDeclarations"
	| "lint/correctness/noUndeclaredDependencies"
	| "lint/correctness/noUndeclaredVariables"
	| "lint/correctness/noUnknownFunction"
	| "lint/correctness/noUnknownMediaFeatureName"
	| "lint/correctness/noUnknownProperty"
	| "lint/correctness/noUnknownUnit"
	| "lint/correctness/noUnmatchableAnbSelector"
	| "lint/correctness/noUnnecessaryContinue"
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
	| "lint/correctness/useArrayLiterals"
	| "lint/correctness/useExhaustiveDependencies"
	| "lint/correctness/useHookAtTopLevel"
	| "lint/correctness/useImportExtensions"
	| "lint/correctness/useIsNan"
	| "lint/correctness/useJsxKeyInIterable"
	| "lint/correctness/useValidForDirection"
	| "lint/correctness/useYield"
	| "lint/nursery/colorNoInvalidHex"
	| "lint/nursery/noAwaitInLoop"
	| "lint/nursery/noColorInvalidHex"
	| "lint/nursery/noCommonJs"
	| "lint/nursery/noConsole"
	| "lint/nursery/noConstantBinaryExpression"
	| "lint/nursery/noDescendingSpecificity"
	| "lint/nursery/noDocumentCookie"
	| "lint/nursery/noDocumentImportInPage"
	| "lint/nursery/noDoneCallback"
	| "lint/nursery/noDuplicateAtImportRules"
	| "lint/nursery/noDuplicateCustomProperties"
	| "lint/nursery/noDuplicateElseIf"
	| "lint/nursery/noDuplicateProperties"
	| "lint/nursery/noDuplicatedFields"
	| "lint/nursery/noDynamicNamespaceImportAccess"
	| "lint/nursery/noEnum"
	| "lint/nursery/noExportedImports"
	| "lint/nursery/noFloatingPromises"
	| "lint/nursery/noGlobalDirnameFilename"
	| "lint/nursery/noHeadElement"
	| "lint/nursery/noHeadImportInDocument"
	| "lint/nursery/noImgElement"
	| "lint/nursery/noImportCycles"
	| "lint/nursery/noImportantInKeyframe"
	| "lint/nursery/noInvalidDirectionInLinearGradient"
	| "lint/nursery/noInvalidGridAreas"
	| "lint/nursery/noInvalidPositionAtImportRule"
	| "lint/nursery/noIrregularWhitespace"
	| "lint/nursery/noMissingGenericFamilyKeyword"
	| "lint/nursery/noMissingVarFunction"
	| "lint/nursery/noNestedTernary"
	| "lint/nursery/noNoninteractiveElementInteractions"
	| "lint/nursery/noOctalEscape"
	| "lint/nursery/noPackagePrivateImports"
	| "lint/nursery/noProcessEnv"
	| "lint/nursery/noProcessGlobal"
	| "lint/nursery/noReactSpecificProps"
	| "lint/nursery/noRestrictedImports"
	| "lint/nursery/noRestrictedTypes"
	| "lint/nursery/noSecrets"
	| "lint/nursery/noShorthandPropertyOverrides"
	| "lint/nursery/noStaticElementInteractions"
	| "lint/nursery/noSubstr"
	| "lint/nursery/noTemplateCurlyInString"
	| "lint/nursery/noTsIgnore"
	| "lint/nursery/noUndeclaredDependencies"
	| "lint/nursery/noUnknownAtRule"
	| "lint/nursery/noUnknownFunction"
	| "lint/nursery/noUnknownMediaFeatureName"
	| "lint/nursery/noUnknownProperty"
	| "lint/nursery/noUnknownPseudoClass"
	| "lint/nursery/noUnknownPseudoClassSelector"
	| "lint/nursery/noUnknownPseudoElement"
	| "lint/nursery/noUnknownSelectorPseudoElement"
	| "lint/nursery/noUnknownTypeSelector"
	| "lint/nursery/noUnknownUnit"
	| "lint/nursery/noUnmatchableAnbSelector"
	| "lint/nursery/noUnusedFunctionParameters"
	| "lint/nursery/noUnwantedPolyfillio"
	| "lint/nursery/noUselessEscapeInRegex"
	| "lint/nursery/noUselessStringRaw"
	| "lint/nursery/noUselessUndefined"
	| "lint/nursery/noValueAtRule"
	| "lint/nursery/useAdjacentOverloadSignatures"
	| "lint/nursery/useAriaPropsSupportedByRole"
	| "lint/nursery/useAtIndex"
	| "lint/nursery/useBiomeSuppressionComment"
	| "lint/nursery/useCollapsedIf"
	| "lint/nursery/useComponentExportOnlyModules"
	| "lint/nursery/useConsistentCurlyBraces"
	| "lint/nursery/useConsistentMemberAccessibility"
	| "lint/nursery/useDeprecatedReason"
	| "lint/nursery/useExplicitFunctionReturnType"
	| "lint/nursery/useExplicitType"
	| "lint/nursery/useExportsLast"
	| "lint/nursery/useGoogleFontDisplay"
	| "lint/nursery/useGoogleFontPreconnect"
	| "lint/nursery/useGuardForIn"
	| "lint/nursery/useImportRestrictions"
	| "lint/nursery/useJsxCurlyBraceConvention"
	| "lint/nursery/useNamedOperation"
	| "lint/nursery/useNamingConvention"
	| "lint/nursery/useParseIntRadix"
	| "lint/nursery/useSortedClasses"
	| "lint/nursery/useSortedProperties"
	| "lint/nursery/useStrictMode"
	| "lint/nursery/useTrimStartEnd"
	| "lint/nursery/useValidAutocomplete"
	| "lint/performance/noAccumulatingSpread"
	| "lint/performance/noBarrelFile"
	| "lint/performance/noDelete"
	| "lint/performance/noReExportAll"
	| "lint/performance/useTopLevelRegex"
	| "lint/security/noDangerouslySetInnerHtml"
	| "lint/security/noDangerouslySetInnerHtmlWithChildren"
	| "lint/security/noGlobalEval"
	| "lint/style/noArguments"
	| "lint/style/noCommaOperator"
	| "lint/style/noDefaultExport"
	| "lint/style/noDoneCallback"
	| "lint/style/noImplicitBoolean"
	| "lint/style/noInferrableTypes"
	| "lint/style/noNamespace"
	| "lint/style/noNamespaceImport"
	| "lint/style/noNegationElse"
	| "lint/style/noNonNullAssertion"
	| "lint/style/noParameterAssign"
	| "lint/style/noParameterProperties"
	| "lint/style/noRestrictedGlobals"
	| "lint/style/noShoutyConstants"
	| "lint/style/noUnusedTemplateLiteral"
	| "lint/style/noUselessElse"
	| "lint/style/noYodaExpression"
	| "lint/style/useAsConstAssertion"
	| "lint/style/useBlockStatements"
	| "lint/style/useCollapsedElseIf"
	| "lint/style/useConsistentArrayType"
	| "lint/style/useConsistentBuiltinInstantiation"
	| "lint/style/useConst"
	| "lint/style/useDefaultParameterLast"
	| "lint/style/useDefaultSwitchClause"
	| "lint/style/useEnumInitializers"
	| "lint/style/useExplicitLengthCheck"
	| "lint/style/useExponentiationOperator"
	| "lint/style/useExportType"
	| "lint/style/useFilenamingConvention"
	| "lint/style/useForOf"
	| "lint/style/useFragmentSyntax"
	| "lint/style/useImportType"
	| "lint/style/useLiteralEnumMembers"
	| "lint/style/useNamingConvention"
	| "lint/style/useNodeAssertStrict"
	| "lint/style/useNodejsImportProtocol"
	| "lint/style/useNumberNamespace"
	| "lint/style/useNumericLiterals"
	| "lint/style/useSelfClosingElements"
	| "lint/style/useShorthandArrayType"
	| "lint/style/useShorthandAssign"
	| "lint/style/useShorthandFunctionType"
	| "lint/style/useSingleCaseStatement"
	| "lint/style/useSingleVarDeclarator"
	| "lint/style/useTemplate"
	| "lint/style/useThrowNewError"
	| "lint/style/useThrowOnlyError"
	| "lint/suspicious/noApproximativeNumericConstant"
	| "lint/suspicious/noArrayIndexKey"
	| "lint/suspicious/noAssignInExpressions"
	| "lint/suspicious/noAsyncPromiseExecutor"
	| "lint/suspicious/noCatchAssign"
	| "lint/suspicious/noClassAssign"
	| "lint/suspicious/noCommentText"
	| "lint/suspicious/noCompareNegZero"
	| "lint/suspicious/noConfusingLabels"
	| "lint/suspicious/noConfusingVoidType"
	| "lint/suspicious/noConsole"
	| "lint/suspicious/noConstEnum"
	| "lint/suspicious/noControlCharactersInRegex"
	| "lint/suspicious/noDebugger"
	| "lint/suspicious/noDoubleEquals"
	| "lint/suspicious/noDuplicateAtImportRules"
	| "lint/suspicious/noDuplicateCase"
	| "lint/suspicious/noDuplicateClassMembers"
	| "lint/suspicious/noDuplicateFontNames"
	| "lint/suspicious/noDuplicateJsxProps"
	| "lint/suspicious/noDuplicateObjectKeys"
	| "lint/suspicious/noDuplicateParameters"
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
	| "lint/suspicious/noImplicitAnyLet"
	| "lint/suspicious/noImportAssign"
	| "lint/suspicious/noImportantInKeyframe"
	| "lint/suspicious/noLabelVar"
	| "lint/suspicious/noMisleadingCharacterClass"
	| "lint/suspicious/noMisleadingInstantiator"
	| "lint/suspicious/noMisplacedAssertion"
	| "lint/suspicious/noMisrefactoredShorthandAssign"
	| "lint/suspicious/noPrototypeBuiltins"
	| "lint/suspicious/noReactSpecificProps"
	| "lint/suspicious/noRedeclare"
	| "lint/suspicious/noRedundantUseStrict"
	| "lint/suspicious/noSelfCompare"
	| "lint/suspicious/noShadowRestrictedNames"
	| "lint/suspicious/noShorthandPropertyOverrides"
	| "lint/suspicious/noSkippedTests"
	| "lint/suspicious/noSparseArray"
	| "lint/suspicious/noSuspiciousSemicolonInJsx"
	| "lint/suspicious/noThenProperty"
	| "lint/suspicious/noUnsafeDeclarationMerging"
	| "lint/suspicious/noUnsafeNegation"
	| "lint/suspicious/noVar"
	| "lint/suspicious/useAwait"
	| "lint/suspicious/useDefaultSwitchClauseLast"
	| "lint/suspicious/useErrorMessage"
	| "lint/suspicious/useGetterReturn"
	| "lint/suspicious/useIsArray"
	| "lint/suspicious/useNamespaceKeyword"
	| "lint/suspicious/useNumberToFixedDigitsArgument"
	| "lint/suspicious/useValidTypeof"
	| "assist/source/useSortedKeys"
	| "syntax/correctness/noTypeOnlyImportAttributes"
	| "syntax/correctness/noSuperWithoutExtends"
	| "syntax/correctness/noInitializerWithDefinite"
	| "syntax/correctness/noDuplicatePrivateClassMembers"
	| "files/missingHandler"
	| "format"
	| "check"
	| "ci"
	| "stdin"
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
	| "reporter/assist"
	| "reporter/linter"
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
	| "suppressions/parse"
	| "suppressions/unknownGroup"
	| "suppressions/unknownRule"
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
	version: number;
}
export type FileContent =
	| { content: string; type: "fromClient" }
	| { type: "fromServer" };
export type DocumentFileSource =
	| "Unknown"
	| { Js: JsFileSource }
	| { Json: JsonFileSource }
	| { Css: CssFileSource }
	| { Graphql: GraphqlFileSource }
	| { Html: HtmlFileSource }
	| { Grit: GritFileSource };
export interface JsFileSource {
	/**
	 * Used to mark if the source is being used for an Astro, Svelte or Vue file
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

Currently, Biome only supports plain CSS, and aims to be compatible with the latest Recommendation level standards. 
	 */
export type CssVariant = "standard";
/**
 * The style of GraphQL contained in the file.
 */
export type GraphqlVariant = "standard";
export type HtmlVariant = "Standard" | "Astro";
export type GritVariant = "Standard";
export interface ChangeFileParams {
	content: string;
	path: BiomePath;
	projectKey: ProjectKey;
	version: number;
}
export interface CloseFileParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
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
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	/**
	 * Rules to apply on top of the configuration
	 */
	enabledRules?: RuleCode[];
	maxDiagnostics: number;
	only?: RuleCode[];
	path: BiomePath;
	projectKey: ProjectKey;
	skip?: RuleCode[];
}
export type RuleCategories = RuleCategory[];
export type RuleCode = string;
export type RuleCategory = "syntax" | "lint" | "action" | "transformation";
export interface PullDiagnosticsResult {
	diagnostics: Diagnostic[];
	errors: number;
	skippedDiagnostics: number;
}
export interface PullActionsParams {
	enabledRules?: RuleCode[];
	only?: RuleCode[];
	path: BiomePath;
	projectKey: ProjectKey;
	range?: TextRange;
	skip?: RuleCode[];
	suppressionReason?: string;
}
export interface PullActionsResult {
	actions: CodeAction[];
}
export interface CodeAction {
	category: ActionCategory;
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
	enabledRules?: RuleCode[];
	fixFileMode: FixFileMode;
	only?: RuleCode[];
	path: BiomePath;
	projectKey: ProjectKey;
	ruleCategories: RuleCategories;
	shouldFormat: boolean;
	skip?: RuleCode[];
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
export type SupportKind =
	| "supported"
	| "ignored"
	| "protected"
	| "featureNotEnabled"
	| "fileNotSupported";
/**
 * Rule domains
 */
export type RuleDomain = "react" | "test" | "solid" | "next";
export type RuleDomainValue = "all" | "none" | "recommended";
export interface Workspace {
	fileFeatures(params: SupportsFeatureParams): Promise<FileFeaturesResult>;
	updateSettings(params: UpdateSettingsParams): Promise<UpdateSettingsResult>;
	openProject(params: OpenProjectParams): Promise<ProjectKey>;
	openFile(params: OpenFileParams): Promise<void>;
	changeFile(params: ChangeFileParams): Promise<void>;
	closeFile(params: CloseFileParams): Promise<void>;
	getSyntaxTree(params: GetSyntaxTreeParams): Promise<GetSyntaxTreeResult>;
	checkFileSize(params: CheckFileSizeParams): Promise<CheckFileSizeResult>;
	getFileContent(params: GetFileContentParams): Promise<string>;
	getControlFlowGraph(params: GetControlFlowGraphParams): Promise<string>;
	getFormatterIr(params: GetFormatterIRParams): Promise<string>;
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
		openFile(params) {
			return transport.request("biome/open_file", params);
		},
		changeFile(params) {
			return transport.request("biome/change_file", params);
		},
		closeFile(params) {
			return transport.request("biome/close_file", params);
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
