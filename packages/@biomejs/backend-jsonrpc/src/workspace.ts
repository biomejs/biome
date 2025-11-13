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
	* Indicates whether this configuration file is at the root of a Biome
project. By default, this is `true`. 
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
	* A list of glob patterns. Biome will include files/folders that will
match these patterns. 
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
	* **Deprecated:** Please use _force-ignore syntax_ in `files.includes`
instead: <https://biomejs.dev/reference/configuration/#filesincludes>

Set of file and folder names that should be unconditionally ignored by
Biome's scanner. 
	 */
	experimentalScannerIgnores?: string[];
	/**
	 * Tells Biome to not emit diagnostics when handling files that it doesn't know
	 */
	ignoreUnknown?: Bool;
	/**
	* A list of glob patterns. Biome will handle only those files/folders that will
match these patterns. 
	 */
	includes?: NormalizedGlob[];
	/**
	* The maximum allowed size for source code files in bytes. Files above
this limit will be ignored for performance reasons. Defaults to 1 MiB 
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
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
	 */
	expand?: Expand;
	/**
	* Whether formatting should be allowed to proceed if a given file
has syntax errors 
	 */
	formatWithErrors?: Bool;
	/**
	* A list of glob patterns. The formatter will include files/folders that will
match these patterns. 
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
	* Use any `.editorconfig` files to configure the formatter. Configuration
in `biome.json` will override `.editorconfig` configuration.

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
	* A list of glob patterns. The analyzer will handle only those files/folders that will
match these patterns. 
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
	* The folder where Biome should check for VCS files. By default, Biome will use the same
folder where `biome.json` was found.

If Biome can't find the configuration, it will attempt to use the current working directory.
If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic
will be emitted 
	 */
	root?: string;
	/**
	* Whether Biome should use the VCS ignore file. When [true], Biome will ignore the files
specified in the ignore file. 
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
export type NormalizedGlob = string;
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
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
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
	trailingCommas?: JsTrailingCommas;
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
	* Enables parsing of Grit metavariables.
Defaults to `false`. 
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
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
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
	trailingCommas?: JsonTrailingCommas;
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
export type RuleDomains = { [K in any]?: any };
export interface Rules {
	a11y?: SeverityOrA11y;
	complexity?: SeverityOrComplexity;
	correctness?: SeverityOrCorrectness;
	nursery?: SeverityOrNursery;
	performance?: SeverityOrPerformance;
	/**
	 * It enables the lint rules recommended by Biome. `true` by default.
	 */
	recommended?: boolean;
	security?: SeverityOrSecurity;
	style?: SeverityOrStyle;
	suspicious?: SeverityOrSuspicious;
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
	* A list of glob patterns. Biome will include files/folders that will
match these patterns. 
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
	organizeImports?: OrganizeImportsConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce attribute sorting in JSX elements.
	 */
	useSortedAttributes?: UseSortedAttributesConfiguration;
	/**
	 * Sort the keys of a JSON object in natural order.
	 */
	useSortedKeys?: UseSortedKeysConfiguration;
	/**
	 * Enforce ordering of CSS properties and nested rules.
	 */
	useSortedProperties?: UseSortedPropertiesConfiguration;
}
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

|                |      html      |    output    |
| -------------- | :------------: | :----------: |
| with spaces    | `1<b> 2 </b>3` | 1<b> 2 </b>3 |
| without spaces |  `1<b>2</b>3`  |  1<b>2</b>3  |

This happens because whitespace is significant in inline elements.

As a consequence of this, the formatter must format blocks that look like this (assume a small line width, <20):
```html
<span>really long content</span>
```
as this, where the content hugs the tags:
```html
<span
   >really long content</span
>
```

Note that this is only necessary for inline elements. Block elements do not have this restriction. 
	 */
export type WhitespaceSensitivity = "css" | "strict" | "ignore";
export type ArrowParentheses = "always" | "asNeeded";
export type OperatorLinebreak = "after" | "before";
export type QuoteProperties = "asNeeded" | "preserve";
export type Semicolons = "always" | "asNeeded";
/**
 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures for JavaScript/TypeScript files.
 */
export type JsTrailingCommas = "all" | "es5" | "none";
/**
 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures for JSON files.
 */
export type JsonTrailingCommas = "none" | "all";
export type SeverityOrA11y = GroupPlainConfiguration | A11y;
export type SeverityOrComplexity = GroupPlainConfiguration | Complexity;
export type SeverityOrCorrectness = GroupPlainConfiguration | Correctness;
export type SeverityOrNursery = GroupPlainConfiguration | Nursery;
export type SeverityOrPerformance = GroupPlainConfiguration | Performance;
export type SeverityOrSecurity = GroupPlainConfiguration | Security;
export type SeverityOrStyle = GroupPlainConfiguration | Style;
export type SeverityOrSuspicious = GroupPlainConfiguration | Suspicious;
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
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
	 */
	expand?: Expand;
	/**
	* Stores whether formatting should be allowed to proceed if a given file
has syntax errors 
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
export type OrganizeImportsConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOrganizeImportsOptions;
export type UseSortedAttributesConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithUseSortedAttributesOptions;
export type UseSortedKeysConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithUseSortedKeysOptions;
export type UseSortedPropertiesConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithUseSortedPropertiesOptions;
export type GroupPlainConfiguration = "off" | "on" | "info" | "warn" | "error";
/**
 * A list of rules that belong to this group
 */
export interface A11y {
	/**
	 * Enforce that the accessKey attribute is not used on any HTML element.
	 */
	noAccessKey?: NoAccessKeyConfiguration;
	/**
	 * Enforce that aria-hidden="true" is not set on focusable elements.
	 */
	noAriaHiddenOnFocusable?: NoAriaHiddenOnFocusableConfiguration;
	/**
	 * Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
	 */
	noAriaUnsupportedElements?: NoAriaUnsupportedElementsConfiguration;
	/**
	 * Enforce that autoFocus prop is not used on elements.
	 */
	noAutofocus?: NoAutofocusConfiguration;
	/**
	 * Enforces that no distracting elements are used.
	 */
	noDistractingElements?: NoDistractingElementsConfiguration;
	/**
	 * The scope prop should be used only on \<th> elements.
	 */
	noHeaderScope?: NoHeaderScopeConfiguration;
	/**
	 * Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
	 */
	noInteractiveElementToNoninteractiveRole?: NoInteractiveElementToNoninteractiveRoleConfiguration;
	/**
	 * Enforce that a label element or component has a text label and an associated input.
	 */
	noLabelWithoutControl?: NoLabelWithoutControlConfiguration;
	/**
	 * Disallow use event handlers on non-interactive elements.
	 */
	noNoninteractiveElementInteractions?: NoNoninteractiveElementInteractionsConfiguration;
	/**
	 * Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveElementToInteractiveRole?: NoNoninteractiveElementToInteractiveRoleConfiguration;
	/**
	 * Enforce that tabIndex is not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveTabindex?: NoNoninteractiveTabindexConfiguration;
	/**
	 * Prevent the usage of positive integers on tabIndex property
	 */
	noPositiveTabindex?: NoPositiveTabindexConfiguration;
	/**
	 * Enforce img alt prop does not contain the word "image", "picture", or "photo".
	 */
	noRedundantAlt?: NoRedundantAltConfiguration;
	/**
	 * Enforce explicit role property is not the same as implicit/default role property on an element.
	 */
	noRedundantRoles?: NoRedundantRolesConfiguration;
	/**
	 * Enforce that static, visible elements (such as \<div>) that have click handlers use the valid role attribute.
	 */
	noStaticElementInteractions?: NoStaticElementInteractionsConfiguration;
	/**
	 * Enforces the usage of the title element for the svg element.
	 */
	noSvgWithoutTitle?: NoSvgWithoutTitleConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce that all elements that require alternative text have meaningful information to relay back to the end user.
	 */
	useAltText?: UseAltTextConfiguration;
	/**
	 * Enforce that anchors have content and that the content is accessible to screen readers.
	 */
	useAnchorContent?: UseAnchorContentConfiguration;
	/**
	 * Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant.
	 */
	useAriaActivedescendantWithTabindex?: UseAriaActivedescendantWithTabindexConfiguration;
	/**
	 * Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
	 */
	useAriaPropsForRole?: UseAriaPropsForRoleConfiguration;
	/**
	 * Enforce that ARIA properties are valid for the roles that are supported by the element.
	 */
	useAriaPropsSupportedByRole?: UseAriaPropsSupportedByRoleConfiguration;
	/**
	 * Enforces the usage of the attribute type for the element button
	 */
	useButtonType?: UseButtonTypeConfiguration;
	/**
	 * Elements with an interactive role and interaction handlers must be focusable.
	 */
	useFocusableInteractive?: UseFocusableInteractiveConfiguration;
	/**
	 * Disallow a missing generic family keyword within font families.
	 */
	useGenericFontNames?: UseGenericFontNamesConfiguration;
	/**
	 * Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.
	 */
	useHeadingContent?: UseHeadingContentConfiguration;
	/**
	 * Enforce that html element has lang attribute.
	 */
	useHtmlLang?: UseHtmlLangConfiguration;
	/**
	 * Enforces the usage of the attribute title for the element iframe.
	 */
	useIframeTitle?: UseIframeTitleConfiguration;
	/**
	 * Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress.
	 */
	useKeyWithClickEvents?: UseKeyWithClickEventsConfiguration;
	/**
	 * Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur.
	 */
	useKeyWithMouseEvents?: UseKeyWithMouseEventsConfiguration;
	/**
	 * Enforces that audio and video elements must have a track for captions.
	 */
	useMediaCaption?: UseMediaCaptionConfiguration;
	/**
	 * It detects the use of role attributes in JSX elements and suggests using semantic elements instead.
	 */
	useSemanticElements?: UseSemanticElementsConfiguration;
	/**
	 * Enforce that all anchors are valid, and they are navigable elements.
	 */
	useValidAnchor?: UseValidAnchorConfiguration;
	/**
	 * Ensures that ARIA properties aria-* are all valid.
	 */
	useValidAriaProps?: UseValidAriaPropsConfiguration;
	/**
	 * Elements with ARIA roles must use a valid, non-abstract ARIA role.
	 */
	useValidAriaRole?: UseValidAriaRoleConfiguration;
	/**
	 * Enforce that ARIA state and property values are valid.
	 */
	useValidAriaValues?: UseValidAriaValuesConfiguration;
	/**
	 * Use valid values for the autocomplete attribute on input elements.
	 */
	useValidAutocomplete?: UseValidAutocompleteConfiguration;
	/**
	 * Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country.
	 */
	useValidLang?: UseValidLangConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Complexity {
	/**
	 * Disallow unclear usage of consecutive space characters in regular expression literals
	 */
	noAdjacentSpacesInRegex?: NoAdjacentSpacesInRegexConfiguration;
	/**
	 * Disallow the use of arguments.
	 */
	noArguments?: NoArgumentsConfiguration;
	/**
	 * Disallow primitive type aliases and misleading types.
	 */
	noBannedTypes?: NoBannedTypesConfiguration;
	/**
	 * Disallow comma operator.
	 */
	noCommaOperator?: NoCommaOperatorConfiguration;
	/**
	 * Disallow empty type parameters in type aliases and interfaces.
	 */
	noEmptyTypeParameters?: NoEmptyTypeParametersConfiguration;
	/**
	 * Disallow functions that exceed a given Cognitive Complexity score.
	 */
	noExcessiveCognitiveComplexity?: NoExcessiveCognitiveComplexityConfiguration;
	/**
	 * Restrict the number of lines of code in a function.
	 */
	noExcessiveLinesPerFunction?: NoExcessiveLinesPerFunctionConfiguration;
	/**
	 * This rule enforces a maximum depth to nested describe() in test files.
	 */
	noExcessiveNestedTestSuites?: NoExcessiveNestedTestSuitesConfiguration;
	/**
	 * Disallow unnecessary boolean casts
	 */
	noExtraBooleanCast?: NoExtraBooleanCastConfiguration;
	/**
	 * Disallow to use unnecessary callback on flatMap.
	 */
	noFlatMapIdentity?: NoFlatMapIdentityConfiguration;
	/**
	 * Prefer for...of statement instead of Array.forEach.
	 */
	noForEach?: NoForEachConfiguration;
	/**
	 * Disallow shorthand type conversions.
	 */
	noImplicitCoercions?: NoImplicitCoercionsConfiguration;
	/**
	 * Disallow the use of the !important style.
	 */
	noImportantStyles?: NoImportantStylesConfiguration;
	/**
	 * This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
	 */
	noStaticOnlyClass?: NoStaticOnlyClassConfiguration;
	/**
	 * Disallow this and super in static contexts.
	 */
	noThisInStatic?: NoThisInStaticConfiguration;
	/**
	 * Disallow unnecessary catch clauses.
	 */
	noUselessCatch?: NoUselessCatchConfiguration;
	/**
	 * Disallow unnecessary constructors.
	 */
	noUselessConstructor?: NoUselessConstructorConfiguration;
	/**
	 * Avoid using unnecessary continue.
	 */
	noUselessContinue?: NoUselessContinueConfiguration;
	/**
	 * Disallow empty exports that don't change anything in a module file.
	 */
	noUselessEmptyExport?: NoUselessEmptyExportConfiguration;
	/**
	 * Disallow unnecessary escape sequence in regular expression literals.
	 */
	noUselessEscapeInRegex?: NoUselessEscapeInRegexConfiguration;
	/**
	 * Disallow unnecessary fragments
	 */
	noUselessFragments?: NoUselessFragmentsConfiguration;
	/**
	 * Disallow unnecessary labels.
	 */
	noUselessLabel?: NoUselessLabelConfiguration;
	/**
	 * Disallow unnecessary nested block statements.
	 */
	noUselessLoneBlockStatements?: NoUselessLoneBlockStatementsConfiguration;
	/**
	 * Disallow renaming import, export, and destructured assignments to the same name.
	 */
	noUselessRename?: NoUselessRenameConfiguration;
	/**
	 * Disallow unnecessary concatenation of string or template literals.
	 */
	noUselessStringConcat?: NoUselessStringConcatConfiguration;
	/**
	 * Disallow unnecessary String.raw function in template string literals without any escape sequence.
	 */
	noUselessStringRaw?: NoUselessStringRawConfiguration;
	/**
	 * Disallow useless case in switch statements.
	 */
	noUselessSwitchCase?: NoUselessSwitchCaseConfiguration;
	/**
	 * Disallow ternary operators when simpler alternatives exist.
	 */
	noUselessTernary?: NoUselessTernaryConfiguration;
	/**
	 * Disallow useless this aliasing.
	 */
	noUselessThisAlias?: NoUselessThisAliasConfiguration;
	/**
	 * Disallow using any or unknown as type constraint.
	 */
	noUselessTypeConstraint?: NoUselessTypeConstraintConfiguration;
	/**
	 * Disallow initializing variables to undefined.
	 */
	noUselessUndefinedInitialization?: NoUselessUndefinedInitializationConfiguration;
	/**
	 * Disallow the use of void operators, which is not a familiar operator.
	 */
	noVoid?: NoVoidConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Use arrow functions over function expressions.
	 */
	useArrowFunction?: UseArrowFunctionConfiguration;
	/**
	 * Use Date.now() to get the number of milliseconds since the Unix Epoch.
	 */
	useDateNow?: UseDateNowConfiguration;
	/**
	 * Promotes the use of .flatMap() when map().flat() are used together.
	 */
	useFlatMap?: UseFlatMapConfiguration;
	/**
	 * Prefer Array#{indexOf,lastIndexOf}() over Array#{findIndex,findLastIndex}() when looking for the index of an item.
	 */
	useIndexOf?: UseIndexOfConfiguration;
	/**
	 * Enforce the usage of a literal access to properties over computed property access.
	 */
	useLiteralKeys?: UseLiteralKeysConfiguration;
	/**
	 * Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals
	 */
	useNumericLiterals?: UseNumericLiteralsConfiguration;
	/**
	 * Enforce using concise optional chain instead of chained logical expressions.
	 */
	useOptionalChain?: UseOptionalChainConfiguration;
	/**
	 * Enforce the use of the regular expression literals instead of the RegExp constructor if possible.
	 */
	useRegexLiterals?: UseRegexLiteralsConfiguration;
	/**
	 * Disallow number literal object member names which are not base 10 or use underscore as separator.
	 */
	useSimpleNumberKeys?: UseSimpleNumberKeysConfiguration;
	/**
	 * Discard redundant terms from logical expressions.
	 */
	useSimplifiedLogicExpression?: UseSimplifiedLogicExpressionConfiguration;
	/**
	 * Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed.
	 */
	useWhile?: UseWhileConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	/**
	 * Prevent passing of children as props.
	 */
	noChildrenProp?: NoChildrenPropConfiguration;
	/**
	 * Prevents from having const variables being re-assigned.
	 */
	noConstAssign?: NoConstAssignConfiguration;
	/**
	 * Disallow constant expressions in conditions
	 */
	noConstantCondition?: NoConstantConditionConfiguration;
	/**
	 * Disallow the use of Math.min and Math.max to clamp a value where the result itself is constant.
	 */
	noConstantMathMinMaxClamp?: NoConstantMathMinMaxClampConfiguration;
	/**
	 * Disallow returning a value from a constructor.
	 */
	noConstructorReturn?: NoConstructorReturnConfiguration;
	/**
	 * Disallow empty character classes in regular expression literals.
	 */
	noEmptyCharacterClassInRegex?: NoEmptyCharacterClassInRegexConfiguration;
	/**
	 * Disallows empty destructuring patterns.
	 */
	noEmptyPattern?: NoEmptyPatternConfiguration;
	/**
	 * Disallow the use of __dirname and __filename in the global scope.
	 */
	noGlobalDirnameFilename?: NoGlobalDirnameFilenameConfiguration;
	/**
	 * Disallow calling global object properties as functions
	 */
	noGlobalObjectCalls?: NoGlobalObjectCallsConfiguration;
	/**
	 * Disallow function and var declarations that are accessible outside their block.
	 */
	noInnerDeclarations?: NoInnerDeclarationsConfiguration;
	/**
	 * Ensure that builtins are correctly instantiated.
	 */
	noInvalidBuiltinInstantiation?: NoInvalidBuiltinInstantiationConfiguration;
	/**
	 * Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors.
	 */
	noInvalidConstructorSuper?: NoInvalidConstructorSuperConfiguration;
	/**
	 * Disallow non-standard direction values for linear gradient functions.
	 */
	noInvalidDirectionInLinearGradient?: NoInvalidDirectionInLinearGradientConfiguration;
	/**
	 * Disallows invalid named grid areas in CSS Grid Layouts.
	 */
	noInvalidGridAreas?: NoInvalidGridAreasConfiguration;
	/**
	 * Disallow the use of @import at-rules in invalid positions.
	 */
	noInvalidPositionAtImportRule?: NoInvalidPositionAtImportRuleConfiguration;
	/**
	 * Disallow the use of variables and function parameters before their declaration
	 */
	noInvalidUseBeforeDeclaration?: NoInvalidUseBeforeDeclarationConfiguration;
	/**
	 * Disallow missing var function for css variables.
	 */
	noMissingVarFunction?: NoMissingVarFunctionConfiguration;
	/**
	 * Disallows defining React components inside other components.
	 */
	noNestedComponentDefinitions?: NoNestedComponentDefinitionsConfiguration;
	/**
	 * Forbid the use of Node.js builtin modules.
	 */
	noNodejsModules?: NoNodejsModulesConfiguration;
	/**
	 * Disallow \8 and \9 escape sequences in string literals.
	 */
	noNonoctalDecimalEscape?: NoNonoctalDecimalEscapeConfiguration;
	/**
	 * Disallow literal numbers that lose precision
	 */
	noPrecisionLoss?: NoPrecisionLossConfiguration;
	/**
	 * Restrict imports of private exports.
	 */
	noPrivateImports?: NoPrivateImportsConfiguration;
	/**
	 * Disallow the use of process global.
	 */
	noProcessGlobal?: NoProcessGlobalConfiguration;
	/**
	 * Disallow useVisibleTask$() functions in Qwik components.
	 */
	noQwikUseVisibleTask?: NoQwikUseVisibleTaskConfiguration;
	/**
	 * Disallow assigning to React component props.
	 */
	noReactPropAssignments?: NoReactPropAssignmentsConfiguration;
	/**
	 * Prevent the usage of the return value of React.render.
	 */
	noRenderReturnValue?: NoRenderReturnValueConfiguration;
	/**
	 * Disallow the use of configured elements.
	 */
	noRestrictedElements?: NoRestrictedElementsConfiguration;
	/**
	 * Disallow assignments where both sides are exactly the same.
	 */
	noSelfAssign?: NoSelfAssignConfiguration;
	/**
	 * Disallow returning a value from a setter
	 */
	noSetterReturn?: NoSetterReturnConfiguration;
	/**
	 * Disallow destructuring props inside JSX components in Solid projects.
	 */
	noSolidDestructuredProps?: NoSolidDestructuredPropsConfiguration;
	/**
	 * Disallow comparison of expressions modifying the string case with non-compliant value.
	 */
	noStringCaseMismatch?: NoStringCaseMismatchConfiguration;
	/**
	 * Disallow lexical declarations in switch clauses.
	 */
	noSwitchDeclarations?: NoSwitchDeclarationsConfiguration;
	/**
	 * Disallow the use of dependencies that aren't specified in the package.json.
	 */
	noUndeclaredDependencies?: NoUndeclaredDependenciesConfiguration;
	/**
	 * Prevents the usage of variables that haven't been declared inside the document.
	 */
	noUndeclaredVariables?: NoUndeclaredVariablesConfiguration;
	/**
	 * Disallow unknown CSS value functions.
	 */
	noUnknownFunction?: NoUnknownFunctionConfiguration;
	/**
	 * Disallow unknown media feature names.
	 */
	noUnknownMediaFeatureName?: NoUnknownMediaFeatureNameConfiguration;
	/**
	 * Disallow unknown properties.
	 */
	noUnknownProperty?: NoUnknownPropertyConfiguration;
	/**
	 * Disallow unknown pseudo-class selectors.
	 */
	noUnknownPseudoClass?: NoUnknownPseudoClassConfiguration;
	/**
	 * Disallow unknown pseudo-element selectors.
	 */
	noUnknownPseudoElement?: NoUnknownPseudoElementConfiguration;
	/**
	 * Disallow unknown type selectors.
	 */
	noUnknownTypeSelector?: NoUnknownTypeSelectorConfiguration;
	/**
	 * Disallow unknown CSS units.
	 */
	noUnknownUnit?: NoUnknownUnitConfiguration;
	/**
	 * Disallow unmatchable An+B selectors.
	 */
	noUnmatchableAnbSelector?: NoUnmatchableAnbSelectorConfiguration;
	/**
	 * Disallow unreachable code
	 */
	noUnreachable?: NoUnreachableConfiguration;
	/**
	 * Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass
	 */
	noUnreachableSuper?: NoUnreachableSuperConfiguration;
	/**
	 * Disallow control flow statements in finally blocks.
	 */
	noUnsafeFinally?: NoUnsafeFinallyConfiguration;
	/**
	 * Disallow the use of optional chaining in contexts where the undefined value is not allowed.
	 */
	noUnsafeOptionalChaining?: NoUnsafeOptionalChainingConfiguration;
	/**
	 * Disallow unused function parameters.
	 */
	noUnusedFunctionParameters?: NoUnusedFunctionParametersConfiguration;
	/**
	 * Disallow unused imports.
	 */
	noUnusedImports?: NoUnusedImportsConfiguration;
	/**
	 * Disallow unused labels.
	 */
	noUnusedLabels?: NoUnusedLabelsConfiguration;
	/**
	 * Disallow unused private class members
	 */
	noUnusedPrivateClassMembers?: NoUnusedPrivateClassMembersConfiguration;
	/**
	 * Disallow unused variables.
	 */
	noUnusedVariables?: NoUnusedVariablesConfiguration;
	/**
	 * This rules prevents void elements (AKA self-closing elements) from having children.
	 */
	noVoidElementsWithChildren?: NoVoidElementsWithChildrenConfiguration;
	/**
	 * Disallow returning a value from a function with the return type 'void'
	 */
	noVoidTypeReturn?: NoVoidTypeReturnConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce all dependencies are correctly specified in a React hook.
	 */
	useExhaustiveDependencies?: UseExhaustiveDependenciesConfiguration;
	/**
	 * Enforce specifying the name of GraphQL operations.
	 */
	useGraphqlNamedOperations?: UseGraphqlNamedOperationsConfiguration;
	/**
	 * Enforce that all React hooks are being called from the Top Level component functions.
	 */
	useHookAtTopLevel?: UseHookAtTopLevelConfiguration;
	/**
	 * Enforces that \<img> elements have both width and height attributes.
	 */
	useImageSize?: UseImageSizeConfiguration;
	/**
	 * Enforce file extensions for relative imports.
	 */
	useImportExtensions?: UseImportExtensionsConfiguration;
	/**
	 * Require calls to isNaN() when checking for NaN.
	 */
	useIsNan?: UseIsNanConfiguration;
	/**
	 * Enforces the use of with { type: "json" } for JSON module imports.
	 */
	useJsonImportAttributes?: UseJsonImportAttributesConfiguration;
	/**
	 * Disallow missing key props in iterators/collection literals.
	 */
	useJsxKeyInIterable?: UseJsxKeyInIterableConfiguration;
	/**
	 * Enforce the consistent use of the radix argument when using parseInt().
	 */
	useParseIntRadix?: UseParseIntRadixConfiguration;
	/**
	 * Prefer using the class prop as a classlist over the classnames helper.
	 */
	useQwikClasslist?: UseQwikClasslistConfiguration;
	/**
	 * Enforce JSDoc comment lines to start with a single asterisk, except for the first one.
	 */
	useSingleJsDocAsterisk?: UseSingleJsDocAsteriskConfiguration;
	/**
	 * Prevent the usage of static string literal id attribute on elements.
	 */
	useUniqueElementIds?: UseUniqueElementIdsConfiguration;
	/**
	 * Enforce "for" loop update clause moving the counter in the right direction.
	 */
	useValidForDirection?: UseValidForDirectionConfiguration;
	/**
	 * This rule checks that the result of a typeof expression is compared to a valid value.
	 */
	useValidTypeof?: UseValidTypeofConfiguration;
	/**
	 * Require generator functions to contain yield.
	 */
	useYield?: UseYieldConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Nursery {
	/**
	 * Disallow continue statements.
	 */
	noContinue?: NoContinueConfiguration;
	/**
	 * Restrict imports of deprecated exports.
	 */
	noDeprecatedImports?: NoDeprecatedImportsConfiguration;
	/**
	 * Prevent the listing of duplicate dependencies. The rule supports the following dependency groups: "bundledDependencies", "bundleDependencies", "dependencies", "devDependencies", "overrides", "optionalDependencies", and "peerDependencies".
	 */
	noDuplicateDependencies?: NoDuplicateDependenciesConfiguration;
	/**
	 * Disallow empty sources.
	 */
	noEmptySource?: NoEmptySourceConfiguration;
	/**
	 * Require Promise-like statements to be handled appropriately.
	 */
	noFloatingPromises?: NoFloatingPromisesConfiguration;
	/**
	 * Disallow iterating using a for-in loop.
	 */
	noForIn?: NoForInConfiguration;
	/**
	 * Prevent import cycles.
	 */
	noImportCycles?: NoImportCyclesConfiguration;
	/**
	 * Disallows the usage of the unary operators ++ and --.
	 */
	noIncrementDecrement?: NoIncrementDecrementConfiguration;
	/**
	 * Disallow string literals inside JSX elements.
	 */
	noJsxLiterals?: NoJsxLiteralsConfiguration;
	/**
	 * Disallow Promises to be used in places where they are almost certainly a mistake.
	 */
	noMisusedPromises?: NoMisusedPromisesConfiguration;
	/**
	 * Prevent client components from being async functions.
	 */
	noNextAsyncClientComponent?: NoNextAsyncClientComponentConfiguration;
	/**
	 * Disallow function parameters that are only used in recursive calls.
	 */
	noParametersOnlyUsedInRecursion?: NoParametersOnlyUsedInRecursionConfiguration;
	/**
	 * Replaces usages of forwardRef with passing ref as a prop.
	 */
	noReactForwardRef?: NoReactForwardRefConfiguration;
	/**
	 * Disallow variable declarations from shadowing variables declared in the outer scope.
	 */
	noShadow?: NoShadowConfiguration;
	/**
	 * Disallow unknown DOM properties.
	 */
	noUnknownAttribute?: NoUnknownAttributeConfiguration;
	/**
	 * Disallow unnecessary type-based conditions that can be statically determined as redundant.
	 */
	noUnnecessaryConditions?: NoUnnecessaryConditionsConfiguration;
	/**
	 * Warn when importing non-existing exports.
	 */
	noUnresolvedImports?: NoUnresolvedImportsConfiguration;
	/**
	 * Disallow expression statements that are neither a function call nor an assignment.
	 */
	noUnusedExpressions?: NoUnusedExpressionsConfiguration;
	/**
	 * Disallow unused catch bindings.
	 */
	noUselessCatchBinding?: NoUselessCatchBindingConfiguration;
	/**
	 * Disallow the use of useless undefined.
	 */
	noUselessUndefined?: NoUselessUndefinedConfiguration;
	/**
	 * Enforce that Vue component data options are declared as functions.
	 */
	noVueDataObjectDeclaration?: NoVueDataObjectDeclarationConfiguration;
	/**
	 * Disallow duplicate keys in Vue component data, methods, computed properties, and other options.
	 */
	noVueDuplicateKeys?: NoVueDuplicateKeysConfiguration;
	/**
	 * Disallow reserved keys in Vue component data and computed properties.
	 */
	noVueReservedKeys?: NoVueReservedKeysConfiguration;
	/**
	 * Disallow reserved names to be used as props.
	 */
	noVueReservedProps?: NoVueReservedPropsConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Require Array#sort and Array#toSorted calls to always provide a compareFunction.
	 */
	useArraySortCompare?: UseArraySortCompareConfiguration;
	/**
	 * Enforce consistent arrow function bodies.
	 */
	useConsistentArrowReturn?: UseConsistentArrowReturnConfiguration;
	/**
	 * Require all descriptions to follow the same style (either block or inline) to  maintain consistency and improve readability across the schema.
	 */
	useConsistentGraphqlDescriptions?: UseConsistentGraphqlDescriptionsConfiguration;
	/**
	 * Require the @deprecated directive to specify a deletion date.
	 */
	useDeprecatedDate?: UseDeprecatedDateConfiguration;
	/**
	 * Require switch-case statements to be exhaustive.
	 */
	useExhaustiveSwitchCases?: UseExhaustiveSwitchCasesConfiguration;
	/**
	 * Enforce types in functions, methods, variables, and parameters.
	 */
	useExplicitType?: UseExplicitTypeConfiguration;
	/**
	 * Enforce a maximum number of parameters in function definitions.
	 */
	useMaxParams?: UseMaxParamsConfiguration;
	/**
	 * Disallow use* hooks outside of component$ or other use* hooks in Qwik applications.
	 */
	useQwikMethodUsage?: UseQwikMethodUsageConfiguration;
	/**
	 * Disallow unserializable expressions in Qwik dollar ($) scopes.
	 */
	useQwikValidLexicalScope?: UseQwikValidLexicalScopeConfiguration;
	/**
	 * Enforce the sorting of CSS utility classes.
	 */
	useSortedClasses?: UseSortedClassesConfiguration;
	/**
	 * Enforce unique operation names across a GraphQL document.
	 */
	useUniqueGraphqlOperationName?: UseUniqueGraphqlOperationNameConfiguration;
	/**
	 * Enforce specific order of Vue compiler macros.
	 */
	useVueDefineMacrosOrder?: UseVueDefineMacrosOrderConfiguration;
	/**
	 * Enforce multi-word component names in Vue components.
	 */
	useVueMultiWordComponentNames?: UseVueMultiWordComponentNamesConfiguration;
	/**
	 * Forbids v-bind directives with missing arguments or invalid modifiers.
	 */
	useVueValidVBind?: UseVueValidVBindConfiguration;
	/**
	 * Enforce valid usage of v-else.
	 */
	useVueValidVElse?: UseVueValidVElseConfiguration;
	/**
	 * Enforce valid v-else-if directives.
	 */
	useVueValidVElseIf?: UseVueValidVElseIfConfiguration;
	/**
	 * Enforce valid v-html directives.
	 */
	useVueValidVHtml?: UseVueValidVHtmlConfiguration;
	/**
	 * Enforces valid v-if usage for Vue templates.
	 */
	useVueValidVIf?: UseVueValidVIfConfiguration;
	/**
	 * Enforce valid v-on directives with proper arguments, modifiers, and handlers.
	 */
	useVueValidVOn?: UseVueValidVOnConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Performance {
	/**
	 * Disallow the use of spread (...) syntax on accumulators.
	 */
	noAccumulatingSpread?: NoAccumulatingSpreadConfiguration;
	/**
	 * Disallow await inside loops.
	 */
	noAwaitInLoops?: NoAwaitInLoopsConfiguration;
	/**
	 * Disallow the use of barrel file.
	 */
	noBarrelFile?: NoBarrelFileConfiguration;
	/**
	 * Disallow the use of the delete operator.
	 */
	noDelete?: NoDeleteConfiguration;
	/**
	 * Disallow accessing namespace imports dynamically.
	 */
	noDynamicNamespaceImportAccess?: NoDynamicNamespaceImportAccessConfiguration;
	/**
	 * Prevent usage of \<img> element in a Next.js project.
	 */
	noImgElement?: NoImgElementConfiguration;
	/**
	 * Disallow the use of namespace imports.
	 */
	noNamespaceImport?: NoNamespaceImportConfiguration;
	/**
	 * Avoid re-export all.
	 */
	noReExportAll?: NoReExportAllConfiguration;
	/**
	 * Prevent duplicate polyfills from Polyfill.io.
	 */
	noUnwantedPolyfillio?: NoUnwantedPolyfillioConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Ensure the preconnect attribute is used when using Google Fonts.
	 */
	useGoogleFontPreconnect?: UseGoogleFontPreconnectConfiguration;
	/**
	 * Enforce using Solid's \<For /> component for mapping an array to JSX elements.
	 */
	useSolidForComponent?: UseSolidForComponentConfiguration;
	/**
	 * Require regex literals to be declared at the top level.
	 */
	useTopLevelRegex?: UseTopLevelRegexConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Security {
	/**
	 * Disallow target="_blank" attribute without rel="noopener".
	 */
	noBlankTarget?: NoBlankTargetConfiguration;
	/**
	 * Prevent the usage of dangerous JSX props
	 */
	noDangerouslySetInnerHtml?: NoDangerouslySetInnerHtmlConfiguration;
	/**
	 * Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop.
	 */
	noDangerouslySetInnerHtmlWithChildren?: NoDangerouslySetInnerHtmlWithChildrenConfiguration;
	/**
	 * Disallow the use of global eval().
	 */
	noGlobalEval?: NoGlobalEvalConfiguration;
	/**
	 * Disallow usage of sensitive data such as API keys and tokens.
	 */
	noSecrets?: NoSecretsConfiguration;
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
	noCommonJs?: NoCommonJsConfiguration;
	/**
	 * Disallow default exports.
	 */
	noDefaultExport?: NoDefaultExportConfiguration;
	/**
	 * Disallow a lower specificity selector from coming after a higher specificity selector.
	 */
	noDescendingSpecificity?: NoDescendingSpecificityConfiguration;
	/**
	 * Disallow using a callback in asynchronous tests and hooks.
	 */
	noDoneCallback?: NoDoneCallbackConfiguration;
	/**
	 * Disallow TypeScript enum.
	 */
	noEnum?: NoEnumConfiguration;
	/**
	 * Disallow exporting an imported variable.
	 */
	noExportedImports?: NoExportedImportsConfiguration;
	/**
	 * Prevent usage of \<head> element in a Next.js project.
	 */
	noHeadElement?: NoHeadElementConfiguration;
	/**
	 * Disallow implicit true values on JSX boolean attributes
	 */
	noImplicitBoolean?: NoImplicitBooleanConfiguration;
	/**
	 * Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
	 */
	noInferrableTypes?: NoInferrableTypesConfiguration;
	/**
	 * Reports usage of "magic numbers" — numbers used directly instead of being assigned to named constants.
	 */
	noMagicNumbers?: NoMagicNumbersConfiguration;
	/**
	 * Disallow the use of TypeScript's namespaces.
	 */
	noNamespace?: NoNamespaceConfiguration;
	/**
	 * Disallow negation in the condition of an if statement if it has an else clause.
	 */
	noNegationElse?: NoNegationElseConfiguration;
	/**
	 * Disallow nested ternary expressions.
	 */
	noNestedTernary?: NoNestedTernaryConfiguration;
	/**
	 * Disallow non-null assertions using the ! postfix operator.
	 */
	noNonNullAssertion?: NoNonNullAssertionConfiguration;
	/**
	 * Disallow reassigning function parameters.
	 */
	noParameterAssign?: NoParameterAssignConfiguration;
	/**
	 * Disallow the use of parameter properties in class constructors.
	 */
	noParameterProperties?: NoParameterPropertiesConfiguration;
	/**
	 * Disallow the use of process.env.
	 */
	noProcessEnv?: NoProcessEnvConfiguration;
	/**
	 * This rule allows you to specify global variable names that you don’t want to use in your application.
	 */
	noRestrictedGlobals?: NoRestrictedGlobalsConfiguration;
	/**
	 * Disallow specified modules when loaded by import or require.
	 */
	noRestrictedImports?: NoRestrictedImportsConfiguration;
	/**
	 * Disallow user defined types.
	 */
	noRestrictedTypes?: NoRestrictedTypesConfiguration;
	/**
	 * Disallow the use of constants which its value is the upper-case version of its name.
	 */
	noShoutyConstants?: NoShoutyConstantsConfiguration;
	/**
	 * Enforce the use of String.slice() over String.substr() and String.substring().
	 */
	noSubstr?: NoSubstrConfiguration;
	/**
	 * Disallow template literals if interpolation and special-character handling are not needed
	 */
	noUnusedTemplateLiteral?: NoUnusedTemplateLiteralConfiguration;
	/**
	 * Disallow else block when the if block breaks early.
	 */
	noUselessElse?: NoUselessElseConfiguration;
	/**
	 * Disallow use of @value rule in css modules.
	 */
	noValueAtRule?: NoValueAtRuleConfiguration;
	/**
	 * Disallow the use of yoda expressions.
	 */
	noYodaExpression?: NoYodaExpressionConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Disallow Array constructors.
	 */
	useArrayLiterals?: UseArrayLiteralsConfiguration;
	/**
	 * Enforce the use of as const over literal type and type annotation.
	 */
	useAsConstAssertion?: UseAsConstAssertionConfiguration;
	/**
	 * Use at() instead of integer index access.
	 */
	useAtIndex?: UseAtIndexConfiguration;
	/**
	 * Requires following curly brace conventions.
	 */
	useBlockStatements?: UseBlockStatementsConfiguration;
	/**
	 * Enforce using else if instead of nested if in else clauses.
	 */
	useCollapsedElseIf?: UseCollapsedElseIfConfiguration;
	/**
	 * Enforce using single if instead of nested if clauses.
	 */
	useCollapsedIf?: UseCollapsedIfConfiguration;
	/**
	 * Enforce declaring components only within modules that export React Components exclusively.
	 */
	useComponentExportOnlyModules?: UseComponentExportOnlyModulesConfiguration;
	/**
	 * Require consistently using either T\[] or Array\<T>
	 */
	useConsistentArrayType?: UseConsistentArrayTypeConfiguration;
	/**
	 * Enforce the use of new for all builtins, except String, Number and Boolean.
	 */
	useConsistentBuiltinInstantiation?: UseConsistentBuiltinInstantiationConfiguration;
	/**
	 * This rule enforces consistent use of curly braces inside JSX attributes and JSX children.
	 */
	useConsistentCurlyBraces?: UseConsistentCurlyBracesConfiguration;
	/**
	 * Require consistent accessibility modifiers on class properties and methods.
	 */
	useConsistentMemberAccessibility?: UseConsistentMemberAccessibilityConfiguration;
	/**
	 * Require the consistent declaration of object literals. Defaults to explicit definitions.
	 */
	useConsistentObjectDefinitions?: UseConsistentObjectDefinitionsConfiguration;
	/**
	 * Enforce type definitions to consistently use either interface or type.
	 */
	useConsistentTypeDefinitions?: UseConsistentTypeDefinitionsConfiguration;
	/**
	 * Require const declarations for variables that are only assigned once.
	 */
	useConst?: UseConstConfiguration;
	/**
	 * Enforce default function parameters and optional function parameters to be last.
	 */
	useDefaultParameterLast?: UseDefaultParameterLastConfiguration;
	/**
	 * Require the default clause in switch statements.
	 */
	useDefaultSwitchClause?: UseDefaultSwitchClauseConfiguration;
	/**
	 * Require specifying the reason argument when using @deprecated directive
	 */
	useDeprecatedReason?: UseDeprecatedReasonConfiguration;
	/**
	 * Require that each enum member value be explicitly initialized.
	 */
	useEnumInitializers?: UseEnumInitializersConfiguration;
	/**
	 * Enforce explicitly comparing the length, size, byteLength or byteOffset property of a value.
	 */
	useExplicitLengthCheck?: UseExplicitLengthCheckConfiguration;
	/**
	 * Disallow the use of Math.pow in favor of the ** operator.
	 */
	useExponentiationOperator?: UseExponentiationOperatorConfiguration;
	/**
	 * Promotes the use of export type for types.
	 */
	useExportType?: UseExportTypeConfiguration;
	/**
	 * Require that all exports are declared after all non-export statements.
	 */
	useExportsLast?: UseExportsLastConfiguration;
	/**
	 * Enforce naming conventions for JavaScript and TypeScript filenames.
	 */
	useFilenamingConvention?: UseFilenamingConventionConfiguration;
	/**
	 * Prefer using for...of loops over standard for loops where possible.
	 */
	useForOf?: UseForOfConfiguration;
	/**
	 * This rule enforces the use of \<>...\</> over \<Fragment>...\</Fragment>.
	 */
	useFragmentSyntax?: UseFragmentSyntaxConfiguration;
	/**
	 * Validates that all enum values are capitalized.
	 */
	useGraphqlNamingConvention?: UseGraphqlNamingConventionConfiguration;
	/**
	 * Enforce that getters and setters for the same property are adjacent in class and object definitions.
	 */
	useGroupedAccessorPairs?: UseGroupedAccessorPairsConfiguration;
	/**
	 * Promotes the use of import type for types.
	 */
	useImportType?: UseImportTypeConfiguration;
	/**
	 * Require all enum members to be literal values.
	 */
	useLiteralEnumMembers?: UseLiteralEnumMembersConfiguration;
	/**
	 * Enforce naming conventions for everything across a codebase.
	 */
	useNamingConvention?: UseNamingConventionConfiguration;
	/**
	 * Promotes the usage of node:assert/strict over node:assert.
	 */
	useNodeAssertStrict?: UseNodeAssertStrictConfiguration;
	/**
	 * Enforces using the node: protocol for Node.js builtin modules.
	 */
	useNodejsImportProtocol?: UseNodejsImportProtocolConfiguration;
	/**
	 * Use the Number properties instead of global ones.
	 */
	useNumberNamespace?: UseNumberNamespaceConfiguration;
	/**
	 * Enforce the use of numeric separators in numeric literals.
	 */
	useNumericSeparators?: UseNumericSeparatorsConfiguration;
	/**
	 * Prefer object spread over Object.assign() when constructing new objects.
	 */
	useObjectSpread?: UseObjectSpreadConfiguration;
	/**
	 * Enforce that components are defined as functions and never as classes.
	 */
	useReactFunctionComponents?: UseReactFunctionComponentsConfiguration;
	/**
	 * Enforce marking members as readonly if they are never modified outside the constructor.
	 */
	useReadonlyClassProperties?: UseReadonlyClassPropertiesConfiguration;
	/**
	 * Prevent extra closing tags for components without children.
	 */
	useSelfClosingElements?: UseSelfClosingElementsConfiguration;
	/**
	 * Require assignment operator shorthand where possible.
	 */
	useShorthandAssign?: UseShorthandAssignConfiguration;
	/**
	 * Enforce using function types instead of object type with call signatures.
	 */
	useShorthandFunctionType?: UseShorthandFunctionTypeConfiguration;
	/**
	 * Disallow multiple variable declarations in the same variable statement
	 */
	useSingleVarDeclarator?: UseSingleVarDeclaratorConfiguration;
	/**
	 * Require a description parameter for the Symbol().
	 */
	useSymbolDescription?: UseSymbolDescriptionConfiguration;
	/**
	 * Prefer template literals over string concatenation.
	 */
	useTemplate?: UseTemplateConfiguration;
	/**
	 * Require new when throwing an error.
	 */
	useThrowNewError?: UseThrowNewErrorConfiguration;
	/**
	 * Disallow throwing non-Error values.
	 */
	useThrowOnlyError?: UseThrowOnlyErrorConfiguration;
	/**
	 * Enforce the use of String.trimStart() and String.trimEnd() over String.trimLeft() and String.trimRight().
	 */
	useTrimStartEnd?: UseTrimStartEndConfiguration;
	/**
	 * Disallow overload signatures that can be unified into a single signature.
	 */
	useUnifiedTypeSignatures?: UseUnifiedTypeSignaturesConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Suspicious {
	/**
	 * Disallow the use of alert, confirm, and prompt.
	 */
	noAlert?: NoAlertConfiguration;
	/**
	 * Use standard constants instead of approximated literals.
	 */
	noApproximativeNumericConstant?: NoApproximativeNumericConstantConfiguration;
	/**
	 * Discourage the usage of Array index in keys.
	 */
	noArrayIndexKey?: NoArrayIndexKeyConfiguration;
	/**
	 * Disallow assignments in expressions.
	 */
	noAssignInExpressions?: NoAssignInExpressionsConfiguration;
	/**
	 * Disallows using an async function as a Promise executor.
	 */
	noAsyncPromiseExecutor?: NoAsyncPromiseExecutorConfiguration;
	/**
	 * Prevents the use of the ! pattern in the first position of files.includes in the configuration file.
	 */
	noBiomeFirstException?: NoBiomeFirstExceptionConfiguration;
	/**
	 * Disallow bitwise operators.
	 */
	noBitwiseOperators?: NoBitwiseOperatorsConfiguration;
	/**
	 * Disallow reassigning exceptions in catch clauses.
	 */
	noCatchAssign?: NoCatchAssignConfiguration;
	/**
	 * Disallow reassigning class members.
	 */
	noClassAssign?: NoClassAssignConfiguration;
	/**
	 * Prevent comments from being inserted as text nodes
	 */
	noCommentText?: NoCommentTextConfiguration;
	/**
	 * Disallow comparing against -0
	 */
	noCompareNegZero?: NoCompareNegZeroConfiguration;
	/**
	 * Disallow labeled statements that are not loops.
	 */
	noConfusingLabels?: NoConfusingLabelsConfiguration;
	/**
	 * Disallow void type outside of generic or return types.
	 */
	noConfusingVoidType?: NoConfusingVoidTypeConfiguration;
	/**
	 * Disallow the use of console.
	 */
	noConsole?: NoConsoleConfiguration;
	/**
	 * Disallow TypeScript const enum
	 */
	noConstEnum?: NoConstEnumConfiguration;
	/**
	 * Disallow expressions where the operation doesn't affect the value
	 */
	noConstantBinaryExpressions?: NoConstantBinaryExpressionsConfiguration;
	/**
	 * Prevents from having control characters and some escape sequences that match control characters in regular expression literals.
	 */
	noControlCharactersInRegex?: NoControlCharactersInRegexConfiguration;
	/**
	 * Disallow the use of debugger
	 */
	noDebugger?: NoDebuggerConfiguration;
	/**
	 * Disallow direct assignments to document.cookie.
	 */
	noDocumentCookie?: NoDocumentCookieConfiguration;
	/**
	 * Prevents importing next/document outside of pages/_document.jsx in Next.js projects.
	 */
	noDocumentImportInPage?: NoDocumentImportInPageConfiguration;
	/**
	 * Require the use of === and !==.
	 */
	noDoubleEquals?: NoDoubleEqualsConfiguration;
	/**
	 * Disallow duplicate @import rules.
	 */
	noDuplicateAtImportRules?: NoDuplicateAtImportRulesConfiguration;
	/**
	 * Disallow duplicate case labels.
	 */
	noDuplicateCase?: NoDuplicateCaseConfiguration;
	/**
	 * Disallow duplicate class members.
	 */
	noDuplicateClassMembers?: NoDuplicateClassMembersConfiguration;
	/**
	 * Disallow duplicate custom properties within declaration blocks.
	 */
	noDuplicateCustomProperties?: NoDuplicateCustomPropertiesConfiguration;
	/**
	 * Disallow duplicate conditions in if-else-if chains
	 */
	noDuplicateElseIf?: NoDuplicateElseIfConfiguration;
	/**
	 * No duplicated fields in GraphQL operations.
	 */
	noDuplicateFields?: NoDuplicateFieldsConfiguration;
	/**
	 * Disallow duplicate names within font families.
	 */
	noDuplicateFontNames?: NoDuplicateFontNamesConfiguration;
	/**
	 * Prevents JSX properties to be assigned multiple times.
	 */
	noDuplicateJsxProps?: NoDuplicateJsxPropsConfiguration;
	/**
	 * Disallow two keys with the same name inside objects.
	 */
	noDuplicateObjectKeys?: NoDuplicateObjectKeysConfiguration;
	/**
	 * Disallow duplicate function parameter name.
	 */
	noDuplicateParameters?: NoDuplicateParametersConfiguration;
	/**
	 * Disallow duplicate properties within declaration blocks.
	 */
	noDuplicateProperties?: NoDuplicatePropertiesConfiguration;
	/**
	 * Disallow duplicate selectors within keyframe blocks.
	 */
	noDuplicateSelectorsKeyframeBlock?: NoDuplicateSelectorsKeyframeBlockConfiguration;
	/**
	 * A describe block should not contain duplicate hooks.
	 */
	noDuplicateTestHooks?: NoDuplicateTestHooksConfiguration;
	/**
	 * Disallow CSS empty blocks.
	 */
	noEmptyBlock?: NoEmptyBlockConfiguration;
	/**
	 * Disallow empty block statements and static blocks.
	 */
	noEmptyBlockStatements?: NoEmptyBlockStatementsConfiguration;
	/**
	 * Disallow the declaration of empty interfaces.
	 */
	noEmptyInterface?: NoEmptyInterfaceConfiguration;
	/**
	 * Disallow variables from evolving into any type through reassignments.
	 */
	noEvolvingTypes?: NoEvolvingTypesConfiguration;
	/**
	 * Disallow the any type usage.
	 */
	noExplicitAny?: NoExplicitAnyConfiguration;
	/**
	 * Disallow using export or module.exports in files containing tests
	 */
	noExportsInTest?: NoExportsInTestConfiguration;
	/**
	 * Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files.
	 */
	noExtraNonNullAssertion?: NoExtraNonNullAssertionConfiguration;
	/**
	 * Disallow fallthrough of switch clauses.
	 */
	noFallthroughSwitchClause?: NoFallthroughSwitchClauseConfiguration;
	/**
	 * Disallow focused tests.
	 */
	noFocusedTests?: NoFocusedTestsConfiguration;
	/**
	 * Disallow reassigning function declarations.
	 */
	noFunctionAssign?: NoFunctionAssignConfiguration;
	/**
	 * Disallow assignments to native objects and read-only global variables.
	 */
	noGlobalAssign?: NoGlobalAssignConfiguration;
	/**
	 * Use Number.isFinite instead of global isFinite.
	 */
	noGlobalIsFinite?: NoGlobalIsFiniteConfiguration;
	/**
	 * Use Number.isNaN instead of global isNaN.
	 */
	noGlobalIsNan?: NoGlobalIsNanConfiguration;
	/**
	 * Prevent using the next/head module in pages/_document.js on Next.js projects.
	 */
	noHeadImportInDocument?: NoHeadImportInDocumentConfiguration;
	/**
	 * Disallow use of implicit any type on variable declarations.
	 */
	noImplicitAnyLet?: NoImplicitAnyLetConfiguration;
	/**
	 * Disallow assigning to imported bindings
	 */
	noImportAssign?: NoImportAssignConfiguration;
	/**
	 * Disallow invalid !important within keyframe declarations
	 */
	noImportantInKeyframe?: NoImportantInKeyframeConfiguration;
	/**
	 * Disallows the use of irregular whitespace characters.
	 */
	noIrregularWhitespace?: NoIrregularWhitespaceConfiguration;
	/**
	 * Disallow labels that share a name with a variable
	 */
	noLabelVar?: NoLabelVarConfiguration;
	/**
	 * Disallow characters made with multiple code points in character class syntax.
	 */
	noMisleadingCharacterClass?: NoMisleadingCharacterClassConfiguration;
	/**
	 * Enforce proper usage of new and constructor.
	 */
	noMisleadingInstantiator?: NoMisleadingInstantiatorConfiguration;
	/**
	 * Checks that the assertion function, for example expect, is placed inside an it() function call.
	 */
	noMisplacedAssertion?: NoMisplacedAssertionConfiguration;
	/**
	 * Disallow shorthand assign when variable appears on both sides.
	 */
	noMisrefactoredShorthandAssign?: NoMisrefactoredShorthandAssignConfiguration;
	/**
	 * Disallow non-null assertions after optional chaining expressions.
	 */
	noNonNullAssertedOptionalChain?: NoNonNullAssertedOptionalChainConfiguration;
	/**
	 * Disallow octal escape sequences in string literals
	 */
	noOctalEscape?: NoOctalEscapeConfiguration;
	/**
	 * Disallow direct use of Object.prototype builtins.
	 */
	noPrototypeBuiltins?: NoPrototypeBuiltinsConfiguration;
	/**
	 * Disallow the use if quickfix.biome inside editor settings file.
	 */
	noQuickfixBiome?: NoQuickfixBiomeConfiguration;
	/**
	 * Prevents React-specific JSX properties from being used.
	 */
	noReactSpecificProps?: NoReactSpecificPropsConfiguration;
	/**
	 * Disallow variable, function, class, and type redeclarations in the same scope.
	 */
	noRedeclare?: NoRedeclareConfiguration;
	/**
	 * Prevents from having redundant "use strict".
	 */
	noRedundantUseStrict?: NoRedundantUseStrictConfiguration;
	/**
	 * Disallow comparisons where both sides are exactly the same.
	 */
	noSelfCompare?: NoSelfCompareConfiguration;
	/**
	 * Disallow identifiers from shadowing restricted names.
	 */
	noShadowRestrictedNames?: NoShadowRestrictedNamesConfiguration;
	/**
	 * Disallow shorthand properties that override related longhand properties.
	 */
	noShorthandPropertyOverrides?: NoShorthandPropertyOverridesConfiguration;
	/**
	 * Disallow disabled tests.
	 */
	noSkippedTests?: NoSkippedTestsConfiguration;
	/**
	 * Prevents the use of sparse arrays (arrays with holes).
	 */
	noSparseArray?: NoSparseArrayConfiguration;
	/**
	 * It detects possible "wrong" semicolons inside JSX elements.
	 */
	noSuspiciousSemicolonInJsx?: NoSuspiciousSemicolonInJsxConfiguration;
	/**
	 * Disallow template literal placeholder syntax in regular strings.
	 */
	noTemplateCurlyInString?: NoTemplateCurlyInStringConfiguration;
	/**
	 * Disallow then property.
	 */
	noThenProperty?: NoThenPropertyConfiguration;
	/**
	 * Prevents the use of the TypeScript directive @ts-ignore.
	 */
	noTsIgnore?: NoTsIgnoreConfiguration;
	/**
	 * Disallow let or var variables that are read but never assigned.
	 */
	noUnassignedVariables?: NoUnassignedVariablesConfiguration;
	/**
	 * Disallow unknown at-rules.
	 */
	noUnknownAtRules?: NoUnknownAtRulesConfiguration;
	/**
	 * Disallow unsafe declaration merging between interfaces and classes.
	 */
	noUnsafeDeclarationMerging?: NoUnsafeDeclarationMergingConfiguration;
	/**
	 * Disallow using unsafe negation.
	 */
	noUnsafeNegation?: NoUnsafeNegationConfiguration;
	/**
	 * Disallow unnecessary escapes in string literals.
	 */
	noUselessEscapeInString?: NoUselessEscapeInStringConfiguration;
	/**
	 * Disallow useless backreferences in regular expression literals that always match an empty string.
	 */
	noUselessRegexBackrefs?: NoUselessRegexBackrefsConfiguration;
	/**
	 * Disallow the use of var
	 */
	noVar?: NoVarConfiguration;
	/**
	 * Disallow with statements in non-strict contexts.
	 */
	noWith?: NoWithConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Disallow the use of overload signatures that are not next to each other.
	 */
	useAdjacentOverloadSignatures?: UseAdjacentOverloadSignaturesConfiguration;
	/**
	 * Ensure async functions utilize await.
	 */
	useAwait?: UseAwaitConfiguration;
	/**
	 * Promotes the correct usage for ignoring folders in the configuration file.
	 */
	useBiomeIgnoreFolder?: UseBiomeIgnoreFolderConfiguration;
	/**
	 * Enforce default clauses in switch statements to be last
	 */
	useDefaultSwitchClauseLast?: UseDefaultSwitchClauseLastConfiguration;
	/**
	 * Enforce passing a message value when creating a built-in error.
	 */
	useErrorMessage?: UseErrorMessageConfiguration;
	/**
	 * Enforce get methods to always return a value.
	 */
	useGetterReturn?: UseGetterReturnConfiguration;
	/**
	 * Enforces the use of a recommended display strategy with Google Fonts.
	 */
	useGoogleFontDisplay?: UseGoogleFontDisplayConfiguration;
	/**
	 * Require for-in loops to include an if statement.
	 */
	useGuardForIn?: UseGuardForInConfiguration;
	/**
	 * Use Array.isArray() instead of instanceof Array.
	 */
	useIsArray?: UseIsArrayConfiguration;
	/**
	 * Enforce consistent return values in iterable callbacks.
	 */
	useIterableCallbackReturn?: UseIterableCallbackReturnConfiguration;
	/**
	 * Require using the namespace keyword over the module keyword to declare TypeScript namespaces.
	 */
	useNamespaceKeyword?: UseNamespaceKeywordConfiguration;
	/**
	 * Enforce using the digits argument with Number#toFixed().
	 */
	useNumberToFixedDigitsArgument?: UseNumberToFixedDigitsArgumentConfiguration;
	/**
	 * Use static Response methods instead of new Response() constructor when possible.
	 */
	useStaticResponseMethods?: UseStaticResponseMethodsConfiguration;
	/**
	 * Enforce the use of the directive "use strict" in script files.
	 */
	useStrictMode?: UseStrictModeConfiguration;
}
export type Glob = string;
export type RuleAssistPlainConfiguration = "off" | "on";
export interface RuleAssistWithOrganizeImportsOptions {
	level: RuleAssistPlainConfiguration;
	options: OrganizeImportsOptions;
}
export interface RuleAssistWithUseSortedAttributesOptions {
	level: RuleAssistPlainConfiguration;
	options: UseSortedAttributesOptions;
}
export interface RuleAssistWithUseSortedKeysOptions {
	level: RuleAssistPlainConfiguration;
	options: UseSortedKeysOptions;
}
export interface RuleAssistWithUseSortedPropertiesOptions {
	level: RuleAssistPlainConfiguration;
	options: UseSortedPropertiesOptions;
}
export type NoAccessKeyConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAccessKeyOptions;
export type NoAriaHiddenOnFocusableConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAriaHiddenOnFocusableOptions;
export type NoAriaUnsupportedElementsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAriaUnsupportedElementsOptions;
export type NoAutofocusConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAutofocusOptions;
export type NoDistractingElementsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDistractingElementsOptions;
export type NoHeaderScopeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoHeaderScopeOptions;
export type NoInteractiveElementToNoninteractiveRoleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInteractiveElementToNoninteractiveRoleOptions;
export type NoLabelWithoutControlConfiguration =
	| RulePlainConfiguration
	| RuleWithNoLabelWithoutControlOptions;
export type NoNoninteractiveElementInteractionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNoninteractiveElementInteractionsOptions;
export type NoNoninteractiveElementToInteractiveRoleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNoninteractiveElementToInteractiveRoleOptions;
export type NoNoninteractiveTabindexConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNoninteractiveTabindexOptions;
export type NoPositiveTabindexConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPositiveTabindexOptions;
export type NoRedundantAltConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRedundantAltOptions;
export type NoRedundantRolesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRedundantRolesOptions;
export type NoStaticElementInteractionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoStaticElementInteractionsOptions;
export type NoSvgWithoutTitleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSvgWithoutTitleOptions;
export type UseAltTextConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAltTextOptions;
export type UseAnchorContentConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAnchorContentOptions;
export type UseAriaActivedescendantWithTabindexConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAriaActivedescendantWithTabindexOptions;
export type UseAriaPropsForRoleConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAriaPropsForRoleOptions;
export type UseAriaPropsSupportedByRoleConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAriaPropsSupportedByRoleOptions;
export type UseButtonTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseButtonTypeOptions;
export type UseFocusableInteractiveConfiguration =
	| RulePlainConfiguration
	| RuleWithUseFocusableInteractiveOptions;
export type UseGenericFontNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGenericFontNamesOptions;
export type UseHeadingContentConfiguration =
	| RulePlainConfiguration
	| RuleWithUseHeadingContentOptions;
export type UseHtmlLangConfiguration =
	| RulePlainConfiguration
	| RuleWithUseHtmlLangOptions;
export type UseIframeTitleConfiguration =
	| RulePlainConfiguration
	| RuleWithUseIframeTitleOptions;
export type UseKeyWithClickEventsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseKeyWithClickEventsOptions;
export type UseKeyWithMouseEventsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseKeyWithMouseEventsOptions;
export type UseMediaCaptionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseMediaCaptionOptions;
export type UseSemanticElementsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSemanticElementsOptions;
export type UseValidAnchorConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidAnchorOptions;
export type UseValidAriaPropsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidAriaPropsOptions;
export type UseValidAriaRoleConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidAriaRoleOptions;
export type UseValidAriaValuesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidAriaValuesOptions;
export type UseValidAutocompleteConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidAutocompleteOptions;
export type UseValidLangConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidLangOptions;
export type NoAdjacentSpacesInRegexConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAdjacentSpacesInRegexOptions;
export type NoArgumentsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoArgumentsOptions;
export type NoBannedTypesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoBannedTypesOptions;
export type NoCommaOperatorConfiguration =
	| RulePlainConfiguration
	| RuleWithNoCommaOperatorOptions;
export type NoEmptyTypeParametersConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptyTypeParametersOptions;
export type NoExcessiveCognitiveComplexityConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExcessiveCognitiveComplexityOptions;
export type NoExcessiveLinesPerFunctionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExcessiveLinesPerFunctionOptions;
export type NoExcessiveNestedTestSuitesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExcessiveNestedTestSuitesOptions;
export type NoExtraBooleanCastConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExtraBooleanCastOptions;
export type NoFlatMapIdentityConfiguration =
	| RulePlainConfiguration
	| RuleWithNoFlatMapIdentityOptions;
export type NoForEachConfiguration =
	| RulePlainConfiguration
	| RuleWithNoForEachOptions;
export type NoImplicitCoercionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImplicitCoercionsOptions;
export type NoImportantStylesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImportantStylesOptions;
export type NoStaticOnlyClassConfiguration =
	| RulePlainConfiguration
	| RuleWithNoStaticOnlyClassOptions;
export type NoThisInStaticConfiguration =
	| RulePlainConfiguration
	| RuleWithNoThisInStaticOptions;
export type NoUselessCatchConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessCatchOptions;
export type NoUselessConstructorConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessConstructorOptions;
export type NoUselessContinueConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessContinueOptions;
export type NoUselessEmptyExportConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessEmptyExportOptions;
export type NoUselessEscapeInRegexConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessEscapeInRegexOptions;
export type NoUselessFragmentsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessFragmentsOptions;
export type NoUselessLabelConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessLabelOptions;
export type NoUselessLoneBlockStatementsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessLoneBlockStatementsOptions;
export type NoUselessRenameConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessRenameOptions;
export type NoUselessStringConcatConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessStringConcatOptions;
export type NoUselessStringRawConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessStringRawOptions;
export type NoUselessSwitchCaseConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessSwitchCaseOptions;
export type NoUselessTernaryConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessTernaryOptions;
export type NoUselessThisAliasConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessThisAliasOptions;
export type NoUselessTypeConstraintConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessTypeConstraintOptions;
export type NoUselessUndefinedInitializationConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessUndefinedInitializationOptions;
export type NoVoidConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVoidOptions;
export type UseArrowFunctionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseArrowFunctionOptions;
export type UseDateNowConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDateNowOptions;
export type UseFlatMapConfiguration =
	| RulePlainConfiguration
	| RuleWithUseFlatMapOptions;
export type UseIndexOfConfiguration =
	| RulePlainConfiguration
	| RuleWithUseIndexOfOptions;
export type UseLiteralKeysConfiguration =
	| RulePlainConfiguration
	| RuleWithUseLiteralKeysOptions;
export type UseNumericLiteralsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNumericLiteralsOptions;
export type UseOptionalChainConfiguration =
	| RulePlainConfiguration
	| RuleWithUseOptionalChainOptions;
export type UseRegexLiteralsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseRegexLiteralsOptions;
export type UseSimpleNumberKeysConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSimpleNumberKeysOptions;
export type UseSimplifiedLogicExpressionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSimplifiedLogicExpressionOptions;
export type UseWhileConfiguration =
	| RulePlainConfiguration
	| RuleWithUseWhileOptions;
export type NoChildrenPropConfiguration =
	| RulePlainConfiguration
	| RuleWithNoChildrenPropOptions;
export type NoConstAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConstAssignOptions;
export type NoConstantConditionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConstantConditionOptions;
export type NoConstantMathMinMaxClampConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConstantMathMinMaxClampOptions;
export type NoConstructorReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConstructorReturnOptions;
export type NoEmptyCharacterClassInRegexConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptyCharacterClassInRegexOptions;
export type NoEmptyPatternConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptyPatternOptions;
export type NoGlobalDirnameFilenameConfiguration =
	| RulePlainConfiguration
	| RuleWithNoGlobalDirnameFilenameOptions;
export type NoGlobalObjectCallsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoGlobalObjectCallsOptions;
export type NoInnerDeclarationsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInnerDeclarationsOptions;
export type NoInvalidBuiltinInstantiationConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInvalidBuiltinInstantiationOptions;
export type NoInvalidConstructorSuperConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInvalidConstructorSuperOptions;
export type NoInvalidDirectionInLinearGradientConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInvalidDirectionInLinearGradientOptions;
export type NoInvalidGridAreasConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInvalidGridAreasOptions;
export type NoInvalidPositionAtImportRuleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInvalidPositionAtImportRuleOptions;
export type NoInvalidUseBeforeDeclarationConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInvalidUseBeforeDeclarationOptions;
export type NoMissingVarFunctionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMissingVarFunctionOptions;
export type NoNestedComponentDefinitionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNestedComponentDefinitionsOptions;
export type NoNodejsModulesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNodejsModulesOptions;
export type NoNonoctalDecimalEscapeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNonoctalDecimalEscapeOptions;
export type NoPrecisionLossConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPrecisionLossOptions;
export type NoPrivateImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPrivateImportsOptions;
export type NoProcessGlobalConfiguration =
	| RulePlainConfiguration
	| RuleWithNoProcessGlobalOptions;
export type NoQwikUseVisibleTaskConfiguration =
	| RulePlainConfiguration
	| RuleWithNoQwikUseVisibleTaskOptions;
export type NoReactPropAssignmentsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoReactPropAssignmentsOptions;
export type NoRenderReturnValueConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRenderReturnValueOptions;
export type NoRestrictedElementsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRestrictedElementsOptions;
export type NoSelfAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSelfAssignOptions;
export type NoSetterReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSetterReturnOptions;
export type NoSolidDestructuredPropsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSolidDestructuredPropsOptions;
export type NoStringCaseMismatchConfiguration =
	| RulePlainConfiguration
	| RuleWithNoStringCaseMismatchOptions;
export type NoSwitchDeclarationsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSwitchDeclarationsOptions;
export type NoUndeclaredDependenciesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUndeclaredDependenciesOptions;
export type NoUndeclaredVariablesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUndeclaredVariablesOptions;
export type NoUnknownFunctionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownFunctionOptions;
export type NoUnknownMediaFeatureNameConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownMediaFeatureNameOptions;
export type NoUnknownPropertyConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownPropertyOptions;
export type NoUnknownPseudoClassConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownPseudoClassOptions;
export type NoUnknownPseudoElementConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownPseudoElementOptions;
export type NoUnknownTypeSelectorConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownTypeSelectorOptions;
export type NoUnknownUnitConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownUnitOptions;
export type NoUnmatchableAnbSelectorConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnmatchableAnbSelectorOptions;
export type NoUnreachableConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnreachableOptions;
export type NoUnreachableSuperConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnreachableSuperOptions;
export type NoUnsafeFinallyConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnsafeFinallyOptions;
export type NoUnsafeOptionalChainingConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnsafeOptionalChainingOptions;
export type NoUnusedFunctionParametersConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedFunctionParametersOptions;
export type NoUnusedImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedImportsOptions;
export type NoUnusedLabelsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedLabelsOptions;
export type NoUnusedPrivateClassMembersConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedPrivateClassMembersOptions;
export type NoUnusedVariablesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedVariablesOptions;
export type NoVoidElementsWithChildrenConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVoidElementsWithChildrenOptions;
export type NoVoidTypeReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVoidTypeReturnOptions;
export type UseExhaustiveDependenciesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExhaustiveDependenciesOptions;
export type UseGraphqlNamedOperationsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGraphqlNamedOperationsOptions;
export type UseHookAtTopLevelConfiguration =
	| RulePlainConfiguration
	| RuleWithUseHookAtTopLevelOptions;
export type UseImageSizeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseImageSizeOptions;
export type UseImportExtensionsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseImportExtensionsOptions;
export type UseIsNanConfiguration =
	| RulePlainConfiguration
	| RuleWithUseIsNanOptions;
export type UseJsonImportAttributesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseJsonImportAttributesOptions;
export type UseJsxKeyInIterableConfiguration =
	| RulePlainConfiguration
	| RuleWithUseJsxKeyInIterableOptions;
export type UseParseIntRadixConfiguration =
	| RulePlainConfiguration
	| RuleWithUseParseIntRadixOptions;
export type UseQwikClasslistConfiguration =
	| RulePlainConfiguration
	| RuleWithUseQwikClasslistOptions;
export type UseSingleJsDocAsteriskConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSingleJsDocAsteriskOptions;
export type UseUniqueElementIdsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseUniqueElementIdsOptions;
export type UseValidForDirectionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidForDirectionOptions;
export type UseValidTypeofConfiguration =
	| RulePlainConfiguration
	| RuleWithUseValidTypeofOptions;
export type UseYieldConfiguration =
	| RulePlainConfiguration
	| RuleWithUseYieldOptions;
export type NoContinueConfiguration =
	| RulePlainConfiguration
	| RuleWithNoContinueOptions;
export type NoDeprecatedImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDeprecatedImportsOptions;
export type NoDuplicateDependenciesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateDependenciesOptions;
export type NoEmptySourceConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptySourceOptions;
export type NoFloatingPromisesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoFloatingPromisesOptions;
export type NoForInConfiguration =
	| RulePlainConfiguration
	| RuleWithNoForInOptions;
export type NoImportCyclesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImportCyclesOptions;
export type NoIncrementDecrementConfiguration =
	| RulePlainConfiguration
	| RuleWithNoIncrementDecrementOptions;
export type NoJsxLiteralsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoJsxLiteralsOptions;
export type NoMisusedPromisesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMisusedPromisesOptions;
export type NoNextAsyncClientComponentConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNextAsyncClientComponentOptions;
export type NoParametersOnlyUsedInRecursionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoParametersOnlyUsedInRecursionOptions;
export type NoReactForwardRefConfiguration =
	| RulePlainConfiguration
	| RuleWithNoReactForwardRefOptions;
export type NoShadowConfiguration =
	| RulePlainConfiguration
	| RuleWithNoShadowOptions;
export type NoUnknownAttributeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownAttributeOptions;
export type NoUnnecessaryConditionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnnecessaryConditionsOptions;
export type NoUnresolvedImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnresolvedImportsOptions;
export type NoUnusedExpressionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedExpressionsOptions;
export type NoUselessCatchBindingConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessCatchBindingOptions;
export type NoUselessUndefinedConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessUndefinedOptions;
export type NoVueDataObjectDeclarationConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueDataObjectDeclarationOptions;
export type NoVueDuplicateKeysConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueDuplicateKeysOptions;
export type NoVueReservedKeysConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueReservedKeysOptions;
export type NoVueReservedPropsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueReservedPropsOptions;
export type UseArraySortCompareConfiguration =
	| RulePlainConfiguration
	| RuleWithUseArraySortCompareOptions;
export type UseConsistentArrowReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentArrowReturnOptions;
export type UseConsistentGraphqlDescriptionsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentGraphqlDescriptionsOptions;
export type UseDeprecatedDateConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDeprecatedDateOptions;
export type UseExhaustiveSwitchCasesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExhaustiveSwitchCasesOptions;
export type UseExplicitTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExplicitTypeOptions;
export type UseMaxParamsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseMaxParamsOptions;
export type UseQwikMethodUsageConfiguration =
	| RulePlainConfiguration
	| RuleWithUseQwikMethodUsageOptions;
export type UseQwikValidLexicalScopeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseQwikValidLexicalScopeOptions;
export type UseSortedClassesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSortedClassesOptions;
export type UseUniqueGraphqlOperationNameConfiguration =
	| RulePlainConfiguration
	| RuleWithUseUniqueGraphqlOperationNameOptions;
export type UseVueDefineMacrosOrderConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueDefineMacrosOrderOptions;
export type UseVueMultiWordComponentNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueMultiWordComponentNamesOptions;
export type UseVueValidVBindConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVBindOptions;
export type UseVueValidVElseConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVElseOptions;
export type UseVueValidVElseIfConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVElseIfOptions;
export type UseVueValidVHtmlConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVHtmlOptions;
export type UseVueValidVIfConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVIfOptions;
export type UseVueValidVOnConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVOnOptions;
export type NoAccumulatingSpreadConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAccumulatingSpreadOptions;
export type NoAwaitInLoopsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAwaitInLoopsOptions;
export type NoBarrelFileConfiguration =
	| RulePlainConfiguration
	| RuleWithNoBarrelFileOptions;
export type NoDeleteConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDeleteOptions;
export type NoDynamicNamespaceImportAccessConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDynamicNamespaceImportAccessOptions;
export type NoImgElementConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImgElementOptions;
export type NoNamespaceImportConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNamespaceImportOptions;
export type NoReExportAllConfiguration =
	| RulePlainConfiguration
	| RuleWithNoReExportAllOptions;
export type NoUnwantedPolyfillioConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnwantedPolyfillioOptions;
export type UseGoogleFontPreconnectConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGoogleFontPreconnectOptions;
export type UseSolidForComponentConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSolidForComponentOptions;
export type UseTopLevelRegexConfiguration =
	| RulePlainConfiguration
	| RuleWithUseTopLevelRegexOptions;
export type NoBlankTargetConfiguration =
	| RulePlainConfiguration
	| RuleWithNoBlankTargetOptions;
export type NoDangerouslySetInnerHtmlConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDangerouslySetInnerHtmlOptions;
export type NoDangerouslySetInnerHtmlWithChildrenConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDangerouslySetInnerHtmlWithChildrenOptions;
export type NoGlobalEvalConfiguration =
	| RulePlainConfiguration
	| RuleWithNoGlobalEvalOptions;
export type NoSecretsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSecretsOptions;
export type NoCommonJsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoCommonJsOptions;
export type NoDefaultExportConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDefaultExportOptions;
export type NoDescendingSpecificityConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDescendingSpecificityOptions;
export type NoDoneCallbackConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDoneCallbackOptions;
export type NoEnumConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEnumOptions;
export type NoExportedImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExportedImportsOptions;
export type NoHeadElementConfiguration =
	| RulePlainConfiguration
	| RuleWithNoHeadElementOptions;
export type NoImplicitBooleanConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImplicitBooleanOptions;
export type NoInferrableTypesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoInferrableTypesOptions;
export type NoMagicNumbersConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMagicNumbersOptions;
export type NoNamespaceConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNamespaceOptions;
export type NoNegationElseConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNegationElseOptions;
export type NoNestedTernaryConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNestedTernaryOptions;
export type NoNonNullAssertionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNonNullAssertionOptions;
export type NoParameterAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoParameterAssignOptions;
export type NoParameterPropertiesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoParameterPropertiesOptions;
export type NoProcessEnvConfiguration =
	| RulePlainConfiguration
	| RuleWithNoProcessEnvOptions;
export type NoRestrictedGlobalsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRestrictedGlobalsOptions;
export type NoRestrictedImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRestrictedImportsOptions;
export type NoRestrictedTypesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRestrictedTypesOptions;
export type NoShoutyConstantsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoShoutyConstantsOptions;
export type NoSubstrConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSubstrOptions;
export type NoUnusedTemplateLiteralConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedTemplateLiteralOptions;
export type NoUselessElseConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessElseOptions;
export type NoValueAtRuleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoValueAtRuleOptions;
export type NoYodaExpressionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoYodaExpressionOptions;
export type UseArrayLiteralsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseArrayLiteralsOptions;
export type UseAsConstAssertionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAsConstAssertionOptions;
export type UseAtIndexConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAtIndexOptions;
export type UseBlockStatementsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseBlockStatementsOptions;
export type UseCollapsedElseIfConfiguration =
	| RulePlainConfiguration
	| RuleWithUseCollapsedElseIfOptions;
export type UseCollapsedIfConfiguration =
	| RulePlainConfiguration
	| RuleWithUseCollapsedIfOptions;
export type UseComponentExportOnlyModulesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseComponentExportOnlyModulesOptions;
export type UseConsistentArrayTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentArrayTypeOptions;
export type UseConsistentBuiltinInstantiationConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentBuiltinInstantiationOptions;
export type UseConsistentCurlyBracesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentCurlyBracesOptions;
export type UseConsistentMemberAccessibilityConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentMemberAccessibilityOptions;
export type UseConsistentObjectDefinitionsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentObjectDefinitionsOptions;
export type UseConsistentTypeDefinitionsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentTypeDefinitionsOptions;
export type UseConstConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConstOptions;
export type UseDefaultParameterLastConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDefaultParameterLastOptions;
export type UseDefaultSwitchClauseConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDefaultSwitchClauseOptions;
export type UseDeprecatedReasonConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDeprecatedReasonOptions;
export type UseEnumInitializersConfiguration =
	| RulePlainConfiguration
	| RuleWithUseEnumInitializersOptions;
export type UseExplicitLengthCheckConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExplicitLengthCheckOptions;
export type UseExponentiationOperatorConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExponentiationOperatorOptions;
export type UseExportTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExportTypeOptions;
export type UseExportsLastConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExportsLastOptions;
export type UseFilenamingConventionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseFilenamingConventionOptions;
export type UseForOfConfiguration =
	| RulePlainConfiguration
	| RuleWithUseForOfOptions;
export type UseFragmentSyntaxConfiguration =
	| RulePlainConfiguration
	| RuleWithUseFragmentSyntaxOptions;
export type UseGraphqlNamingConventionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGraphqlNamingConventionOptions;
export type UseGroupedAccessorPairsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGroupedAccessorPairsOptions;
export type UseImportTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseImportTypeOptions;
export type UseLiteralEnumMembersConfiguration =
	| RulePlainConfiguration
	| RuleWithUseLiteralEnumMembersOptions;
export type UseNamingConventionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNamingConventionOptions;
export type UseNodeAssertStrictConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNodeAssertStrictOptions;
export type UseNodejsImportProtocolConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNodejsImportProtocolOptions;
export type UseNumberNamespaceConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNumberNamespaceOptions;
export type UseNumericSeparatorsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNumericSeparatorsOptions;
export type UseObjectSpreadConfiguration =
	| RulePlainConfiguration
	| RuleWithUseObjectSpreadOptions;
export type UseReactFunctionComponentsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseReactFunctionComponentsOptions;
export type UseReadonlyClassPropertiesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseReadonlyClassPropertiesOptions;
export type UseSelfClosingElementsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSelfClosingElementsOptions;
export type UseShorthandAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithUseShorthandAssignOptions;
export type UseShorthandFunctionTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseShorthandFunctionTypeOptions;
export type UseSingleVarDeclaratorConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSingleVarDeclaratorOptions;
export type UseSymbolDescriptionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSymbolDescriptionOptions;
export type UseTemplateConfiguration =
	| RulePlainConfiguration
	| RuleWithUseTemplateOptions;
export type UseThrowNewErrorConfiguration =
	| RulePlainConfiguration
	| RuleWithUseThrowNewErrorOptions;
export type UseThrowOnlyErrorConfiguration =
	| RulePlainConfiguration
	| RuleWithUseThrowOnlyErrorOptions;
export type UseTrimStartEndConfiguration =
	| RulePlainConfiguration
	| RuleWithUseTrimStartEndOptions;
export type UseUnifiedTypeSignaturesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseUnifiedTypeSignaturesOptions;
export type NoAlertConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAlertOptions;
export type NoApproximativeNumericConstantConfiguration =
	| RulePlainConfiguration
	| RuleWithNoApproximativeNumericConstantOptions;
export type NoArrayIndexKeyConfiguration =
	| RulePlainConfiguration
	| RuleWithNoArrayIndexKeyOptions;
export type NoAssignInExpressionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAssignInExpressionsOptions;
export type NoAsyncPromiseExecutorConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAsyncPromiseExecutorOptions;
export type NoBiomeFirstExceptionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoBiomeFirstExceptionOptions;
export type NoBitwiseOperatorsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoBitwiseOperatorsOptions;
export type NoCatchAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoCatchAssignOptions;
export type NoClassAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoClassAssignOptions;
export type NoCommentTextConfiguration =
	| RulePlainConfiguration
	| RuleWithNoCommentTextOptions;
export type NoCompareNegZeroConfiguration =
	| RulePlainConfiguration
	| RuleWithNoCompareNegZeroOptions;
export type NoConfusingLabelsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConfusingLabelsOptions;
export type NoConfusingVoidTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConfusingVoidTypeOptions;
export type NoConsoleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConsoleOptions;
export type NoConstEnumConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConstEnumOptions;
export type NoConstantBinaryExpressionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConstantBinaryExpressionsOptions;
export type NoControlCharactersInRegexConfiguration =
	| RulePlainConfiguration
	| RuleWithNoControlCharactersInRegexOptions;
export type NoDebuggerConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDebuggerOptions;
export type NoDocumentCookieConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDocumentCookieOptions;
export type NoDocumentImportInPageConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDocumentImportInPageOptions;
export type NoDoubleEqualsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDoubleEqualsOptions;
export type NoDuplicateAtImportRulesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateAtImportRulesOptions;
export type NoDuplicateCaseConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateCaseOptions;
export type NoDuplicateClassMembersConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateClassMembersOptions;
export type NoDuplicateCustomPropertiesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateCustomPropertiesOptions;
export type NoDuplicateElseIfConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateElseIfOptions;
export type NoDuplicateFieldsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateFieldsOptions;
export type NoDuplicateFontNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateFontNamesOptions;
export type NoDuplicateJsxPropsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateJsxPropsOptions;
export type NoDuplicateObjectKeysConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateObjectKeysOptions;
export type NoDuplicateParametersConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateParametersOptions;
export type NoDuplicatePropertiesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicatePropertiesOptions;
export type NoDuplicateSelectorsKeyframeBlockConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateSelectorsKeyframeBlockOptions;
export type NoDuplicateTestHooksConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateTestHooksOptions;
export type NoEmptyBlockConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptyBlockOptions;
export type NoEmptyBlockStatementsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptyBlockStatementsOptions;
export type NoEmptyInterfaceConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptyInterfaceOptions;
export type NoEvolvingTypesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEvolvingTypesOptions;
export type NoExplicitAnyConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExplicitAnyOptions;
export type NoExportsInTestConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExportsInTestOptions;
export type NoExtraNonNullAssertionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExtraNonNullAssertionOptions;
export type NoFallthroughSwitchClauseConfiguration =
	| RulePlainConfiguration
	| RuleWithNoFallthroughSwitchClauseOptions;
export type NoFocusedTestsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoFocusedTestsOptions;
export type NoFunctionAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoFunctionAssignOptions;
export type NoGlobalAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoGlobalAssignOptions;
export type NoGlobalIsFiniteConfiguration =
	| RulePlainConfiguration
	| RuleWithNoGlobalIsFiniteOptions;
export type NoGlobalIsNanConfiguration =
	| RulePlainConfiguration
	| RuleWithNoGlobalIsNanOptions;
export type NoHeadImportInDocumentConfiguration =
	| RulePlainConfiguration
	| RuleWithNoHeadImportInDocumentOptions;
export type NoImplicitAnyLetConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImplicitAnyLetOptions;
export type NoImportAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImportAssignOptions;
export type NoImportantInKeyframeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImportantInKeyframeOptions;
export type NoIrregularWhitespaceConfiguration =
	| RulePlainConfiguration
	| RuleWithNoIrregularWhitespaceOptions;
export type NoLabelVarConfiguration =
	| RulePlainConfiguration
	| RuleWithNoLabelVarOptions;
export type NoMisleadingCharacterClassConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMisleadingCharacterClassOptions;
export type NoMisleadingInstantiatorConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMisleadingInstantiatorOptions;
export type NoMisplacedAssertionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMisplacedAssertionOptions;
export type NoMisrefactoredShorthandAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMisrefactoredShorthandAssignOptions;
export type NoNonNullAssertedOptionalChainConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNonNullAssertedOptionalChainOptions;
export type NoOctalEscapeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoOctalEscapeOptions;
export type NoPrototypeBuiltinsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPrototypeBuiltinsOptions;
export type NoQuickfixBiomeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoQuickfixBiomeOptions;
export type NoReactSpecificPropsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoReactSpecificPropsOptions;
export type NoRedeclareConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRedeclareOptions;
export type NoRedundantUseStrictConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRedundantUseStrictOptions;
export type NoSelfCompareConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSelfCompareOptions;
export type NoShadowRestrictedNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoShadowRestrictedNamesOptions;
export type NoShorthandPropertyOverridesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoShorthandPropertyOverridesOptions;
export type NoSkippedTestsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSkippedTestsOptions;
export type NoSparseArrayConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSparseArrayOptions;
export type NoSuspiciousSemicolonInJsxConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSuspiciousSemicolonInJsxOptions;
export type NoTemplateCurlyInStringConfiguration =
	| RulePlainConfiguration
	| RuleWithNoTemplateCurlyInStringOptions;
export type NoThenPropertyConfiguration =
	| RulePlainConfiguration
	| RuleWithNoThenPropertyOptions;
export type NoTsIgnoreConfiguration =
	| RulePlainConfiguration
	| RuleWithNoTsIgnoreOptions;
export type NoUnassignedVariablesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnassignedVariablesOptions;
export type NoUnknownAtRulesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownAtRulesOptions;
export type NoUnsafeDeclarationMergingConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnsafeDeclarationMergingOptions;
export type NoUnsafeNegationConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnsafeNegationOptions;
export type NoUselessEscapeInStringConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessEscapeInStringOptions;
export type NoUselessRegexBackrefsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessRegexBackrefsOptions;
export type NoVarConfiguration = RulePlainConfiguration | RuleWithNoVarOptions;
export type NoWithConfiguration =
	| RulePlainConfiguration
	| RuleWithNoWithOptions;
export type UseAdjacentOverloadSignaturesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAdjacentOverloadSignaturesOptions;
export type UseAwaitConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAwaitOptions;
export type UseBiomeIgnoreFolderConfiguration =
	| RulePlainConfiguration
	| RuleWithUseBiomeIgnoreFolderOptions;
export type UseDefaultSwitchClauseLastConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDefaultSwitchClauseLastOptions;
export type UseErrorMessageConfiguration =
	| RulePlainConfiguration
	| RuleWithUseErrorMessageOptions;
export type UseGetterReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGetterReturnOptions;
export type UseGoogleFontDisplayConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGoogleFontDisplayOptions;
export type UseGuardForInConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGuardForInOptions;
export type UseIsArrayConfiguration =
	| RulePlainConfiguration
	| RuleWithUseIsArrayOptions;
export type UseIterableCallbackReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithUseIterableCallbackReturnOptions;
export type UseNamespaceKeywordConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNamespaceKeywordOptions;
export type UseNumberToFixedDigitsArgumentConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNumberToFixedDigitsArgumentOptions;
export type UseStaticResponseMethodsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseStaticResponseMethodsOptions;
export type UseStrictModeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseStrictModeOptions;
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
export type UseSortedPropertiesOptions = {};
export type RulePlainConfiguration = "off" | "on" | "info" | "warn" | "error";
export interface RuleWithNoAccessKeyOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoAccessKeyOptions;
}
export interface RuleWithNoAriaHiddenOnFocusableOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoAriaHiddenOnFocusableOptions;
}
export interface RuleWithNoAriaUnsupportedElementsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoAriaUnsupportedElementsOptions;
}
export interface RuleWithNoAutofocusOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoAutofocusOptions;
}
export interface RuleWithNoDistractingElementsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoDistractingElementsOptions;
}
export interface RuleWithNoHeaderScopeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoHeaderScopeOptions;
}
export interface RuleWithNoInteractiveElementToNoninteractiveRoleOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoInteractiveElementToNoninteractiveRoleOptions;
}
export interface RuleWithNoLabelWithoutControlOptions {
	level: RulePlainConfiguration;
	options?: NoLabelWithoutControlOptions;
}
export interface RuleWithNoNoninteractiveElementInteractionsOptions {
	level: RulePlainConfiguration;
	options?: NoNoninteractiveElementInteractionsOptions;
}
export interface RuleWithNoNoninteractiveElementToInteractiveRoleOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoNoninteractiveElementToInteractiveRoleOptions;
}
export interface RuleWithNoNoninteractiveTabindexOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoNoninteractiveTabindexOptions;
}
export interface RuleWithNoPositiveTabindexOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoPositiveTabindexOptions;
}
export interface RuleWithNoRedundantAltOptions {
	level: RulePlainConfiguration;
	options?: NoRedundantAltOptions;
}
export interface RuleWithNoRedundantRolesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoRedundantRolesOptions;
}
export interface RuleWithNoStaticElementInteractionsOptions {
	level: RulePlainConfiguration;
	options?: NoStaticElementInteractionsOptions;
}
export interface RuleWithNoSvgWithoutTitleOptions {
	level: RulePlainConfiguration;
	options?: NoSvgWithoutTitleOptions;
}
export interface RuleWithUseAltTextOptions {
	level: RulePlainConfiguration;
	options?: UseAltTextOptions;
}
export interface RuleWithUseAnchorContentOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseAnchorContentOptions;
}
export interface RuleWithUseAriaActivedescendantWithTabindexOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseAriaActivedescendantWithTabindexOptions;
}
export interface RuleWithUseAriaPropsForRoleOptions {
	level: RulePlainConfiguration;
	options?: UseAriaPropsForRoleOptions;
}
export interface RuleWithUseAriaPropsSupportedByRoleOptions {
	level: RulePlainConfiguration;
	options?: UseAriaPropsSupportedByRoleOptions;
}
export interface RuleWithUseButtonTypeOptions {
	level: RulePlainConfiguration;
	options?: UseButtonTypeOptions;
}
export interface RuleWithUseFocusableInteractiveOptions {
	level: RulePlainConfiguration;
	options?: UseFocusableInteractiveOptions;
}
export interface RuleWithUseGenericFontNamesOptions {
	level: RulePlainConfiguration;
	options?: UseGenericFontNamesOptions;
}
export interface RuleWithUseHeadingContentOptions {
	level: RulePlainConfiguration;
	options?: UseHeadingContentOptions;
}
export interface RuleWithUseHtmlLangOptions {
	level: RulePlainConfiguration;
	options?: UseHtmlLangOptions;
}
export interface RuleWithUseIframeTitleOptions {
	level: RulePlainConfiguration;
	options?: UseIframeTitleOptions;
}
export interface RuleWithUseKeyWithClickEventsOptions {
	level: RulePlainConfiguration;
	options?: UseKeyWithClickEventsOptions;
}
export interface RuleWithUseKeyWithMouseEventsOptions {
	level: RulePlainConfiguration;
	options?: UseKeyWithMouseEventsOptions;
}
export interface RuleWithUseMediaCaptionOptions {
	level: RulePlainConfiguration;
	options?: UseMediaCaptionOptions;
}
export interface RuleWithUseSemanticElementsOptions {
	level: RulePlainConfiguration;
	options?: UseSemanticElementsOptions;
}
export interface RuleWithUseValidAnchorOptions {
	level: RulePlainConfiguration;
	options?: UseValidAnchorOptions;
}
export interface RuleWithUseValidAriaPropsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseValidAriaPropsOptions;
}
export interface RuleWithUseValidAriaRoleOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseValidAriaRoleOptions;
}
export interface RuleWithUseValidAriaValuesOptions {
	level: RulePlainConfiguration;
	options?: UseValidAriaValuesOptions;
}
export interface RuleWithUseValidAutocompleteOptions {
	level: RulePlainConfiguration;
	options?: UseValidAutocompleteOptions;
}
export interface RuleWithUseValidLangOptions {
	level: RulePlainConfiguration;
	options?: UseValidLangOptions;
}
export interface RuleWithNoAdjacentSpacesInRegexOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoAdjacentSpacesInRegexOptions;
}
export interface RuleWithNoArgumentsOptions {
	level: RulePlainConfiguration;
	options?: NoArgumentsOptions;
}
export interface RuleWithNoBannedTypesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoBannedTypesOptions;
}
export interface RuleWithNoCommaOperatorOptions {
	level: RulePlainConfiguration;
	options?: NoCommaOperatorOptions;
}
export interface RuleWithNoEmptyTypeParametersOptions {
	level: RulePlainConfiguration;
	options?: NoEmptyTypeParametersOptions;
}
export interface RuleWithNoExcessiveCognitiveComplexityOptions {
	level: RulePlainConfiguration;
	options?: NoExcessiveCognitiveComplexityOptions;
}
export interface RuleWithNoExcessiveLinesPerFunctionOptions {
	level: RulePlainConfiguration;
	options?: NoExcessiveLinesPerFunctionOptions;
}
export interface RuleWithNoExcessiveNestedTestSuitesOptions {
	level: RulePlainConfiguration;
	options?: NoExcessiveNestedTestSuitesOptions;
}
export interface RuleWithNoExtraBooleanCastOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoExtraBooleanCastOptions;
}
export interface RuleWithNoFlatMapIdentityOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoFlatMapIdentityOptions;
}
export interface RuleWithNoForEachOptions {
	level: RulePlainConfiguration;
	options?: NoForEachOptions;
}
export interface RuleWithNoImplicitCoercionsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoImplicitCoercionsOptions;
}
export interface RuleWithNoImportantStylesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoImportantStylesOptions;
}
export interface RuleWithNoStaticOnlyClassOptions {
	level: RulePlainConfiguration;
	options?: NoStaticOnlyClassOptions;
}
export interface RuleWithNoThisInStaticOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoThisInStaticOptions;
}
export interface RuleWithNoUselessCatchOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessCatchOptions;
}
export interface RuleWithNoUselessConstructorOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessConstructorOptions;
}
export interface RuleWithNoUselessContinueOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessContinueOptions;
}
export interface RuleWithNoUselessEmptyExportOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessEmptyExportOptions;
}
export interface RuleWithNoUselessEscapeInRegexOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessEscapeInRegexOptions;
}
export interface RuleWithNoUselessFragmentsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessFragmentsOptions;
}
export interface RuleWithNoUselessLabelOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessLabelOptions;
}
export interface RuleWithNoUselessLoneBlockStatementsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessLoneBlockStatementsOptions;
}
export interface RuleWithNoUselessRenameOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessRenameOptions;
}
export interface RuleWithNoUselessStringConcatOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessStringConcatOptions;
}
export interface RuleWithNoUselessStringRawOptions {
	level: RulePlainConfiguration;
	options?: NoUselessStringRawOptions;
}
export interface RuleWithNoUselessSwitchCaseOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessSwitchCaseOptions;
}
export interface RuleWithNoUselessTernaryOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessTernaryOptions;
}
export interface RuleWithNoUselessThisAliasOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessThisAliasOptions;
}
export interface RuleWithNoUselessTypeConstraintOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessTypeConstraintOptions;
}
export interface RuleWithNoUselessUndefinedInitializationOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessUndefinedInitializationOptions;
}
export interface RuleWithNoVoidOptions {
	level: RulePlainConfiguration;
	options?: NoVoidOptions;
}
export interface RuleWithUseArrowFunctionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseArrowFunctionOptions;
}
export interface RuleWithUseDateNowOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseDateNowOptions;
}
export interface RuleWithUseFlatMapOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseFlatMapOptions;
}
export interface RuleWithUseIndexOfOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseIndexOfOptions;
}
export interface RuleWithUseLiteralKeysOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseLiteralKeysOptions;
}
export interface RuleWithUseNumericLiteralsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNumericLiteralsOptions;
}
export interface RuleWithUseOptionalChainOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseOptionalChainOptions;
}
export interface RuleWithUseRegexLiteralsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseRegexLiteralsOptions;
}
export interface RuleWithUseSimpleNumberKeysOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSimpleNumberKeysOptions;
}
export interface RuleWithUseSimplifiedLogicExpressionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSimplifiedLogicExpressionOptions;
}
export interface RuleWithUseWhileOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseWhileOptions;
}
export interface RuleWithNoChildrenPropOptions {
	level: RulePlainConfiguration;
	options?: NoChildrenPropOptions;
}
export interface RuleWithNoConstAssignOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoConstAssignOptions;
}
export interface RuleWithNoConstantConditionOptions {
	level: RulePlainConfiguration;
	options?: NoConstantConditionOptions;
}
export interface RuleWithNoConstantMathMinMaxClampOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoConstantMathMinMaxClampOptions;
}
export interface RuleWithNoConstructorReturnOptions {
	level: RulePlainConfiguration;
	options?: NoConstructorReturnOptions;
}
export interface RuleWithNoEmptyCharacterClassInRegexOptions {
	level: RulePlainConfiguration;
	options?: NoEmptyCharacterClassInRegexOptions;
}
export interface RuleWithNoEmptyPatternOptions {
	level: RulePlainConfiguration;
	options?: NoEmptyPatternOptions;
}
export interface RuleWithNoGlobalDirnameFilenameOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoGlobalDirnameFilenameOptions;
}
export interface RuleWithNoGlobalObjectCallsOptions {
	level: RulePlainConfiguration;
	options?: NoGlobalObjectCallsOptions;
}
export interface RuleWithNoInnerDeclarationsOptions {
	level: RulePlainConfiguration;
	options?: NoInnerDeclarationsOptions;
}
export interface RuleWithNoInvalidBuiltinInstantiationOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoInvalidBuiltinInstantiationOptions;
}
export interface RuleWithNoInvalidConstructorSuperOptions {
	level: RulePlainConfiguration;
	options?: NoInvalidConstructorSuperOptions;
}
export interface RuleWithNoInvalidDirectionInLinearGradientOptions {
	level: RulePlainConfiguration;
	options?: NoInvalidDirectionInLinearGradientOptions;
}
export interface RuleWithNoInvalidGridAreasOptions {
	level: RulePlainConfiguration;
	options?: NoInvalidGridAreasOptions;
}
export interface RuleWithNoInvalidPositionAtImportRuleOptions {
	level: RulePlainConfiguration;
	options?: NoInvalidPositionAtImportRuleOptions;
}
export interface RuleWithNoInvalidUseBeforeDeclarationOptions {
	level: RulePlainConfiguration;
	options?: NoInvalidUseBeforeDeclarationOptions;
}
export interface RuleWithNoMissingVarFunctionOptions {
	level: RulePlainConfiguration;
	options?: NoMissingVarFunctionOptions;
}
export interface RuleWithNoNestedComponentDefinitionsOptions {
	level: RulePlainConfiguration;
	options?: NoNestedComponentDefinitionsOptions;
}
export interface RuleWithNoNodejsModulesOptions {
	level: RulePlainConfiguration;
	options?: NoNodejsModulesOptions;
}
export interface RuleWithNoNonoctalDecimalEscapeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoNonoctalDecimalEscapeOptions;
}
export interface RuleWithNoPrecisionLossOptions {
	level: RulePlainConfiguration;
	options?: NoPrecisionLossOptions;
}
export interface RuleWithNoPrivateImportsOptions {
	level: RulePlainConfiguration;
	options?: NoPrivateImportsOptions;
}
export interface RuleWithNoProcessGlobalOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoProcessGlobalOptions;
}
export interface RuleWithNoQwikUseVisibleTaskOptions {
	level: RulePlainConfiguration;
	options?: NoQwikUseVisibleTaskOptions;
}
export interface RuleWithNoReactPropAssignmentsOptions {
	level: RulePlainConfiguration;
	options?: NoReactPropAssignmentsOptions;
}
export interface RuleWithNoRenderReturnValueOptions {
	level: RulePlainConfiguration;
	options?: NoRenderReturnValueOptions;
}
export interface RuleWithNoRestrictedElementsOptions {
	level: RulePlainConfiguration;
	options?: NoRestrictedElementsOptions;
}
export interface RuleWithNoSelfAssignOptions {
	level: RulePlainConfiguration;
	options?: NoSelfAssignOptions;
}
export interface RuleWithNoSetterReturnOptions {
	level: RulePlainConfiguration;
	options?: NoSetterReturnOptions;
}
export interface RuleWithNoSolidDestructuredPropsOptions {
	level: RulePlainConfiguration;
	options?: NoSolidDestructuredPropsOptions;
}
export interface RuleWithNoStringCaseMismatchOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoStringCaseMismatchOptions;
}
export interface RuleWithNoSwitchDeclarationsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoSwitchDeclarationsOptions;
}
export interface RuleWithNoUndeclaredDependenciesOptions {
	level: RulePlainConfiguration;
	options?: NoUndeclaredDependenciesOptions;
}
export interface RuleWithNoUndeclaredVariablesOptions {
	level: RulePlainConfiguration;
	options?: NoUndeclaredVariablesOptions;
}
export interface RuleWithNoUnknownFunctionOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownFunctionOptions;
}
export interface RuleWithNoUnknownMediaFeatureNameOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownMediaFeatureNameOptions;
}
export interface RuleWithNoUnknownPropertyOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownPropertyOptions;
}
export interface RuleWithNoUnknownPseudoClassOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownPseudoClassOptions;
}
export interface RuleWithNoUnknownPseudoElementOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownPseudoElementOptions;
}
export interface RuleWithNoUnknownTypeSelectorOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownTypeSelectorOptions;
}
export interface RuleWithNoUnknownUnitOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownUnitOptions;
}
export interface RuleWithNoUnmatchableAnbSelectorOptions {
	level: RulePlainConfiguration;
	options?: NoUnmatchableAnbSelectorOptions;
}
export interface RuleWithNoUnreachableOptions {
	level: RulePlainConfiguration;
	options?: NoUnreachableOptions;
}
export interface RuleWithNoUnreachableSuperOptions {
	level: RulePlainConfiguration;
	options?: NoUnreachableSuperOptions;
}
export interface RuleWithNoUnsafeFinallyOptions {
	level: RulePlainConfiguration;
	options?: NoUnsafeFinallyOptions;
}
export interface RuleWithNoUnsafeOptionalChainingOptions {
	level: RulePlainConfiguration;
	options?: NoUnsafeOptionalChainingOptions;
}
export interface RuleWithNoUnusedFunctionParametersOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUnusedFunctionParametersOptions;
}
export interface RuleWithNoUnusedImportsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUnusedImportsOptions;
}
export interface RuleWithNoUnusedLabelsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUnusedLabelsOptions;
}
export interface RuleWithNoUnusedPrivateClassMembersOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUnusedPrivateClassMembersOptions;
}
export interface RuleWithNoUnusedVariablesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUnusedVariablesOptions;
}
export interface RuleWithNoVoidElementsWithChildrenOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoVoidElementsWithChildrenOptions;
}
export interface RuleWithNoVoidTypeReturnOptions {
	level: RulePlainConfiguration;
	options?: NoVoidTypeReturnOptions;
}
export interface RuleWithUseExhaustiveDependenciesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseExhaustiveDependenciesOptions;
}
export interface RuleWithUseGraphqlNamedOperationsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseGraphqlNamedOperationsOptions;
}
export interface RuleWithUseHookAtTopLevelOptions {
	level: RulePlainConfiguration;
	options?: UseHookAtTopLevelOptions;
}
export interface RuleWithUseImageSizeOptions {
	level: RulePlainConfiguration;
	options?: UseImageSizeOptions;
}
export interface RuleWithUseImportExtensionsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseImportExtensionsOptions;
}
export interface RuleWithUseIsNanOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseIsNanOptions;
}
export interface RuleWithUseJsonImportAttributesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseJsonImportAttributesOptions;
}
export interface RuleWithUseJsxKeyInIterableOptions {
	level: RulePlainConfiguration;
	options?: UseJsxKeyInIterableOptions;
}
export interface RuleWithUseParseIntRadixOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseParseIntRadixOptions;
}
export interface RuleWithUseQwikClasslistOptions {
	level: RulePlainConfiguration;
	options?: UseQwikClasslistOptions;
}
export interface RuleWithUseSingleJsDocAsteriskOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSingleJsDocAsteriskOptions;
}
export interface RuleWithUseUniqueElementIdsOptions {
	level: RulePlainConfiguration;
	options?: UseUniqueElementIdsOptions;
}
export interface RuleWithUseValidForDirectionOptions {
	level: RulePlainConfiguration;
	options?: UseValidForDirectionOptions;
}
export interface RuleWithUseValidTypeofOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseValidTypeofOptions;
}
export interface RuleWithUseYieldOptions {
	level: RulePlainConfiguration;
	options?: UseYieldOptions;
}
export interface RuleWithNoContinueOptions {
	level: RulePlainConfiguration;
	options?: NoContinueOptions;
}
export interface RuleWithNoDeprecatedImportsOptions {
	level: RulePlainConfiguration;
	options?: NoDeprecatedImportsOptions;
}
export interface RuleWithNoDuplicateDependenciesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateDependenciesOptions;
}
export interface RuleWithNoEmptySourceOptions {
	level: RulePlainConfiguration;
	options?: NoEmptySourceOptions;
}
export interface RuleWithNoFloatingPromisesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoFloatingPromisesOptions;
}
export interface RuleWithNoForInOptions {
	level: RulePlainConfiguration;
	options?: NoForInOptions;
}
export interface RuleWithNoImportCyclesOptions {
	level: RulePlainConfiguration;
	options?: NoImportCyclesOptions;
}
export interface RuleWithNoIncrementDecrementOptions {
	level: RulePlainConfiguration;
	options?: NoIncrementDecrementOptions;
}
export interface RuleWithNoJsxLiteralsOptions {
	level: RulePlainConfiguration;
	options?: NoJsxLiteralsOptions;
}
export interface RuleWithNoMisusedPromisesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoMisusedPromisesOptions;
}
export interface RuleWithNoNextAsyncClientComponentOptions {
	level: RulePlainConfiguration;
	options?: NoNextAsyncClientComponentOptions;
}
export interface RuleWithNoParametersOnlyUsedInRecursionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoParametersOnlyUsedInRecursionOptions;
}
export interface RuleWithNoReactForwardRefOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoReactForwardRefOptions;
}
export interface RuleWithNoShadowOptions {
	level: RulePlainConfiguration;
	options?: NoShadowOptions;
}
export interface RuleWithNoUnknownAttributeOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownAttributeOptions;
}
export interface RuleWithNoUnnecessaryConditionsOptions {
	level: RulePlainConfiguration;
	options?: NoUnnecessaryConditionsOptions;
}
export interface RuleWithNoUnresolvedImportsOptions {
	level: RulePlainConfiguration;
	options?: NoUnresolvedImportsOptions;
}
export interface RuleWithNoUnusedExpressionsOptions {
	level: RulePlainConfiguration;
	options?: NoUnusedExpressionsOptions;
}
export interface RuleWithNoUselessCatchBindingOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessCatchBindingOptions;
}
export interface RuleWithNoUselessUndefinedOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessUndefinedOptions;
}
export interface RuleWithNoVueDataObjectDeclarationOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoVueDataObjectDeclarationOptions;
}
export interface RuleWithNoVueDuplicateKeysOptions {
	level: RulePlainConfiguration;
	options?: NoVueDuplicateKeysOptions;
}
export interface RuleWithNoVueReservedKeysOptions {
	level: RulePlainConfiguration;
	options?: NoVueReservedKeysOptions;
}
export interface RuleWithNoVueReservedPropsOptions {
	level: RulePlainConfiguration;
	options?: NoVueReservedPropsOptions;
}
export interface RuleWithUseArraySortCompareOptions {
	level: RulePlainConfiguration;
	options?: UseArraySortCompareOptions;
}
export interface RuleWithUseConsistentArrowReturnOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConsistentArrowReturnOptions;
}
export interface RuleWithUseConsistentGraphqlDescriptionsOptions {
	level: RulePlainConfiguration;
	options?: UseConsistentGraphqlDescriptionsOptions;
}
export interface RuleWithUseDeprecatedDateOptions {
	level: RulePlainConfiguration;
	options?: UseDeprecatedDateOptions;
}
export interface RuleWithUseExhaustiveSwitchCasesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseExhaustiveSwitchCasesOptions;
}
export interface RuleWithUseExplicitTypeOptions {
	level: RulePlainConfiguration;
	options?: UseExplicitTypeOptions;
}
export interface RuleWithUseMaxParamsOptions {
	level: RulePlainConfiguration;
	options?: UseMaxParamsOptions;
}
export interface RuleWithUseQwikMethodUsageOptions {
	level: RulePlainConfiguration;
	options?: UseQwikMethodUsageOptions;
}
export interface RuleWithUseQwikValidLexicalScopeOptions {
	level: RulePlainConfiguration;
	options?: UseQwikValidLexicalScopeOptions;
}
export interface RuleWithUseSortedClassesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSortedClassesOptions;
}
export interface RuleWithUseUniqueGraphqlOperationNameOptions {
	level: RulePlainConfiguration;
	options?: UseUniqueGraphqlOperationNameOptions;
}
export interface RuleWithUseVueDefineMacrosOrderOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueDefineMacrosOrderOptions;
}
export interface RuleWithUseVueMultiWordComponentNamesOptions {
	level: RulePlainConfiguration;
	options?: UseVueMultiWordComponentNamesOptions;
}
export interface RuleWithUseVueValidVBindOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVBindOptions;
}
export interface RuleWithUseVueValidVElseOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVElseOptions;
}
export interface RuleWithUseVueValidVElseIfOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVElseIfOptions;
}
export interface RuleWithUseVueValidVHtmlOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVHtmlOptions;
}
export interface RuleWithUseVueValidVIfOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVIfOptions;
}
export interface RuleWithUseVueValidVOnOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVOnOptions;
}
export interface RuleWithNoAccumulatingSpreadOptions {
	level: RulePlainConfiguration;
	options?: NoAccumulatingSpreadOptions;
}
export interface RuleWithNoAwaitInLoopsOptions {
	level: RulePlainConfiguration;
	options?: NoAwaitInLoopsOptions;
}
export interface RuleWithNoBarrelFileOptions {
	level: RulePlainConfiguration;
	options?: NoBarrelFileOptions;
}
export interface RuleWithNoDeleteOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoDeleteOptions;
}
export interface RuleWithNoDynamicNamespaceImportAccessOptions {
	level: RulePlainConfiguration;
	options?: NoDynamicNamespaceImportAccessOptions;
}
export interface RuleWithNoImgElementOptions {
	level: RulePlainConfiguration;
	options?: NoImgElementOptions;
}
export interface RuleWithNoNamespaceImportOptions {
	level: RulePlainConfiguration;
	options?: NoNamespaceImportOptions;
}
export interface RuleWithNoReExportAllOptions {
	level: RulePlainConfiguration;
	options?: NoReExportAllOptions;
}
export interface RuleWithNoUnwantedPolyfillioOptions {
	level: RulePlainConfiguration;
	options?: NoUnwantedPolyfillioOptions;
}
export interface RuleWithUseGoogleFontPreconnectOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseGoogleFontPreconnectOptions;
}
export interface RuleWithUseSolidForComponentOptions {
	level: RulePlainConfiguration;
	options?: UseSolidForComponentOptions;
}
export interface RuleWithUseTopLevelRegexOptions {
	level: RulePlainConfiguration;
	options?: UseTopLevelRegexOptions;
}
export interface RuleWithNoBlankTargetOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoBlankTargetOptions;
}
export interface RuleWithNoDangerouslySetInnerHtmlOptions {
	level: RulePlainConfiguration;
	options?: NoDangerouslySetInnerHtmlOptions;
}
export interface RuleWithNoDangerouslySetInnerHtmlWithChildrenOptions {
	level: RulePlainConfiguration;
	options?: NoDangerouslySetInnerHtmlWithChildrenOptions;
}
export interface RuleWithNoGlobalEvalOptions {
	level: RulePlainConfiguration;
	options?: NoGlobalEvalOptions;
}
export interface RuleWithNoSecretsOptions {
	level: RulePlainConfiguration;
	options?: NoSecretsOptions;
}
export interface RuleWithNoCommonJsOptions {
	level: RulePlainConfiguration;
	options?: NoCommonJsOptions;
}
export interface RuleWithNoDefaultExportOptions {
	level: RulePlainConfiguration;
	options?: NoDefaultExportOptions;
}
export interface RuleWithNoDescendingSpecificityOptions {
	level: RulePlainConfiguration;
	options?: NoDescendingSpecificityOptions;
}
export interface RuleWithNoDoneCallbackOptions {
	level: RulePlainConfiguration;
	options?: NoDoneCallbackOptions;
}
export interface RuleWithNoEnumOptions {
	level: RulePlainConfiguration;
	options?: NoEnumOptions;
}
export interface RuleWithNoExportedImportsOptions {
	level: RulePlainConfiguration;
	options?: NoExportedImportsOptions;
}
export interface RuleWithNoHeadElementOptions {
	level: RulePlainConfiguration;
	options?: NoHeadElementOptions;
}
export interface RuleWithNoImplicitBooleanOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoImplicitBooleanOptions;
}
export interface RuleWithNoInferrableTypesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoInferrableTypesOptions;
}
export interface RuleWithNoMagicNumbersOptions {
	level: RulePlainConfiguration;
	options?: NoMagicNumbersOptions;
}
export interface RuleWithNoNamespaceOptions {
	level: RulePlainConfiguration;
	options?: NoNamespaceOptions;
}
export interface RuleWithNoNegationElseOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoNegationElseOptions;
}
export interface RuleWithNoNestedTernaryOptions {
	level: RulePlainConfiguration;
	options?: NoNestedTernaryOptions;
}
export interface RuleWithNoNonNullAssertionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoNonNullAssertionOptions;
}
export interface RuleWithNoParameterAssignOptions {
	level: RulePlainConfiguration;
	options?: NoParameterAssignOptions;
}
export interface RuleWithNoParameterPropertiesOptions {
	level: RulePlainConfiguration;
	options?: NoParameterPropertiesOptions;
}
export interface RuleWithNoProcessEnvOptions {
	level: RulePlainConfiguration;
	options?: NoProcessEnvOptions;
}
export interface RuleWithNoRestrictedGlobalsOptions {
	level: RulePlainConfiguration;
	options?: NoRestrictedGlobalsOptions;
}
export interface RuleWithNoRestrictedImportsOptions {
	level: RulePlainConfiguration;
	options?: NoRestrictedImportsOptions;
}
export interface RuleWithNoRestrictedTypesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoRestrictedTypesOptions;
}
export interface RuleWithNoShoutyConstantsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoShoutyConstantsOptions;
}
export interface RuleWithNoSubstrOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoSubstrOptions;
}
export interface RuleWithNoUnusedTemplateLiteralOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUnusedTemplateLiteralOptions;
}
export interface RuleWithNoUselessElseOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessElseOptions;
}
export interface RuleWithNoValueAtRuleOptions {
	level: RulePlainConfiguration;
	options?: NoValueAtRuleOptions;
}
export interface RuleWithNoYodaExpressionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoYodaExpressionOptions;
}
export interface RuleWithUseArrayLiteralsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseArrayLiteralsOptions;
}
export interface RuleWithUseAsConstAssertionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseAsConstAssertionOptions;
}
export interface RuleWithUseAtIndexOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseAtIndexOptions;
}
export interface RuleWithUseBlockStatementsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseBlockStatementsOptions;
}
export interface RuleWithUseCollapsedElseIfOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseCollapsedElseIfOptions;
}
export interface RuleWithUseCollapsedIfOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseCollapsedIfOptions;
}
export interface RuleWithUseComponentExportOnlyModulesOptions {
	level: RulePlainConfiguration;
	options?: UseComponentExportOnlyModulesOptions;
}
export interface RuleWithUseConsistentArrayTypeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConsistentArrayTypeOptions;
}
export interface RuleWithUseConsistentBuiltinInstantiationOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConsistentBuiltinInstantiationOptions;
}
export interface RuleWithUseConsistentCurlyBracesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConsistentCurlyBracesOptions;
}
export interface RuleWithUseConsistentMemberAccessibilityOptions {
	level: RulePlainConfiguration;
	options?: UseConsistentMemberAccessibilityOptions;
}
export interface RuleWithUseConsistentObjectDefinitionsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConsistentObjectDefinitionsOptions;
}
export interface RuleWithUseConsistentTypeDefinitionsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConsistentTypeDefinitionsOptions;
}
export interface RuleWithUseConstOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConstOptions;
}
export interface RuleWithUseDefaultParameterLastOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseDefaultParameterLastOptions;
}
export interface RuleWithUseDefaultSwitchClauseOptions {
	level: RulePlainConfiguration;
	options?: UseDefaultSwitchClauseOptions;
}
export interface RuleWithUseDeprecatedReasonOptions {
	level: RulePlainConfiguration;
	options?: UseDeprecatedReasonOptions;
}
export interface RuleWithUseEnumInitializersOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseEnumInitializersOptions;
}
export interface RuleWithUseExplicitLengthCheckOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseExplicitLengthCheckOptions;
}
export interface RuleWithUseExponentiationOperatorOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseExponentiationOperatorOptions;
}
export interface RuleWithUseExportTypeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseExportTypeOptions;
}
export interface RuleWithUseExportsLastOptions {
	level: RulePlainConfiguration;
	options?: UseExportsLastOptions;
}
export interface RuleWithUseFilenamingConventionOptions {
	level: RulePlainConfiguration;
	options?: UseFilenamingConventionOptions;
}
export interface RuleWithUseForOfOptions {
	level: RulePlainConfiguration;
	options?: UseForOfOptions;
}
export interface RuleWithUseFragmentSyntaxOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseFragmentSyntaxOptions;
}
export interface RuleWithUseGraphqlNamingConventionOptions {
	level: RulePlainConfiguration;
	options?: UseGraphqlNamingConventionOptions;
}
export interface RuleWithUseGroupedAccessorPairsOptions {
	level: RulePlainConfiguration;
	options?: UseGroupedAccessorPairsOptions;
}
export interface RuleWithUseImportTypeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseImportTypeOptions;
}
export interface RuleWithUseLiteralEnumMembersOptions {
	level: RulePlainConfiguration;
	options?: UseLiteralEnumMembersOptions;
}
export interface RuleWithUseNamingConventionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNamingConventionOptions;
}
export interface RuleWithUseNodeAssertStrictOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNodeAssertStrictOptions;
}
export interface RuleWithUseNodejsImportProtocolOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNodejsImportProtocolOptions;
}
export interface RuleWithUseNumberNamespaceOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNumberNamespaceOptions;
}
export interface RuleWithUseNumericSeparatorsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNumericSeparatorsOptions;
}
export interface RuleWithUseObjectSpreadOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseObjectSpreadOptions;
}
export interface RuleWithUseReactFunctionComponentsOptions {
	level: RulePlainConfiguration;
	options?: UseReactFunctionComponentsOptions;
}
export interface RuleWithUseReadonlyClassPropertiesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseReadonlyClassPropertiesOptions;
}
export interface RuleWithUseSelfClosingElementsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSelfClosingElementsOptions;
}
export interface RuleWithUseShorthandAssignOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseShorthandAssignOptions;
}
export interface RuleWithUseShorthandFunctionTypeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseShorthandFunctionTypeOptions;
}
export interface RuleWithUseSingleVarDeclaratorOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSingleVarDeclaratorOptions;
}
export interface RuleWithUseSymbolDescriptionOptions {
	level: RulePlainConfiguration;
	options?: UseSymbolDescriptionOptions;
}
export interface RuleWithUseTemplateOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseTemplateOptions;
}
export interface RuleWithUseThrowNewErrorOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseThrowNewErrorOptions;
}
export interface RuleWithUseThrowOnlyErrorOptions {
	level: RulePlainConfiguration;
	options?: UseThrowOnlyErrorOptions;
}
export interface RuleWithUseTrimStartEndOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseTrimStartEndOptions;
}
export interface RuleWithUseUnifiedTypeSignaturesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseUnifiedTypeSignaturesOptions;
}
export interface RuleWithNoAlertOptions {
	level: RulePlainConfiguration;
	options?: NoAlertOptions;
}
export interface RuleWithNoApproximativeNumericConstantOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoApproximativeNumericConstantOptions;
}
export interface RuleWithNoArrayIndexKeyOptions {
	level: RulePlainConfiguration;
	options?: NoArrayIndexKeyOptions;
}
export interface RuleWithNoAssignInExpressionsOptions {
	level: RulePlainConfiguration;
	options?: NoAssignInExpressionsOptions;
}
export interface RuleWithNoAsyncPromiseExecutorOptions {
	level: RulePlainConfiguration;
	options?: NoAsyncPromiseExecutorOptions;
}
export interface RuleWithNoBiomeFirstExceptionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoBiomeFirstExceptionOptions;
}
export interface RuleWithNoBitwiseOperatorsOptions {
	level: RulePlainConfiguration;
	options?: NoBitwiseOperatorsOptions;
}
export interface RuleWithNoCatchAssignOptions {
	level: RulePlainConfiguration;
	options?: NoCatchAssignOptions;
}
export interface RuleWithNoClassAssignOptions {
	level: RulePlainConfiguration;
	options?: NoClassAssignOptions;
}
export interface RuleWithNoCommentTextOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoCommentTextOptions;
}
export interface RuleWithNoCompareNegZeroOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoCompareNegZeroOptions;
}
export interface RuleWithNoConfusingLabelsOptions {
	level: RulePlainConfiguration;
	options?: NoConfusingLabelsOptions;
}
export interface RuleWithNoConfusingVoidTypeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoConfusingVoidTypeOptions;
}
export interface RuleWithNoConsoleOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoConsoleOptions;
}
export interface RuleWithNoConstEnumOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoConstEnumOptions;
}
export interface RuleWithNoConstantBinaryExpressionsOptions {
	level: RulePlainConfiguration;
	options?: NoConstantBinaryExpressionsOptions;
}
export interface RuleWithNoControlCharactersInRegexOptions {
	level: RulePlainConfiguration;
	options?: NoControlCharactersInRegexOptions;
}
export interface RuleWithNoDebuggerOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoDebuggerOptions;
}
export interface RuleWithNoDocumentCookieOptions {
	level: RulePlainConfiguration;
	options?: NoDocumentCookieOptions;
}
export interface RuleWithNoDocumentImportInPageOptions {
	level: RulePlainConfiguration;
	options?: NoDocumentImportInPageOptions;
}
export interface RuleWithNoDoubleEqualsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoDoubleEqualsOptions;
}
export interface RuleWithNoDuplicateAtImportRulesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateAtImportRulesOptions;
}
export interface RuleWithNoDuplicateCaseOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateCaseOptions;
}
export interface RuleWithNoDuplicateClassMembersOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateClassMembersOptions;
}
export interface RuleWithNoDuplicateCustomPropertiesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateCustomPropertiesOptions;
}
export interface RuleWithNoDuplicateElseIfOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateElseIfOptions;
}
export interface RuleWithNoDuplicateFieldsOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateFieldsOptions;
}
export interface RuleWithNoDuplicateFontNamesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateFontNamesOptions;
}
export interface RuleWithNoDuplicateJsxPropsOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateJsxPropsOptions;
}
export interface RuleWithNoDuplicateObjectKeysOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateObjectKeysOptions;
}
export interface RuleWithNoDuplicateParametersOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateParametersOptions;
}
export interface RuleWithNoDuplicatePropertiesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicatePropertiesOptions;
}
export interface RuleWithNoDuplicateSelectorsKeyframeBlockOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateSelectorsKeyframeBlockOptions;
}
export interface RuleWithNoDuplicateTestHooksOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateTestHooksOptions;
}
export interface RuleWithNoEmptyBlockOptions {
	level: RulePlainConfiguration;
	options?: NoEmptyBlockOptions;
}
export interface RuleWithNoEmptyBlockStatementsOptions {
	level: RulePlainConfiguration;
	options?: NoEmptyBlockStatementsOptions;
}
export interface RuleWithNoEmptyInterfaceOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoEmptyInterfaceOptions;
}
export interface RuleWithNoEvolvingTypesOptions {
	level: RulePlainConfiguration;
	options?: NoEvolvingTypesOptions;
}
export interface RuleWithNoExplicitAnyOptions {
	level: RulePlainConfiguration;
	options?: NoExplicitAnyOptions;
}
export interface RuleWithNoExportsInTestOptions {
	level: RulePlainConfiguration;
	options?: NoExportsInTestOptions;
}
export interface RuleWithNoExtraNonNullAssertionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoExtraNonNullAssertionOptions;
}
export interface RuleWithNoFallthroughSwitchClauseOptions {
	level: RulePlainConfiguration;
	options?: NoFallthroughSwitchClauseOptions;
}
export interface RuleWithNoFocusedTestsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoFocusedTestsOptions;
}
export interface RuleWithNoFunctionAssignOptions {
	level: RulePlainConfiguration;
	options?: NoFunctionAssignOptions;
}
export interface RuleWithNoGlobalAssignOptions {
	level: RulePlainConfiguration;
	options?: NoGlobalAssignOptions;
}
export interface RuleWithNoGlobalIsFiniteOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoGlobalIsFiniteOptions;
}
export interface RuleWithNoGlobalIsNanOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoGlobalIsNanOptions;
}
export interface RuleWithNoHeadImportInDocumentOptions {
	level: RulePlainConfiguration;
	options?: NoHeadImportInDocumentOptions;
}
export interface RuleWithNoImplicitAnyLetOptions {
	level: RulePlainConfiguration;
	options?: NoImplicitAnyLetOptions;
}
export interface RuleWithNoImportAssignOptions {
	level: RulePlainConfiguration;
	options?: NoImportAssignOptions;
}
export interface RuleWithNoImportantInKeyframeOptions {
	level: RulePlainConfiguration;
	options?: NoImportantInKeyframeOptions;
}
export interface RuleWithNoIrregularWhitespaceOptions {
	level: RulePlainConfiguration;
	options?: NoIrregularWhitespaceOptions;
}
export interface RuleWithNoLabelVarOptions {
	level: RulePlainConfiguration;
	options?: NoLabelVarOptions;
}
export interface RuleWithNoMisleadingCharacterClassOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoMisleadingCharacterClassOptions;
}
export interface RuleWithNoMisleadingInstantiatorOptions {
	level: RulePlainConfiguration;
	options?: NoMisleadingInstantiatorOptions;
}
export interface RuleWithNoMisplacedAssertionOptions {
	level: RulePlainConfiguration;
	options?: NoMisplacedAssertionOptions;
}
export interface RuleWithNoMisrefactoredShorthandAssignOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoMisrefactoredShorthandAssignOptions;
}
export interface RuleWithNoNonNullAssertedOptionalChainOptions {
	level: RulePlainConfiguration;
	options?: NoNonNullAssertedOptionalChainOptions;
}
export interface RuleWithNoOctalEscapeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoOctalEscapeOptions;
}
export interface RuleWithNoPrototypeBuiltinsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoPrototypeBuiltinsOptions;
}
export interface RuleWithNoQuickfixBiomeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoQuickfixBiomeOptions;
}
export interface RuleWithNoReactSpecificPropsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoReactSpecificPropsOptions;
}
export interface RuleWithNoRedeclareOptions {
	level: RulePlainConfiguration;
	options?: NoRedeclareOptions;
}
export interface RuleWithNoRedundantUseStrictOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoRedundantUseStrictOptions;
}
export interface RuleWithNoSelfCompareOptions {
	level: RulePlainConfiguration;
	options?: NoSelfCompareOptions;
}
export interface RuleWithNoShadowRestrictedNamesOptions {
	level: RulePlainConfiguration;
	options?: NoShadowRestrictedNamesOptions;
}
export interface RuleWithNoShorthandPropertyOverridesOptions {
	level: RulePlainConfiguration;
	options?: NoShorthandPropertyOverridesOptions;
}
export interface RuleWithNoSkippedTestsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoSkippedTestsOptions;
}
export interface RuleWithNoSparseArrayOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoSparseArrayOptions;
}
export interface RuleWithNoSuspiciousSemicolonInJsxOptions {
	level: RulePlainConfiguration;
	options?: NoSuspiciousSemicolonInJsxOptions;
}
export interface RuleWithNoTemplateCurlyInStringOptions {
	level: RulePlainConfiguration;
	options?: NoTemplateCurlyInStringOptions;
}
export interface RuleWithNoThenPropertyOptions {
	level: RulePlainConfiguration;
	options?: NoThenPropertyOptions;
}
export interface RuleWithNoTsIgnoreOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoTsIgnoreOptions;
}
export interface RuleWithNoUnassignedVariablesOptions {
	level: RulePlainConfiguration;
	options?: NoUnassignedVariablesOptions;
}
export interface RuleWithNoUnknownAtRulesOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownAtRulesOptions;
}
export interface RuleWithNoUnsafeDeclarationMergingOptions {
	level: RulePlainConfiguration;
	options?: NoUnsafeDeclarationMergingOptions;
}
export interface RuleWithNoUnsafeNegationOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUnsafeNegationOptions;
}
export interface RuleWithNoUselessEscapeInStringOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessEscapeInStringOptions;
}
export interface RuleWithNoUselessRegexBackrefsOptions {
	level: RulePlainConfiguration;
	options?: NoUselessRegexBackrefsOptions;
}
export interface RuleWithNoVarOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoVarOptions;
}
export interface RuleWithNoWithOptions {
	level: RulePlainConfiguration;
	options?: NoWithOptions;
}
export interface RuleWithUseAdjacentOverloadSignaturesOptions {
	level: RulePlainConfiguration;
	options?: UseAdjacentOverloadSignaturesOptions;
}
export interface RuleWithUseAwaitOptions {
	level: RulePlainConfiguration;
	options?: UseAwaitOptions;
}
export interface RuleWithUseBiomeIgnoreFolderOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseBiomeIgnoreFolderOptions;
}
export interface RuleWithUseDefaultSwitchClauseLastOptions {
	level: RulePlainConfiguration;
	options?: UseDefaultSwitchClauseLastOptions;
}
export interface RuleWithUseErrorMessageOptions {
	level: RulePlainConfiguration;
	options?: UseErrorMessageOptions;
}
export interface RuleWithUseGetterReturnOptions {
	level: RulePlainConfiguration;
	options?: UseGetterReturnOptions;
}
export interface RuleWithUseGoogleFontDisplayOptions {
	level: RulePlainConfiguration;
	options?: UseGoogleFontDisplayOptions;
}
export interface RuleWithUseGuardForInOptions {
	level: RulePlainConfiguration;
	options?: UseGuardForInOptions;
}
export interface RuleWithUseIsArrayOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseIsArrayOptions;
}
export interface RuleWithUseIterableCallbackReturnOptions {
	level: RulePlainConfiguration;
	options?: UseIterableCallbackReturnOptions;
}
export interface RuleWithUseNamespaceKeywordOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNamespaceKeywordOptions;
}
export interface RuleWithUseNumberToFixedDigitsArgumentOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNumberToFixedDigitsArgumentOptions;
}
export interface RuleWithUseStaticResponseMethodsOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseStaticResponseMethodsOptions;
}
export interface RuleWithUseStrictModeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseStrictModeOptions;
}
export type ImportGroups = ImportGroup[];
export type SortOrder = "natural" | "lexicographic";
/**
 * Used to identify the kind of code action emitted by a rule
 */
export type FixKind = "none" | "safe" | "unsafe";
export type NoAccessKeyOptions = {};
export type NoAriaHiddenOnFocusableOptions = {};
export type NoAriaUnsupportedElementsOptions = {};
export type NoAutofocusOptions = {};
export type NoDistractingElementsOptions = {};
export type NoHeaderScopeOptions = {};
export type NoInteractiveElementToNoninteractiveRoleOptions = {};
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
export type NoNoninteractiveElementInteractionsOptions = {};
export type NoNoninteractiveElementToInteractiveRoleOptions = {};
export type NoNoninteractiveTabindexOptions = {};
export type NoPositiveTabindexOptions = {};
export type NoRedundantAltOptions = {};
export type NoRedundantRolesOptions = {};
export type NoStaticElementInteractionsOptions = {};
export type NoSvgWithoutTitleOptions = {};
export type UseAltTextOptions = {};
export type UseAnchorContentOptions = {};
export type UseAriaActivedescendantWithTabindexOptions = {};
export type UseAriaPropsForRoleOptions = {};
export type UseAriaPropsSupportedByRoleOptions = {};
export type UseButtonTypeOptions = {};
export type UseFocusableInteractiveOptions = {};
export type UseGenericFontNamesOptions = {};
export type UseHeadingContentOptions = {};
export type UseHtmlLangOptions = {};
export type UseIframeTitleOptions = {};
export type UseKeyWithClickEventsOptions = {};
export type UseKeyWithMouseEventsOptions = {};
export type UseMediaCaptionOptions = {};
export type UseSemanticElementsOptions = {};
export type UseValidAnchorOptions = {};
export type UseValidAriaPropsOptions = {};
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
export type UseValidAriaValuesOptions = {};
export interface UseValidAutocompleteOptions {
	/**
	 * `input` like custom components that should be checked.
	 */
	inputComponents?: string[];
}
export type UseValidLangOptions = {};
export type NoAdjacentSpacesInRegexOptions = {};
export type NoArgumentsOptions = {};
export type NoBannedTypesOptions = {};
export type NoCommaOperatorOptions = {};
export type NoEmptyTypeParametersOptions = {};
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
export type NoExcessiveNestedTestSuitesOptions = {};
export type NoExtraBooleanCastOptions = {};
export type NoFlatMapIdentityOptions = {};
export interface NoForEachOptions {
	/**
	 * A list of variable names allowed for `forEach` calls.
	 */
	allowedIdentifiers?: string[];
}
export type NoImplicitCoercionsOptions = {};
export type NoImportantStylesOptions = {};
export type NoStaticOnlyClassOptions = {};
export type NoThisInStaticOptions = {};
export type NoUselessCatchOptions = {};
export type NoUselessConstructorOptions = {};
export type NoUselessContinueOptions = {};
export type NoUselessEmptyExportOptions = {};
export type NoUselessEscapeInRegexOptions = {};
export type NoUselessFragmentsOptions = {};
export type NoUselessLabelOptions = {};
export type NoUselessLoneBlockStatementsOptions = {};
export type NoUselessRenameOptions = {};
export type NoUselessStringConcatOptions = {};
export type NoUselessStringRawOptions = {};
export type NoUselessSwitchCaseOptions = {};
export type NoUselessTernaryOptions = {};
export type NoUselessThisAliasOptions = {};
export type NoUselessTypeConstraintOptions = {};
export type NoUselessUndefinedInitializationOptions = {};
export type NoVoidOptions = {};
export type UseArrowFunctionOptions = {};
export type UseDateNowOptions = {};
export type UseFlatMapOptions = {};
export type UseIndexOfOptions = {};
export type UseLiteralKeysOptions = {};
export type UseNumericLiteralsOptions = {};
export type UseOptionalChainOptions = {};
export type UseRegexLiteralsOptions = {};
export type UseSimpleNumberKeysOptions = {};
export type UseSimplifiedLogicExpressionOptions = {};
export type UseWhileOptions = {};
export type NoChildrenPropOptions = {};
export type NoConstAssignOptions = {};
export type NoConstantConditionOptions = {};
export type NoConstantMathMinMaxClampOptions = {};
export type NoConstructorReturnOptions = {};
export type NoEmptyCharacterClassInRegexOptions = {};
export type NoEmptyPatternOptions = {};
export type NoGlobalDirnameFilenameOptions = {};
export type NoGlobalObjectCallsOptions = {};
export type NoInnerDeclarationsOptions = {};
export type NoInvalidBuiltinInstantiationOptions = {};
export type NoInvalidConstructorSuperOptions = {};
export type NoInvalidDirectionInLinearGradientOptions = {};
export type NoInvalidGridAreasOptions = {};
export type NoInvalidPositionAtImportRuleOptions = {};
export type NoInvalidUseBeforeDeclarationOptions = {};
export type NoMissingVarFunctionOptions = {};
export type NoNestedComponentDefinitionsOptions = {};
export type NoNodejsModulesOptions = {};
export type NoNonoctalDecimalEscapeOptions = {};
export type NoPrecisionLossOptions = {};
export interface NoPrivateImportsOptions {
	/**
	* The default visibility to assume for symbols without visibility tag.

Default: **public**. 
	 */
	defaultVisibility?: Visibility;
}
export type NoProcessGlobalOptions = {};
export type NoQwikUseVisibleTaskOptions = {};
export type NoReactPropAssignmentsOptions = {};
export type NoRenderReturnValueOptions = {};
export interface NoRestrictedElementsOptions {
	/**
	* Elements to restrict.
Each key is the element name, and the value is the message to show when the element is used. 
	 */
	elements?: CustomRestrictedElements;
}
export type NoSelfAssignOptions = {};
export type NoSetterReturnOptions = {};
export type NoSolidDestructuredPropsOptions = {};
export type NoStringCaseMismatchOptions = {};
export type NoSwitchDeclarationsOptions = {};
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
export type NoUnknownFunctionOptions = {};
export type NoUnknownMediaFeatureNameOptions = {};
export type NoUnknownPropertyOptions = {};
export type NoUnknownPseudoClassOptions = {};
export type NoUnknownPseudoElementOptions = {};
export type NoUnknownTypeSelectorOptions = {};
export type NoUnknownUnitOptions = {};
export type NoUnmatchableAnbSelectorOptions = {};
export type NoUnreachableOptions = {};
export type NoUnreachableSuperOptions = {};
export type NoUnsafeFinallyOptions = {};
export type NoUnsafeOptionalChainingOptions = {};
export interface NoUnusedFunctionParametersOptions {
	/**
	 * Whether to ignore unused variables from an object destructuring with a spread.
	 */
	ignoreRestSiblings?: boolean;
}
export type NoUnusedImportsOptions = {};
export type NoUnusedLabelsOptions = {};
export type NoUnusedPrivateClassMembersOptions = {};
export interface NoUnusedVariablesOptions {
	/**
	 * Whether to ignore unused variables from an object destructuring with a spread.
	 */
	ignoreRestSiblings?: boolean;
}
export type NoVoidElementsWithChildrenOptions = {};
export type NoVoidTypeReturnOptions = {};
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
export type UseGraphqlNamedOperationsOptions = {};
export type UseHookAtTopLevelOptions = {};
export type UseImageSizeOptions = null;
export interface UseImportExtensionsOptions {
	/**
	* If `true`, the suggested extension is always `.js` regardless of what
extension the source file has in your project. 
	 */
	forceJsExtensions?: boolean;
}
export type UseIsNanOptions = {};
export type UseJsonImportAttributesOptions = {};
export interface UseJsxKeyInIterableOptions {
	/**
	 * Set to `true` to check shorthand fragments (`<></>`)
	 */
	checkShorthandFragments?: boolean;
}
export type UseParseIntRadixOptions = {};
export type UseQwikClasslistOptions = {};
export type UseSingleJsDocAsteriskOptions = {};
export interface UseUniqueElementIdsOptions {
	/**
	* Component names that accept an `id` prop that does not translate
to a DOM element id. 
	 */
	excludedComponents?: string[];
}
export type UseValidForDirectionOptions = {};
export type UseValidTypeofOptions = {};
export type UseYieldOptions = {};
export type NoContinueOptions = {};
export type NoDeprecatedImportsOptions = {};
export type NoDuplicateDependenciesOptions = {};
export interface NoEmptySourceOptions {
	/**
	 * Whether comments are considered meaningful
	 */
	allowComments?: boolean;
}
export type NoFloatingPromisesOptions = {};
export type NoForInOptions = {};
export interface NoImportCyclesOptions {
	/**
	* Ignores type-only imports when finding an import cycle. A type-only import (`import type`)
will be removed by the compiler, so it cuts an import cycle at runtime. Note that named type
imports (`import { type Foo }`) aren't considered as type-only because it's not removed by
the compiler if the `verbatimModuleSyntax` option is enabled. Enabled by default. 
	 */
	ignoreTypes?: boolean;
}
export interface NoIncrementDecrementOptions {
	/**
	 * Allows unary operators ++ and -- in the afterthought (final expression) of a for loop.
	 */
	allowForLoopAfterthoughts?: boolean;
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
export type NoMisusedPromisesOptions = {};
export type NoNextAsyncClientComponentOptions = {};
export type NoParametersOnlyUsedInRecursionOptions = {};
export type NoReactForwardRefOptions = {};
export type NoShadowOptions = {};
export interface NoUnknownAttributeOptions {
	ignore?: string[];
}
export type NoUnnecessaryConditionsOptions = {};
export type NoUnresolvedImportsOptions = {};
export type NoUnusedExpressionsOptions = {};
/**
	* Options for the `noUselessCatchBinding` rule.
Currently empty; reserved for future extensions (e.g. allowlist of names). 
	 */
export type NoUselessCatchBindingOptions = {};
export type NoUselessUndefinedOptions = {};
export type NoVueDataObjectDeclarationOptions = {};
export type NoVueDuplicateKeysOptions = {};
export type NoVueReservedKeysOptions = {};
export type NoVueReservedPropsOptions = {};
export type UseArraySortCompareOptions = {};
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
export interface UseConsistentGraphqlDescriptionsOptions {
	/**
	 * The description style to enforce. Defaults to "block"
	 */
	style?: UseConsistentGraphqlDescriptionsStyle;
}
export interface UseDeprecatedDateOptions {
	argumentName?: string;
}
export type UseExhaustiveSwitchCasesOptions = {};
export type UseExplicitTypeOptions = {};
export interface UseMaxParamsOptions {
	/**
	 * Maximum number of parameters allowed (default: 4)
	 */
	max?: number;
}
export type UseQwikMethodUsageOptions = {};
export type UseQwikValidLexicalScopeOptions = {};
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
export type UseUniqueGraphqlOperationNameOptions = {};
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
	ignores?: string[];
}
export type UseVueValidVBindOptions = {};
export type UseVueValidVElseOptions = {};
export type UseVueValidVElseIfOptions = {};
export type UseVueValidVHtmlOptions = {};
export type UseVueValidVIfOptions = {};
export interface UseVueValidVOnOptions {
	/**
	 * Additional modifiers that should be considered valid
	 */
	modifiers?: string[];
}
export type NoAccumulatingSpreadOptions = {};
export type NoAwaitInLoopsOptions = {};
export type NoBarrelFileOptions = {};
export type NoDeleteOptions = {};
export type NoDynamicNamespaceImportAccessOptions = {};
export type NoImgElementOptions = {};
export type NoNamespaceImportOptions = {};
export type NoReExportAllOptions = {};
export type NoUnwantedPolyfillioOptions = {};
export type UseGoogleFontPreconnectOptions = {};
export type UseSolidForComponentOptions = {};
export type UseTopLevelRegexOptions = {};
export interface NoBlankTargetOptions {
	/**
	* List of domains where `target="_blank"` is allowed without
`rel="noopener"`. 
	 */
	allowDomains?: string[];
	/**
	 * Whether `noreferrer` is allowed in addition to `noopener`.
	 */
	allowNoReferrer?: boolean;
}
export type NoDangerouslySetInnerHtmlOptions = {};
export type NoDangerouslySetInnerHtmlWithChildrenOptions = {};
export type NoGlobalEvalOptions = {};
export interface NoSecretsOptions {
	/**
	 * Set entropy threshold (default is 41).
	 */
	entropyThreshold?: number;
}
export type NoCommonJsOptions = {};
export type NoDefaultExportOptions = {};
export type NoDescendingSpecificityOptions = {};
export type NoDoneCallbackOptions = {};
export type NoEnumOptions = {};
export type NoExportedImportsOptions = {};
export type NoHeadElementOptions = {};
export type NoImplicitBooleanOptions = {};
export type NoInferrableTypesOptions = {};
export type NoMagicNumbersOptions = {};
export type NoNamespaceOptions = {};
export type NoNegationElseOptions = {};
export type NoNestedTernaryOptions = {};
export type NoNonNullAssertionOptions = {};
export interface NoParameterAssignOptions {
	/**
	 * Whether to report an error when a dependency is listed in the dependencies array but isn't used. Defaults to `allow`.
	 */
	propertyAssignment?: PropertyAssignmentMode;
}
export type NoParameterPropertiesOptions = {};
export type NoProcessEnvOptions = {};
export interface NoRestrictedGlobalsOptions {
	/**
	 * A list of names that should trigger the rule
	 */
	deniedGlobals?: Record<string, string>;
}
export interface NoRestrictedImportsOptions {
	/**
	 * A list of import paths that should trigger the rule.
	 */
	paths?: Record<string, Paths>;
	/**
	 * gitignore-style patterns that should trigger the rule.
	 */
	patterns?: Patterns[];
}
export interface NoRestrictedTypesOptions {
	types?: Record<string, CustomRestrictedType>;
}
export type NoShoutyConstantsOptions = {};
export type NoSubstrOptions = {};
export type NoUnusedTemplateLiteralOptions = {};
export type NoUselessElseOptions = {};
export type NoValueAtRuleOptions = {};
export type NoYodaExpressionOptions = {};
export type UseArrayLiteralsOptions = {};
export type UseAsConstAssertionOptions = {};
export type UseAtIndexOptions = {};
export type UseBlockStatementsOptions = {};
export type UseCollapsedElseIfOptions = {};
export type UseCollapsedIfOptions = {};
export interface UseComponentExportOnlyModulesOptions {
	/**
	 * Allows the export of constants. This option is for environments that support it, such as [Vite](https://vitejs.dev/)
	 */
	allowConstantExport?: boolean;
	/**
	 * A list of names that can be additionally exported from the module This option is for exports that do not hinder [React Fast Refresh](https://github.com/facebook/react/tree/main/packages/react-refresh), such as [`meta` in Remix](https://remix.run/docs/en/main/route/meta)
	 */
	allowExportNames?: string[];
}
export interface UseConsistentArrayTypeOptions {
	syntax?: ConsistentArrayType;
}
export type UseConsistentBuiltinInstantiationOptions = {};
export type UseConsistentCurlyBracesOptions = {};
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
export type UseConstOptions = {};
export type UseDefaultParameterLastOptions = {};
export type UseDefaultSwitchClauseOptions = {};
export type UseDeprecatedReasonOptions = {};
export type UseEnumInitializersOptions = {};
export type UseExplicitLengthCheckOptions = {};
export type UseExponentiationOperatorOptions = {};
export type UseExportTypeOptions = {};
export type UseExportsLastOptions = {};
export interface UseFilenamingConventionOptions {
	/**
	 * Allowed cases for file names.
	 */
	filenameCases?: FilenameCases;
	/**
	 * Regular expression to enforce
	 */
	match?: Regex;
	/**
	 * If `false`, then non-ASCII characters are allowed.
	 */
	requireAscii?: boolean;
	/**
	* If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases.
This does not affect other [Case]. 
	 */
	strictCase?: boolean;
}
export type UseForOfOptions = {};
export type UseFragmentSyntaxOptions = {};
export type UseGraphqlNamingConventionOptions = {};
export type UseGroupedAccessorPairsOptions = {};
export interface UseImportTypeOptions {
	/**
	 * The style to apply when import types. Default to "auto"
	 */
	style?: UseImportTypeStyle;
}
export type UseLiteralEnumMembersOptions = {};
/**
 * Rule's options.
 */
export interface UseNamingConventionOptions {
	/**
	 * Custom conventions.
	 */
	conventions?: Convention[];
	/**
	 * If `false`, then non-ASCII characters are allowed.
	 */
	requireAscii?: boolean;
	/**
	* If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases.
This does not affect other [Case]. 
	 */
	strictCase?: boolean;
}
export type UseNodeAssertStrictOptions = {};
export type UseNodejsImportProtocolOptions = {};
export type UseNumberNamespaceOptions = {};
export type UseNumericSeparatorsOptions = {};
export type UseObjectSpreadOptions = {};
export type UseReactFunctionComponentsOptions = {};
export interface UseReadonlyClassPropertiesOptions {
	/**
	 * When `true`, the keywords `public`, `protected`, and `private` are analyzed by the rule.
	 */
	checkAllProperties?: boolean;
}
export interface UseSelfClosingElementsOptions {
	ignoreHtmlElements?: boolean;
}
export type UseShorthandAssignOptions = {};
export type UseShorthandFunctionTypeOptions = {};
export type UseSingleVarDeclaratorOptions = {};
export type UseSymbolDescriptionOptions = {};
export type UseTemplateOptions = {};
export type UseThrowNewErrorOptions = {};
export type UseThrowOnlyErrorOptions = {};
export type UseTrimStartEndOptions = {};
export type UseUnifiedTypeSignaturesOptions = {};
export type NoAlertOptions = {};
export type NoApproximativeNumericConstantOptions = {};
export type NoArrayIndexKeyOptions = {};
export type NoAssignInExpressionsOptions = {};
export type NoAsyncPromiseExecutorOptions = {};
export type NoBiomeFirstExceptionOptions = {};
export interface NoBitwiseOperatorsOptions {
	/**
	 * Allows a list of bitwise operators to be used as exceptions.
	 */
	allow?: string[];
}
export type NoCatchAssignOptions = {};
export type NoClassAssignOptions = {};
export type NoCommentTextOptions = {};
export type NoCompareNegZeroOptions = {};
export interface NoConfusingLabelsOptions {
	/**
	 * A list of (non-confusing) labels that should be allowed
	 */
	allowedLabels?: string[];
}
export type NoConfusingVoidTypeOptions = {};
export interface NoConsoleOptions {
	/**
	 * Allowed calls on the console object.
	 */
	allow?: string[];
}
export type NoConstEnumOptions = {};
export type NoConstantBinaryExpressionsOptions = {};
export type NoControlCharactersInRegexOptions = {};
export type NoDebuggerOptions = {};
export type NoDocumentCookieOptions = {};
export type NoDocumentImportInPageOptions = {};
export interface NoDoubleEqualsOptions {
	/**
	* If `true`, an exception is made when comparing with `null`, as it's often relied on to check
both for `null` or `undefined`.

If `false`, no such exception will be made. 
	 */
	ignoreNull?: boolean;
}
export type NoDuplicateAtImportRulesOptions = {};
export type NoDuplicateCaseOptions = {};
export type NoDuplicateClassMembersOptions = {};
export type NoDuplicateCustomPropertiesOptions = {};
export type NoDuplicateElseIfOptions = {};
export type NoDuplicateFieldsOptions = {};
export type NoDuplicateFontNamesOptions = {};
export type NoDuplicateJsxPropsOptions = {};
export type NoDuplicateObjectKeysOptions = {};
export type NoDuplicateParametersOptions = {};
export type NoDuplicatePropertiesOptions = {};
export type NoDuplicateSelectorsKeyframeBlockOptions = {};
export type NoDuplicateTestHooksOptions = {};
export type NoEmptyBlockOptions = {};
export type NoEmptyBlockStatementsOptions = {};
export type NoEmptyInterfaceOptions = {};
export type NoEvolvingTypesOptions = {};
export type NoExplicitAnyOptions = {};
export type NoExportsInTestOptions = {};
export type NoExtraNonNullAssertionOptions = {};
export type NoFallthroughSwitchClauseOptions = {};
export type NoFocusedTestsOptions = {};
export type NoFunctionAssignOptions = {};
export type NoGlobalAssignOptions = {};
export type NoGlobalIsFiniteOptions = {};
export type NoGlobalIsNanOptions = {};
export type NoHeadImportInDocumentOptions = {};
export type NoImplicitAnyLetOptions = {};
export type NoImportAssignOptions = {};
export type NoImportantInKeyframeOptions = {};
export type NoIrregularWhitespaceOptions = {};
export type NoLabelVarOptions = {};
export type NoMisleadingCharacterClassOptions = {};
export type NoMisleadingInstantiatorOptions = {};
export type NoMisplacedAssertionOptions = {};
export type NoMisrefactoredShorthandAssignOptions = {};
export type NoNonNullAssertedOptionalChainOptions = {};
export type NoOctalEscapeOptions = {};
export type NoPrototypeBuiltinsOptions = {};
export interface NoQuickfixBiomeOptions {
	/**
	 * A list of additional JSON files that should be checked.
	 */
	additionalPaths?: string[];
}
export type NoReactSpecificPropsOptions = {};
export type NoRedeclareOptions = {};
export type NoRedundantUseStrictOptions = {};
export type NoSelfCompareOptions = {};
export type NoShadowRestrictedNamesOptions = {};
export type NoShorthandPropertyOverridesOptions = {};
export type NoSkippedTestsOptions = {};
export type NoSparseArrayOptions = {};
export type NoSuspiciousSemicolonInJsxOptions = {};
export type NoTemplateCurlyInStringOptions = {};
export type NoThenPropertyOptions = {};
export type NoTsIgnoreOptions = {};
export type NoUnassignedVariablesOptions = {};
export interface NoUnknownAtRulesOptions {
	/**
	 * A list of unknown at-rule names to ignore (case-insensitive).
	 */
	ignore?: string[];
}
export type NoUnsafeDeclarationMergingOptions = {};
export type NoUnsafeNegationOptions = {};
export type NoUselessEscapeInStringOptions = {};
export type NoUselessRegexBackrefsOptions = {};
export type NoVarOptions = {};
export type NoWithOptions = {};
export type UseAdjacentOverloadSignaturesOptions = {};
export type UseAwaitOptions = {};
export type UseBiomeIgnoreFolderOptions = {};
export type UseDefaultSwitchClauseLastOptions = {};
export type UseErrorMessageOptions = {};
export type UseGetterReturnOptions = {};
export type UseGoogleFontDisplayOptions = {};
export type UseGuardForInOptions = {};
export type UseIsArrayOptions = {};
export type UseIterableCallbackReturnOptions = {};
export type UseNamespaceKeywordOptions = {};
export type UseNumberToFixedDigitsArgumentOptions = {};
export type UseStaticResponseMethodsOptions = {};
export type UseStrictModeOptions = {};
export type ImportGroup = null | GroupMatcher | GroupMatcher[];
export type Visibility = "public" | "package" | "private";
/**
 * Elements to restrict. Each key is the element name, and the value is the message to show when the element is used.
 */
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

Set to `true` to mark the identity of the hook's return value as stable,
or use a number/an array of numbers to mark the "positions" in the
return array as stable.

For example, for React's `useRef()` hook the value would be `true`,
while for `useState()` it would be `[1]`. 
	 */
	stableResult?: StableHookResult;
}
export type UseConsistentArrowReturnStyle = "asNeeded" | "always" | "never";
/**
 * The GraphQL description style to enforce.
 */
export type UseConsistentGraphqlDescriptionsStyle = "block" | "inline";
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
 * The style to apply when importing types.
 */
export type UseImportTypeStyle = "auto" | "inlineType" | "separatedType";
export interface Convention {
	/**
	 * String cases to enforce
	 */
	formats?: Formats;
	/**
	 * Regular expression to enforce
	 */
	match?: Regex;
	/**
	 * Declarations concerned by this convention
	 */
	selector?: Selector;
}
export type GroupMatcher = ImportMatcher | SourceMatcher;
export type StableHookResult = boolean | number[] | string[];
export interface PathOptions {
	/**
	 * Names of the exported members that allowed to be not be used.
	 */
	allowImportNames?: string[];
	/**
	 * Names of the exported members that should not be used.
	 */
	importNames?: string[];
	/**
	 * The message to display when this module is imported.
	 */
	message?: string;
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
	kind?: Kind;
	/**
	 * Modifiers used on the declaration
	 */
	modifiers?: Modifiers;
	/**
	 * Scope of the declaration
	 */
	scope?: Scope;
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
	| "lint/nursery/noContinue"
	| "lint/nursery/noDeprecatedImports"
	| "lint/nursery/noDuplicateDependencies"
	| "lint/nursery/noEmptySource"
	| "lint/nursery/noFloatingPromises"
	| "lint/nursery/noForIn"
	| "lint/nursery/noImplicitCoercion"
	| "lint/nursery/noImportCycles"
	| "lint/nursery/noIncrementDecrement"
	| "lint/nursery/noJsxLiterals"
	| "lint/nursery/noMissingGenericFamilyKeyword"
	| "lint/nursery/noMisusedPromises"
	| "lint/nursery/noNextAsyncClientComponent"
	| "lint/nursery/noParametersOnlyUsedInRecursion"
	| "lint/nursery/noReactForwardRef"
	| "lint/nursery/noShadow"
	| "lint/nursery/noUnknownAttribute"
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
	| "lint/nursery/useArraySortCompare"
	| "lint/nursery/useBiomeSuppressionComment"
	| "lint/nursery/useConsistentArrowReturn"
	| "lint/nursery/useConsistentGraphqlDescriptions"
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
	| "lint/nursery/useUniqueGraphqlOperationName"
	| "lint/nursery/useVueDefineMacrosOrder"
	| "lint/nursery/useVueMultiWordComponentNames"
	| "lint/nursery/useVueValidVBind"
	| "lint/nursery/useVueValidVElse"
	| "lint/nursery/useVueValidVElseIf"
	| "lint/nursery/useVueValidVFor"
	| "lint/nursery/useVueValidVHtml"
	| "lint/nursery/useVueValidVIf"
	| "lint/nursery/useVueValidVModel"
	| "lint/nursery/useVueValidVOn"
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
	path?: Resource;
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

See the [Visitor] trait for additional documentation on all the supported
advice types. 
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
export type Resource = "argv" | "memory" | { file: string };
export type TextRange = [TextSize, TextSize];
export interface MarkupNodeBuf {
	content: string;
	elements: MarkupElement[];
}
/**
	* Internal enum used to automatically generate bit offsets for [DiagnosticTags]
and help with the implementation of `serde` and `schemars` for tags. 
	 */
export type DiagnosticTag =
	| "fixable"
	| "internal"
	| "unnecessaryCode"
	| "deprecatedCode"
	| "verbose";
/**
	* The category for a log advice, defines how the message should be presented
to the user. 
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
	* Whether the folder should be opened as a project, even if no
`biome.json` can be found. 
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
	verbose?: boolean;
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
	* Determines whether the file scanner should descend into
subdirectories of the target paths. 
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
	* Set to `true` to persist the node cache used during parsing, in order to
speed up subsequent reparsing if the document has been edited.

This should only be enabled if reparsing is to be expected, such as when
the file is opened through the LSP Proxy. 
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
	* Used to mark if the JavaScript is embedded inside some particular files. This affects the parsing.
For example, if inside an Astro file, a top-level return statement is allowed. 
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
export type EmbeddingKind =
	| "Vue"
	| "Svelte"
	| "None"
	| {
			Astro: {
				/**
				 * Whether the script is inside Astro frontmatter
				 */
				frontmatter: boolean;
			};
	  };
export type Language =
	| "javaScript"
	| { typeScript: { definition_file: boolean } };
/**
	* Is the source file an ECMAScript Module or Script.
Changes the parsing semantic. 
	 */
export type ModuleKind = "script" | "module";
export type LanguageVariant = "standard" | "standardRestricted" | "jsx";
/**
	* Enum of the different ECMAScript standard versions.
The versions are ordered in increasing order; The newest version comes last.

Defaults to the latest stable ECMAScript standard. 
	 */
export type LanguageVersion = "eS2022" | "eSNext";
/**
 * It represents the extension of the file
 */
export type JsonFileVariant = "standard" | "jsonc";
/**
	* The style of CSS contained in the file.

Currently, Biome aims to be compatible with
the latest Recommendation level standards.

It also supports Tailwind CSS syntax additions, when the parser option is enabled. 
	 */
export type CssVariant = "standard" | "cssModules" | "tailwindCss";
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
	* Whether the path is ignored for specific features e.g. `formatter.includes`.
When this field is empty, Biome checks only `files.includes`. 
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
export type GetModuleGraphParams = {};
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

Maps from the source specifier name to the absolute path it resolves to.
Specifiers that could not be resolved to an absolute will map to the
specifier itself.

## Example

```json
{
  "./foo": "/absolute/path/to/foo.js",
  "react": "react"
}
``` 
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
	* The category of a code action, this type maps directly to the
[CodeActionKind] type in the Language Server Protocol specification

[CodeActionKind]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind 
	 */
export type ActionCategory =
	| { quickFix: string }
	| { refactor: RefactorKind }
	| { source: SourceActionKind }
	| { other: OtherActionCategory };
/**
	* A Suggestion that is provided by Biome's linter, and
can be reported to the user, and can be automatically
applied if it has the right [`Applicability`]. 
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
export interface PullDiagnosticsAndActionsParams {
	categories?: RuleCategories;
	enabledRules?: AnalyzerSelector[];
	only?: AnalyzerSelector[];
	path: BiomePath;
	projectKey: ProjectKey;
	skip?: AnalyzerSelector[];
}
export interface PullDiagnosticsAndActionsResult {
	diagnostics: [Diagnostic, CodeAction[]][];
}
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
export interface Workspace {
	fileFeatures(params: SupportsFeatureParams): Promise<FileFeaturesResult>;
	updateSettings(params: UpdateSettingsParams): Promise<UpdateSettingsResult>;
	openProject(params: OpenProjectParams): Promise<OpenProjectResult>;
	scanProject(params: ScanProjectParams): Promise<ScanProjectResult>;
	openFile(params: OpenFileParams): Promise<OpenFileResult>;
	changeFile(params: ChangeFileParams): Promise<ChangeFileResult>;
	closeFile(params: CloseFileParams): Promise<null>;
	fileExists(params: FileExitsParams): Promise<boolean>;
	isPathIgnored(params: PathIsIgnoredParams): Promise<boolean>;
	updateModuleGraph(params: UpdateModuleGraphParams): Promise<null>;
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
	pullDiagnosticsAndActions(
		params: PullDiagnosticsAndActionsParams,
	): Promise<PullDiagnosticsAndActionsResult>;
	formatFile(params: FormatFileParams): Promise<Printed>;
	formatRange(params: FormatRangeParams): Promise<Printed>;
	formatOnType(params: FormatOnTypeParams): Promise<Printed>;
	fixFile(params: FixFileParams): Promise<FixFileResult>;
	rename(params: RenameParams): Promise<RenameResult>;
	parsePattern(params: ParsePatternParams): Promise<ParsePatternResult>;
	searchPattern(params: SearchPatternParams): Promise<SearchResults>;
	dropPattern(params: DropPatternParams): Promise<null>;
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
		pullDiagnosticsAndActions(params) {
			return transport.request("biome/pull_diagnostics_and_actions", params);
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
