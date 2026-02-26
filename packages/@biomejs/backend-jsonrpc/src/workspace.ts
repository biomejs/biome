// Generated file, do not edit by hand, see `xtask/codegen`
import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	features: FeatureName;
	inlineConfig?: Configuration;
	/**
	 * Features that shouldn't be enabled
	 */
	notRequestedFeatures?: FeatureName;
	path: BiomePath;
	projectKey: ProjectKey;
	skipIgnoreCheck?: boolean;
}
export type FeatureName = FeatureKind[];
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
export type BiomePath = string;
export type ProjectKey = number;
export type FeatureKind =
	| "format"
	| "lint"
	| "search"
	| "assist"
	| "debug"
	| "htmlFullSupport";
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
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
	/**
	* Use any `.editorconfig` files to configure the formatter. Configuration
in `biome.json` will override `.editorconfig` configuration.

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
	 * Enables support for embedding snippets.
	 */
	experimentalEmbeddedSnippetsEnabled?: Bool;
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
	/**
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
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
	* Enables parsing of CSS Modules specific features. Enable this feature only
when your files don't end in `.module.css`. 
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
export type TrailingNewline = boolean;
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
	/**
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
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
	/**
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
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
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
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
	/**
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
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
	/**
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
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
	* Remove duplicate CSS classes.
See https://biomejs.dev/assist/actions/no-duplicate-classes 
	 */
	noDuplicateClasses?: NoDuplicateClassesConfiguration;
	/**
	* Provides a code action to sort the imports and exports in the file using a built-in or custom order.
See https://biomejs.dev/assist/actions/organize-imports 
	 */
	organizeImports?: OrganizeImportsConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Enforce attribute sorting in JSX elements.
See https://biomejs.dev/assist/actions/use-sorted-attributes 
	 */
	useSortedAttributes?: UseSortedAttributesConfiguration;
	/**
	* Sort interface members by key.
See https://biomejs.dev/assist/actions/use-sorted-interface-members 
	 */
	useSortedInterfaceMembers?: UseSortedInterfaceMembersConfiguration;
	/**
	* Sort the keys of a JSON object in natural order.
See https://biomejs.dev/assist/actions/use-sorted-keys 
	 */
	useSortedKeys?: UseSortedKeysConfiguration;
	/**
	* Enforce ordering of CSS properties and nested rules.
See https://biomejs.dev/assist/actions/use-sorted-properties 
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
	| "tailwind"
	| "turborepo"
	| "playwright"
	| "types";
export type RuleDomainValue = "all" | "none" | "recommended";
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
	/**
	* Whether to add a trailing newline at the end of the file.

Setting this option to `false` is **highly discouraged** because it could cause many problems with other tools:
- https://thoughtbot.com/blog/no-newline-at-end-of-file
- https://callmeryan.medium.com/no-newline-at-end-of-file-navigating-gits-warning-for-android-developers-af14e73dd804
- https://unix.stackexchange.com/questions/345548/how-to-cat-files-together-adding-missing-newlines-at-end-of-some-files

Disable the option at your own risk.

Defaults to true. 
	 */
	trailingNewline?: TrailingNewline;
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
export type NoDuplicateClassesConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithNoDuplicateClassesOptions;
export type OrganizeImportsConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithOrganizeImportsOptions;
export type UseSortedAttributesConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithUseSortedAttributesOptions;
export type UseSortedInterfaceMembersConfiguration =
	| RuleAssistPlainConfiguration
	| RuleAssistWithUseSortedInterfaceMembersOptions;
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
	* Enforce that the accesskey attribute is not used on any HTML element.
See https://biomejs.dev/linter/rules/no-access-key 
	 */
	noAccessKey?: NoAccessKeyConfiguration;
	/**
	* Enforce that aria-hidden="true" is not set on focusable elements.
See https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable 
	 */
	noAriaHiddenOnFocusable?: NoAriaHiddenOnFocusableConfiguration;
	/**
	* Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
See https://biomejs.dev/linter/rules/no-aria-unsupported-elements 
	 */
	noAriaUnsupportedElements?: NoAriaUnsupportedElementsConfiguration;
	/**
	* Enforce that the autofocus attribute is not used on elements.
See https://biomejs.dev/linter/rules/no-autofocus 
	 */
	noAutofocus?: NoAutofocusConfiguration;
	/**
	* Enforces that no distracting elements are used.
See https://biomejs.dev/linter/rules/no-distracting-elements 
	 */
	noDistractingElements?: NoDistractingElementsConfiguration;
	/**
	* The scope prop should be used only on \<th> elements.
See https://biomejs.dev/linter/rules/no-header-scope 
	 */
	noHeaderScope?: NoHeaderScopeConfiguration;
	/**
	* Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
See https://biomejs.dev/linter/rules/no-interactive-element-to-noninteractive-role 
	 */
	noInteractiveElementToNoninteractiveRole?: NoInteractiveElementToNoninteractiveRoleConfiguration;
	/**
	* Enforce that a label element or component has a text label and an associated input.
See https://biomejs.dev/linter/rules/no-label-without-control 
	 */
	noLabelWithoutControl?: NoLabelWithoutControlConfiguration;
	/**
	* Disallow use event handlers on non-interactive elements.
See https://biomejs.dev/linter/rules/no-noninteractive-element-interactions 
	 */
	noNoninteractiveElementInteractions?: NoNoninteractiveElementInteractionsConfiguration;
	/**
	* Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
See https://biomejs.dev/linter/rules/no-noninteractive-element-to-interactive-role 
	 */
	noNoninteractiveElementToInteractiveRole?: NoNoninteractiveElementToInteractiveRoleConfiguration;
	/**
	* Enforce that tabIndex is not assigned to non-interactive HTML elements.
See https://biomejs.dev/linter/rules/no-noninteractive-tabindex 
	 */
	noNoninteractiveTabindex?: NoNoninteractiveTabindexConfiguration;
	/**
	* Prevent the usage of positive integers on tabindex attribute.
See https://biomejs.dev/linter/rules/no-positive-tabindex 
	 */
	noPositiveTabindex?: NoPositiveTabindexConfiguration;
	/**
	* Enforce img alt prop does not contain the word "image", "picture", or "photo".
See https://biomejs.dev/linter/rules/no-redundant-alt 
	 */
	noRedundantAlt?: NoRedundantAltConfiguration;
	/**
	* Enforce explicit role property is not the same as implicit/default role property on an element.
See https://biomejs.dev/linter/rules/no-redundant-roles 
	 */
	noRedundantRoles?: NoRedundantRolesConfiguration;
	/**
	* Enforce that static, visible elements (such as \<div>) that have click handlers use the valid role attribute.
See https://biomejs.dev/linter/rules/no-static-element-interactions 
	 */
	noStaticElementInteractions?: NoStaticElementInteractionsConfiguration;
	/**
	* Enforces the usage of the title element for the svg element.
See https://biomejs.dev/linter/rules/no-svg-without-title 
	 */
	noSvgWithoutTitle?: NoSvgWithoutTitleConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Enforce that all elements that require alternative text have meaningful information to relay back to the end user.
See https://biomejs.dev/linter/rules/use-alt-text 
	 */
	useAltText?: UseAltTextConfiguration;
	/**
	* Enforce that anchors have content and that the content is accessible to screen readers.
See https://biomejs.dev/linter/rules/use-anchor-content 
	 */
	useAnchorContent?: UseAnchorContentConfiguration;
	/**
	* Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant.
See https://biomejs.dev/linter/rules/use-aria-activedescendant-with-tabindex 
	 */
	useAriaActivedescendantWithTabindex?: UseAriaActivedescendantWithTabindexConfiguration;
	/**
	* Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
See https://biomejs.dev/linter/rules/use-aria-props-for-role 
	 */
	useAriaPropsForRole?: UseAriaPropsForRoleConfiguration;
	/**
	* Enforce that ARIA properties are valid for the roles that are supported by the element.
See https://biomejs.dev/linter/rules/use-aria-props-supported-by-role 
	 */
	useAriaPropsSupportedByRole?: UseAriaPropsSupportedByRoleConfiguration;
	/**
	* Enforces the usage and validity of the attribute type for the element button.
See https://biomejs.dev/linter/rules/use-button-type 
	 */
	useButtonType?: UseButtonTypeConfiguration;
	/**
	* Elements with an interactive role and interaction handlers must be focusable.
See https://biomejs.dev/linter/rules/use-focusable-interactive 
	 */
	useFocusableInteractive?: UseFocusableInteractiveConfiguration;
	/**
	* Disallow a missing generic family keyword within font families.
See https://biomejs.dev/linter/rules/use-generic-font-names 
	 */
	useGenericFontNames?: UseGenericFontNamesConfiguration;
	/**
	* Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.
See https://biomejs.dev/linter/rules/use-heading-content 
	 */
	useHeadingContent?: UseHeadingContentConfiguration;
	/**
	* Enforce that html element has lang attribute.
See https://biomejs.dev/linter/rules/use-html-lang 
	 */
	useHtmlLang?: UseHtmlLangConfiguration;
	/**
	* Enforces the usage of the attribute title for the element iframe.
See https://biomejs.dev/linter/rules/use-iframe-title 
	 */
	useIframeTitle?: UseIframeTitleConfiguration;
	/**
	* Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress.
See https://biomejs.dev/linter/rules/use-key-with-click-events 
	 */
	useKeyWithClickEvents?: UseKeyWithClickEventsConfiguration;
	/**
	* Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur.
See https://biomejs.dev/linter/rules/use-key-with-mouse-events 
	 */
	useKeyWithMouseEvents?: UseKeyWithMouseEventsConfiguration;
	/**
	* Enforces that audio and video elements must have a track for captions.
See https://biomejs.dev/linter/rules/use-media-caption 
	 */
	useMediaCaption?: UseMediaCaptionConfiguration;
	/**
	* It detects the use of role attributes in JSX elements and suggests using semantic elements instead.
See https://biomejs.dev/linter/rules/use-semantic-elements 
	 */
	useSemanticElements?: UseSemanticElementsConfiguration;
	/**
	* Enforce that all anchors are valid, and they are navigable elements.
See https://biomejs.dev/linter/rules/use-valid-anchor 
	 */
	useValidAnchor?: UseValidAnchorConfiguration;
	/**
	* Ensures that ARIA properties aria-* are all valid.
See https://biomejs.dev/linter/rules/use-valid-aria-props 
	 */
	useValidAriaProps?: UseValidAriaPropsConfiguration;
	/**
	* Elements with ARIA roles must use a valid, non-abstract ARIA role.
See https://biomejs.dev/linter/rules/use-valid-aria-role 
	 */
	useValidAriaRole?: UseValidAriaRoleConfiguration;
	/**
	* Enforce that ARIA state and property values are valid.
See https://biomejs.dev/linter/rules/use-valid-aria-values 
	 */
	useValidAriaValues?: UseValidAriaValuesConfiguration;
	/**
	* Use valid values for the autocomplete attribute on input elements.
See https://biomejs.dev/linter/rules/use-valid-autocomplete 
	 */
	useValidAutocomplete?: UseValidAutocompleteConfiguration;
	/**
	* Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country.
See https://biomejs.dev/linter/rules/use-valid-lang 
	 */
	useValidLang?: UseValidLangConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Complexity {
	/**
	* Disallow unclear usage of consecutive space characters in regular expression literals.
See https://biomejs.dev/linter/rules/no-adjacent-spaces-in-regex 
	 */
	noAdjacentSpacesInRegex?: NoAdjacentSpacesInRegexConfiguration;
	/**
	* Disallow the use of arguments.
See https://biomejs.dev/linter/rules/no-arguments 
	 */
	noArguments?: NoArgumentsConfiguration;
	/**
	* Disallow primitive type aliases and misleading types.
See https://biomejs.dev/linter/rules/no-banned-types 
	 */
	noBannedTypes?: NoBannedTypesConfiguration;
	/**
	* Disallow comma operator.
See https://biomejs.dev/linter/rules/no-comma-operator 
	 */
	noCommaOperator?: NoCommaOperatorConfiguration;
	/**
	* Disallow empty type parameters in type aliases and interfaces.
See https://biomejs.dev/linter/rules/no-empty-type-parameters 
	 */
	noEmptyTypeParameters?: NoEmptyTypeParametersConfiguration;
	/**
	* Disallow functions that exceed a given Cognitive Complexity score.
See https://biomejs.dev/linter/rules/no-excessive-cognitive-complexity 
	 */
	noExcessiveCognitiveComplexity?: NoExcessiveCognitiveComplexityConfiguration;
	/**
	* Restrict the number of lines of code in a function.
See https://biomejs.dev/linter/rules/no-excessive-lines-per-function 
	 */
	noExcessiveLinesPerFunction?: NoExcessiveLinesPerFunctionConfiguration;
	/**
	* This rule enforces a maximum depth to nested describe() in test files.
See https://biomejs.dev/linter/rules/no-excessive-nested-test-suites 
	 */
	noExcessiveNestedTestSuites?: NoExcessiveNestedTestSuitesConfiguration;
	/**
	* Disallow unnecessary boolean casts.
See https://biomejs.dev/linter/rules/no-extra-boolean-cast 
	 */
	noExtraBooleanCast?: NoExtraBooleanCastConfiguration;
	/**
	* Disallow to use unnecessary callback on flatMap.
See https://biomejs.dev/linter/rules/no-flat-map-identity 
	 */
	noFlatMapIdentity?: NoFlatMapIdentityConfiguration;
	/**
	* Prefer for...of statement instead of Array.forEach.
See https://biomejs.dev/linter/rules/no-for-each 
	 */
	noForEach?: NoForEachConfiguration;
	/**
	* Disallow shorthand type conversions.
See https://biomejs.dev/linter/rules/no-implicit-coercions 
	 */
	noImplicitCoercions?: NoImplicitCoercionsConfiguration;
	/**
	* Disallow the use of the !important style.
See https://biomejs.dev/linter/rules/no-important-styles 
	 */
	noImportantStyles?: NoImportantStylesConfiguration;
	/**
	* This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
See https://biomejs.dev/linter/rules/no-static-only-class 
	 */
	noStaticOnlyClass?: NoStaticOnlyClassConfiguration;
	/**
	* Disallow this and super in static contexts.
See https://biomejs.dev/linter/rules/no-this-in-static 
	 */
	noThisInStatic?: NoThisInStaticConfiguration;
	/**
	* Disallow unnecessary catch clauses.
See https://biomejs.dev/linter/rules/no-useless-catch 
	 */
	noUselessCatch?: NoUselessCatchConfiguration;
	/**
	* Disallow unused catch bindings.
See https://biomejs.dev/linter/rules/no-useless-catch-binding 
	 */
	noUselessCatchBinding?: NoUselessCatchBindingConfiguration;
	/**
	* Disallow unnecessary constructors.
See https://biomejs.dev/linter/rules/no-useless-constructor 
	 */
	noUselessConstructor?: NoUselessConstructorConfiguration;
	/**
	* Avoid using unnecessary continue.
See https://biomejs.dev/linter/rules/no-useless-continue 
	 */
	noUselessContinue?: NoUselessContinueConfiguration;
	/**
	* Disallow empty exports that don't change anything in a module file.
See https://biomejs.dev/linter/rules/no-useless-empty-export 
	 */
	noUselessEmptyExport?: NoUselessEmptyExportConfiguration;
	/**
	* Disallow unnecessary escape sequence in regular expression literals.
See https://biomejs.dev/linter/rules/no-useless-escape-in-regex 
	 */
	noUselessEscapeInRegex?: NoUselessEscapeInRegexConfiguration;
	/**
	* Disallow unnecessary fragments.
See https://biomejs.dev/linter/rules/no-useless-fragments 
	 */
	noUselessFragments?: NoUselessFragmentsConfiguration;
	/**
	* Disallow unnecessary labels.
See https://biomejs.dev/linter/rules/no-useless-label 
	 */
	noUselessLabel?: NoUselessLabelConfiguration;
	/**
	* Disallow unnecessary nested block statements.
See https://biomejs.dev/linter/rules/no-useless-lone-block-statements 
	 */
	noUselessLoneBlockStatements?: NoUselessLoneBlockStatementsConfiguration;
	/**
	* Disallow renaming import, export, and destructured assignments to the same name.
See https://biomejs.dev/linter/rules/no-useless-rename 
	 */
	noUselessRename?: NoUselessRenameConfiguration;
	/**
	* Disallow unnecessary concatenation of string or template literals.
See https://biomejs.dev/linter/rules/no-useless-string-concat 
	 */
	noUselessStringConcat?: NoUselessStringConcatConfiguration;
	/**
	* Disallow unnecessary String.raw function in template string literals without any escape sequence.
See https://biomejs.dev/linter/rules/no-useless-string-raw 
	 */
	noUselessStringRaw?: NoUselessStringRawConfiguration;
	/**
	* Disallow useless case in switch statements.
See https://biomejs.dev/linter/rules/no-useless-switch-case 
	 */
	noUselessSwitchCase?: NoUselessSwitchCaseConfiguration;
	/**
	* Disallow ternary operators when simpler alternatives exist.
See https://biomejs.dev/linter/rules/no-useless-ternary 
	 */
	noUselessTernary?: NoUselessTernaryConfiguration;
	/**
	* Disallow useless this aliasing.
See https://biomejs.dev/linter/rules/no-useless-this-alias 
	 */
	noUselessThisAlias?: NoUselessThisAliasConfiguration;
	/**
	* Disallow using any or unknown as type constraint.
See https://biomejs.dev/linter/rules/no-useless-type-constraint 
	 */
	noUselessTypeConstraint?: NoUselessTypeConstraintConfiguration;
	/**
	* Disallow the use of useless undefined.
See https://biomejs.dev/linter/rules/no-useless-undefined 
	 */
	noUselessUndefined?: NoUselessUndefinedConfiguration;
	/**
	* Disallow initializing variables to undefined.
See https://biomejs.dev/linter/rules/no-useless-undefined-initialization 
	 */
	noUselessUndefinedInitialization?: NoUselessUndefinedInitializationConfiguration;
	/**
	* Disallow the use of void operators, which is not a familiar operator.
See https://biomejs.dev/linter/rules/no-void 
	 */
	noVoid?: NoVoidConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Use arrow functions over function expressions.
See https://biomejs.dev/linter/rules/use-arrow-function 
	 */
	useArrowFunction?: UseArrowFunctionConfiguration;
	/**
	* Use Date.now() to get the number of milliseconds since the Unix Epoch.
See https://biomejs.dev/linter/rules/use-date-now 
	 */
	useDateNow?: UseDateNowConfiguration;
	/**
	* Promotes the use of .flatMap() when map().flat() are used together.
See https://biomejs.dev/linter/rules/use-flat-map 
	 */
	useFlatMap?: UseFlatMapConfiguration;
	/**
	* Prefer Array#{indexOf,lastIndexOf}() over Array#{findIndex,findLastIndex}() when looking for the index of an item.
See https://biomejs.dev/linter/rules/use-index-of 
	 */
	useIndexOf?: UseIndexOfConfiguration;
	/**
	* Enforce the usage of a literal access to properties over computed property access.
See https://biomejs.dev/linter/rules/use-literal-keys 
	 */
	useLiteralKeys?: UseLiteralKeysConfiguration;
	/**
	* Enforce a maximum number of parameters in function definitions.
See https://biomejs.dev/linter/rules/use-max-params 
	 */
	useMaxParams?: UseMaxParamsConfiguration;
	/**
	* Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals.
See https://biomejs.dev/linter/rules/use-numeric-literals 
	 */
	useNumericLiterals?: UseNumericLiteralsConfiguration;
	/**
	* Enforce using concise optional chain instead of chained logical expressions.
See https://biomejs.dev/linter/rules/use-optional-chain 
	 */
	useOptionalChain?: UseOptionalChainConfiguration;
	/**
	* Enforce the use of the regular expression literals instead of the RegExp constructor if possible.
See https://biomejs.dev/linter/rules/use-regex-literals 
	 */
	useRegexLiterals?: UseRegexLiteralsConfiguration;
	/**
	* Disallow number literal object member names which are not base 10 or use underscore as separator.
See https://biomejs.dev/linter/rules/use-simple-number-keys 
	 */
	useSimpleNumberKeys?: UseSimpleNumberKeysConfiguration;
	/**
	* Discard redundant terms from logical expressions.
See https://biomejs.dev/linter/rules/use-simplified-logic-expression 
	 */
	useSimplifiedLogicExpression?: UseSimplifiedLogicExpressionConfiguration;
	/**
	* Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed.
See https://biomejs.dev/linter/rules/use-while 
	 */
	useWhile?: UseWhileConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	/**
	* Prevent passing of children as props.
See https://biomejs.dev/linter/rules/no-children-prop 
	 */
	noChildrenProp?: NoChildrenPropConfiguration;
	/**
	* Prevents from having const variables being re-assigned.
See https://biomejs.dev/linter/rules/no-const-assign 
	 */
	noConstAssign?: NoConstAssignConfiguration;
	/**
	* Disallow constant expressions in conditions.
See https://biomejs.dev/linter/rules/no-constant-condition 
	 */
	noConstantCondition?: NoConstantConditionConfiguration;
	/**
	* Disallow the use of Math.min and Math.max to clamp a value where the result itself is constant.
See https://biomejs.dev/linter/rules/no-constant-math-min-max-clamp 
	 */
	noConstantMathMinMaxClamp?: NoConstantMathMinMaxClampConfiguration;
	/**
	* Disallow returning a value from a constructor.
See https://biomejs.dev/linter/rules/no-constructor-return 
	 */
	noConstructorReturn?: NoConstructorReturnConfiguration;
	/**
	* Disallow empty character classes in regular expression literals.
See https://biomejs.dev/linter/rules/no-empty-character-class-in-regex 
	 */
	noEmptyCharacterClassInRegex?: NoEmptyCharacterClassInRegexConfiguration;
	/**
	* Disallows empty destructuring patterns.
See https://biomejs.dev/linter/rules/no-empty-pattern 
	 */
	noEmptyPattern?: NoEmptyPatternConfiguration;
	/**
	* Disallow the use of __dirname and __filename in the global scope.
See https://biomejs.dev/linter/rules/no-global-dirname-filename 
	 */
	noGlobalDirnameFilename?: NoGlobalDirnameFilenameConfiguration;
	/**
	* Disallow calling global object properties as functions.
See https://biomejs.dev/linter/rules/no-global-object-calls 
	 */
	noGlobalObjectCalls?: NoGlobalObjectCallsConfiguration;
	/**
	* Disallow function and var declarations that are accessible outside their block.
See https://biomejs.dev/linter/rules/no-inner-declarations 
	 */
	noInnerDeclarations?: NoInnerDeclarationsConfiguration;
	/**
	* Ensure that builtins are correctly instantiated.
See https://biomejs.dev/linter/rules/no-invalid-builtin-instantiation 
	 */
	noInvalidBuiltinInstantiation?: NoInvalidBuiltinInstantiationConfiguration;
	/**
	* Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors.
See https://biomejs.dev/linter/rules/no-invalid-constructor-super 
	 */
	noInvalidConstructorSuper?: NoInvalidConstructorSuperConfiguration;
	/**
	* Disallow non-standard direction values for linear gradient functions.
See https://biomejs.dev/linter/rules/no-invalid-direction-in-linear-gradient 
	 */
	noInvalidDirectionInLinearGradient?: NoInvalidDirectionInLinearGradientConfiguration;
	/**
	* Disallows invalid named grid areas in CSS Grid Layouts.
See https://biomejs.dev/linter/rules/no-invalid-grid-areas 
	 */
	noInvalidGridAreas?: NoInvalidGridAreasConfiguration;
	/**
	* Disallow the use of @import at-rules in invalid positions.
See https://biomejs.dev/linter/rules/no-invalid-position-at-import-rule 
	 */
	noInvalidPositionAtImportRule?: NoInvalidPositionAtImportRuleConfiguration;
	/**
	* Disallow the use of variables, function parameters, classes, and enums before their declaration.
See https://biomejs.dev/linter/rules/no-invalid-use-before-declaration 
	 */
	noInvalidUseBeforeDeclaration?: NoInvalidUseBeforeDeclarationConfiguration;
	/**
	* Disallow missing var function for css variables.
See https://biomejs.dev/linter/rules/no-missing-var-function 
	 */
	noMissingVarFunction?: NoMissingVarFunctionConfiguration;
	/**
	* Disallows defining React components inside other components.
See https://biomejs.dev/linter/rules/no-nested-component-definitions 
	 */
	noNestedComponentDefinitions?: NoNestedComponentDefinitionsConfiguration;
	/**
	* Prevent client components from being async functions.
See https://biomejs.dev/linter/rules/no-next-async-client-component 
	 */
	noNextAsyncClientComponent?: NoNextAsyncClientComponentConfiguration;
	/**
	* Forbid the use of Node.js builtin modules.
See https://biomejs.dev/linter/rules/no-nodejs-modules 
	 */
	noNodejsModules?: NoNodejsModulesConfiguration;
	/**
	* Disallow \8 and \9 escape sequences in string literals.
See https://biomejs.dev/linter/rules/no-nonoctal-decimal-escape 
	 */
	noNonoctalDecimalEscape?: NoNonoctalDecimalEscapeConfiguration;
	/**
	* Disallow literal numbers that lose precision.
See https://biomejs.dev/linter/rules/no-precision-loss 
	 */
	noPrecisionLoss?: NoPrecisionLossConfiguration;
	/**
	* Restrict imports of private exports.
See https://biomejs.dev/linter/rules/no-private-imports 
	 */
	noPrivateImports?: NoPrivateImportsConfiguration;
	/**
	* Disallow the use of process global.
See https://biomejs.dev/linter/rules/no-process-global 
	 */
	noProcessGlobal?: NoProcessGlobalConfiguration;
	/**
	* Disallow useVisibleTask$() functions in Qwik components.
See https://biomejs.dev/linter/rules/no-qwik-use-visible-task 
	 */
	noQwikUseVisibleTask?: NoQwikUseVisibleTaskConfiguration;
	/**
	* Disallow assigning to React component props.
See https://biomejs.dev/linter/rules/no-react-prop-assignments 
	 */
	noReactPropAssignments?: NoReactPropAssignmentsConfiguration;
	/**
	* Prevent the usage of the return value of React.render.
See https://biomejs.dev/linter/rules/no-render-return-value 
	 */
	noRenderReturnValue?: NoRenderReturnValueConfiguration;
	/**
	* Disallow the use of configured elements.
See https://biomejs.dev/linter/rules/no-restricted-elements 
	 */
	noRestrictedElements?: NoRestrictedElementsConfiguration;
	/**
	* Disallow assignments where both sides are exactly the same.
See https://biomejs.dev/linter/rules/no-self-assign 
	 */
	noSelfAssign?: NoSelfAssignConfiguration;
	/**
	* Disallow returning a value from a setter.
See https://biomejs.dev/linter/rules/no-setter-return 
	 */
	noSetterReturn?: NoSetterReturnConfiguration;
	/**
	* Disallow destructuring props inside JSX components in Solid projects.
See https://biomejs.dev/linter/rules/no-solid-destructured-props 
	 */
	noSolidDestructuredProps?: NoSolidDestructuredPropsConfiguration;
	/**
	* Disallow comparison of expressions modifying the string case with non-compliant value.
See https://biomejs.dev/linter/rules/no-string-case-mismatch 
	 */
	noStringCaseMismatch?: NoStringCaseMismatchConfiguration;
	/**
	* Disallow lexical declarations in switch clauses.
See https://biomejs.dev/linter/rules/no-switch-declarations 
	 */
	noSwitchDeclarations?: NoSwitchDeclarationsConfiguration;
	/**
	* Disallow the use of dependencies that aren't specified in the package.json.
See https://biomejs.dev/linter/rules/no-undeclared-dependencies 
	 */
	noUndeclaredDependencies?: NoUndeclaredDependenciesConfiguration;
	/**
	* Prevents the usage of variables that haven't been declared inside the document.
See https://biomejs.dev/linter/rules/no-undeclared-variables 
	 */
	noUndeclaredVariables?: NoUndeclaredVariablesConfiguration;
	/**
	* Disallow unknown CSS value functions.
See https://biomejs.dev/linter/rules/no-unknown-function 
	 */
	noUnknownFunction?: NoUnknownFunctionConfiguration;
	/**
	* Disallow unknown media feature names.
See https://biomejs.dev/linter/rules/no-unknown-media-feature-name 
	 */
	noUnknownMediaFeatureName?: NoUnknownMediaFeatureNameConfiguration;
	/**
	* Disallow unknown properties.
See https://biomejs.dev/linter/rules/no-unknown-property 
	 */
	noUnknownProperty?: NoUnknownPropertyConfiguration;
	/**
	* Disallow unknown pseudo-class selectors.
See https://biomejs.dev/linter/rules/no-unknown-pseudo-class 
	 */
	noUnknownPseudoClass?: NoUnknownPseudoClassConfiguration;
	/**
	* Disallow unknown pseudo-element selectors.
See https://biomejs.dev/linter/rules/no-unknown-pseudo-element 
	 */
	noUnknownPseudoElement?: NoUnknownPseudoElementConfiguration;
	/**
	* Disallow unknown type selectors.
See https://biomejs.dev/linter/rules/no-unknown-type-selector 
	 */
	noUnknownTypeSelector?: NoUnknownTypeSelectorConfiguration;
	/**
	* Disallow unknown CSS units.
See https://biomejs.dev/linter/rules/no-unknown-unit 
	 */
	noUnknownUnit?: NoUnknownUnitConfiguration;
	/**
	* Disallow unmatchable An+B selectors.
See https://biomejs.dev/linter/rules/no-unmatchable-anb-selector 
	 */
	noUnmatchableAnbSelector?: NoUnmatchableAnbSelectorConfiguration;
	/**
	* Disallow unreachable code.
See https://biomejs.dev/linter/rules/no-unreachable 
	 */
	noUnreachable?: NoUnreachableConfiguration;
	/**
	* Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass.
See https://biomejs.dev/linter/rules/no-unreachable-super 
	 */
	noUnreachableSuper?: NoUnreachableSuperConfiguration;
	/**
	* Warn when importing non-existing exports.
See https://biomejs.dev/linter/rules/no-unresolved-imports 
	 */
	noUnresolvedImports?: NoUnresolvedImportsConfiguration;
	/**
	* Disallow control flow statements in finally blocks.
See https://biomejs.dev/linter/rules/no-unsafe-finally 
	 */
	noUnsafeFinally?: NoUnsafeFinallyConfiguration;
	/**
	* Disallow the use of optional chaining in contexts where the undefined value is not allowed.
See https://biomejs.dev/linter/rules/no-unsafe-optional-chaining 
	 */
	noUnsafeOptionalChaining?: NoUnsafeOptionalChainingConfiguration;
	/**
	* Disallow unused function parameters.
See https://biomejs.dev/linter/rules/no-unused-function-parameters 
	 */
	noUnusedFunctionParameters?: NoUnusedFunctionParametersConfiguration;
	/**
	* Disallow unused imports.
See https://biomejs.dev/linter/rules/no-unused-imports 
	 */
	noUnusedImports?: NoUnusedImportsConfiguration;
	/**
	* Disallow unused labels.
See https://biomejs.dev/linter/rules/no-unused-labels 
	 */
	noUnusedLabels?: NoUnusedLabelsConfiguration;
	/**
	* Disallow unused private class members.
See https://biomejs.dev/linter/rules/no-unused-private-class-members 
	 */
	noUnusedPrivateClassMembers?: NoUnusedPrivateClassMembersConfiguration;
	/**
	* Disallow unused variables.
See https://biomejs.dev/linter/rules/no-unused-variables 
	 */
	noUnusedVariables?: NoUnusedVariablesConfiguration;
	/**
	* This rules prevents void elements (AKA self-closing elements) from having children.
See https://biomejs.dev/linter/rules/no-void-elements-with-children 
	 */
	noVoidElementsWithChildren?: NoVoidElementsWithChildrenConfiguration;
	/**
	* Disallow returning a value from a function with the return type 'void'.
See https://biomejs.dev/linter/rules/no-void-type-return 
	 */
	noVoidTypeReturn?: NoVoidTypeReturnConfiguration;
	/**
	* Enforce that Vue component data options are declared as functions.
See https://biomejs.dev/linter/rules/no-vue-data-object-declaration 
	 */
	noVueDataObjectDeclaration?: NoVueDataObjectDeclarationConfiguration;
	/**
	* Disallow duplicate keys in Vue component data, methods, computed properties, and other options.
See https://biomejs.dev/linter/rules/no-vue-duplicate-keys 
	 */
	noVueDuplicateKeys?: NoVueDuplicateKeysConfiguration;
	/**
	* Disallow reserved keys in Vue component data and computed properties.
See https://biomejs.dev/linter/rules/no-vue-reserved-keys 
	 */
	noVueReservedKeys?: NoVueReservedKeysConfiguration;
	/**
	* Disallow reserved names to be used as props.
See https://biomejs.dev/linter/rules/no-vue-reserved-props 
	 */
	noVueReservedProps?: NoVueReservedPropsConfiguration;
	/**
	* Disallow destructuring of props passed to setup in Vue projects.
See https://biomejs.dev/linter/rules/no-vue-setup-props-reactivity-loss 
	 */
	noVueSetupPropsReactivityLoss?: NoVueSetupPropsReactivityLossConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Enforce correct dependency usage within React hooks.
See https://biomejs.dev/linter/rules/use-exhaustive-dependencies 
	 */
	useExhaustiveDependencies?: UseExhaustiveDependenciesConfiguration;
	/**
	* Enforce specifying the name of GraphQL operations.
See https://biomejs.dev/linter/rules/use-graphql-named-operations 
	 */
	useGraphqlNamedOperations?: UseGraphqlNamedOperationsConfiguration;
	/**
	* Enforce that all React hooks are being called from the Top Level component functions.
See https://biomejs.dev/linter/rules/use-hook-at-top-level 
	 */
	useHookAtTopLevel?: UseHookAtTopLevelConfiguration;
	/**
	* Enforces that \<img> elements have both width and height attributes.
See https://biomejs.dev/linter/rules/use-image-size 
	 */
	useImageSize?: UseImageSizeConfiguration;
	/**
	* Enforce file extensions for relative imports.
See https://biomejs.dev/linter/rules/use-import-extensions 
	 */
	useImportExtensions?: UseImportExtensionsConfiguration;
	/**
	* Require calls to isNaN() when checking for NaN.
See https://biomejs.dev/linter/rules/use-is-nan 
	 */
	useIsNan?: UseIsNanConfiguration;
	/**
	* Enforces the use of with { type: "json" } for JSON module imports.
See https://biomejs.dev/linter/rules/use-json-import-attributes 
	 */
	useJsonImportAttributes?: UseJsonImportAttributesConfiguration;
	/**
	* Disallow missing key props in iterators/collection literals.
See https://biomejs.dev/linter/rules/use-jsx-key-in-iterable 
	 */
	useJsxKeyInIterable?: UseJsxKeyInIterableConfiguration;
	/**
	* Enforce the consistent use of the radix argument when using parseInt().
See https://biomejs.dev/linter/rules/use-parse-int-radix 
	 */
	useParseIntRadix?: UseParseIntRadixConfiguration;
	/**
	* Prefer using the class prop as a classlist over the classnames helper.
See https://biomejs.dev/linter/rules/use-qwik-classlist 
	 */
	useQwikClasslist?: UseQwikClasslistConfiguration;
	/**
	* Disallow use* hooks outside of component$ or other use* hooks in Qwik applications.
See https://biomejs.dev/linter/rules/use-qwik-method-usage 
	 */
	useQwikMethodUsage?: UseQwikMethodUsageConfiguration;
	/**
	* Disallow unserializable expressions in Qwik dollar ($) scopes.
See https://biomejs.dev/linter/rules/use-qwik-valid-lexical-scope 
	 */
	useQwikValidLexicalScope?: UseQwikValidLexicalScopeConfiguration;
	/**
	* Enforce JSDoc comment lines to start with a single asterisk, except for the first one.
See https://biomejs.dev/linter/rules/use-single-js-doc-asterisk 
	 */
	useSingleJsDocAsterisk?: UseSingleJsDocAsteriskConfiguration;
	/**
	* Prevent the usage of static string literal id attribute on elements.
See https://biomejs.dev/linter/rules/use-unique-element-ids 
	 */
	useUniqueElementIds?: UseUniqueElementIdsConfiguration;
	/**
	* Enforce "for" loop update clause moving the counter in the right direction.
See https://biomejs.dev/linter/rules/use-valid-for-direction 
	 */
	useValidForDirection?: UseValidForDirectionConfiguration;
	/**
	* This rule checks that the result of a typeof expression is compared to a valid value.
See https://biomejs.dev/linter/rules/use-valid-typeof 
	 */
	useValidTypeof?: UseValidTypeofConfiguration;
	/**
	* Require generator functions to contain yield.
See https://biomejs.dev/linter/rules/use-yield 
	 */
	useYield?: UseYieldConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Nursery {
	/**
	* Disallow ambiguous anchor descriptions.
See https://biomejs.dev/linter/rules/no-ambiguous-anchor-text 
	 */
	noAmbiguousAnchorText?: NoAmbiguousAnchorTextConfiguration;
	/**
	* Prevent usage of next/script's beforeInteractive strategy outside of pages/_document.js in a Next.js project.
See https://biomejs.dev/linter/rules/no-before-interactive-script-outside-document 
	 */
	noBeforeInteractiveScriptOutsideDocument?: NoBeforeInteractiveScriptOutsideDocumentConfiguration;
	/**
	* Disallow conditional expect() calls inside tests.
See https://biomejs.dev/linter/rules/no-conditional-expect 
	 */
	noConditionalExpect?: NoConditionalExpectConfiguration;
	/**
	* Disallow continue statements.
See https://biomejs.dev/linter/rules/no-continue 
	 */
	noContinue?: NoContinueConfiguration;
	/**
	* Disallow deprecated media types.
See https://biomejs.dev/linter/rules/no-deprecated-media-type 
	 */
	noDeprecatedMediaType?: NoDeprecatedMediaTypeConfiguration;
	/**
	* Disallow equal signs explicitly at the beginning of regular expressions.
See https://biomejs.dev/linter/rules/no-div-regex 
	 */
	noDivRegex?: NoDivRegexConfiguration;
	/**
	* Require all argument names for fields & directives to be unique.
See https://biomejs.dev/linter/rules/no-duplicate-argument-names 
	 */
	noDuplicateArgumentNames?: NoDuplicateArgumentNamesConfiguration;
	/**
	* Disallow duplication of attributes.
See https://biomejs.dev/linter/rules/no-duplicate-attributes 
	 */
	noDuplicateAttributes?: NoDuplicateAttributesConfiguration;
	/**
	* Require all enum value names to be unique.
See https://biomejs.dev/linter/rules/no-duplicate-enum-value-names 
	 */
	noDuplicateEnumValueNames?: NoDuplicateEnumValueNamesConfiguration;
	/**
	* Disallow duplicate enum member values.
See https://biomejs.dev/linter/rules/no-duplicate-enum-values 
	 */
	noDuplicateEnumValues?: NoDuplicateEnumValuesConfiguration;
	/**
	* Require all fields of a type to be unique.
See https://biomejs.dev/linter/rules/no-duplicate-field-definition-names 
	 */
	noDuplicateFieldDefinitionNames?: NoDuplicateFieldDefinitionNamesConfiguration;
	/**
	* Enforce unique operation names across a GraphQL document.
See https://biomejs.dev/linter/rules/no-duplicate-graphql-operation-name 
	 */
	noDuplicateGraphqlOperationName?: NoDuplicateGraphqlOperationNameConfiguration;
	/**
	* Require fields within an input object to be unique.
See https://biomejs.dev/linter/rules/no-duplicate-input-field-names 
	 */
	noDuplicateInputFieldNames?: NoDuplicateInputFieldNamesConfiguration;
	/**
	* Require all variable definitions to be unique.
See https://biomejs.dev/linter/rules/no-duplicate-variable-names 
	 */
	noDuplicateVariableNames?: NoDuplicateVariableNamesConfiguration;
	/**
	* Disallow JSX prop spreading the same identifier multiple times.
See https://biomejs.dev/linter/rules/no-duplicated-spread-props 
	 */
	noDuplicatedSpreadProps?: NoDuplicatedSpreadPropsConfiguration;
	/**
	* Require the use of === or !== for comparison with null.
See https://biomejs.dev/linter/rules/no-equals-to-null 
	 */
	noEqualsToNull?: NoEqualsToNullConfiguration;
	/**
	* Enforce a maximum number of classes per file.
See https://biomejs.dev/linter/rules/no-excessive-classes-per-file 
	 */
	noExcessiveClassesPerFile?: NoExcessiveClassesPerFileConfiguration;
	/**
	* Restrict the number of lines in a file.
See https://biomejs.dev/linter/rules/no-excessive-lines-per-file 
	 */
	noExcessiveLinesPerFile?: NoExcessiveLinesPerFileConfiguration;
	/**
	* Disallow new operators outside of assignments or comparisons.
See https://biomejs.dev/linter/rules/no-floating-classes 
	 */
	noFloatingClasses?: NoFloatingClassesConfiguration;
	/**
	* Require Promise-like statements to be handled appropriately.
See https://biomejs.dev/linter/rules/no-floating-promises 
	 */
	noFloatingPromises?: NoFloatingPromisesConfiguration;
	/**
	* Disallow iterating using a for-in loop.
See https://biomejs.dev/linter/rules/no-for-in 
	 */
	noForIn?: NoForInConfiguration;
	/**
	* Disallow hex colors.
See https://biomejs.dev/linter/rules/no-hex-colors 
	 */
	noHexColors?: NoHexColorsConfiguration;
	/**
	* Disallows the usage of the unary operators ++ and --.
See https://biomejs.dev/linter/rules/no-increment-decrement 
	 */
	noIncrementDecrement?: NoIncrementDecrementConfiguration;
	/**
	* Disallow .bind(), arrow functions, or function expressions in JSX props.
See https://biomejs.dev/linter/rules/no-jsx-props-bind 
	 */
	noJsxPropsBind?: NoJsxPropsBindConfiguration;
	/**
	* Prevent problematic leaked values from being rendered.
See https://biomejs.dev/linter/rules/no-leaked-render 
	 */
	noLeakedRender?: NoLeakedRenderConfiguration;
	/**
	* Disallow Promises to be used in places where they are almost certainly a mistake.
See https://biomejs.dev/linter/rules/no-misused-promises 
	 */
	noMisusedPromises?: NoMisusedPromisesConfiguration;
	/**
	* Disallow use of chained assignment expressions.
See https://biomejs.dev/linter/rules/no-multi-assign 
	 */
	noMultiAssign?: NoMultiAssignConfiguration;
	/**
	* Disallow creating multiline strings by escaping newlines.
See https://biomejs.dev/linter/rules/no-multi-str 
	 */
	noMultiStr?: NoMultiStrConfiguration;
	/**
	* Disallow nested .then() or .catch() promise calls.
See https://biomejs.dev/linter/rules/no-nested-promises 
	 */
	noNestedPromises?: NoNestedPromisesConfiguration;
	/**
	* Disallow function parameters that are only used in recursive calls.
See https://biomejs.dev/linter/rules/no-parameters-only-used-in-recursion 
	 */
	noParametersOnlyUsedInRecursion?: NoParametersOnlyUsedInRecursionConfiguration;
	/**
	* Disallow usage of element handles (page.$() and page.$$()).
See https://biomejs.dev/linter/rules/no-playwright-element-handle 
	 */
	noPlaywrightElementHandle?: NoPlaywrightElementHandleConfiguration;
	/**
	* Disallow usage of page.$eval() and page.$$eval().
See https://biomejs.dev/linter/rules/no-playwright-eval 
	 */
	noPlaywrightEval?: NoPlaywrightEvalConfiguration;
	/**
	* Disallow usage of the { force: true } option.
See https://biomejs.dev/linter/rules/no-playwright-force-option 
	 */
	noPlaywrightForceOption?: NoPlaywrightForceOptionConfiguration;
	/**
	* Enforce Playwright async APIs to be awaited or returned.
See https://biomejs.dev/linter/rules/no-playwright-missing-await 
	 */
	noPlaywrightMissingAwait?: NoPlaywrightMissingAwaitConfiguration;
	/**
	* Disallow usage of the networkidle option.
See https://biomejs.dev/linter/rules/no-playwright-networkidle 
	 */
	noPlaywrightNetworkidle?: NoPlaywrightNetworkidleConfiguration;
	/**
	* Disallow using page.pause().
See https://biomejs.dev/linter/rules/no-playwright-page-pause 
	 */
	noPlaywrightPagePause?: NoPlaywrightPagePauseConfiguration;
	/**
	* Disallow unnecessary await for Playwright methods that don't return promises.
See https://biomejs.dev/linter/rules/no-playwright-useless-await 
	 */
	noPlaywrightUselessAwait?: NoPlaywrightUselessAwaitConfiguration;
	/**
	* Disallow using page.waitForNavigation().
See https://biomejs.dev/linter/rules/no-playwright-wait-for-navigation 
	 */
	noPlaywrightWaitForNavigation?: NoPlaywrightWaitForNavigationConfiguration;
	/**
	* Disallow using page.waitForSelector().
See https://biomejs.dev/linter/rules/no-playwright-wait-for-selector 
	 */
	noPlaywrightWaitForSelector?: NoPlaywrightWaitForSelectorConfiguration;
	/**
	* Disallow using page.waitForTimeout().
See https://biomejs.dev/linter/rules/no-playwright-wait-for-timeout 
	 */
	noPlaywrightWaitForTimeout?: NoPlaywrightWaitForTimeoutConfiguration;
	/**
	* Disallow the use of the deprecated __proto__ object property.
See https://biomejs.dev/linter/rules/no-proto 
	 */
	noProto?: NoProtoConfiguration;
	/**
	* Checks if a default export exports the same symbol as a named export.
See https://biomejs.dev/linter/rules/no-redundant-default-export 
	 */
	noRedundantDefaultExport?: NoRedundantDefaultExportConfiguration;
	/**
	* Disallow assignments in return statements.
See https://biomejs.dev/linter/rules/no-return-assign 
	 */
	noReturnAssign?: NoReturnAssignConfiguration;
	/**
	* Disallow the usage of specified root types.
See https://biomejs.dev/linter/rules/no-root-type 
	 */
	noRootType?: NoRootTypeConfiguration;
	/**
	* Disallow javascript: URLs in HTML.
See https://biomejs.dev/linter/rules/no-script-url 
	 */
	noScriptUrl?: NoScriptUrlConfiguration;
	/**
	* Disallow variable declarations from shadowing variables declared in the outer scope.
See https://biomejs.dev/linter/rules/no-shadow 
	 */
	noShadow?: NoShadowConfiguration;
	/**
	* Prevent the usage of synchronous scripts.
See https://biomejs.dev/linter/rules/no-sync-scripts 
	 */
	noSyncScripts?: NoSyncScriptsConfiguration;
	/**
	* Disallow ternary operators.
See https://biomejs.dev/linter/rules/no-ternary 
	 */
	noTernary?: NoTernaryConfiguration;
	/**
	* Disallow the use of undeclared environment variables.
See https://biomejs.dev/linter/rules/no-undeclared-env-vars 
	 */
	noUndeclaredEnvVars?: NoUndeclaredEnvVarsConfiguration;
	/**
	* Disallow unknown DOM properties.
See https://biomejs.dev/linter/rules/no-unknown-attribute 
	 */
	noUnknownAttribute?: NoUnknownAttributeConfiguration;
	/**
	* Disallow unnecessary type-based conditions that can be statically determined as redundant.
See https://biomejs.dev/linter/rules/no-unnecessary-conditions 
	 */
	noUnnecessaryConditions?: NoUnnecessaryConditionsConfiguration;
	/**
	* Disallow redundant return statements.
See https://biomejs.dev/linter/rules/no-useless-return 
	 */
	noUselessReturn?: NoUselessReturnConfiguration;
	/**
	* Disallows using arrow functions when defining a watcher.
See https://biomejs.dev/linter/rules/no-vue-arrow-func-in-watch 
	 */
	noVueArrowFuncInWatch?: NoVueArrowFuncInWatchConfiguration;
	/**
	* Disallow the use of Vue Options API.
See https://biomejs.dev/linter/rules/no-vue-options-api 
	 */
	noVueOptionsApi?: NoVueOptionsApiConfiguration;
	/**
	* Disallow the use of value wrapped by ref()(Composition API) as operand.
See https://biomejs.dev/linter/rules/no-vue-ref-as-operand 
	 */
	noVueRefAsOperand?: NoVueRefAsOperandConfiguration;
	/**
	* Disallow using v-if and v-for directives on the same element.
See https://biomejs.dev/linter/rules/no-vue-v-if-with-v-for 
	 */
	noVueVIfWithVFor?: NoVueVIfWithVForConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Prefer Array.prototype.some() over verbose existence checks.
See https://biomejs.dev/linter/rules/use-array-some 
	 */
	useArraySome?: UseArraySomeConfiguration;
	/**
	* Require Array#sort and Array#toSorted calls to always provide a compareFunction.
See https://biomejs.dev/linter/rules/use-array-sort-compare 
	 */
	useArraySortCompare?: UseArraySortCompareConfiguration;
	/**
	* Enforce that await is only used on Promise values.
See https://biomejs.dev/linter/rules/use-await-thenable 
	 */
	useAwaitThenable?: UseAwaitThenableConfiguration;
	/**
	* Disallow enums from having both number and string members.
See https://biomejs.dev/linter/rules/use-consistent-enum-value-type 
	 */
	useConsistentEnumValueType?: UseConsistentEnumValueTypeConfiguration;
	/**
	* Require all descriptions to follow the same style (either block or inline) to  maintain consistency and improve readability across the schema.
See https://biomejs.dev/linter/rules/use-consistent-graphql-descriptions 
	 */
	useConsistentGraphqlDescriptions?: UseConsistentGraphqlDescriptionsConfiguration;
	/**
	* Enforce consistent use of either method signatures or function properties within interfaces and type aliases.
See https://biomejs.dev/linter/rules/use-consistent-method-signatures 
	 */
	useConsistentMethodSignatures?: UseConsistentMethodSignaturesConfiguration;
	/**
	* Require destructuring from arrays and/or objects.
See https://biomejs.dev/linter/rules/use-destructuring 
	 */
	useDestructuring?: UseDestructuringConfiguration;
	/**
	* Enforce that new Error() is thrown with the original error as cause.
See https://biomejs.dev/linter/rules/use-error-cause 
	 */
	useErrorCause?: UseErrorCauseConfiguration;
	/**
	* Require switch-case statements to be exhaustive.
See https://biomejs.dev/linter/rules/use-exhaustive-switch-cases 
	 */
	useExhaustiveSwitchCases?: UseExhaustiveSwitchCasesConfiguration;
	/**
	* Ensure that test functions contain at least one expect() or similar assertion.
See https://biomejs.dev/linter/rules/use-expect 
	 */
	useExpect?: UseExpectConfiguration;
	/**
	* Enforce types in functions, methods, variables, and parameters.
See https://biomejs.dev/linter/rules/use-explicit-type 
	 */
	useExplicitType?: UseExplicitTypeConfiguration;
	/**
	* Enforce the use of Array.prototype.find() over Array.prototype.filter() followed by [0] when looking for a single result.
See https://biomejs.dev/linter/rules/use-find 
	 */
	useFind?: UseFindConfiguration;
	/**
	* Enforce the use of globalThis over window, self, and global.
See https://biomejs.dev/linter/rules/use-global-this 
	 */
	useGlobalThis?: UseGlobalThisConfiguration;
	/**
	* Enforce id attribute on next/script components with inline content or dangerouslySetInnerHTML.
See https://biomejs.dev/linter/rules/use-inline-script-id 
	 */
	useInlineScriptId?: UseInlineScriptIdConfiguration;
	/**
	* Require mutation argument to be always called "input".
See https://biomejs.dev/linter/rules/use-input-name 
	 */
	useInputName?: UseInputNameConfiguration;
	/**
	* Disallow anonymous operations when more than one operation specified in document.
See https://biomejs.dev/linter/rules/use-lone-anonymous-operation 
	 */
	useLoneAnonymousOperation?: UseLoneAnonymousOperationConfiguration;
	/**
	* Require queries, mutations, subscriptions or fragments each to be located in separate files.
See https://biomejs.dev/linter/rules/use-lone-executable-definition 
	 */
	useLoneExecutableDefinition?: UseLoneExecutableDefinitionConfiguration;
	/**
	* Enforce using nullish coalescing operator (??) instead of logical or (||).
See https://biomejs.dev/linter/rules/use-nullish-coalescing 
	 */
	useNullishCoalescing?: UseNullishCoalescingConfiguration;
	/**
	* Enforce valid describe() callback.
See https://biomejs.dev/linter/rules/use-playwright-valid-describe-callback 
	 */
	usePlaywrightValidDescribeCallback?: UsePlaywrightValidDescribeCallbackConfiguration;
	/**
	* Enforce RegExp#exec over String#match if no global flag is provided.
See https://biomejs.dev/linter/rules/use-regexp-exec 
	 */
	useRegexpExec?: UseRegexpExecConfiguration;
	/**
	* Enforce the presence of required scripts in package.json.
See https://biomejs.dev/linter/rules/use-required-scripts 
	 */
	useRequiredScripts?: UseRequiredScriptsConfiguration;
	/**
	* Enforce the sorting of CSS utility classes.
See https://biomejs.dev/linter/rules/use-sorted-classes 
	 */
	useSortedClasses?: UseSortedClassesConfiguration;
	/**
	* Enforce the use of the spread operator over .apply().
See https://biomejs.dev/linter/rules/use-spread 
	 */
	useSpread?: UseSpreadConfiguration;
	/**
	* Enforce consistent defineProps declaration style.
See https://biomejs.dev/linter/rules/use-vue-consistent-define-props-declaration 
	 */
	useVueConsistentDefinePropsDeclaration?: UseVueConsistentDefinePropsDeclarationConfiguration;
	/**
	* Enforce a consistent style for v-bind in Vue templates.
See https://biomejs.dev/linter/rules/use-vue-consistent-v-bind-style 
	 */
	useVueConsistentVBindStyle?: UseVueConsistentVBindStyleConfiguration;
	/**
	* Enforce a consistent style for v-on in Vue templates.
See https://biomejs.dev/linter/rules/use-vue-consistent-v-on-style 
	 */
	useVueConsistentVOnStyle?: UseVueConsistentVOnStyleConfiguration;
	/**
	* Enforce specific order of Vue compiler macros.
See https://biomejs.dev/linter/rules/use-vue-define-macros-order 
	 */
	useVueDefineMacrosOrder?: UseVueDefineMacrosOrderConfiguration;
	/**
	* Enforce hyphenated (kebab-case) attribute names in Vue templates.
See https://biomejs.dev/linter/rules/use-vue-hyphenated-attributes 
	 */
	useVueHyphenatedAttributes?: UseVueHyphenatedAttributesConfiguration;
	/**
	* Enforce multi-word component names in Vue components.
See https://biomejs.dev/linter/rules/use-vue-multi-word-component-names 
	 */
	useVueMultiWordComponentNames?: UseVueMultiWordComponentNamesConfiguration;
	/**
	* Enforce that elements using v-for also specify a unique key.
See https://biomejs.dev/linter/rules/use-vue-v-for-key 
	 */
	useVueVForKey?: UseVueVForKeyConfiguration;
	/**
	* Enforce valid Vue \<template> root usage.
See https://biomejs.dev/linter/rules/use-vue-valid-template-root 
	 */
	useVueValidTemplateRoot?: UseVueValidTemplateRootConfiguration;
	/**
	* Forbids v-bind directives with missing arguments or invalid modifiers.
See https://biomejs.dev/linter/rules/use-vue-valid-v-bind 
	 */
	useVueValidVBind?: UseVueValidVBindConfiguration;
	/**
	* Enforce valid v-cloak Vue directives.
See https://biomejs.dev/linter/rules/use-vue-valid-v-cloak 
	 */
	useVueValidVCloak?: UseVueValidVCloakConfiguration;
	/**
	* Enforce valid usage of v-else.
See https://biomejs.dev/linter/rules/use-vue-valid-v-else 
	 */
	useVueValidVElse?: UseVueValidVElseConfiguration;
	/**
	* Enforce valid v-else-if directives.
See https://biomejs.dev/linter/rules/use-vue-valid-v-else-if 
	 */
	useVueValidVElseIf?: UseVueValidVElseIfConfiguration;
	/**
	* Enforce valid v-html directives.
See https://biomejs.dev/linter/rules/use-vue-valid-v-html 
	 */
	useVueValidVHtml?: UseVueValidVHtmlConfiguration;
	/**
	* Enforces valid v-if usage for Vue templates.
See https://biomejs.dev/linter/rules/use-vue-valid-v-if 
	 */
	useVueValidVIf?: UseVueValidVIfConfiguration;
	/**
	* Enforce valid v-on directives with proper arguments, modifiers, and handlers.
See https://biomejs.dev/linter/rules/use-vue-valid-v-on 
	 */
	useVueValidVOn?: UseVueValidVOnConfiguration;
	/**
	* Enforce valid v-once Vue directives.
See https://biomejs.dev/linter/rules/use-vue-valid-v-once 
	 */
	useVueValidVOnce?: UseVueValidVOnceConfiguration;
	/**
	* Enforce valid v-pre Vue directives.
See https://biomejs.dev/linter/rules/use-vue-valid-v-pre 
	 */
	useVueValidVPre?: UseVueValidVPreConfiguration;
	/**
	* Enforce valid v-text Vue directives.
See https://biomejs.dev/linter/rules/use-vue-valid-v-text 
	 */
	useVueValidVText?: UseVueValidVTextConfiguration;
	/**
	* Enforce opting in to Vue Vapor mode in \<script setup> blocks.
See https://biomejs.dev/linter/rules/use-vue-vapor 
	 */
	useVueVapor?: UseVueVaporConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Performance {
	/**
	* Disallow the use of spread (...) syntax on accumulators.
See https://biomejs.dev/linter/rules/no-accumulating-spread 
	 */
	noAccumulatingSpread?: NoAccumulatingSpreadConfiguration;
	/**
	* Disallow await inside loops.
See https://biomejs.dev/linter/rules/no-await-in-loops 
	 */
	noAwaitInLoops?: NoAwaitInLoopsConfiguration;
	/**
	* Disallow the use of barrel file.
See https://biomejs.dev/linter/rules/no-barrel-file 
	 */
	noBarrelFile?: NoBarrelFileConfiguration;
	/**
	* Disallow the use of the delete operator.
See https://biomejs.dev/linter/rules/no-delete 
	 */
	noDelete?: NoDeleteConfiguration;
	/**
	* Disallow accessing namespace imports dynamically.
See https://biomejs.dev/linter/rules/no-dynamic-namespace-import-access 
	 */
	noDynamicNamespaceImportAccess?: NoDynamicNamespaceImportAccessConfiguration;
	/**
	* Prevent usage of \<img> element in a Next.js project.
See https://biomejs.dev/linter/rules/no-img-element 
	 */
	noImgElement?: NoImgElementConfiguration;
	/**
	* Disallow the use of namespace imports.
See https://biomejs.dev/linter/rules/no-namespace-import 
	 */
	noNamespaceImport?: NoNamespaceImportConfiguration;
	/**
	* Avoid re-export all.
See https://biomejs.dev/linter/rules/no-re-export-all 
	 */
	noReExportAll?: NoReExportAllConfiguration;
	/**
	* Prevent duplicate polyfills from Polyfill.io.
See https://biomejs.dev/linter/rules/no-unwanted-polyfillio 
	 */
	noUnwantedPolyfillio?: NoUnwantedPolyfillioConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Ensure the preconnect attribute is used when using Google Fonts.
See https://biomejs.dev/linter/rules/use-google-font-preconnect 
	 */
	useGoogleFontPreconnect?: UseGoogleFontPreconnectConfiguration;
	/**
	* Enforce using Solid's \<For /> component for mapping an array to JSX elements.
See https://biomejs.dev/linter/rules/use-solid-for-component 
	 */
	useSolidForComponent?: UseSolidForComponentConfiguration;
	/**
	* Require regex literals to be declared at the top level.
See https://biomejs.dev/linter/rules/use-top-level-regex 
	 */
	useTopLevelRegex?: UseTopLevelRegexConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Security {
	/**
	* Disallow target="_blank" attribute without rel="noopener".
See https://biomejs.dev/linter/rules/no-blank-target 
	 */
	noBlankTarget?: NoBlankTargetConfiguration;
	/**
	* Prevent the usage of dangerous JSX props.
See https://biomejs.dev/linter/rules/no-dangerously-set-inner-html 
	 */
	noDangerouslySetInnerHtml?: NoDangerouslySetInnerHtmlConfiguration;
	/**
	* Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop.
See https://biomejs.dev/linter/rules/no-dangerously-set-inner-html-with-children 
	 */
	noDangerouslySetInnerHtmlWithChildren?: NoDangerouslySetInnerHtmlWithChildrenConfiguration;
	/**
	* Disallow the use of global eval().
See https://biomejs.dev/linter/rules/no-global-eval 
	 */
	noGlobalEval?: NoGlobalEvalConfiguration;
	/**
	* Disallow usage of sensitive data such as API keys and tokens.
See https://biomejs.dev/linter/rules/no-secrets 
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
See https://biomejs.dev/linter/rules/no-common-js 
	 */
	noCommonJs?: NoCommonJsConfiguration;
	/**
	* Disallow default exports.
See https://biomejs.dev/linter/rules/no-default-export 
	 */
	noDefaultExport?: NoDefaultExportConfiguration;
	/**
	* Disallow a lower specificity selector from coming after a higher specificity selector.
See https://biomejs.dev/linter/rules/no-descending-specificity 
	 */
	noDescendingSpecificity?: NoDescendingSpecificityConfiguration;
	/**
	* Disallow using a callback in asynchronous tests and hooks.
See https://biomejs.dev/linter/rules/no-done-callback 
	 */
	noDoneCallback?: NoDoneCallbackConfiguration;
	/**
	* Disallow TypeScript enum.
See https://biomejs.dev/linter/rules/no-enum 
	 */
	noEnum?: NoEnumConfiguration;
	/**
	* Disallow exporting an imported variable.
See https://biomejs.dev/linter/rules/no-exported-imports 
	 */
	noExportedImports?: NoExportedImportsConfiguration;
	/**
	* Prevent usage of \<head> element in a Next.js project.
See https://biomejs.dev/linter/rules/no-head-element 
	 */
	noHeadElement?: NoHeadElementConfiguration;
	/**
	* Disallow implicit true values on JSX boolean attributes.
See https://biomejs.dev/linter/rules/no-implicit-boolean 
	 */
	noImplicitBoolean?: NoImplicitBooleanConfiguration;
	/**
	* Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
See https://biomejs.dev/linter/rules/no-inferrable-types 
	 */
	noInferrableTypes?: NoInferrableTypesConfiguration;
	/**
	* Disallow string literals inside JSX elements.
See https://biomejs.dev/linter/rules/no-jsx-literals 
	 */
	noJsxLiterals?: NoJsxLiteralsConfiguration;
	/**
	* Reports usage of "magic numbers" — numbers used directly instead of being assigned to named constants.
See https://biomejs.dev/linter/rules/no-magic-numbers 
	 */
	noMagicNumbers?: NoMagicNumbersConfiguration;
	/**
	* Disallow the use of TypeScript's namespaces.
See https://biomejs.dev/linter/rules/no-namespace 
	 */
	noNamespace?: NoNamespaceConfiguration;
	/**
	* Disallow negation in the condition of an if statement if it has an else clause.
See https://biomejs.dev/linter/rules/no-negation-else 
	 */
	noNegationElse?: NoNegationElseConfiguration;
	/**
	* Disallow nested ternary expressions.
See https://biomejs.dev/linter/rules/no-nested-ternary 
	 */
	noNestedTernary?: NoNestedTernaryConfiguration;
	/**
	* Disallow non-null assertions using the ! postfix operator.
See https://biomejs.dev/linter/rules/no-non-null-assertion 
	 */
	noNonNullAssertion?: NoNonNullAssertionConfiguration;
	/**
	* Disallow reassigning function parameters.
See https://biomejs.dev/linter/rules/no-parameter-assign 
	 */
	noParameterAssign?: NoParameterAssignConfiguration;
	/**
	* Disallow the use of parameter properties in class constructors.
See https://biomejs.dev/linter/rules/no-parameter-properties 
	 */
	noParameterProperties?: NoParameterPropertiesConfiguration;
	/**
	* Disallow the use of process.env.
See https://biomejs.dev/linter/rules/no-process-env 
	 */
	noProcessEnv?: NoProcessEnvConfiguration;
	/**
	* This rule allows you to specify global variable names that you don’t want to use in your application.
See https://biomejs.dev/linter/rules/no-restricted-globals 
	 */
	noRestrictedGlobals?: NoRestrictedGlobalsConfiguration;
	/**
	* Disallow specified modules when loaded by import or require.
See https://biomejs.dev/linter/rules/no-restricted-imports 
	 */
	noRestrictedImports?: NoRestrictedImportsConfiguration;
	/**
	* Disallow user defined types.
See https://biomejs.dev/linter/rules/no-restricted-types 
	 */
	noRestrictedTypes?: NoRestrictedTypesConfiguration;
	/**
	* Disallow the use of constants which its value is the upper-case version of its name.
See https://biomejs.dev/linter/rules/no-shouty-constants 
	 */
	noShoutyConstants?: NoShoutyConstantsConfiguration;
	/**
	* Enforce the use of String.slice() over String.substr() and String.substring().
See https://biomejs.dev/linter/rules/no-substr 
	 */
	noSubstr?: NoSubstrConfiguration;
	/**
	* Disallow template literals if interpolation and special-character handling are not needed.
See https://biomejs.dev/linter/rules/no-unused-template-literal 
	 */
	noUnusedTemplateLiteral?: NoUnusedTemplateLiteralConfiguration;
	/**
	* Disallow else block when the if block breaks early.
See https://biomejs.dev/linter/rules/no-useless-else 
	 */
	noUselessElse?: NoUselessElseConfiguration;
	/**
	* Disallow use of @value rule in CSS modules.
See https://biomejs.dev/linter/rules/no-value-at-rule 
	 */
	noValueAtRule?: NoValueAtRuleConfiguration;
	/**
	* Disallow the use of yoda expressions.
See https://biomejs.dev/linter/rules/no-yoda-expression 
	 */
	noYodaExpression?: NoYodaExpressionConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Disallow Array constructors.
See https://biomejs.dev/linter/rules/use-array-literals 
	 */
	useArrayLiterals?: UseArrayLiteralsConfiguration;
	/**
	* Enforce the use of as const over literal type and type annotation.
See https://biomejs.dev/linter/rules/use-as-const-assertion 
	 */
	useAsConstAssertion?: UseAsConstAssertionConfiguration;
	/**
	* Use at() instead of integer index access.
See https://biomejs.dev/linter/rules/use-at-index 
	 */
	useAtIndex?: UseAtIndexConfiguration;
	/**
	* Requires following curly brace conventions.
See https://biomejs.dev/linter/rules/use-block-statements 
	 */
	useBlockStatements?: UseBlockStatementsConfiguration;
	/**
	* Enforce using else if instead of nested if in else clauses.
See https://biomejs.dev/linter/rules/use-collapsed-else-if 
	 */
	useCollapsedElseIf?: UseCollapsedElseIfConfiguration;
	/**
	* Enforce using single if instead of nested if clauses.
See https://biomejs.dev/linter/rules/use-collapsed-if 
	 */
	useCollapsedIf?: UseCollapsedIfConfiguration;
	/**
	* Enforce declaring components only within modules that export React Components exclusively.
See https://biomejs.dev/linter/rules/use-component-export-only-modules 
	 */
	useComponentExportOnlyModules?: UseComponentExportOnlyModulesConfiguration;
	/**
	* Require consistently using either T\[] or Array\<T>.
See https://biomejs.dev/linter/rules/use-consistent-array-type 
	 */
	useConsistentArrayType?: UseConsistentArrayTypeConfiguration;
	/**
	* Enforce consistent arrow function bodies.
See https://biomejs.dev/linter/rules/use-consistent-arrow-return 
	 */
	useConsistentArrowReturn?: UseConsistentArrowReturnConfiguration;
	/**
	* Enforce the use of new for all builtins, except String, Number and Boolean.
See https://biomejs.dev/linter/rules/use-consistent-builtin-instantiation 
	 */
	useConsistentBuiltinInstantiation?: UseConsistentBuiltinInstantiationConfiguration;
	/**
	* This rule enforces consistent use of curly braces inside JSX attributes and JSX children.
See https://biomejs.dev/linter/rules/use-consistent-curly-braces 
	 */
	useConsistentCurlyBraces?: UseConsistentCurlyBracesConfiguration;
	/**
	* Require consistent accessibility modifiers on class properties and methods.
See https://biomejs.dev/linter/rules/use-consistent-member-accessibility 
	 */
	useConsistentMemberAccessibility?: UseConsistentMemberAccessibilityConfiguration;
	/**
	* Require the consistent declaration of object literals. Defaults to explicit definitions.
See https://biomejs.dev/linter/rules/use-consistent-object-definitions 
	 */
	useConsistentObjectDefinitions?: UseConsistentObjectDefinitionsConfiguration;
	/**
	* Enforce type definitions to consistently use either interface or type.
See https://biomejs.dev/linter/rules/use-consistent-type-definitions 
	 */
	useConsistentTypeDefinitions?: UseConsistentTypeDefinitionsConfiguration;
	/**
	* Require const declarations for variables that are only assigned once.
See https://biomejs.dev/linter/rules/use-const 
	 */
	useConst?: UseConstConfiguration;
	/**
	* Enforce default function parameters and optional function parameters to be last.
See https://biomejs.dev/linter/rules/use-default-parameter-last 
	 */
	useDefaultParameterLast?: UseDefaultParameterLastConfiguration;
	/**
	* Require the default clause in switch statements.
See https://biomejs.dev/linter/rules/use-default-switch-clause 
	 */
	useDefaultSwitchClause?: UseDefaultSwitchClauseConfiguration;
	/**
	* Require specifying the reason argument when using @deprecated directive.
See https://biomejs.dev/linter/rules/use-deprecated-reason 
	 */
	useDeprecatedReason?: UseDeprecatedReasonConfiguration;
	/**
	* Require that each enum member value be explicitly initialized.
See https://biomejs.dev/linter/rules/use-enum-initializers 
	 */
	useEnumInitializers?: UseEnumInitializersConfiguration;
	/**
	* Enforce explicitly comparing the length, size, byteLength or byteOffset property of a value.
See https://biomejs.dev/linter/rules/use-explicit-length-check 
	 */
	useExplicitLengthCheck?: UseExplicitLengthCheckConfiguration;
	/**
	* Disallow the use of Math.pow in favor of the ** operator.
See https://biomejs.dev/linter/rules/use-exponentiation-operator 
	 */
	useExponentiationOperator?: UseExponentiationOperatorConfiguration;
	/**
	* Promotes the use of export type for types.
See https://biomejs.dev/linter/rules/use-export-type 
	 */
	useExportType?: UseExportTypeConfiguration;
	/**
	* Require that all exports are declared after all non-export statements.
See https://biomejs.dev/linter/rules/use-exports-last 
	 */
	useExportsLast?: UseExportsLastConfiguration;
	/**
	* Enforce naming conventions for JavaScript and TypeScript filenames.
See https://biomejs.dev/linter/rules/use-filenaming-convention 
	 */
	useFilenamingConvention?: UseFilenamingConventionConfiguration;
	/**
	* Prefer using for...of loops over standard for loops where possible.
See https://biomejs.dev/linter/rules/use-for-of 
	 */
	useForOf?: UseForOfConfiguration;
	/**
	* This rule enforces the use of \<>...\</> over \<Fragment>...\</Fragment>.
See https://biomejs.dev/linter/rules/use-fragment-syntax 
	 */
	useFragmentSyntax?: UseFragmentSyntaxConfiguration;
	/**
	* Validates that all enum values are capitalized.
See https://biomejs.dev/linter/rules/use-graphql-naming-convention 
	 */
	useGraphqlNamingConvention?: UseGraphqlNamingConventionConfiguration;
	/**
	* Enforce that getters and setters for the same property are adjacent in class and object definitions.
See https://biomejs.dev/linter/rules/use-grouped-accessor-pairs 
	 */
	useGroupedAccessorPairs?: UseGroupedAccessorPairsConfiguration;
	/**
	* Promotes the use of import type for types.
See https://biomejs.dev/linter/rules/use-import-type 
	 */
	useImportType?: UseImportTypeConfiguration;
	/**
	* Require all enum members to be literal values.
See https://biomejs.dev/linter/rules/use-literal-enum-members 
	 */
	useLiteralEnumMembers?: UseLiteralEnumMembersConfiguration;
	/**
	* Enforce naming conventions for everything across a codebase.
See https://biomejs.dev/linter/rules/use-naming-convention 
	 */
	useNamingConvention?: UseNamingConventionConfiguration;
	/**
	* Promotes the usage of node:assert/strict over node:assert.
See https://biomejs.dev/linter/rules/use-node-assert-strict 
	 */
	useNodeAssertStrict?: UseNodeAssertStrictConfiguration;
	/**
	* Enforces using the node: protocol for Node.js builtin modules.
See https://biomejs.dev/linter/rules/use-nodejs-import-protocol 
	 */
	useNodejsImportProtocol?: UseNodejsImportProtocolConfiguration;
	/**
	* Use the Number properties instead of global ones.
See https://biomejs.dev/linter/rules/use-number-namespace 
	 */
	useNumberNamespace?: UseNumberNamespaceConfiguration;
	/**
	* Enforce the use of numeric separators in numeric literals.
See https://biomejs.dev/linter/rules/use-numeric-separators 
	 */
	useNumericSeparators?: UseNumericSeparatorsConfiguration;
	/**
	* Prefer object spread over Object.assign() when constructing new objects.
See https://biomejs.dev/linter/rules/use-object-spread 
	 */
	useObjectSpread?: UseObjectSpreadConfiguration;
	/**
	* Enforce that components are defined as functions and never as classes.
See https://biomejs.dev/linter/rules/use-react-function-components 
	 */
	useReactFunctionComponents?: UseReactFunctionComponentsConfiguration;
	/**
	* Enforce marking members as readonly if they are never modified outside the constructor.
See https://biomejs.dev/linter/rules/use-readonly-class-properties 
	 */
	useReadonlyClassProperties?: UseReadonlyClassPropertiesConfiguration;
	/**
	* Prevent extra closing tags for components without children.
See https://biomejs.dev/linter/rules/use-self-closing-elements 
	 */
	useSelfClosingElements?: UseSelfClosingElementsConfiguration;
	/**
	* Require assignment operator shorthand where possible.
See https://biomejs.dev/linter/rules/use-shorthand-assign 
	 */
	useShorthandAssign?: UseShorthandAssignConfiguration;
	/**
	* Enforce using function types instead of object type with call signatures.
See https://biomejs.dev/linter/rules/use-shorthand-function-type 
	 */
	useShorthandFunctionType?: UseShorthandFunctionTypeConfiguration;
	/**
	* Disallow multiple variable declarations in the same variable statement.
See https://biomejs.dev/linter/rules/use-single-var-declarator 
	 */
	useSingleVarDeclarator?: UseSingleVarDeclaratorConfiguration;
	/**
	* Require a description parameter for the Symbol().
See https://biomejs.dev/linter/rules/use-symbol-description 
	 */
	useSymbolDescription?: UseSymbolDescriptionConfiguration;
	/**
	* Prefer template literals over string concatenation.
See https://biomejs.dev/linter/rules/use-template 
	 */
	useTemplate?: UseTemplateConfiguration;
	/**
	* Require new when throwing an error.
See https://biomejs.dev/linter/rules/use-throw-new-error 
	 */
	useThrowNewError?: UseThrowNewErrorConfiguration;
	/**
	* Disallow throwing non-Error values.
See https://biomejs.dev/linter/rules/use-throw-only-error 
	 */
	useThrowOnlyError?: UseThrowOnlyErrorConfiguration;
	/**
	* Enforce the use of String.trimStart() and String.trimEnd() over String.trimLeft() and String.trimRight().
See https://biomejs.dev/linter/rules/use-trim-start-end 
	 */
	useTrimStartEnd?: UseTrimStartEndConfiguration;
	/**
	* Disallow overload signatures that can be unified into a single signature.
See https://biomejs.dev/linter/rules/use-unified-type-signatures 
	 */
	useUnifiedTypeSignatures?: UseUnifiedTypeSignaturesConfiguration;
}
/**
 * A list of rules that belong to this group
 */
export interface Suspicious {
	/**
	* Disallow the use of alert, confirm, and prompt.
See https://biomejs.dev/linter/rules/no-alert 
	 */
	noAlert?: NoAlertConfiguration;
	/**
	* Use standard constants instead of approximated literals.
See https://biomejs.dev/linter/rules/no-approximative-numeric-constant 
	 */
	noApproximativeNumericConstant?: NoApproximativeNumericConstantConfiguration;
	/**
	* Discourage the usage of Array index in keys.
See https://biomejs.dev/linter/rules/no-array-index-key 
	 */
	noArrayIndexKey?: NoArrayIndexKeyConfiguration;
	/**
	* Disallow assignments in expressions.
See https://biomejs.dev/linter/rules/no-assign-in-expressions 
	 */
	noAssignInExpressions?: NoAssignInExpressionsConfiguration;
	/**
	* Disallows using an async function as a Promise executor.
See https://biomejs.dev/linter/rules/no-async-promise-executor 
	 */
	noAsyncPromiseExecutor?: NoAsyncPromiseExecutorConfiguration;
	/**
	* Prevents the misuse of glob patterns inside the files.includes field.
See https://biomejs.dev/linter/rules/no-biome-first-exception 
	 */
	noBiomeFirstException?: NoBiomeFirstExceptionConfiguration;
	/**
	* Disallow bitwise operators.
See https://biomejs.dev/linter/rules/no-bitwise-operators 
	 */
	noBitwiseOperators?: NoBitwiseOperatorsConfiguration;
	/**
	* Disallow reassigning exceptions in catch clauses.
See https://biomejs.dev/linter/rules/no-catch-assign 
	 */
	noCatchAssign?: NoCatchAssignConfiguration;
	/**
	* Disallow reassigning class members.
See https://biomejs.dev/linter/rules/no-class-assign 
	 */
	noClassAssign?: NoClassAssignConfiguration;
	/**
	* Prevent comments from being inserted as text nodes.
See https://biomejs.dev/linter/rules/no-comment-text 
	 */
	noCommentText?: NoCommentTextConfiguration;
	/**
	* Disallow comparing against -0.
See https://biomejs.dev/linter/rules/no-compare-neg-zero 
	 */
	noCompareNegZero?: NoCompareNegZeroConfiguration;
	/**
	* Disallow labeled statements that are not loops.
See https://biomejs.dev/linter/rules/no-confusing-labels 
	 */
	noConfusingLabels?: NoConfusingLabelsConfiguration;
	/**
	* Disallow void type outside of generic or return types.
See https://biomejs.dev/linter/rules/no-confusing-void-type 
	 */
	noConfusingVoidType?: NoConfusingVoidTypeConfiguration;
	/**
	* Disallow the use of console.
See https://biomejs.dev/linter/rules/no-console 
	 */
	noConsole?: NoConsoleConfiguration;
	/**
	* Disallow TypeScript const enum.
See https://biomejs.dev/linter/rules/no-const-enum 
	 */
	noConstEnum?: NoConstEnumConfiguration;
	/**
	* Disallow expressions where the operation doesn't affect the value.
See https://biomejs.dev/linter/rules/no-constant-binary-expressions 
	 */
	noConstantBinaryExpressions?: NoConstantBinaryExpressionsConfiguration;
	/**
	* Prevents from having control characters and some escape sequences that match control characters in regular expression literals.
See https://biomejs.dev/linter/rules/no-control-characters-in-regex 
	 */
	noControlCharactersInRegex?: NoControlCharactersInRegexConfiguration;
	/**
	* Disallow the use of debugger.
See https://biomejs.dev/linter/rules/no-debugger 
	 */
	noDebugger?: NoDebuggerConfiguration;
	/**
	* Restrict imports of deprecated exports.
See https://biomejs.dev/linter/rules/no-deprecated-imports 
	 */
	noDeprecatedImports?: NoDeprecatedImportsConfiguration;
	/**
	* Disallow direct assignments to document.cookie.
See https://biomejs.dev/linter/rules/no-document-cookie 
	 */
	noDocumentCookie?: NoDocumentCookieConfiguration;
	/**
	* Prevents importing next/document outside of pages/_document.jsx in Next.js projects.
See https://biomejs.dev/linter/rules/no-document-import-in-page 
	 */
	noDocumentImportInPage?: NoDocumentImportInPageConfiguration;
	/**
	* Require the use of === and !==.
See https://biomejs.dev/linter/rules/no-double-equals 
	 */
	noDoubleEquals?: NoDoubleEqualsConfiguration;
	/**
	* Disallow duplicate @import rules.
See https://biomejs.dev/linter/rules/no-duplicate-at-import-rules 
	 */
	noDuplicateAtImportRules?: NoDuplicateAtImportRulesConfiguration;
	/**
	* Disallow duplicate case labels.
See https://biomejs.dev/linter/rules/no-duplicate-case 
	 */
	noDuplicateCase?: NoDuplicateCaseConfiguration;
	/**
	* Disallow duplicate class members.
See https://biomejs.dev/linter/rules/no-duplicate-class-members 
	 */
	noDuplicateClassMembers?: NoDuplicateClassMembersConfiguration;
	/**
	* Disallow duplicate custom properties within declaration blocks.
See https://biomejs.dev/linter/rules/no-duplicate-custom-properties 
	 */
	noDuplicateCustomProperties?: NoDuplicateCustomPropertiesConfiguration;
	/**
	* Prevent the listing of duplicate dependencies. The rule supports the following dependency groups: "bundledDependencies", "bundleDependencies", "dependencies", "devDependencies", "overrides", "optionalDependencies", and "peerDependencies".
See https://biomejs.dev/linter/rules/no-duplicate-dependencies 
	 */
	noDuplicateDependencies?: NoDuplicateDependenciesConfiguration;
	/**
	* Disallow duplicate conditions in if-else-if chains.
See https://biomejs.dev/linter/rules/no-duplicate-else-if 
	 */
	noDuplicateElseIf?: NoDuplicateElseIfConfiguration;
	/**
	* No duplicated fields in GraphQL operations.
See https://biomejs.dev/linter/rules/no-duplicate-fields 
	 */
	noDuplicateFields?: NoDuplicateFieldsConfiguration;
	/**
	* Disallow duplicate names within font families.
See https://biomejs.dev/linter/rules/no-duplicate-font-names 
	 */
	noDuplicateFontNames?: NoDuplicateFontNamesConfiguration;
	/**
	* Prevents JSX properties to be assigned multiple times.
See https://biomejs.dev/linter/rules/no-duplicate-jsx-props 
	 */
	noDuplicateJsxProps?: NoDuplicateJsxPropsConfiguration;
	/**
	* Disallow two keys with the same name inside objects.
See https://biomejs.dev/linter/rules/no-duplicate-object-keys 
	 */
	noDuplicateObjectKeys?: NoDuplicateObjectKeysConfiguration;
	/**
	* Disallow duplicate function parameter name.
See https://biomejs.dev/linter/rules/no-duplicate-parameters 
	 */
	noDuplicateParameters?: NoDuplicateParametersConfiguration;
	/**
	* Disallow duplicate properties within declaration blocks.
See https://biomejs.dev/linter/rules/no-duplicate-properties 
	 */
	noDuplicateProperties?: NoDuplicatePropertiesConfiguration;
	/**
	* Disallow duplicate selectors within keyframe blocks.
See https://biomejs.dev/linter/rules/no-duplicate-selectors-keyframe-block 
	 */
	noDuplicateSelectorsKeyframeBlock?: NoDuplicateSelectorsKeyframeBlockConfiguration;
	/**
	* A describe block should not contain duplicate hooks.
See https://biomejs.dev/linter/rules/no-duplicate-test-hooks 
	 */
	noDuplicateTestHooks?: NoDuplicateTestHooksConfiguration;
	/**
	* Disallow CSS empty blocks.
See https://biomejs.dev/linter/rules/no-empty-block 
	 */
	noEmptyBlock?: NoEmptyBlockConfiguration;
	/**
	* Disallow empty block statements and static blocks.
See https://biomejs.dev/linter/rules/no-empty-block-statements 
	 */
	noEmptyBlockStatements?: NoEmptyBlockStatementsConfiguration;
	/**
	* Disallow the declaration of empty interfaces.
See https://biomejs.dev/linter/rules/no-empty-interface 
	 */
	noEmptyInterface?: NoEmptyInterfaceConfiguration;
	/**
	* Disallow empty sources.
See https://biomejs.dev/linter/rules/no-empty-source 
	 */
	noEmptySource?: NoEmptySourceConfiguration;
	/**
	* Disallow variables from evolving into any type through reassignments.
See https://biomejs.dev/linter/rules/no-evolving-types 
	 */
	noEvolvingTypes?: NoEvolvingTypesConfiguration;
	/**
	* Disallow the any type usage.
See https://biomejs.dev/linter/rules/no-explicit-any 
	 */
	noExplicitAny?: NoExplicitAnyConfiguration;
	/**
	* Disallow using export or module.exports in files containing tests.
See https://biomejs.dev/linter/rules/no-exports-in-test 
	 */
	noExportsInTest?: NoExportsInTestConfiguration;
	/**
	* Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files.
See https://biomejs.dev/linter/rules/no-extra-non-null-assertion 
	 */
	noExtraNonNullAssertion?: NoExtraNonNullAssertionConfiguration;
	/**
	* Disallow fallthrough of switch clauses.
See https://biomejs.dev/linter/rules/no-fallthrough-switch-clause 
	 */
	noFallthroughSwitchClause?: NoFallthroughSwitchClauseConfiguration;
	/**
	* Disallow focused tests.
See https://biomejs.dev/linter/rules/no-focused-tests 
	 */
	noFocusedTests?: NoFocusedTestsConfiguration;
	/**
	* Disallow reassigning function declarations.
See https://biomejs.dev/linter/rules/no-function-assign 
	 */
	noFunctionAssign?: NoFunctionAssignConfiguration;
	/**
	* Disallow assignments to native objects and read-only global variables.
See https://biomejs.dev/linter/rules/no-global-assign 
	 */
	noGlobalAssign?: NoGlobalAssignConfiguration;
	/**
	* Use Number.isFinite instead of global isFinite.
See https://biomejs.dev/linter/rules/no-global-is-finite 
	 */
	noGlobalIsFinite?: NoGlobalIsFiniteConfiguration;
	/**
	* Use Number.isNaN instead of global isNaN.
See https://biomejs.dev/linter/rules/no-global-is-nan 
	 */
	noGlobalIsNan?: NoGlobalIsNanConfiguration;
	/**
	* Prevent using the next/head module in pages/_document.js on Next.js projects.
See https://biomejs.dev/linter/rules/no-head-import-in-document 
	 */
	noHeadImportInDocument?: NoHeadImportInDocumentConfiguration;
	/**
	* Disallow use of implicit any type on variable declarations.
See https://biomejs.dev/linter/rules/no-implicit-any-let 
	 */
	noImplicitAnyLet?: NoImplicitAnyLetConfiguration;
	/**
	* Disallow assigning to imported bindings.
See https://biomejs.dev/linter/rules/no-import-assign 
	 */
	noImportAssign?: NoImportAssignConfiguration;
	/**
	* Prevent import cycles.
See https://biomejs.dev/linter/rules/no-import-cycles 
	 */
	noImportCycles?: NoImportCyclesConfiguration;
	/**
	* Disallow invalid !important within keyframe declarations.
See https://biomejs.dev/linter/rules/no-important-in-keyframe 
	 */
	noImportantInKeyframe?: NoImportantInKeyframeConfiguration;
	/**
	* Disallows the use of irregular whitespace characters.
See https://biomejs.dev/linter/rules/no-irregular-whitespace 
	 */
	noIrregularWhitespace?: NoIrregularWhitespaceConfiguration;
	/**
	* Disallow labels that share a name with a variable.
See https://biomejs.dev/linter/rules/no-label-var 
	 */
	noLabelVar?: NoLabelVarConfiguration;
	/**
	* Disallow characters made with multiple code points in character class syntax.
See https://biomejs.dev/linter/rules/no-misleading-character-class 
	 */
	noMisleadingCharacterClass?: NoMisleadingCharacterClassConfiguration;
	/**
	* Enforce proper usage of new and constructor.
See https://biomejs.dev/linter/rules/no-misleading-instantiator 
	 */
	noMisleadingInstantiator?: NoMisleadingInstantiatorConfiguration;
	/**
	* Checks that the assertion function, for example expect, is placed inside an it() function call.
See https://biomejs.dev/linter/rules/no-misplaced-assertion 
	 */
	noMisplacedAssertion?: NoMisplacedAssertionConfiguration;
	/**
	* Disallow shorthand assign when variable appears on both sides.
See https://biomejs.dev/linter/rules/no-misrefactored-shorthand-assign 
	 */
	noMisrefactoredShorthandAssign?: NoMisrefactoredShorthandAssignConfiguration;
	/**
	* Disallow non-null assertions after optional chaining expressions.
See https://biomejs.dev/linter/rules/no-non-null-asserted-optional-chain 
	 */
	noNonNullAssertedOptionalChain?: NoNonNullAssertedOptionalChainConfiguration;
	/**
	* Disallow octal escape sequences in string literals.
See https://biomejs.dev/linter/rules/no-octal-escape 
	 */
	noOctalEscape?: NoOctalEscapeConfiguration;
	/**
	* Disallow direct use of Object.prototype builtins.
See https://biomejs.dev/linter/rules/no-prototype-builtins 
	 */
	noPrototypeBuiltins?: NoPrototypeBuiltinsConfiguration;
	/**
	* Disallow the use if quickfix.biome inside editor settings file.
See https://biomejs.dev/linter/rules/no-quickfix-biome 
	 */
	noQuickfixBiome?: NoQuickfixBiomeConfiguration;
	/**
	* Replaces usages of forwardRef with passing ref as a prop.
See https://biomejs.dev/linter/rules/no-react-forward-ref 
	 */
	noReactForwardRef?: NoReactForwardRefConfiguration;
	/**
	* Prevents React-specific JSX properties from being used.
See https://biomejs.dev/linter/rules/no-react-specific-props 
	 */
	noReactSpecificProps?: NoReactSpecificPropsConfiguration;
	/**
	* Disallow variable, function, class, and type redeclarations in the same scope.
See https://biomejs.dev/linter/rules/no-redeclare 
	 */
	noRedeclare?: NoRedeclareConfiguration;
	/**
	* Prevents from having redundant "use strict".
See https://biomejs.dev/linter/rules/no-redundant-use-strict 
	 */
	noRedundantUseStrict?: NoRedundantUseStrictConfiguration;
	/**
	* Disallow comparisons where both sides are exactly the same.
See https://biomejs.dev/linter/rules/no-self-compare 
	 */
	noSelfCompare?: NoSelfCompareConfiguration;
	/**
	* Disallow identifiers from shadowing restricted names.
See https://biomejs.dev/linter/rules/no-shadow-restricted-names 
	 */
	noShadowRestrictedNames?: NoShadowRestrictedNamesConfiguration;
	/**
	* Disallow shorthand properties that override related longhand properties.
See https://biomejs.dev/linter/rules/no-shorthand-property-overrides 
	 */
	noShorthandPropertyOverrides?: NoShorthandPropertyOverridesConfiguration;
	/**
	* Disallow disabled tests.
See https://biomejs.dev/linter/rules/no-skipped-tests 
	 */
	noSkippedTests?: NoSkippedTestsConfiguration;
	/**
	* Prevents the use of sparse arrays (arrays with holes).
See https://biomejs.dev/linter/rules/no-sparse-array 
	 */
	noSparseArray?: NoSparseArrayConfiguration;
	/**
	* It detects possible "wrong" semicolons inside JSX elements.
See https://biomejs.dev/linter/rules/no-suspicious-semicolon-in-jsx 
	 */
	noSuspiciousSemicolonInJsx?: NoSuspiciousSemicolonInJsxConfiguration;
	/**
	* Disallow template literal placeholder syntax in regular strings.
See https://biomejs.dev/linter/rules/no-template-curly-in-string 
	 */
	noTemplateCurlyInString?: NoTemplateCurlyInStringConfiguration;
	/**
	* Disallow then property.
See https://biomejs.dev/linter/rules/no-then-property 
	 */
	noThenProperty?: NoThenPropertyConfiguration;
	/**
	* Prevents the use of the TypeScript directive @ts-ignore.
See https://biomejs.dev/linter/rules/no-ts-ignore 
	 */
	noTsIgnore?: NoTsIgnoreConfiguration;
	/**
	* Disallow let or var variables that are read but never assigned.
See https://biomejs.dev/linter/rules/no-unassigned-variables 
	 */
	noUnassignedVariables?: NoUnassignedVariablesConfiguration;
	/**
	* Disallow unknown at-rules.
See https://biomejs.dev/linter/rules/no-unknown-at-rules 
	 */
	noUnknownAtRules?: NoUnknownAtRulesConfiguration;
	/**
	* Disallow unsafe declaration merging between interfaces and classes.
See https://biomejs.dev/linter/rules/no-unsafe-declaration-merging 
	 */
	noUnsafeDeclarationMerging?: NoUnsafeDeclarationMergingConfiguration;
	/**
	* Disallow using unsafe negation.
See https://biomejs.dev/linter/rules/no-unsafe-negation 
	 */
	noUnsafeNegation?: NoUnsafeNegationConfiguration;
	/**
	* Disallow expression statements that are neither a function call nor an assignment.
See https://biomejs.dev/linter/rules/no-unused-expressions 
	 */
	noUnusedExpressions?: NoUnusedExpressionsConfiguration;
	/**
	* Disallow unnecessary escapes in string literals.
See https://biomejs.dev/linter/rules/no-useless-escape-in-string 
	 */
	noUselessEscapeInString?: NoUselessEscapeInStringConfiguration;
	/**
	* Disallow useless backreferences in regular expression literals that always match an empty string.
See https://biomejs.dev/linter/rules/no-useless-regex-backrefs 
	 */
	noUselessRegexBackrefs?: NoUselessRegexBackrefsConfiguration;
	/**
	* Disallow the use of var.
See https://biomejs.dev/linter/rules/no-var 
	 */
	noVar?: NoVarConfiguration;
	/**
	* Disallow with statements in non-strict contexts.
See https://biomejs.dev/linter/rules/no-with 
	 */
	noWith?: NoWithConfiguration;
	/**
	 * Enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	* Disallow the use of overload signatures that are not next to each other.
See https://biomejs.dev/linter/rules/use-adjacent-overload-signatures 
	 */
	useAdjacentOverloadSignatures?: UseAdjacentOverloadSignaturesConfiguration;
	/**
	* Ensure async functions utilize await.
See https://biomejs.dev/linter/rules/use-await 
	 */
	useAwait?: UseAwaitConfiguration;
	/**
	* Promotes the correct usage for ignoring folders in the configuration file.
See https://biomejs.dev/linter/rules/use-biome-ignore-folder 
	 */
	useBiomeIgnoreFolder?: UseBiomeIgnoreFolderConfiguration;
	/**
	* Enforce default clauses in switch statements to be last.
See https://biomejs.dev/linter/rules/use-default-switch-clause-last 
	 */
	useDefaultSwitchClauseLast?: UseDefaultSwitchClauseLastConfiguration;
	/**
	* Require the @deprecated directive to specify a deletion date.
See https://biomejs.dev/linter/rules/use-deprecated-date 
	 */
	useDeprecatedDate?: UseDeprecatedDateConfiguration;
	/**
	* Enforce passing a message value when creating a built-in error.
See https://biomejs.dev/linter/rules/use-error-message 
	 */
	useErrorMessage?: UseErrorMessageConfiguration;
	/**
	* Enforce get methods to always return a value.
See https://biomejs.dev/linter/rules/use-getter-return 
	 */
	useGetterReturn?: UseGetterReturnConfiguration;
	/**
	* Enforces the use of a recommended display strategy with Google Fonts.
See https://biomejs.dev/linter/rules/use-google-font-display 
	 */
	useGoogleFontDisplay?: UseGoogleFontDisplayConfiguration;
	/**
	* Require for-in loops to include an if statement.
See https://biomejs.dev/linter/rules/use-guard-for-in 
	 */
	useGuardForIn?: UseGuardForInConfiguration;
	/**
	* Use Array.isArray() instead of instanceof Array.
See https://biomejs.dev/linter/rules/use-is-array 
	 */
	useIsArray?: UseIsArrayConfiguration;
	/**
	* Enforce consistent return values in iterable callbacks.
See https://biomejs.dev/linter/rules/use-iterable-callback-return 
	 */
	useIterableCallbackReturn?: UseIterableCallbackReturnConfiguration;
	/**
	* Require using the namespace keyword over the module keyword to declare TypeScript namespaces.
See https://biomejs.dev/linter/rules/use-namespace-keyword 
	 */
	useNamespaceKeyword?: UseNamespaceKeywordConfiguration;
	/**
	* Enforce using the digits argument with Number#toFixed().
See https://biomejs.dev/linter/rules/use-number-to-fixed-digits-argument 
	 */
	useNumberToFixedDigitsArgument?: UseNumberToFixedDigitsArgumentConfiguration;
	/**
	* Use static Response methods instead of new Response() constructor when possible.
See https://biomejs.dev/linter/rules/use-static-response-methods 
	 */
	useStaticResponseMethods?: UseStaticResponseMethodsConfiguration;
	/**
	* Enforce the use of the directive "use strict" in script files.
See https://biomejs.dev/linter/rules/use-strict-mode 
	 */
	useStrictMode?: UseStrictModeConfiguration;
}
export type Glob = string;
export type RuleAssistPlainConfiguration = "off" | "on";
export interface RuleAssistWithNoDuplicateClassesOptions {
	level: RuleAssistPlainConfiguration;
	options: NoDuplicateClassesOptions;
}
export interface RuleAssistWithOrganizeImportsOptions {
	level: RuleAssistPlainConfiguration;
	options: OrganizeImportsOptions;
}
export interface RuleAssistWithUseSortedAttributesOptions {
	level: RuleAssistPlainConfiguration;
	options: UseSortedAttributesOptions;
}
export interface RuleAssistWithUseSortedInterfaceMembersOptions {
	level: RuleAssistPlainConfiguration;
	options: UseSortedInterfaceMembersOptions;
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
export type NoUselessCatchBindingConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessCatchBindingOptions;
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
export type NoUselessUndefinedConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessUndefinedOptions;
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
export type UseMaxParamsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseMaxParamsOptions;
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
export type NoNextAsyncClientComponentConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNextAsyncClientComponentOptions;
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
export type NoUnresolvedImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnresolvedImportsOptions;
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
export type NoVueSetupPropsReactivityLossConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueSetupPropsReactivityLossOptions;
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
export type UseQwikMethodUsageConfiguration =
	| RulePlainConfiguration
	| RuleWithUseQwikMethodUsageOptions;
export type UseQwikValidLexicalScopeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseQwikValidLexicalScopeOptions;
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
export type NoAmbiguousAnchorTextConfiguration =
	| RulePlainConfiguration
	| RuleWithNoAmbiguousAnchorTextOptions;
export type NoBeforeInteractiveScriptOutsideDocumentConfiguration =
	| RulePlainConfiguration
	| RuleWithNoBeforeInteractiveScriptOutsideDocumentOptions;
export type NoConditionalExpectConfiguration =
	| RulePlainConfiguration
	| RuleWithNoConditionalExpectOptions;
export type NoContinueConfiguration =
	| RulePlainConfiguration
	| RuleWithNoContinueOptions;
export type NoDeprecatedMediaTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDeprecatedMediaTypeOptions;
export type NoDivRegexConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDivRegexOptions;
export type NoDuplicateArgumentNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateArgumentNamesOptions;
export type NoDuplicateAttributesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateAttributesOptions;
export type NoDuplicateEnumValueNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateEnumValueNamesOptions;
export type NoDuplicateEnumValuesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateEnumValuesOptions;
export type NoDuplicateFieldDefinitionNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateFieldDefinitionNamesOptions;
export type NoDuplicateGraphqlOperationNameConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateGraphqlOperationNameOptions;
export type NoDuplicateInputFieldNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateInputFieldNamesOptions;
export type NoDuplicateVariableNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateVariableNamesOptions;
export type NoDuplicatedSpreadPropsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicatedSpreadPropsOptions;
export type NoEqualsToNullConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEqualsToNullOptions;
export type NoExcessiveClassesPerFileConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExcessiveClassesPerFileOptions;
export type NoExcessiveLinesPerFileConfiguration =
	| RulePlainConfiguration
	| RuleWithNoExcessiveLinesPerFileOptions;
export type NoFloatingClassesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoFloatingClassesOptions;
export type NoFloatingPromisesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoFloatingPromisesOptions;
export type NoForInConfiguration =
	| RulePlainConfiguration
	| RuleWithNoForInOptions;
export type NoHexColorsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoHexColorsOptions;
export type NoIncrementDecrementConfiguration =
	| RulePlainConfiguration
	| RuleWithNoIncrementDecrementOptions;
export type NoJsxPropsBindConfiguration =
	| RulePlainConfiguration
	| RuleWithNoJsxPropsBindOptions;
export type NoLeakedRenderConfiguration =
	| RulePlainConfiguration
	| RuleWithNoLeakedRenderOptions;
export type NoMisusedPromisesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMisusedPromisesOptions;
export type NoMultiAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMultiAssignOptions;
export type NoMultiStrConfiguration =
	| RulePlainConfiguration
	| RuleWithNoMultiStrOptions;
export type NoNestedPromisesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoNestedPromisesOptions;
export type NoParametersOnlyUsedInRecursionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoParametersOnlyUsedInRecursionOptions;
export type NoPlaywrightElementHandleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightElementHandleOptions;
export type NoPlaywrightEvalConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightEvalOptions;
export type NoPlaywrightForceOptionConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightForceOptionOptions;
export type NoPlaywrightMissingAwaitConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightMissingAwaitOptions;
export type NoPlaywrightNetworkidleConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightNetworkidleOptions;
export type NoPlaywrightPagePauseConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightPagePauseOptions;
export type NoPlaywrightUselessAwaitConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightUselessAwaitOptions;
export type NoPlaywrightWaitForNavigationConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightWaitForNavigationOptions;
export type NoPlaywrightWaitForSelectorConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightWaitForSelectorOptions;
export type NoPlaywrightWaitForTimeoutConfiguration =
	| RulePlainConfiguration
	| RuleWithNoPlaywrightWaitForTimeoutOptions;
export type NoProtoConfiguration =
	| RulePlainConfiguration
	| RuleWithNoProtoOptions;
export type NoRedundantDefaultExportConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRedundantDefaultExportOptions;
export type NoReturnAssignConfiguration =
	| RulePlainConfiguration
	| RuleWithNoReturnAssignOptions;
export type NoRootTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoRootTypeOptions;
export type NoScriptUrlConfiguration =
	| RulePlainConfiguration
	| RuleWithNoScriptUrlOptions;
export type NoShadowConfiguration =
	| RulePlainConfiguration
	| RuleWithNoShadowOptions;
export type NoSyncScriptsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoSyncScriptsOptions;
export type NoTernaryConfiguration =
	| RulePlainConfiguration
	| RuleWithNoTernaryOptions;
export type NoUndeclaredEnvVarsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUndeclaredEnvVarsOptions;
export type NoUnknownAttributeConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnknownAttributeOptions;
export type NoUnnecessaryConditionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnnecessaryConditionsOptions;
export type NoUselessReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUselessReturnOptions;
export type NoVueArrowFuncInWatchConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueArrowFuncInWatchOptions;
export type NoVueOptionsApiConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueOptionsApiOptions;
export type NoVueRefAsOperandConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueRefAsOperandOptions;
export type NoVueVIfWithVForConfiguration =
	| RulePlainConfiguration
	| RuleWithNoVueVIfWithVForOptions;
export type UseArraySomeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseArraySomeOptions;
export type UseArraySortCompareConfiguration =
	| RulePlainConfiguration
	| RuleWithUseArraySortCompareOptions;
export type UseAwaitThenableConfiguration =
	| RulePlainConfiguration
	| RuleWithUseAwaitThenableOptions;
export type UseConsistentEnumValueTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentEnumValueTypeOptions;
export type UseConsistentGraphqlDescriptionsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentGraphqlDescriptionsOptions;
export type UseConsistentMethodSignaturesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentMethodSignaturesOptions;
export type UseDestructuringConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDestructuringOptions;
export type UseErrorCauseConfiguration =
	| RulePlainConfiguration
	| RuleWithUseErrorCauseOptions;
export type UseExhaustiveSwitchCasesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExhaustiveSwitchCasesOptions;
export type UseExpectConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExpectOptions;
export type UseExplicitTypeConfiguration =
	| RulePlainConfiguration
	| RuleWithUseExplicitTypeOptions;
export type UseFindConfiguration =
	| RulePlainConfiguration
	| RuleWithUseFindOptions;
export type UseGlobalThisConfiguration =
	| RulePlainConfiguration
	| RuleWithUseGlobalThisOptions;
export type UseInlineScriptIdConfiguration =
	| RulePlainConfiguration
	| RuleWithUseInlineScriptIdOptions;
export type UseInputNameConfiguration =
	| RulePlainConfiguration
	| RuleWithUseInputNameOptions;
export type UseLoneAnonymousOperationConfiguration =
	| RulePlainConfiguration
	| RuleWithUseLoneAnonymousOperationOptions;
export type UseLoneExecutableDefinitionConfiguration =
	| RulePlainConfiguration
	| RuleWithUseLoneExecutableDefinitionOptions;
export type UseNullishCoalescingConfiguration =
	| RulePlainConfiguration
	| RuleWithUseNullishCoalescingOptions;
export type UsePlaywrightValidDescribeCallbackConfiguration =
	| RulePlainConfiguration
	| RuleWithUsePlaywrightValidDescribeCallbackOptions;
export type UseRegexpExecConfiguration =
	| RulePlainConfiguration
	| RuleWithUseRegexpExecOptions;
export type UseRequiredScriptsConfiguration =
	| RulePlainConfiguration
	| RuleWithUseRequiredScriptsOptions;
export type UseSortedClassesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSortedClassesOptions;
export type UseSpreadConfiguration =
	| RulePlainConfiguration
	| RuleWithUseSpreadOptions;
export type UseVueConsistentDefinePropsDeclarationConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueConsistentDefinePropsDeclarationOptions;
export type UseVueConsistentVBindStyleConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueConsistentVBindStyleOptions;
export type UseVueConsistentVOnStyleConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueConsistentVOnStyleOptions;
export type UseVueDefineMacrosOrderConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueDefineMacrosOrderOptions;
export type UseVueHyphenatedAttributesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueHyphenatedAttributesOptions;
export type UseVueMultiWordComponentNamesConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueMultiWordComponentNamesOptions;
export type UseVueVForKeyConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueVForKeyOptions;
export type UseVueValidTemplateRootConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidTemplateRootOptions;
export type UseVueValidVBindConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVBindOptions;
export type UseVueValidVCloakConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVCloakOptions;
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
export type UseVueValidVOnceConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVOnceOptions;
export type UseVueValidVPreConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVPreOptions;
export type UseVueValidVTextConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueValidVTextOptions;
export type UseVueVaporConfiguration =
	| RulePlainConfiguration
	| RuleWithUseVueVaporOptions;
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
export type NoJsxLiteralsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoJsxLiteralsOptions;
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
export type UseConsistentArrowReturnConfiguration =
	| RulePlainConfiguration
	| RuleWithUseConsistentArrowReturnOptions;
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
export type NoDeprecatedImportsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDeprecatedImportsOptions;
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
export type NoDuplicateDependenciesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoDuplicateDependenciesOptions;
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
export type NoEmptySourceConfiguration =
	| RulePlainConfiguration
	| RuleWithNoEmptySourceOptions;
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
export type NoImportCyclesConfiguration =
	| RulePlainConfiguration
	| RuleWithNoImportCyclesOptions;
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
export type NoReactForwardRefConfiguration =
	| RulePlainConfiguration
	| RuleWithNoReactForwardRefOptions;
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
export type NoUnusedExpressionsConfiguration =
	| RulePlainConfiguration
	| RuleWithNoUnusedExpressionsOptions;
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
export type UseDeprecatedDateConfiguration =
	| RulePlainConfiguration
	| RuleWithUseDeprecatedDateOptions;
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
export interface NoDuplicateClassesOptions {
	/**
	 * Additional attributes that will be sorted.
	 */
	attributes?: string[];
	/**
	 * Names of the functions or tagged templates that will be sorted.
	 */
	functions?: string[];
}
export interface OrganizeImportsOptions {
	groups?: ImportGroups;
	identifierOrder?: SortOrder;
}
export interface UseSortedAttributesOptions {
	sortOrder?: SortOrder;
}
export type UseSortedInterfaceMembersOptions = {};
export interface UseSortedKeysOptions {
	/**
	* When enabled, groups object keys by their value's nesting depth before sorting.
Simple values (primitives, single-line arrays, single-line objects) are sorted first,
followed by nested values (multi-line objects, multi-line arrays). 
	 */
	groupByNesting?: boolean;
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
export interface RuleWithNoUselessCatchBindingOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessCatchBindingOptions;
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
export interface RuleWithNoUselessUndefinedOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessUndefinedOptions;
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
export interface RuleWithUseMaxParamsOptions {
	level: RulePlainConfiguration;
	options?: UseMaxParamsOptions;
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
export interface RuleWithNoNextAsyncClientComponentOptions {
	level: RulePlainConfiguration;
	options?: NoNextAsyncClientComponentOptions;
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
export interface RuleWithNoUnresolvedImportsOptions {
	level: RulePlainConfiguration;
	options?: NoUnresolvedImportsOptions;
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
export interface RuleWithNoVueSetupPropsReactivityLossOptions {
	level: RulePlainConfiguration;
	options?: NoVueSetupPropsReactivityLossOptions;
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
export interface RuleWithUseQwikMethodUsageOptions {
	level: RulePlainConfiguration;
	options?: UseQwikMethodUsageOptions;
}
export interface RuleWithUseQwikValidLexicalScopeOptions {
	level: RulePlainConfiguration;
	options?: UseQwikValidLexicalScopeOptions;
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
export interface RuleWithNoAmbiguousAnchorTextOptions {
	level: RulePlainConfiguration;
	options?: NoAmbiguousAnchorTextOptions;
}
export interface RuleWithNoBeforeInteractiveScriptOutsideDocumentOptions {
	level: RulePlainConfiguration;
	options?: NoBeforeInteractiveScriptOutsideDocumentOptions;
}
export interface RuleWithNoConditionalExpectOptions {
	level: RulePlainConfiguration;
	options?: NoConditionalExpectOptions;
}
export interface RuleWithNoContinueOptions {
	level: RulePlainConfiguration;
	options?: NoContinueOptions;
}
export interface RuleWithNoDeprecatedMediaTypeOptions {
	level: RulePlainConfiguration;
	options?: NoDeprecatedMediaTypeOptions;
}
export interface RuleWithNoDivRegexOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoDivRegexOptions;
}
export interface RuleWithNoDuplicateArgumentNamesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateArgumentNamesOptions;
}
export interface RuleWithNoDuplicateAttributesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateAttributesOptions;
}
export interface RuleWithNoDuplicateEnumValueNamesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateEnumValueNamesOptions;
}
export interface RuleWithNoDuplicateEnumValuesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateEnumValuesOptions;
}
export interface RuleWithNoDuplicateFieldDefinitionNamesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateFieldDefinitionNamesOptions;
}
export interface RuleWithNoDuplicateGraphqlOperationNameOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateGraphqlOperationNameOptions;
}
export interface RuleWithNoDuplicateInputFieldNamesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateInputFieldNamesOptions;
}
export interface RuleWithNoDuplicateVariableNamesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateVariableNamesOptions;
}
export interface RuleWithNoDuplicatedSpreadPropsOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicatedSpreadPropsOptions;
}
export interface RuleWithNoEqualsToNullOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoEqualsToNullOptions;
}
export interface RuleWithNoExcessiveClassesPerFileOptions {
	level: RulePlainConfiguration;
	options?: NoExcessiveClassesPerFileOptions;
}
export interface RuleWithNoExcessiveLinesPerFileOptions {
	level: RulePlainConfiguration;
	options?: NoExcessiveLinesPerFileOptions;
}
export interface RuleWithNoFloatingClassesOptions {
	level: RulePlainConfiguration;
	options?: NoFloatingClassesOptions;
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
export interface RuleWithNoHexColorsOptions {
	level: RulePlainConfiguration;
	options?: NoHexColorsOptions;
}
export interface RuleWithNoIncrementDecrementOptions {
	level: RulePlainConfiguration;
	options?: NoIncrementDecrementOptions;
}
export interface RuleWithNoJsxPropsBindOptions {
	level: RulePlainConfiguration;
	options?: NoJsxPropsBindOptions;
}
export interface RuleWithNoLeakedRenderOptions {
	level: RulePlainConfiguration;
	options?: NoLeakedRenderOptions;
}
export interface RuleWithNoMisusedPromisesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoMisusedPromisesOptions;
}
export interface RuleWithNoMultiAssignOptions {
	level: RulePlainConfiguration;
	options?: NoMultiAssignOptions;
}
export interface RuleWithNoMultiStrOptions {
	level: RulePlainConfiguration;
	options?: NoMultiStrOptions;
}
export interface RuleWithNoNestedPromisesOptions {
	level: RulePlainConfiguration;
	options?: NoNestedPromisesOptions;
}
export interface RuleWithNoParametersOnlyUsedInRecursionOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoParametersOnlyUsedInRecursionOptions;
}
export interface RuleWithNoPlaywrightElementHandleOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoPlaywrightElementHandleOptions;
}
export interface RuleWithNoPlaywrightEvalOptions {
	level: RulePlainConfiguration;
	options?: NoPlaywrightEvalOptions;
}
export interface RuleWithNoPlaywrightForceOptionOptions {
	level: RulePlainConfiguration;
	options?: NoPlaywrightForceOptionOptions;
}
export interface RuleWithNoPlaywrightMissingAwaitOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoPlaywrightMissingAwaitOptions;
}
export interface RuleWithNoPlaywrightNetworkidleOptions {
	level: RulePlainConfiguration;
	options?: NoPlaywrightNetworkidleOptions;
}
export interface RuleWithNoPlaywrightPagePauseOptions {
	level: RulePlainConfiguration;
	options?: NoPlaywrightPagePauseOptions;
}
export interface RuleWithNoPlaywrightUselessAwaitOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoPlaywrightUselessAwaitOptions;
}
export interface RuleWithNoPlaywrightWaitForNavigationOptions {
	level: RulePlainConfiguration;
	options?: NoPlaywrightWaitForNavigationOptions;
}
export interface RuleWithNoPlaywrightWaitForSelectorOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoPlaywrightWaitForSelectorOptions;
}
export interface RuleWithNoPlaywrightWaitForTimeoutOptions {
	level: RulePlainConfiguration;
	options?: NoPlaywrightWaitForTimeoutOptions;
}
export interface RuleWithNoProtoOptions {
	level: RulePlainConfiguration;
	options?: NoProtoOptions;
}
export interface RuleWithNoRedundantDefaultExportOptions {
	level: RulePlainConfiguration;
	options?: NoRedundantDefaultExportOptions;
}
export interface RuleWithNoReturnAssignOptions {
	level: RulePlainConfiguration;
	options?: NoReturnAssignOptions;
}
export interface RuleWithNoRootTypeOptions {
	level: RulePlainConfiguration;
	options?: NoRootTypeOptions;
}
export interface RuleWithNoScriptUrlOptions {
	level: RulePlainConfiguration;
	options?: NoScriptUrlOptions;
}
export interface RuleWithNoShadowOptions {
	level: RulePlainConfiguration;
	options?: NoShadowOptions;
}
export interface RuleWithNoSyncScriptsOptions {
	level: RulePlainConfiguration;
	options?: NoSyncScriptsOptions;
}
export interface RuleWithNoTernaryOptions {
	level: RulePlainConfiguration;
	options?: NoTernaryOptions;
}
export interface RuleWithNoUndeclaredEnvVarsOptions {
	level: RulePlainConfiguration;
	options?: NoUndeclaredEnvVarsOptions;
}
export interface RuleWithNoUnknownAttributeOptions {
	level: RulePlainConfiguration;
	options?: NoUnknownAttributeOptions;
}
export interface RuleWithNoUnnecessaryConditionsOptions {
	level: RulePlainConfiguration;
	options?: NoUnnecessaryConditionsOptions;
}
export interface RuleWithNoUselessReturnOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoUselessReturnOptions;
}
export interface RuleWithNoVueArrowFuncInWatchOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoVueArrowFuncInWatchOptions;
}
export interface RuleWithNoVueOptionsApiOptions {
	level: RulePlainConfiguration;
	options?: NoVueOptionsApiOptions;
}
export interface RuleWithNoVueRefAsOperandOptions {
	level: RulePlainConfiguration;
	options?: NoVueRefAsOperandOptions;
}
export interface RuleWithNoVueVIfWithVForOptions {
	level: RulePlainConfiguration;
	options?: NoVueVIfWithVForOptions;
}
export interface RuleWithUseArraySomeOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseArraySomeOptions;
}
export interface RuleWithUseArraySortCompareOptions {
	level: RulePlainConfiguration;
	options?: UseArraySortCompareOptions;
}
export interface RuleWithUseAwaitThenableOptions {
	level: RulePlainConfiguration;
	options?: UseAwaitThenableOptions;
}
export interface RuleWithUseConsistentEnumValueTypeOptions {
	level: RulePlainConfiguration;
	options?: UseConsistentEnumValueTypeOptions;
}
export interface RuleWithUseConsistentGraphqlDescriptionsOptions {
	level: RulePlainConfiguration;
	options?: UseConsistentGraphqlDescriptionsOptions;
}
export interface RuleWithUseConsistentMethodSignaturesOptions {
	level: RulePlainConfiguration;
	options?: UseConsistentMethodSignaturesOptions;
}
export interface RuleWithUseDestructuringOptions {
	level: RulePlainConfiguration;
	options?: UseDestructuringOptions;
}
export interface RuleWithUseErrorCauseOptions {
	level: RulePlainConfiguration;
	options?: UseErrorCauseOptions;
}
export interface RuleWithUseExhaustiveSwitchCasesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseExhaustiveSwitchCasesOptions;
}
export interface RuleWithUseExpectOptions {
	level: RulePlainConfiguration;
	options?: UseExpectOptions;
}
export interface RuleWithUseExplicitTypeOptions {
	level: RulePlainConfiguration;
	options?: UseExplicitTypeOptions;
}
export interface RuleWithUseFindOptions {
	level: RulePlainConfiguration;
	options?: UseFindOptions;
}
export interface RuleWithUseGlobalThisOptions {
	level: RulePlainConfiguration;
	options?: UseGlobalThisOptions;
}
export interface RuleWithUseInlineScriptIdOptions {
	level: RulePlainConfiguration;
	options?: UseInlineScriptIdOptions;
}
export interface RuleWithUseInputNameOptions {
	level: RulePlainConfiguration;
	options?: UseInputNameOptions;
}
export interface RuleWithUseLoneAnonymousOperationOptions {
	level: RulePlainConfiguration;
	options?: UseLoneAnonymousOperationOptions;
}
export interface RuleWithUseLoneExecutableDefinitionOptions {
	level: RulePlainConfiguration;
	options?: UseLoneExecutableDefinitionOptions;
}
export interface RuleWithUseNullishCoalescingOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseNullishCoalescingOptions;
}
export interface RuleWithUsePlaywrightValidDescribeCallbackOptions {
	level: RulePlainConfiguration;
	options?: UsePlaywrightValidDescribeCallbackOptions;
}
export interface RuleWithUseRegexpExecOptions {
	level: RulePlainConfiguration;
	options?: UseRegexpExecOptions;
}
export interface RuleWithUseRequiredScriptsOptions {
	level: RulePlainConfiguration;
	options?: UseRequiredScriptsOptions;
}
export interface RuleWithUseSortedClassesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSortedClassesOptions;
}
export interface RuleWithUseSpreadOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseSpreadOptions;
}
export interface RuleWithUseVueConsistentDefinePropsDeclarationOptions {
	level: RulePlainConfiguration;
	options?: UseVueConsistentDefinePropsDeclarationOptions;
}
export interface RuleWithUseVueConsistentVBindStyleOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueConsistentVBindStyleOptions;
}
export interface RuleWithUseVueConsistentVOnStyleOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueConsistentVOnStyleOptions;
}
export interface RuleWithUseVueDefineMacrosOrderOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueDefineMacrosOrderOptions;
}
export interface RuleWithUseVueHyphenatedAttributesOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueHyphenatedAttributesOptions;
}
export interface RuleWithUseVueMultiWordComponentNamesOptions {
	level: RulePlainConfiguration;
	options?: UseVueMultiWordComponentNamesOptions;
}
export interface RuleWithUseVueVForKeyOptions {
	level: RulePlainConfiguration;
	options?: UseVueVForKeyOptions;
}
export interface RuleWithUseVueValidTemplateRootOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueValidTemplateRootOptions;
}
export interface RuleWithUseVueValidVBindOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVBindOptions;
}
export interface RuleWithUseVueValidVCloakOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueValidVCloakOptions;
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
export interface RuleWithUseVueValidVOnceOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueValidVOnceOptions;
}
export interface RuleWithUseVueValidVPreOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueValidVPreOptions;
}
export interface RuleWithUseVueValidVTextOptions {
	level: RulePlainConfiguration;
	options?: UseVueValidVTextOptions;
}
export interface RuleWithUseVueVaporOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseVueVaporOptions;
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
export interface RuleWithNoJsxLiteralsOptions {
	level: RulePlainConfiguration;
	options?: NoJsxLiteralsOptions;
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
export interface RuleWithUseConsistentArrowReturnOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: UseConsistentArrowReturnOptions;
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
export interface RuleWithNoDeprecatedImportsOptions {
	level: RulePlainConfiguration;
	options?: NoDeprecatedImportsOptions;
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
export interface RuleWithNoDuplicateDependenciesOptions {
	level: RulePlainConfiguration;
	options?: NoDuplicateDependenciesOptions;
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
export interface RuleWithNoEmptySourceOptions {
	level: RulePlainConfiguration;
	options?: NoEmptySourceOptions;
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
export interface RuleWithNoImportCyclesOptions {
	level: RulePlainConfiguration;
	options?: NoImportCyclesOptions;
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
export interface RuleWithNoReactForwardRefOptions {
	fix?: FixKind;
	level: RulePlainConfiguration;
	options?: NoReactForwardRefOptions;
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
export interface RuleWithNoUnusedExpressionsOptions {
	level: RulePlainConfiguration;
	options?: NoUnusedExpressionsOptions;
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
export interface RuleWithUseDeprecatedDateOptions {
	level: RulePlainConfiguration;
	options?: UseDeprecatedDateOptions;
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
/**
	* Options for the `noUselessCatchBinding` rule.
Currently empty; reserved for future extensions (e.g. allowlist of names). 
	 */
export type NoUselessCatchBindingOptions = {};
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
export type NoUselessUndefinedOptions = {};
export type NoUselessUndefinedInitializationOptions = {};
export type NoVoidOptions = {};
export type UseArrowFunctionOptions = {};
export type UseDateNowOptions = {};
export type UseFlatMapOptions = {};
export type UseIndexOfOptions = {};
export type UseLiteralKeysOptions = {};
export interface UseMaxParamsOptions {
	/**
	 * Maximum number of parameters allowed (default: 4)
	 */
	max?: number;
}
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
export type NoNextAsyncClientComponentOptions = {};
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
export interface NoUnknownFunctionOptions {
	/**
	 * A list of unknown function names to ignore (case-insensitive).
	 */
	ignore?: string[];
}
export type NoUnknownMediaFeatureNameOptions = {};
export interface NoUnknownPropertyOptions {
	/**
	 * A list of unknown property names to ignore (case-insensitive).
	 */
	ignore?: string[];
}
export interface NoUnknownPseudoClassOptions {
	/**
	 * A list of unknown pseudo-class names to ignore (case-insensitive).
	 */
	ignore?: string[];
}
export interface NoUnknownPseudoElementOptions {
	/**
	 * A list of unknown pseudo-element names to ignore (case-insensitive).
	 */
	ignore?: string[];
}
export type NoUnknownTypeSelectorOptions = {};
export type NoUnknownUnitOptions = {};
export type NoUnmatchableAnbSelectorOptions = {};
export type NoUnreachableOptions = {};
export type NoUnreachableSuperOptions = {};
export type NoUnresolvedImportsOptions = {};
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
export type NoVueDataObjectDeclarationOptions = {};
export type NoVueDuplicateKeysOptions = {};
export type NoVueReservedKeysOptions = {};
export type NoVueReservedPropsOptions = {};
export type NoVueSetupPropsReactivityLossOptions = {};
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
export interface UseHookAtTopLevelOptions {
	/**
	* List of function names that should not be treated as hooks.
Functions in this list will be ignored by the rule even if they follow the `use*` naming convention. 
	 */
	ignore?: string[];
}
export type UseImageSizeOptions = null;
export interface UseImportExtensionsOptions {
	/**
	* A map of file extensions to their suggested replacements.
For example, `{"ts": "js"}` would suggest `.js` extensions for TypeScript imports. 
	 */
	extensionMappings?: Record<string, string>;
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
export type UseQwikMethodUsageOptions = {};
export type UseQwikValidLexicalScopeOptions = {};
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
export interface NoAmbiguousAnchorTextOptions {
	/**
	 * It allows users to modify the strings that can be checked for in the anchor text. Useful for specifying other words in other languages
	 */
	words?: string[];
}
export type NoBeforeInteractiveScriptOutsideDocumentOptions = {};
export type NoConditionalExpectOptions = {};
export type NoContinueOptions = {};
export interface NoDeprecatedMediaTypeOptions {
	/**
	 * Media types to allow (case-insensitive).
	 */
	allow?: string[];
}
export type NoDivRegexOptions = {};
export type NoDuplicateArgumentNamesOptions = {};
export type NoDuplicateAttributesOptions = {};
export type NoDuplicateEnumValueNamesOptions = {};
export type NoDuplicateEnumValuesOptions = {};
export type NoDuplicateFieldDefinitionNamesOptions = {};
export type NoDuplicateGraphqlOperationNameOptions = {};
export type NoDuplicateInputFieldNamesOptions = {};
export type NoDuplicateVariableNamesOptions = {};
export type NoDuplicatedSpreadPropsOptions = {};
export type NoEqualsToNullOptions = {};
export interface NoExcessiveClassesPerFileOptions {
	/**
	 * The maximum number of classes allowed in a file.
	 */
	maxClasses?: number;
}
export interface NoExcessiveLinesPerFileOptions {
	/**
	 * The maximum number of lines allowed in a file.
	 */
	maxLines?: number;
	/**
	 * When this option is set to `true`, blank lines are not counted towards the maximum line limit.
	 */
	skipBlankLines?: boolean;
}
export type NoFloatingClassesOptions = {};
export type NoFloatingPromisesOptions = {};
export type NoForInOptions = {};
export type NoHexColorsOptions = {};
export interface NoIncrementDecrementOptions {
	/**
	 * Allows unary operators ++ and -- in the afterthought (final expression) of a for loop.
	 */
	allowForLoopAfterthoughts?: boolean;
}
export type NoJsxPropsBindOptions = {};
export type NoLeakedRenderOptions = {};
export type NoMisusedPromisesOptions = {};
export type NoMultiAssignOptions = {};
export type NoMultiStrOptions = {};
export type NoNestedPromisesOptions = {};
export type NoParametersOnlyUsedInRecursionOptions = {};
export type NoPlaywrightElementHandleOptions = {};
export type NoPlaywrightEvalOptions = {};
export type NoPlaywrightForceOptionOptions = {};
export type NoPlaywrightMissingAwaitOptions = {};
export type NoPlaywrightNetworkidleOptions = {};
export type NoPlaywrightPagePauseOptions = {};
export type NoPlaywrightUselessAwaitOptions = {};
export type NoPlaywrightWaitForNavigationOptions = {};
export type NoPlaywrightWaitForSelectorOptions = {};
export type NoPlaywrightWaitForTimeoutOptions = {};
export type NoProtoOptions = {};
export type NoRedundantDefaultExportOptions = {};
export type NoReturnAssignOptions = {};
export interface NoRootTypeOptions {
	/**
	* A list of disallowed root types (e.g. "mutation" and/or "subscription").
The values of the list are case-insensitive. 
	 */
	disallow?: string[];
}
export type NoScriptUrlOptions = {};
export type NoShadowOptions = {};
export type NoSyncScriptsOptions = {};
export type NoTernaryOptions = {};
export interface NoUndeclaredEnvVarsOptions {
	/**
	* Environment variables that should always be allowed.
Use this to specify environment variables that are always available
in your environment, even when not declared in turbo.json.
Supports regular expressions, e.g. `["MY_ENV_.*"]`. 
	 */
	allowedEnvVars?: Regex[];
}
export interface NoUnknownAttributeOptions {
	ignore?: string[];
}
export type NoUnnecessaryConditionsOptions = {};
export type NoUselessReturnOptions = {};
export type NoVueArrowFuncInWatchOptions = {};
export type NoVueOptionsApiOptions = {};
export type NoVueRefAsOperandOptions = {};
export type NoVueVIfWithVForOptions = {};
export type UseArraySomeOptions = {};
export type UseArraySortCompareOptions = {};
export type UseAwaitThenableOptions = {};
export type UseConsistentEnumValueTypeOptions = {};
export interface UseConsistentGraphqlDescriptionsOptions {
	/**
	 * The description style to enforce. Defaults to "block"
	 */
	style?: UseConsistentGraphqlDescriptionsStyle;
}
/**
 * Options type for `useConsistentMethodSignatures`.
 */
export interface UseConsistentMethodSignaturesOptions {
	/**
	* The style of method signatures whose usage will be enforced.

Default: "property" 
	 */
	style?: MethodSignatureStyle;
}
export type UseDestructuringOptions = {};
/**
 * Options for the `useErrorCause` rule.
 */
export interface UseErrorCauseOptions {
	/**
	 * When set to `true`, the rule requires that `catch` clauses have a parameter.
	 */
	requireCatchParameter?: boolean;
}
export type UseExhaustiveSwitchCasesOptions = {};
export type UseExpectOptions = {};
export type UseExplicitTypeOptions = {};
export type UseFindOptions = {};
export type UseGlobalThisOptions = {};
export type UseInlineScriptIdOptions = {};
export interface UseInputNameOptions {
	/**
	 * Check that the input type name follows the convention <mutationName>Input
	 */
	checkInputType?: CheckInputType;
}
export type UseLoneAnonymousOperationOptions = {};
export type UseLoneExecutableDefinitionOptions = {};
export interface UseNullishCoalescingOptions {
	/**
	* Whether to ignore `||` expressions in conditional test positions
(if/while/for/do-while/ternary conditions).

When `true` (the default), the rule will not report `||` expressions
that appear in places where the falsy-checking behavior may be intentional.

Default: `true` 
	 */
	ignoreConditionalTests?: boolean;
}
export type UsePlaywrightValidDescribeCallbackOptions = {};
export type UseRegexpExecOptions = {};
export interface UseRequiredScriptsOptions {
	/**
	 * List of script names that must be present in package.json
	 */
	requiredScripts?: string[];
}
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
export type UseSpreadOptions = {};
export interface UseVueConsistentDefinePropsDeclarationOptions {
	style?: DeclarationStyle;
}
export interface UseVueConsistentVBindStyleOptions {
	/**
	* Preferred style for `v-bind` usage: "shorthand" or "longhand".
If omitted, shorthand is preferred. 
	 */
	style?: VueDirectiveStyle;
}
export interface UseVueConsistentVOnStyleOptions {
	/**
	* Preferred style for `v-on` usage: "shorthand" or "longhand".
If omitted, shorthand is preferred. 
	 */
	style?: VueDirectiveStyle2;
}
export interface UseVueDefineMacrosOrderOptions {
	/**
	 * The order of the Vue define macros.
	 */
	order?: string[];
}
export interface UseVueHyphenatedAttributesOptions {
	/**
	 * List of attribute names to ignore when checking for hyphenated attributes.
	 */
	ignore?: string[];
	/**
	 * List of HTML tags to ignore when checking for hyphenated attributes.
	 */
	ignoreTags?: string[];
}
export interface UseVueMultiWordComponentNamesOptions {
	/**
	 * Component names to ignore (allowed to be single-word).
	 */
	ignores?: string[];
}
export type UseVueVForKeyOptions = {};
export type UseVueValidTemplateRootOptions = {};
export type UseVueValidVBindOptions = {};
export type UseVueValidVCloakOptions = {};
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
export type UseVueValidVOnceOptions = {};
export type UseVueValidVPreOptions = {};
export type UseVueValidVTextOptions = {};
export type UseVueVaporOptions = {};
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
export interface UseUnifiedTypeSignaturesOptions {
	/**
	 * Whether to ignore overloads with different JSDoc comments.
	 */
	ignoreDifferentJsDoc?: boolean;
	/**
	 * Whether to ignore overloads with differently named parameters.
	 */
	ignoreDifferentlyNamedParameters?: boolean;
}
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
export type NoDeprecatedImportsOptions = {};
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
export type NoDuplicateDependenciesOptions = {};
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
export interface NoEmptySourceOptions {
	/**
	 * Whether comments are considered meaningful
	 */
	allowComments?: boolean;
}
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
export interface NoImportCyclesOptions {
	/**
	* Ignores type-only imports when finding an import cycle. A type-only import (`import type`)
will be removed by the compiler, so it cuts an import cycle at runtime. Note that named type
imports (`import { type Foo }`) aren't considered as type-only because it's not removed by
the compiler if the `verbatimModuleSyntax` option is enabled. Enabled by default. 
	 */
	ignoreTypes?: boolean;
}
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
export type NoReactForwardRefOptions = {};
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
export type NoUnusedExpressionsOptions = {};
export type NoUselessEscapeInStringOptions = {};
export type NoUselessRegexBackrefsOptions = {};
export type NoVarOptions = {};
export type NoWithOptions = {};
export type UseAdjacentOverloadSignaturesOptions = {};
export type UseAwaitOptions = {};
export type UseBiomeIgnoreFolderOptions = {};
export type UseDefaultSwitchClauseLastOptions = {};
export interface UseDeprecatedDateOptions {
	argumentName?: string;
}
export type UseErrorMessageOptions = {};
export type UseGetterReturnOptions = {};
export type UseGoogleFontDisplayOptions = {};
export type UseGuardForInOptions = {};
export type UseIsArrayOptions = {};
export interface UseIterableCallbackReturnOptions {
	/**
	* When `true`, the rule reports `forEach` callbacks that return a value (default behaviour).
When `false` or unset, such callbacks are ignored. 
	 */
	checkForEach?: boolean;
}
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
export type Regex = string;
/**
 * The GraphQL description style to enforce.
 */
export type UseConsistentGraphqlDescriptionsStyle = "block" | "inline";
export type MethodSignatureStyle = "property" | "method";
export type CheckInputType = "off" | "loose" | "strict";
export type DeclarationStyle = "type" | "runtime";
export type VueDirectiveStyle = "shorthand" | "longhand";
export type VueDirectiveStyle2 = "shorthand" | "longhand";
/**
 * Specifies whether property assignments on function parameters are allowed or denied.
 */
export type PropertyAssignmentMode = "allow" | "deny";
export type Paths = string | PathOptions;
export type Patterns = PatternOptions;
export type CustomRestrictedType = string | CustomRestrictedTypeOptions;
export type ConsistentArrayType = "shorthand" | "generic";
export type UseConsistentArrowReturnStyle = "asNeeded" | "always" | "never";
export type Accessibility = "noPublic" | "explicit" | "none";
export type ObjectPropertySyntax = "explicit" | "shorthand";
export type ConsistentTypeDefinition = "interface" | "type";
export type FilenameCases = FilenameCase[];
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
export interface FileFeaturesResult {
	featuresSupported: FeaturesSupported;
}
export type FeaturesSupported = { [K in FeatureKind]?: SupportKind };
export type SupportKind =
	| "supported"
	| "ignored"
	| "protected"
	| "featureNotEnabled"
	| "fileNotSupported"
	| "notRequested";
export interface UpdateSettingsParams {
	configuration: Configuration;
	extendedConfigurations?: [BiomePath, Configuration][];
	moduleGraphResolutionKind?: ModuleGraphResolutionKind;
	projectKey: ProjectKey;
	workspaceDirectory?: BiomePath;
}
export type ModuleGraphResolutionKind = "none" | "modules" | "modulesAndTypes";
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
	| "lint/complexity/noUselessCatchBinding"
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
	| "lint/complexity/noUselessUndefined"
	| "lint/complexity/noUselessUndefinedInitialization"
	| "lint/complexity/noVoid"
	| "lint/complexity/useArrowFunction"
	| "lint/complexity/useDateNow"
	| "lint/complexity/useFlatMap"
	| "lint/complexity/useIndexOf"
	| "lint/complexity/useLiteralKeys"
	| "lint/complexity/useMaxParams"
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
	| "lint/correctness/noInvalidPositionAtImportRule"
	| "lint/correctness/noInvalidUseBeforeDeclaration"
	| "lint/correctness/noMissingVarFunction"
	| "lint/correctness/noNestedComponentDefinitions"
	| "lint/correctness/noNextAsyncClientComponent"
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
	| "lint/correctness/noUnknownPseudoElement"
	| "lint/correctness/noUnknownTypeSelector"
	| "lint/correctness/noUnknownUnit"
	| "lint/correctness/noUnmatchableAnbSelector"
	| "lint/correctness/noUnreachable"
	| "lint/correctness/noUnreachableSuper"
	| "lint/correctness/noUnresolvedImports"
	| "lint/correctness/noUnsafeFinally"
	| "lint/correctness/noUnsafeOptionalChaining"
	| "lint/correctness/noUnusedFunctionParameters"
	| "lint/correctness/noUnusedImports"
	| "lint/correctness/noUnusedLabels"
	| "lint/correctness/noUnusedPrivateClassMembers"
	| "lint/correctness/noUnusedVariables"
	| "lint/correctness/noVoidElementsWithChildren"
	| "lint/correctness/noVoidTypeReturn"
	| "lint/correctness/noVueDataObjectDeclaration"
	| "lint/correctness/noVueDuplicateKeys"
	| "lint/correctness/noVueReservedKeys"
	| "lint/correctness/noVueReservedProps"
	| "lint/correctness/noVueSetupPropsReactivityLoss"
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
	| "lint/correctness/useQwikMethodUsage"
	| "lint/correctness/useQwikValidLexicalScope"
	| "lint/correctness/useSingleJsDocAsterisk"
	| "lint/correctness/useUniqueElementIds"
	| "lint/correctness/useValidForDirection"
	| "lint/correctness/useValidTypeof"
	| "lint/correctness/useYield"
	| "lint/nursery/useExpect"
	| "lint/nursery/noAmbiguousAnchorText"
	| "lint/nursery/noBeforeInteractiveScriptOutsideDocument"
	| "lint/nursery/noColorInvalidHex"
	| "lint/nursery/noContinue"
	| "lint/nursery/noDeprecatedMediaType"
	| "lint/nursery/noDivRegex"
	| "lint/nursery/noDuplicateArgumentNames"
	| "lint/nursery/noDuplicateAttributes"
	| "lint/nursery/noDuplicateEnumValueNames"
	| "lint/nursery/noDuplicateEnumValues"
	| "lint/nursery/noDuplicateFieldDefinitionNames"
	| "lint/nursery/noDuplicateGraphqlOperationName"
	| "lint/nursery/noDuplicateInputFieldNames"
	| "lint/nursery/noDuplicateVariableNames"
	| "lint/nursery/noDuplicatedSpreadProps"
	| "lint/nursery/noEqualsToNull"
	| "lint/nursery/noExcessiveClassesPerFile"
	| "lint/nursery/noExcessiveLinesPerFile"
	| "lint/nursery/noFloatingClasses"
	| "lint/nursery/noFloatingPromises"
	| "lint/nursery/noForIn"
	| "lint/nursery/noHexColors"
	| "lint/nursery/noImplicitCoercion"
	| "lint/nursery/noIncrementDecrement"
	| "lint/nursery/noJsxPropsBind"
	| "lint/nursery/noLeakedRender"
	| "lint/nursery/noMissingGenericFamilyKeyword"
	| "lint/nursery/noMisusedPromises"
	| "lint/nursery/noMultiAssign"
	| "lint/nursery/noMultiStr"
	| "lint/nursery/noNestedPromises"
	| "lint/nursery/noParametersOnlyUsedInRecursion"
	| "lint/nursery/noConditionalExpect"
	| "lint/nursery/noPlaywrightElementHandle"
	| "lint/nursery/noPlaywrightEval"
	| "lint/nursery/noPlaywrightForceOption"
	| "lint/nursery/noPlaywrightMissingAwait"
	| "lint/nursery/noPlaywrightNetworkidle"
	| "lint/nursery/noPlaywrightPagePause"
	| "lint/nursery/noPlaywrightUselessAwait"
	| "lint/nursery/noPlaywrightWaitForNavigation"
	| "lint/nursery/noPlaywrightWaitForSelector"
	| "lint/nursery/noPlaywrightWaitForTimeout"
	| "lint/nursery/noProto"
	| "lint/nursery/noRedundantDefaultExport"
	| "lint/nursery/noReturnAssign"
	| "lint/nursery/noRootType"
	| "lint/nursery/noScriptUrl"
	| "lint/nursery/noShadow"
	| "lint/nursery/noSyncScripts"
	| "lint/nursery/noTernary"
	| "lint/nursery/noUndeclaredEnvVars"
	| "lint/nursery/noUnknownAttribute"
	| "lint/nursery/noUnnecessaryConditions"
	| "lint/nursery/noUnwantedPolyfillio"
	| "lint/nursery/noUselessBackrefInRegex"
	| "lint/nursery/noUselessReturn"
	| "lint/nursery/noVueArrowFuncInWatch"
	| "lint/nursery/noVueOptionsApi"
	| "lint/nursery/noVueRefAsOperand"
	| "lint/nursery/noVueVIfWithVFor"
	| "lint/nursery/useArraySome"
	| "lint/nursery/useArraySortCompare"
	| "lint/nursery/useAwaitThenable"
	| "lint/nursery/useBiomeSuppressionComment"
	| "lint/nursery/useConsistentEnumValueType"
	| "lint/nursery/useConsistentGraphqlDescriptions"
	| "lint/nursery/useConsistentMethodSignatures"
	| "lint/nursery/useConsistentObjectDefinition"
	| "lint/nursery/useDestructuring"
	| "lint/nursery/useErrorCause"
	| "lint/nursery/useExhaustiveSwitchCases"
	| "lint/nursery/useExplicitFunctionReturnType"
	| "lint/nursery/useExplicitType"
	| "lint/nursery/useFind"
	| "lint/nursery/useGlobalThis"
	| "lint/nursery/useImportRestrictions"
	| "lint/nursery/useInlineScriptId"
	| "lint/nursery/useInputName"
	| "lint/nursery/useJsxCurlyBraceConvention"
	| "lint/nursery/useLoneAnonymousOperation"
	| "lint/nursery/useLoneExecutableDefinition"
	| "lint/nursery/useNullishCoalescing"
	| "lint/nursery/usePlaywrightValidDescribeCallback"
	| "lint/nursery/useRegexpExec"
	| "lint/nursery/useRequiredScripts"
	| "lint/nursery/useSortedClasses"
	| "lint/nursery/useSpread"
	| "lint/nursery/useUniqueArgumentNames"
	| "lint/nursery/useUniqueFieldDefinitionNames"
	| "lint/nursery/useUniqueGraphqlOperationName"
	| "lint/nursery/useUniqueInputFieldNames"
	| "lint/nursery/useUniqueVariableNames"
	| "lint/nursery/useVueConsistentDefinePropsDeclaration"
	| "lint/nursery/useVueConsistentVBindStyle"
	| "lint/nursery/useVueConsistentVOnStyle"
	| "lint/nursery/useVueDefineMacrosOrder"
	| "lint/nursery/useVueHyphenatedAttributes"
	| "lint/nursery/useVueMultiWordComponentNames"
	| "lint/nursery/useVueVForKey"
	| "lint/nursery/useVueValidTemplateRoot"
	| "lint/nursery/useVueValidVBind"
	| "lint/nursery/useVueValidVCloak"
	| "lint/nursery/useVueValidVElse"
	| "lint/nursery/useVueValidVElseIf"
	| "lint/nursery/useVueValidVFor"
	| "lint/nursery/useVueValidVHtml"
	| "lint/nursery/useVueValidVIf"
	| "lint/nursery/useVueValidVModel"
	| "lint/nursery/useVueValidVOn"
	| "lint/nursery/useVueValidVOnce"
	| "lint/nursery/useVueValidVPre"
	| "lint/nursery/useVueValidVText"
	| "lint/nursery/useVueVapor"
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
	| "lint/style/noJsxLiterals"
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
	| "lint/style/useConsistentArrowReturn"
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
	| "lint/suspicious/noDeprecatedImports"
	| "lint/suspicious/noDocumentCookie"
	| "lint/suspicious/noDocumentImportInPage"
	| "lint/suspicious/noDoubleEquals"
	| "lint/suspicious/noDuplicateAtImportRules"
	| "lint/suspicious/noDuplicateCase"
	| "lint/suspicious/noDuplicateClassMembers"
	| "lint/suspicious/noDuplicateCustomProperties"
	| "lint/suspicious/noDuplicateDependencies"
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
	| "lint/suspicious/noEmptySource"
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
	| "lint/suspicious/noImportCycles"
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
	| "lint/suspicious/noReactForwardRef"
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
	| "lint/suspicious/noUnusedExpressions"
	| "lint/suspicious/noUselessEscapeInString"
	| "lint/suspicious/noUselessRegexBackrefs"
	| "lint/suspicious/noVar"
	| "lint/suspicious/noWith"
	| "lint/suspicious/useAdjacentOverloadSignatures"
	| "lint/suspicious/useAwait"
	| "lint/suspicious/useBiomeIgnoreFolder"
	| "lint/suspicious/useDefaultSwitchClauseLast"
	| "lint/suspicious/useDeprecatedDate"
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
	| "assist/source/noDuplicateClasses"
	| "assist/source/useSortedInterfaceMembers"
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
	| "project"
	| "typeAware";
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
	inlineConfig?: Configuration;
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
	| { Grit: GritFileSource }
	| { Markdown: MdFileSource };
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
	/**
	* Used to mark if the CSS is embedded inside some particular files. This affects the parsing.
For example, if inside a styled`` literal, a top-level declaration is allowed. 
	 */
	embeddingKind: EmbeddingKind2;
	language: CssFileLanguage;
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
export interface MdFileSource {
	variant: MarkdownVariant;
}
export type EmbeddingKind =
	| "None"
	| {
			Astro: {
				/**
				 * Whether the script is inside Astro frontmatter
				 */
				frontmatter: boolean;
			};
	  }
	| {
			Vue: {
				/**
				 * Whether this is a v-on event handler (e.g., @click="handler")
				 */
				event_handler: boolean;
				/**
				 * Where the bindings are defined
				 */
				is_source: boolean;
				/**
				 * Whether the script is inside script tag with setup attribute
				 */
				setup: boolean;
			};
	  }
	| {
			Svelte: {
				/**
				 * Where the bindings are defined
				 */
				is_source: boolean;
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
export type EmbeddingKind2 = "None" | "Styled" | { Html: EmbeddingHtmlKind };
/**
 * The language of the stylesheet.
 */
export type CssFileLanguage = "css" | "scss";
/**
	* Extra CSS features enabled for the file.

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
export type MarkdownVariant = "Standard";
export type EmbeddingHtmlKind = "None" | "Html" | "Vue" | "Astro" | "Svelte";
export type HtmlTextExpressions = "None" | "Single" | "Double";
export interface OpenFileResult {
	diagnostics: Diagnostic[];
}
export interface ChangeFileParams {
	content: string;
	inlineConfig?: Configuration;
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
	projectKey: ProjectKey;
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
	data: Record<string, SerializedModuleInfo>;
}
export type SerializedModuleInfo =
	| { js: SerializedJsModuleInfo }
	| { css: SerializedCssModuleInfo };
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
export interface SerializedCssModuleInfo {
	/**
	* Map of all static imports found in the module.

Maps from the local imported name to the absolute path it resolves to. 
	 */
	imports: string[];
}
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	/**
	 * Rules to apply on top of the configuration
	 */
	enabledRules?: AnalyzerSelector[];
	inlineConfig?: Configuration;
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
	inlineConfig?: Configuration;
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
	inlineConfig?: Configuration;
	only?: AnalyzerSelector[];
	path: BiomePath;
	projectKey: ProjectKey;
	skip?: AnalyzerSelector[];
}
export interface PullDiagnosticsAndActionsResult {
	diagnostics: [Diagnostic, CodeAction[]][];
}
export interface FormatFileParams {
	inlineConfig?: Configuration;
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
	inlineConfig?: Configuration;
	path: BiomePath;
	projectKey: ProjectKey;
	range: TextRange;
}
export interface FormatOnTypeParams {
	inlineConfig?: Configuration;
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
	inlineConfig?: Configuration;
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
export type GritTargetLanguage = "CSS" | "JavaScript" | "JSON";
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
