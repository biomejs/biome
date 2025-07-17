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
export type FeatureKind = string;
export interface FileFeaturesResult {
	featuresSupported: FeaturesSupported;
}
export type FeaturesSupported = { [K in FeatureKind]?: SupportKind };
export interface SupportKind {}
export interface UpdateSettingsParams {
	configuration: Configuration;
	projectKey: ProjectKey;
	workspaceDirectory: BiomePath | any;
}
/**
 * The configuration that is contained inside the file `biome.json`
 */
export interface Configuration {
	/**
	 * A field for the [JSON schema](https://json-schema.org/) specification
	 */
	$schema: Schema | null;
	/**
	 * Specific configuration for assists
	 */
	assist: AssistConfiguration | null;
	/**
	 * Specific configuration for the Css language
	 */
	css: CssConfiguration | null;
	/**
	 * A list of paths to other JSON files, used to extends the current configuration.
	 */
	extends: Extends | null;
	/**
	 * The configuration of the filesystem
	 */
	files: FilesConfiguration | null;
	/**
	 * The configuration of the formatter
	 */
	formatter: FormatterConfiguration | null;
	/**
	 * Specific configuration for the GraphQL language
	 */
	graphql: GraphqlConfiguration | null;
	/**
	 * Specific configuration for the GraphQL language
	 */
	grit: GritConfiguration | null;
	/**
	 * Specific configuration for the HTML language
	 */
	html: HtmlConfiguration | null;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript: JsConfiguration | null;
	/**
	 * Specific configuration for the Json language
	 */
	json: JsonConfiguration | null;
	/**
	 * The configuration for the linter
	 */
	linter: LinterConfiguration | null;
	/**
	 * A list of granular patterns that should be applied only to a sub set of files
	 */
	overrides: Overrides | null;
	/**
	 * List of plugins to load.
	 */
	plugins: Plugins | null;
	/**
	* Indicates whether this configuration file is at the root of a Biome
project. By default, this is `true`. 
	 */
	root: Bool | null;
	/**
	 * The configuration of the VCS integration
	 */
	vcs: VcsConfiguration | null;
}
export type Schema = string;
export interface AssistConfiguration {
	/**
	 * Whether Biome should fail in CLI if the assist were not applied to the code.
	 */
	actions: Actions | null;
	/**
	 * Whether Biome should enable assist via LSP and CLI.
	 */
	enabled: Bool | null;
	/**
	* A list of glob patterns. Biome will include files/folders that will
match these patterns. 
	 */
	includes: ;
}
/**
 * Options applied to CSS files
 */
export interface CssConfiguration {
	/**
	 * CSS assist options
	 */
	assist?: CssAssistConfiguration | null;
	/**
	 * CSS formatter options
	 */
	formatter?: CssFormatterConfiguration | null;
	/**
	 * CSS globals
	 */
	globals: ;
	/**
	 * CSS linter options
	 */
	linter?: CssLinterConfiguration | null;
	/**
	 * CSS parsing options
	 */
	parser?: CssParserConfiguration | null;
}
export interface Extends {}
/**
 * The configuration of the filesystem
 */
export interface FilesConfiguration {
	/**
	* Set of file and folder names that should be unconditionally ignored by
Biome's scanner.

Biome maintains an internal list of default ignore entries, which is
based on user feedback and which may change in any release. This setting
allows overriding this internal list completely.

This is considered an advanced feature that users _should_ not need to
tweak themselves, but they can as a last resort. This setting can only
be configured in root configurations, and is ignored in nested configs.

Entries must be file or folder *names*. Specific paths and globs are not
supported.

Examples where this may be useful:

```jsonc
{
    "files": {
        "experimentalScannerIgnores": [
            // You almost certainly don't want to scan your `.git`
            // folder, which is why it's already ignored by default:
            ".git",

            // But the scanner does scan `node_modules` by default. If
            // you *really* don't want this, you can ignore it like
            // this:
            "node_modules",

            // But it's probably better to ignore a specific dependency.
            // For instance, one that happens to be particularly slow to
            // scan:
            "RedisCommander.d.ts",
        ],
    }
}
```

Please be aware that rules relying on the module graph or type inference
information may be negatively affected if dependencies of your project
aren't (fully) scanned. 
	 */
	experimentalScannerIgnores: ;
	/**
	 * Tells Biome to not emit diagnostics when handling files that doesn't know
	 */
	ignoreUnknown: Bool | null;
	/**
	* A list of glob patterns. Biome will handle only those files/folders that will
match these patterns. 
	 */
	includes: ;
	/**
	* The maximum allowed size for source code files in bytes. Files above
this limit will be ignored for performance reasons. Defaults to 1 MiB 
	 */
	maxSize: MaxSize | null;
}
/**
 * Generic options applied to all files
 */
export interface FormatterConfiguration {
	/**
	 * The attribute position style in HTML-ish languages. Defaults to auto.
	 */
	attributePosition: AttributePosition | null;
	/**
	 * Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
	 */
	bracketSameLine: BracketSameLine | null;
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing: BracketSpacing | null;
	enabled: Bool | null;
	/**
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
	 */
	expand: Expand | null;
	/**
	* Stores whether formatting should be allowed to proceed if a given file
has syntax errors 
	 */
	formatWithErrors: Bool | null;
	/**
	* A list of glob patterns. The formatter will include files/folders that will
match these patterns. 
	 */
	includes: ;
	/**
	 * The indent style.
	 */
	indentStyle: IndentStyle | null;
	/**
	 * The size of the indentation, 2 by default
	 */
	indentWidth: IndentWidth | null;
	/**
	 * The type of line ending.
	 */
	lineEnding: LineEnding | null;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth: LineWidth | null;
	/**
	* Use any `.editorconfig` files to configure the formatter. Configuration
in `biome.json` will override `.editorconfig` configuration.

Default: `true`. 
	 */
	useEditorconfig: Bool | null;
}
/**
 * Options applied to GraphQL files
 */
export interface GraphqlConfiguration {
	/**
	 * Assist options
	 */
	assist: GraphqlAssistConfiguration | null;
	/**
	 * GraphQL formatter options
	 */
	formatter: GraphqlFormatterConfiguration | null;
	linter: GraphqlLinterConfiguration | null;
}
/**
 * Options applied to GritQL files
 */
export interface GritConfiguration {
	/**
	 * Assist options
	 */
	assist: GritAssistConfiguration | null;
	/**
	 * Formatting options
	 */
	formatter: GritFormatterConfiguration | null;
	/**
	 * Formatting options
	 */
	linter: GritLinterConfiguration | null;
}
/**
 * Options applied to HTML files
 */
export interface HtmlConfiguration {
	/**
	 * HTML formatter options
	 */
	formatter: HtmlFormatterConfiguration | null;
	/**
	 * HTML parsing options
	 */
	parser: HtmlParserConfiguration | null;
}
/**
 * A set of options applied to the JavaScript files
 */
export interface JsConfiguration {
	/**
	 * Assist options
	 */
	assist: JsAssistConfiguration | null;
	/**
	 * Formatting options
	 */
	formatter: JsFormatterConfiguration | null;
	/**
	* A list of global bindings that should be ignored by the analyzers

If defined here, they should not emit diagnostics. 
	 */
	globals: ;
	/**
	 * Indicates the type of runtime or transformation used for interpreting JSX.
	 */
	jsxRuntime: JsxRuntime | null;
	/**
	 * Linter options
	 */
	linter: JsLinterConfiguration | null;
	/**
	 * Parsing options
	 */
	parser: JsParserConfiguration | null;
}
/**
 * Options applied to JSON files
 */
export interface JsonConfiguration {
	/**
	 * Assist options
	 */
	assist: JsonAssistConfiguration | null;
	/**
	 * Formatting options
	 */
	formatter: JsonFormatterConfiguration | null;
	/**
	 * Linting options
	 */
	linter: JsonLinterConfiguration | null;
	/**
	 * Parsing options
	 */
	parser: JsonParserConfiguration | null;
}
export interface LinterConfiguration {
	/**
	 * An object where the keys are the names of the domains, and the values are `all`, `recommended`, or `none`.
	 */
	domains: RuleDomains | null;
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled: Bool | null;
	/**
	* A list of glob patterns. The analyzer will handle only those files/folders that will
match these patterns. 
	 */
	includes: ;
	/**
	 * List of rules
	 */
	rules: Rules | null;
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
	clientKind: VcsClientKind | null;
	/**
	 * The main branch of the project
	 */
	defaultBranch: ;
	/**
	 * Whether Biome should integrate itself with the VCS client
	 */
	enabled: Bool | null;
	/**
	* The folder where Biome should check for VCS files. By default, Biome will use the same
folder where `biome.json` was found.

If Biome can't find the configuration, it will attempt to use the current working directory.
If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic
will be emitted 
	 */
	root: ;
	/**
	* Whether Biome should use the VCS ignore file. When [true], Biome will ignore the files
specified in the ignore file. 
	 */
	useIgnoreFile: Bool | null;
}
export interface Actions {
	/**
	 * It enables the assist actions recommended by Biome. `true` by default.
	 */
	recommended: ;
	source: Source | null;
}
/**
 * Options that changes how the CSS assist behaves
 */
export interface CssAssistConfiguration {
	/**
	 * Control the assist for CSS files.
	 */
	enabled: Bool | null;
}
/**
 * Options that changes how the CSS formatter behaves
 */
export interface CssFormatterConfiguration {
	/**
	 * Control the formatter for CSS (and its super languages) files.
	 */
	enabled: Bool | null;
	/**
	 * The indent style applied to CSS (and its super languages) files.
	 */
	indentStyle: IndentStyle | null;
	/**
	 * The size of the indentation applied to CSS (and its super languages) files. Default to 2.
	 */
	indentWidth: IndentWidth | null;
	/**
	 * The type of line ending applied to CSS (and its super languages) files.
	 */
	lineEnding: LineEnding | null;
	/**
	 * What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
	 */
	lineWidth: LineWidth | null;
	/**
	 * The type of quotes used in CSS code. Defaults to double.
	 */
	quoteStyle: QuoteStyle | null;
}
/**
 * Options that changes how the CSS linter behaves
 */
export interface CssLinterConfiguration {
	/**
	 * Control the linter for CSS files.
	 */
	enabled: Bool | null;
}
/**
 * Options that changes how the CSS parser behaves
 */
export interface CssParserConfiguration {
	/**
	 * Allow comments to appear on incorrect lines in `.css` files
	 */
	allowWrongLineComments?: Bool | null;
	/**
	 * Enables parsing of CSS Modules specific features.
	 */
	cssModules?: Bool | null;
}
export type MaxSize = number;
export type AttributePosition = string;
/**
 * Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
 */
export type BracketSameLine = boolean;
export type BracketSpacing = boolean;
export interface Expand {}
export interface IndentStyle {}
export type IndentWidth = number;
export interface LineEnding {}
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
	enabled?: Bool | null;
}
/**
 * Options that changes how the GraphQL formatter behaves
 */
export interface GraphqlFormatterConfiguration {
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing?: BracketSpacing | null;
	/**
	 * Control the formatter for GraphQL files.
	 */
	enabled?: Bool | null;
	/**
	 * The indent style applied to GraphQL files.
	 */
	indentStyle?: IndentStyle | null;
	/**
	 * The size of the indentation applied to GraphQL files. Default to 2.
	 */
	indentWidth?: IndentWidth | null;
	/**
	 * The type of line ending applied to GraphQL files.
	 */
	lineEnding?: LineEnding | null;
	/**
	 * What's the max width of a line applied to GraphQL files. Defaults to 80.
	 */
	lineWidth?: LineWidth | null;
	/**
	 * The type of quotes used in GraphQL code. Defaults to double.
	 */
	quoteStyle?: QuoteStyle | null;
}
/**
 * Options that change how the GraphQL linter behaves.
 */
export interface GraphqlLinterConfiguration {
	/**
	 * Control the formatter for GraphQL files.
	 */
	enabled?: Bool | null;
}
export interface GritAssistConfiguration {
	/**
	 * Control the assist functionality for Grit files.
	 */
	enabled: Bool | null;
}
export interface GritFormatterConfiguration {
	/**
	 * Control the formatter for Grit files.
	 */
	enabled: Bool | null;
	/**
	 * The indent style applied to Grit files.
	 */
	indentStyle: IndentStyle | null;
	/**
	 * The size of the indentation applied to Grit files. Default to 2.
	 */
	indentWidth: IndentWidth | null;
	/**
	 * The type of line ending applied to Grit files.
	 */
	lineEnding: LineEnding | null;
	/**
	 * What's the max width of a line applied to Grit files. Defaults to 80.
	 */
	lineWidth: LineWidth | null;
}
export interface GritLinterConfiguration {
	/**
	 * Control the linter for Grit files.
	 */
	enabled: Bool | null;
}
/**
 * Options that changes how the HTML formatter behaves
 */
export interface HtmlFormatterConfiguration {
	/**
	 * The attribute position style in HTML elements. Defaults to auto.
	 */
	attributePosition: AttributePosition | null;
	/**
	 * Whether to hug the closing bracket of multiline HTML tags to the end of the last line, rather than being alone on the following line. Defaults to false.
	 */
	bracketSameLine: BracketSameLine | null;
	/**
	 * Control the formatter for HTML (and its super languages) files.
	 */
	enabled: Bool | null;
	/**
	 * Whether to indent the `<script>` and `<style>` tags for HTML (and its super languages). Defaults to false.
	 */
	indentScriptAndStyle: IndentScriptAndStyle | null;
	/**
	 * The indent style applied to HTML (and its super languages) files.
	 */
	indentStyle: IndentStyle | null;
	/**
	 * The size of the indentation applied to HTML (and its super languages) files. Default to 2.
	 */
	indentWidth: IndentWidth | null;
	/**
	 * The type of line ending applied to HTML (and its super languages) files.
	 */
	lineEnding: LineEnding | null;
	/**
	 * What's the max width of a line applied to HTML (and its super languages) files. Defaults to 80.
	 */
	lineWidth: LineWidth | null;
	/**
	 * Whether void elements should be self-closed. Defaults to never.
	 */
	selfCloseVoidElements: SelfCloseVoidElements | null;
	/**
	 * Whether to account for whitespace sensitivity when formatting HTML (and its super languages). Defaults to "css".
	 */
	whitespaceSensitivity: WhitespaceSensitivity | null;
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
	enabled: Bool | null;
}
/**
 * Formatting options specific to the JavaScript files
 */
export interface JsFormatterConfiguration {
	/**
	 * Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
	 */
	arrowParentheses: ArrowParentheses | null;
	/**
	 * The attribute position style in JSX elements. Defaults to auto.
	 */
	attributePosition: AttributePosition | null;
	/**
	 * Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
	 */
	bracketSameLine: BracketSameLine | null;
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing: BracketSpacing | null;
	/**
	 * Control the formatter for JavaScript (and its super languages) files.
	 */
	enabled: Bool | null;
	/**
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
	 */
	expand: Expand | null;
	/**
	 * The indent style applied to JavaScript (and its super languages) files.
	 */
	indentStyle: IndentStyle | null;
	/**
	 * The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
	 */
	indentWidth: IndentWidth | null;
	/**
	 * The type of quotes used in JSX. Defaults to double.
	 */
	jsxQuoteStyle: QuoteStyle | null;
	/**
	 * The type of line ending applied to JavaScript (and its super languages) files.
	 */
	lineEnding: LineEnding | null;
	/**
	 * What's the max width of a line applied to JavaScript (and its super languages) files. Defaults to 80.
	 */
	lineWidth: LineWidth | null;
	/**
	 * When properties in objects are quoted. Defaults to asNeeded.
	 */
	quoteProperties: QuoteProperties | null;
	/**
	 * The type of quotes used in JavaScript code. Defaults to double.
	 */
	quoteStyle: QuoteStyle | null;
	/**
	 * Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
	 */
	semicolons: Semicolons | null;
	/**
	 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
	 */
	trailingCommas: TrailingCommas | null;
}
/**
 * Indicates the type of runtime or transformation used for interpreting JSX.
 */
export interface JsxRuntime {}
/**
 * Linter options specific to the JavaScript linter
 */
export interface JsLinterConfiguration {
	/**
	 * Control the linter for JavaScript (and its super languages) files.
	 */
	enabled: Bool | null;
}
/**
 * Options that changes how the JavaScript parser behaves
 */
export interface JsParserConfiguration {
	/**
	* Enables parsing of Grit metavariables.
Defaults to `false`. 
	 */
	gritMetavariables: Bool | null;
	/**
	* When enabled, files like `.js`/`.mjs`/`.cjs` may contain JSX syntax.

Defaults to `true`. 
	 */
	jsxEverywhere: Bool | null;
	/**
	* It enables the experimental and unsafe parsing of parameter decorators

These decorators belong to an old proposal, and they are subject to change. 
	 */
	unsafeParameterDecoratorsEnabled: Bool | null;
}
/**
 * Linter options specific to the JSON linter
 */
export interface JsonAssistConfiguration {
	/**
	 * Control the assist for JSON (and its super languages) files.
	 */
	enabled: Bool | null;
}
export interface JsonFormatterConfiguration {
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing: BracketSpacing | null;
	/**
	 * Control the formatter for JSON (and its super languages) files.
	 */
	enabled: Bool | null;
	/**
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
	 */
	expand: Expand | null;
	/**
	 * The indent style applied to JSON (and its super languages) files.
	 */
	indentStyle: IndentStyle | null;
	/**
	 * The size of the indentation applied to JSON (and its super languages) files. Default to 2.
	 */
	indentWidth: IndentWidth | null;
	/**
	 * The type of line ending applied to JSON (and its super languages) files.
	 */
	lineEnding: LineEnding | null;
	/**
	 * What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
	 */
	lineWidth: LineWidth | null;
	/**
	 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "none".
	 */
	trailingCommas: TrailingCommas2 | null;
}
/**
 * Linter options specific to the JSON linter
 */
export interface JsonLinterConfiguration {
	/**
	 * Control the linter for JSON (and its super languages) files.
	 */
	enabled: Bool | null;
}
/**
 * Options that changes how the JSON parser behaves
 */
export interface JsonParserConfiguration {
	/**
	 * Allow parsing comments in `.json` files
	 */
	allowComments: Bool | null;
	/**
	 * Allow parsing trailing commas in `.json` files
	 */
	allowTrailingCommas: Bool | null;
}
export type RuleDomains = { [K in RuleDomain]?: RuleDomainValue };
export interface Rules {
	a11y: SeverityOrGroup | null;
	complexity: SeverityOrGroup2 | null;
	correctness: SeverityOrGroup3 | null;
	nursery: SeverityOrGroup4 | null;
	performance: SeverityOrGroup5 | null;
	/**
	 * It enables the lint rules recommended by Biome. `true` by default.
	 */
	recommended: ;
	security: SeverityOrGroup6 | null;
	style: SeverityOrGroup7 | null;
	suspicious: SeverityOrGroup8 | null;
}
export interface OverridePattern {
	/**
	 * Specific configuration for the Json language
	 */
	assist: OverrideAssistConfiguration | null;
	/**
	 * Specific configuration for the CSS language
	 */
	css: CssConfiguration | null;
	/**
	 * Specific configuration for the filesystem
	 */
	files: OverrideFilesConfiguration | null;
	/**
	 * Specific configuration for the Json language
	 */
	formatter: OverrideFormatterConfiguration | null;
	/**
	 * Specific configuration for the Graphql language
	 */
	graphql: GraphqlConfiguration | null;
	/**
	 * Specific configuration for the GritQL language
	 */
	grit: GritConfiguration | null;
	/**
	 * Specific configuration for the GritQL language
	 */
	html: HtmlConfiguration | null;
	/**
	* A list of glob patterns. Biome will include files/folders that will
match these patterns. 
	 */
	includes: OverrideGlobs | null;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript: JsConfiguration | null;
	/**
	 * Specific configuration for the Json language
	 */
	json: JsonConfiguration | null;
	/**
	 * Specific configuration for the Json language
	 */
	linter: OverrideLinterConfiguration | null;
	/**
	 * Specific configuration for additional plugins
	 */
	plugins: Plugins | null;
}
export interface PluginConfiguration {}
export interface VcsClientKind {}
/**
 * A list of rules that belong to this group
 */
export interface Source {
	/**
	 * Provides a code action to sort the imports and exports in the file using a built-in or custom order.
	 */
	organizeImports: RuleAssistConfiguration | null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended: ;
	/**
	 * Enforce attribute sorting in JSX elements.
	 */
	useSortedAttributes: RuleAssistConfiguration2 | null;
	/**
	 * Sorts the keys of a JSON object in natural order
	 */
	useSortedKeys: RuleAssistConfiguration3 | null;
	/**
	 * Enforce ordering of CSS properties and nested rules.
	 */
	useSortedProperties: RuleAssistConfiguration4 | null;
}
export type QuoteStyle = string;
/**
	* Whether to indent the content of `<script>` and `<style>` tags for HTML-ish templating languages (Vue, Svelte, etc.).

When true, the content of `<script>` and `<style>` tags will be indented one level. 
	 */
export type IndentScriptAndStyle = boolean;
/**
 * Controls whether void-elements should be self closed
 */
export interface SelfCloseVoidElements {}
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
export interface WhitespaceSensitivity {}
export type ArrowParentheses = string;
export type QuoteProperties = string;
export type Semicolons = string;
/**
 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures.
 */
export interface TrailingCommas {}
export interface TrailingCommas2 {}
/**
 * Rule domains
 */
export interface RuleDomain {}
export interface RuleDomainValue {}
export interface SeverityOrGroup {}
export interface SeverityOrGroup2 {}
export interface SeverityOrGroup3 {}
export interface SeverityOrGroup4 {}
export interface SeverityOrGroup5 {}
export interface SeverityOrGroup6 {}
export interface SeverityOrGroup7 {}
export interface SeverityOrGroup8 {}
export interface OverrideAssistConfiguration {
	/**
	 * List of actions
	 */
	actions: Actions | null;
	/**
	 * if `false`, it disables the feature and the assist won't be executed. `true` by default
	 */
	enabled: Bool | null;
}
export interface OverrideFilesConfiguration {
	/**
	 * File size limit in bytes
	 */
	maxSize: MaxSize | null;
}
export interface OverrideFormatterConfiguration {
	/**
	 * The attribute position style.
	 */
	attributePosition: AttributePosition | null;
	/**
	 * Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
	 */
	bracketSameLine: BracketSameLine | null;
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing: BracketSpacing | null;
	enabled: Bool | null;
	/**
	* Whether to expand arrays and objects on multiple lines.
When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
and array literals are formatted on a single line if it fits in the line.
When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
When set to `never`, these literals are formatted on a single line if it fits in the line.
When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto". 
	 */
	expand: Expand | null;
	/**
	* Stores whether formatting should be allowed to proceed if a given file
has syntax errors 
	 */
	formatWithErrors: Bool | null;
	/**
	 * The size of the indentation, 2 by default (deprecated, use `indent-width`)
	 */
	indentSize: IndentWidth | null;
	/**
	 * The indent style.
	 */
	indentStyle: IndentStyle | null;
	/**
	 * The size of the indentation, 2 by default
	 */
	indentWidth: IndentWidth | null;
	/**
	 * The type of line ending.
	 */
	lineEnding: LineEnding | null;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth: LineWidth | null;
}
export type OverrideGlobs = Glob[];
export interface OverrideLinterConfiguration {
	/**
	 * List of rules
	 */
	domains: RuleDomains | null;
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled: Bool | null;
	/**
	 * List of rules
	 */
	rules: Rules | null;
}
export interface RuleAssistConfiguration {}
export interface RuleAssistConfiguration2 {}
export interface RuleAssistConfiguration3 {}
export interface RuleAssistConfiguration4 {}
export type Glob = string;
export interface UpdateSettingsResult {
	diagnostics: Diagnostic[];
}
/**
 * Serializable representation for a [Diagnostic](super::Diagnostic).
 */
export interface Diagnostic {
	advices: Advices;
	category: Category | null;
	description: string;
	location: Location;
	message: MarkupBuf;
	severity: Severity;
	source: Diagnostic | null;
	tags: DiagnosticTags;
	verboseAdvices: Advices;
}
/**
 * Implementation of [Visitor] collecting serializable [Advice] into a vector.
 */
export interface Advices {
	advices: Advice[];
}
export interface Category {}
export interface Location {
	path: Resource | null;
	sourceCode: ;
	span: TextRange | null;
}
export type MarkupBuf = MarkupNodeBuf[];
/**
 * The severity to associate to a diagnostic.
 */
export interface Severity {}
export type DiagnosticTags = DiagnosticTag[];
/**
	* Serializable representation of a [Diagnostic](super::Diagnostic) advice

See the [Visitor] trait for additional documentation on all the supported
advice types. 
	 */
export interface Advice {}
/**
 * Represents the resource a diagnostic is associated with.
 */
export interface Resource {}
export type TextRange = any;
export interface MarkupNodeBuf {
	content: string;
	elements: MarkupElement[];
}
/**
	* Internal enum used to automatically generate bit offsets for [DiagnosticTags]
and help with the implementation of `serde` and `schemars` for tags. 
	 */
export type DiagnosticTag = string;
/**
 * Enumeration of all the supported markup elements
 */
export interface MarkupElement {}
export interface OpenProjectParams {
	/**
	 * Whether the client wants to run only certain rules. This is needed to compute the kind of [ScanKind].
	 */
	onlyRules?: RuleCode[];
	/**
	* Whether the folder should be opened as a project, even if no
`biome.json` can be found. 
	 */
	openUninitialized: boolean;
	/**
	 * The path to open
	 */
	path: BiomePath;
	/**
	 * Whether the client wants to skip some lint rule. This is needed to compute the kind of [ScanKind].
	 */
	skipRules?: RuleCode[];
}
export type RuleCode = string;
export interface OpenProjectResult {
	/**
	 * A unique identifier for this project
	 */
	projectKey: ProjectKey;
	/**
	 * How to scan this project
	 */
	scanKind: ScanKind;
}
export interface ScanKind {}
export interface OpenFileParams {
	content: FileContent;
	documentFileSource: DocumentFileSource | any;
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
export interface FileContent {}
export interface DocumentFileSource {}
export interface null {}
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
export interface FileExitsParams {
	filePath: BiomePath;
}
export type boolean = boolean;
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
export type string = string;
export interface GetControlFlowGraphParams {
	cursor: TextSize;
	path: BiomePath;
	projectKey: ProjectKey;
}
export type TextSize = number;
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
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	/**
	 * Rules to apply on top of the configuration
	 */
	enabledRules?: RuleCode[];
	only?: RuleCode[];
	path: BiomePath;
	projectKey: ProjectKey;
	/**
	 * When `false` the diagnostics, don't have code frames of the code actions (fixes, suppressions, etc.)
	 */
	pullCodeActions: boolean;
	skip?: RuleCode[];
}
export type RuleCategories = RuleCategory[];
export interface RuleCategory {}
export interface PullDiagnosticsResult {
	diagnostics: Diagnostic[];
	errors: number;
	skippedDiagnostics: number;
}
export interface PullActionsParams {
	categories?: RuleCategories;
	enabledRules?: RuleCode[];
	only?: RuleCode[];
	path: BiomePath;
	projectKey: ProjectKey;
	range: TextRange | any;
	skip?: RuleCode[];
	suppressionReason?: string;
}
export interface PullActionsResult {
	actions: CodeAction[];
}
export interface CodeAction {
	category: ActionCategory;
	ruleName: ;
	suggestion: CodeSuggestion;
}
/**
	* The category of a code action, this type maps directly to the
[CodeActionKind] type in the Language Server Protocol specification

[CodeActionKind]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind 
	 */
export interface ActionCategory {}
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
 * Indicates how a tool should manage this suggestion.
 */
export interface Applicability {}
export interface TextEdit {
	dictionary: string;
	ops: CompressedOp[];
}
export interface CompressedOp {}
export interface FormatFileParams {
	path: BiomePath;
	projectKey: ProjectKey;
}
export interface Printed {
	code: string;
	range: TextRange | any;
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
export interface FixFileMode {}
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
	rule_name: ;
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
export interface GritTargetLanguage {}
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
	openFile(params: OpenFileParams): Promise<null>;
	changeFile(params: ChangeFileParams): Promise<null>;
	closeFile(params: CloseFileParams): Promise<null>;
	getSyntaxTree(params: GetSyntaxTreeParams): Promise<GetSyntaxTreeResult>;
	fileExists(params: FileExitsParams): Promise<boolean>;
	checkFileSize(params: CheckFileSizeParams): Promise<CheckFileSizeResult>;
	getFileContent(params: GetFileContentParams): Promise<string>;
	getControlFlowGraph(params: GetControlFlowGraphParams): Promise<string>;
	getFormatterIr(params: GetFormatterIRParams): Promise<string>;
	getTypeInfo(params: GetTypeInfoParams): Promise<string>;
	getRegisteredTypes(params: GetRegisteredTypesParams): Promise<string>;
	getSemanticModel(params: GetSemanticModelParams): Promise<string>;
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
		fileExists(params) {
			return transport.request("biome/file_exists", params);
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
