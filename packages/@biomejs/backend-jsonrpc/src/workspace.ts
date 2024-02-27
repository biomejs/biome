// Generated file, do not edit by hand, see `xtask/codegen`
import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	feature: FeatureName[];
	path: BiomePath;
}
export type FeatureName = "Format" | "Lint" | "OrganizeImports";
export interface BiomePath {
	path: string;
}
export interface SupportsFeatureResult {
	reason?: SupportKind;
}
export type SupportKind =
	| "Supported"
	| "Ignored"
	| "Protected"
	| "FeatureNotEnabled"
	| "FileNotSupported";
export interface UpdateSettingsParams {
	configuration: PartialConfiguration;
	gitignore_matches: string[];
	vcs_base_path?: string;
	working_directory?: string;
}
/**
 * The configuration that is contained inside the file `biome.json`
 */
export interface PartialConfiguration {
	/**
	 * A field for the [JSON schema](https://json-schema.org/) specification
	 */
	$schema?: string;
	/**
	 * Specific configuration for the Css language
	 */
	css?: PartialCssConfiguration;
	/**
	 * A list of paths to other JSON files, used to extends the current configuration.
	 */
	extends?: StringSet;
	/**
	 * The configuration of the filesystem
	 */
	files?: PartialFilesConfiguration;
	/**
	 * The configuration of the formatter
	 */
	formatter?: PartialFormatterConfiguration;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript?: PartialJavascriptConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	json?: PartialJsonConfiguration;
	/**
	 * The configuration for the linter
	 */
	linter?: PartialLinterConfiguration;
	/**
	 * The configuration of the import sorting
	 */
	organizeImports?: PartialOrganizeImports;
	/**
	 * A list of granular patterns that should be applied only to a sub set of files
	 */
	overrides?: Overrides;
	/**
	 * The configuration of the VCS integration
	 */
	vcs?: PartialVcsConfiguration;
}
/**
 * Options applied to CSS files
 */
export interface PartialCssConfiguration {
	/**
	 * Formatting options
	 */
	formatter?: PartialCssFormatter;
	/**
	 * Parsing options
	 */
	parser?: PartialCssParser;
}
export type StringSet = string[];
/**
 * The configuration of the filesystem
 */
export interface PartialFilesConfiguration {
	/**
	 * A list of Unix shell style patterns. Biome will ignore files/folders that will match these patterns.
	 */
	ignore?: StringSet;
	/**
	 * Tells Biome to not emit diagnostics when handling files that doesn't know
	 */
	ignoreUnknown?: boolean;
	/**
	 * A list of Unix shell style patterns. Biome will handle only those files/folders that will match these patterns.
	 */
	include?: StringSet;
	/**
	 * The maximum allowed size for source code files in bytes. Files above this limit will be ignored for performance reasons. Defaults to 1 MiB
	 */
	maxSize?: number;
}
/**
 * Generic options applied to all files
 */
export interface PartialFormatterConfiguration {
	/**
	 * The attribute position style. By default auto.
	 */
	attributePosition?: AttributePosition;
	enabled?: boolean;
	/**
	 * Stores whether formatting should be allowed to proceed if a given file has syntax errors
	 */
	formatWithErrors?: boolean;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: StringSet;
	/**
	 * A list of Unix shell style patterns. The formatter will include files/folders that will match these patterns.
	 */
	include?: StringSet;
	/**
	 * The size of the indentation, 2 by default (deprecated, use `indent-width`)
	 */
	indentSize?: number;
	/**
	 * The indent style.
	 */
	indentStyle?: PlainIndentStyle;
	/**
	 * The size of the indentation, 2 by default
	 */
	indentWidth?: number;
	/**
	 * The type of line ending.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth?: LineWidth;
}
/**
 * A set of options applied to the JavaScript files
 */
export interface PartialJavascriptConfiguration {
	/**
	 * Formatting options
	 */
	formatter?: PartialJavascriptFormatter;
	/**
	* A list of global bindings that should be ignored by the analyzers

If defined here, they should not emit diagnostics. 
	 */
	globals?: StringSet;
	organize_imports?: PartialJavascriptOrganizeImports;
	/**
	 * Parsing options
	 */
	parser?: PartialJavascriptParser;
}
/**
 * Options applied to JSON files
 */
export interface PartialJsonConfiguration {
	/**
	 * Formatting options
	 */
	formatter?: PartialJsonFormatter;
	/**
	 * Parsing options
	 */
	parser?: PartialJsonParser;
}
export interface PartialLinterConfiguration {
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: boolean;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: StringSet;
	/**
	 * A list of Unix shell style patterns. The formatter will include files/folders that will match these patterns.
	 */
	include?: StringSet;
	/**
	 * List of rules
	 */
	rules?: Rules;
}
export interface PartialOrganizeImports {
	/**
	 * Enables the organization of imports
	 */
	enabled?: boolean;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: StringSet;
	/**
	 * A list of Unix shell style patterns. The formatter will include files/folders that will match these patterns.
	 */
	include?: StringSet;
}
export type Overrides = OverridePattern[];
/**
 * Set of properties to integrate Biome with a VCS software.
 */
export interface PartialVcsConfiguration {
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
	enabled?: boolean;
	/**
	* The folder where Biome should check for VCS files. By default, Biome will use the same folder where `biome.json` was found.

If Biome can't find the configuration, it will attempt to use the current working directory. If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic will be emitted 
	 */
	root?: string;
	/**
	 * Whether Biome should use the VCS ignore file. When [true], Biome will ignore the files specified in the ignore file.
	 */
	useIgnoreFile?: boolean;
}
export interface PartialCssFormatter {
	/**
	 * Control the formatter for CSS (and its super languages) files.
	 */
	enabled?: boolean;
	/**
	 * The size of the indentation applied to CSS (and its super languages) files. Default to 2.
	 */
	indentSize?: number;
	/**
	 * The indent style applied to CSS (and its super languages) files.
	 */
	indentStyle?: PlainIndentStyle;
	/**
	 * The size of the indentation applied to CSS (and its super languages) files. Default to 2.
	 */
	indentWidth?: number;
	/**
	 * The type of line ending applied to CSS (and its super languages) files.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to CSS (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
	quoteStyle?: QuoteStyle;
}
/**
 * Options that changes how the CSS parser behaves
 */
export interface PartialCssParser {
	/**
	 * Allow comments to appear on incorrect lines in `.css` files
	 */
	allowWrongLineComments?: boolean;
}
export type AttributePosition = "auto" | "multiline";
export type PlainIndentStyle = "tab" | "space";
export type LineEnding = "lf" | "crlf" | "cr";
/**
	* Validated value for the `line_width` formatter options

The allowed range of values is 1..=320 
	 */
export type LineWidth = number;
/**
 * Formatting options specific to the JavaScript files
 */
export interface PartialJavascriptFormatter {
	/**
	 * Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
	 */
	arrowParentheses?: ArrowParentheses;
	/**
	 * The attribute position style in JavaScript code. Defaults to auto.
	 */
	attributePosition?: AttributePosition;
	/**
	 * Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
	 */
	bracketSameLine?: boolean;
	/**
	 * Whether to insert spaces around brackets in object literals. Defaults to true.
	 */
	bracketSpacing?: boolean;
	/**
	 * Control the formatter for JavaScript (and its super languages) files.
	 */
	enabled?: boolean;
	/**
	 * The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
	 */
	indentSize?: number;
	/**
	 * The indent style applied to JavaScript (and its super languages) files.
	 */
	indentStyle?: PlainIndentStyle;
	/**
	 * The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
	 */
	indentWidth?: number;
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
	trailingComma?: TrailingComma;
}
export interface PartialJavascriptOrganizeImports {}
/**
 * Options that changes how the JavaScript parser behaves
 */
export interface PartialJavascriptParser {
	/**
	* It enables the experimental and unsafe parsing of parameter decorators

These decorators belong to an old proposal, and they are subject to change. 
	 */
	unsafeParameterDecoratorsEnabled?: boolean;
}
export interface PartialJsonFormatter {
	/**
	 * Control the formatter for JSON (and its super languages) files.
	 */
	enabled?: boolean;
	/**
	 * The size of the indentation applied to JSON (and its super languages) files. Default to 2.
	 */
	indentSize?: number;
	/**
	 * The indent style applied to JSON (and its super languages) files.
	 */
	indentStyle?: PlainIndentStyle;
	/**
	 * The size of the indentation applied to JSON (and its super languages) files. Default to 2.
	 */
	indentWidth?: number;
	/**
	 * The type of line ending applied to JSON (and its super languages) files.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line applied to JSON (and its super languages) files. Defaults to 80.
	 */
	lineWidth?: LineWidth;
}
/**
 * Options that changes how the JSON parser behaves
 */
export interface PartialJsonParser {
	/**
	 * Allow parsing comments in `.json` files
	 */
	allowComments?: boolean;
	/**
	 * Allow parsing trailing commas in `.json` files
	 */
	allowTrailingCommas?: boolean;
}
export interface Rules {
	a11y?: A11y;
	/**
	 * It enables ALL rules. The rules that belong to `nursery` won't be enabled.
	 */
	all?: boolean;
	complexity?: Complexity;
	correctness?: Correctness;
	nursery?: Nursery;
	performance?: Performance;
	/**
	 * It enables the lint rules recommended by Biome. `true` by default.
	 */
	recommended?: boolean;
	security?: Security;
	style?: Style;
	suspicious?: Suspicious;
}
export interface OverridePattern {
	/**
	 * Specific configuration for the Css language
	 */
	css?: PartialCssConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	formatter?: OverrideFormatterConfiguration;
	/**
	 * A list of Unix shell style patterns. The formatter will ignore files/folders that will match these patterns.
	 */
	ignore?: StringSet;
	/**
	 * A list of Unix shell style patterns. The formatter will include files/folders that will match these patterns.
	 */
	include?: StringSet;
	/**
	 * Specific configuration for the JavaScript language
	 */
	javascript?: PartialJavascriptConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	json?: PartialJsonConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	linter?: OverrideLinterConfiguration;
	/**
	 * Specific configuration for the Json language
	 */
	organizeImports?: OverrideOrganizeImportsConfiguration;
}
export type VcsClientKind = "git";
export type QuoteStyle = "double" | "single";
export type ArrowParentheses = "always" | "asNeeded";
export type QuoteProperties = "asNeeded" | "preserve";
export type Semicolons = "always" | "asNeeded";
/**
 * Print trailing commas wherever possible in multi-line comma-separated syntactic structures.
 */
export type TrailingComma = "all" | "es5" | "none";
/**
 * A list of rules that belong to this group
 */
export interface A11y {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Enforce that the accessKey attribute is not used on any HTML element.
	 */
	noAccessKey?: RuleConfiguration_for_Null;
	/**
	 * Enforce that aria-hidden="true" is not set on focusable elements.
	 */
	noAriaHiddenOnFocusable?: RuleConfiguration_for_Null;
	/**
	 * Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
	 */
	noAriaUnsupportedElements?: RuleConfiguration_for_Null;
	/**
	 * Enforce that autoFocus prop is not used on elements.
	 */
	noAutofocus?: RuleConfiguration_for_Null;
	/**
	 * Disallow target="_blank" attribute without rel="noreferrer"
	 */
	noBlankTarget?: RuleConfiguration_for_Null;
	/**
	 * Enforces that no distracting elements are used.
	 */
	noDistractingElements?: RuleConfiguration_for_Null;
	/**
	 * The scope prop should be used only on <th> elements.
	 */
	noHeaderScope?: RuleConfiguration_for_Null;
	/**
	 * Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
	 */
	noInteractiveElementToNoninteractiveRole?: RuleConfiguration_for_Null;
	/**
	 * Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveElementToInteractiveRole?: RuleConfiguration_for_Null;
	/**
	 * Enforce that tabIndex is not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveTabindex?: RuleConfiguration_for_Null;
	/**
	 * Prevent the usage of positive integers on tabIndex property
	 */
	noPositiveTabindex?: RuleConfiguration_for_Null;
	/**
	 * Enforce img alt prop does not contain the word "image", "picture", or "photo".
	 */
	noRedundantAlt?: RuleConfiguration_for_Null;
	/**
	 * Enforce explicit role property is not the same as implicit/default role property on an element.
	 */
	noRedundantRoles?: RuleConfiguration_for_Null;
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
	useAnchorContent?: RuleConfiguration_for_Null;
	/**
	 * Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant.
	 */
	useAriaActivedescendantWithTabindex?: RuleConfiguration_for_Null;
	/**
	 * Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
	 */
	useAriaPropsForRole?: RuleConfiguration_for_Null;
	/**
	 * Enforces the usage of the attribute type for the element button
	 */
	useButtonType?: RuleConfiguration_for_Null;
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
	 * Enforce that all anchors are valid, and they are navigable elements.
	 */
	useValidAnchor?: RuleConfiguration_for_Null;
	/**
	 * Ensures that ARIA properties aria-* are all valid.
	 */
	useValidAriaProps?: RuleConfiguration_for_Null;
	/**
	 * Elements with ARIA roles must use a valid, non-abstract ARIA role.
	 */
	useValidAriaRole?: RuleConfiguration_for_ValidAriaRoleOptions;
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
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Disallow primitive type aliases and misleading types.
	 */
	noBannedTypes?: RuleConfiguration_for_Null;
	/**
	 * Disallow functions that exceed a given Cognitive Complexity score.
	 */
	noExcessiveCognitiveComplexity?: RuleConfiguration_for_ComplexityOptions;
	/**
	 * Disallow unnecessary boolean casts
	 */
	noExtraBooleanCast?: RuleConfiguration_for_Null;
	/**
	 * Prefer for...of statement instead of Array.forEach.
	 */
	noForEach?: RuleConfiguration_for_Null;
	/**
	 * Disallow unclear usage of consecutive space characters in regular expression literals
	 */
	noMultipleSpacesInRegularExpressionLiterals?: RuleConfiguration_for_Null;
	/**
	 * This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
	 */
	noStaticOnlyClass?: RuleConfiguration_for_Null;
	/**
	 * Disallow this and super in static contexts.
	 */
	noThisInStatic?: RuleConfiguration_for_Null;
	/**
	 * Disallow unnecessary catch clauses.
	 */
	noUselessCatch?: RuleConfiguration_for_Null;
	/**
	 * Disallow unnecessary constructors.
	 */
	noUselessConstructor?: RuleConfiguration_for_Null;
	/**
	 * Disallow empty exports that don't change anything in a module file.
	 */
	noUselessEmptyExport?: RuleConfiguration_for_Null;
	/**
	 * Disallow unnecessary fragments
	 */
	noUselessFragments?: RuleConfiguration_for_Null;
	/**
	 * Disallow unnecessary labels.
	 */
	noUselessLabel?: RuleConfiguration_for_Null;
	/**
	 * Disallow renaming import, export, and destructured assignments to the same name.
	 */
	noUselessRename?: RuleConfiguration_for_Null;
	/**
	 * Disallow useless case in switch statements.
	 */
	noUselessSwitchCase?: RuleConfiguration_for_Null;
	/**
	 * Disallow useless this aliasing.
	 */
	noUselessThisAlias?: RuleConfiguration_for_Null;
	/**
	 * Disallow using any or unknown as type constraint.
	 */
	noUselessTypeConstraint?: RuleConfiguration_for_Null;
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
	useArrowFunction?: RuleConfiguration_for_Null;
	/**
	 * Promotes the use of .flatMap() when map().flat() are used together.
	 */
	useFlatMap?: RuleConfiguration_for_Null;
	/**
	 * Enforce the usage of a literal access to properties over computed property access.
	 */
	useLiteralKeys?: RuleConfiguration_for_Null;
	/**
	 * Enforce using concise optional chain instead of chained logical expressions.
	 */
	useOptionalChain?: RuleConfiguration_for_Null;
	/**
	 * Enforce the use of the regular expression literals instead of the RegExp constructor if possible.
	 */
	useRegexLiterals?: RuleConfiguration_for_Null;
	/**
	 * Disallow number literal object member names which are not base10 or uses underscore as separator
	 */
	useSimpleNumberKeys?: RuleConfiguration_for_Null;
	/**
	 * Discard redundant terms from logical expressions.
	 */
	useSimplifiedLogicExpression?: RuleConfiguration_for_Null;
}
/**
 * A list of rules that belong to this group
 */
export interface Correctness {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Prevent passing of children as props.
	 */
	noChildrenProp?: RuleConfiguration_for_Null;
	/**
	 * Prevents from having const variables being re-assigned.
	 */
	noConstAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow constant expressions in conditions
	 */
	noConstantCondition?: RuleConfiguration_for_Null;
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
	 * Disallow calling global object properties as functions
	 */
	noGlobalObjectCalls?: RuleConfiguration_for_Null;
	/**
	 * Disallow function and var declarations that are accessible outside their block.
	 */
	noInnerDeclarations?: RuleConfiguration_for_Null;
	/**
	 * Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors.
	 */
	noInvalidConstructorSuper?: RuleConfiguration_for_Null;
	/**
	 * Disallow new operators with global non-constructor functions.
	 */
	noInvalidNewBuiltin?: RuleConfiguration_for_Null;
	/**
	 * Disallow new operators with the Symbol object.
	 */
	noNewSymbol?: RuleConfiguration_for_Null;
	/**
	 * Disallow \8 and \9 escape sequences in string literals.
	 */
	noNonoctalDecimalEscape?: RuleConfiguration_for_Null;
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
	noStringCaseMismatch?: RuleConfiguration_for_Null;
	/**
	 * Disallow lexical declarations in switch clauses.
	 */
	noSwitchDeclarations?: RuleConfiguration_for_Null;
	/**
	 * Prevents the usage of variables that haven't been declared inside the document.
	 */
	noUndeclaredVariables?: RuleConfiguration_for_Null;
	/**
	 * Avoid using unnecessary continue.
	 */
	noUnnecessaryContinue?: RuleConfiguration_for_Null;
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
	 * Disallow unused labels.
	 */
	noUnusedLabels?: RuleConfiguration_for_Null;
	/**
	 * Disallow unused variables.
	 */
	noUnusedVariables?: RuleConfiguration_for_Null;
	/**
	 * This rules prevents void elements (AKA self-closing elements) from having children.
	 */
	noVoidElementsWithChildren?: RuleConfiguration_for_Null;
	/**
	 * Disallow returning a value from a function with the return type 'void'
	 */
	noVoidTypeReturn?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce all dependencies are correctly specified in a React hook.
	 */
	useExhaustiveDependencies?: RuleConfiguration_for_HooksOptions;
	/**
	 * Enforce that all React hooks are being called from the Top Level component functions.
	 */
	useHookAtTopLevel?: RuleConfiguration_for_DeprecatedHooksOptions;
	/**
	 * Require calls to isNaN() when checking for NaN.
	 */
	useIsNan?: RuleConfiguration_for_Null;
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
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Disallow the use of console.
	 */
	noConsole?: RuleConfiguration_for_Null;
	/**
	 * Disallow two keys with the same name inside a JSON object.
	 */
	noDuplicateJsonKeys?: RuleConfiguration_for_Null;
	/**
	 * A describe block should not contain duplicate hooks.
	 */
	noDuplicateTestHooks?: RuleConfiguration_for_Null;
	/**
	 * Disallow empty block statements and static blocks.
	 */
	noEmptyBlockStatements?: RuleConfiguration_for_Null;
	/**
	 * Disallow empty type parameters in type aliases and interfaces.
	 */
	noEmptyTypeParameters?: RuleConfiguration_for_Null;
	/**
	 * This rule enforces a maximum depth to nested describe() in test files.
	 */
	noExcessiveNestedTestSuites?: RuleConfiguration_for_Null;
	/**
	 * Disallow using export or module.exports in files containing tests
	 */
	noExportsInTest?: RuleConfiguration_for_Null;
	/**
	 * Disallow focused tests.
	 */
	noFocusedTests?: RuleConfiguration_for_Null;
	/**
	 * Disallow assignments to native objects and read-only global variables.
	 */
	noGlobalAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of global eval().
	 */
	noGlobalEval?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of variables and function parameters before their declaration
	 */
	noInvalidUseBeforeDeclaration?: RuleConfiguration_for_Null;
	/**
	 * Disallow characters made with multiple code points in character class syntax.
	 */
	noMisleadingCharacterClass?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of namespace imports.
	 */
	noNamespaceImport?: RuleConfiguration_for_Null;
	/**
	 * Forbid the use of Node.js builtin modules.
	 */
	noNodejsModules?: RuleConfiguration_for_Null;
	/**
	 * Avoid re-export all.
	 */
	noReExportAll?: RuleConfiguration_for_Null;
	/**
	 * Disallow specified modules when loaded by import or require.
	 */
	noRestrictedImports?: RuleConfiguration_for_RestrictedImportsOptions;
	/**
	 * It detects possible "wrong" semicolons inside JSX elements.
	 */
	noSemicolonInJsx?: RuleConfiguration_for_Null;
	/**
	 * Disallow disabled tests.
	 */
	noSkippedTests?: RuleConfiguration_for_Null;
	/**
	 * Disallow then property.
	 */
	noThenProperty?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of dependencies that aren't specified in the package.json.
	 */
	noUndeclaredDependencies?: RuleConfiguration_for_Null;
	/**
	 * Disallow unused imports.
	 */
	noUnusedImports?: RuleConfiguration_for_Null;
	/**
	 * Disallow unused private class members
	 */
	noUnusedPrivateClassMembers?: RuleConfiguration_for_Null;
	/**
	 * Disallow unnecessary nested block statements.
	 */
	noUselessLoneBlockStatements?: RuleConfiguration_for_Null;
	/**
	 * Disallow ternary operators when simpler alternatives exist.
	 */
	noUselessTernary?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Ensure async functions utilize await.
	 */
	useAwait?: RuleConfiguration_for_Null;
	/**
	 * Require consistently using either T[] or Array<T>
	 */
	useConsistentArrayType?: RuleConfiguration_for_ConsistentArrayTypeOptions;
	/**
	 * Promotes the use of export type for types.
	 */
	useExportType?: RuleConfiguration_for_Null;
	/**
	 * Enforce naming conventions for JavaScript and TypeScript filenames.
	 */
	useFilenamingConvention?: RuleConfiguration_for_FilenamingConventionOptions;
	/**
	 * This rule recommends a for-of loop when in a for loop, the index used to extract an item from the iterated array.
	 */
	useForOf?: RuleConfiguration_for_Null;
	/**
	 * Enforce the use of import type when an import only has specifiers with type qualifier.
	 */
	useGroupedTypeImport?: RuleConfiguration_for_Null;
	/**
	 * Disallows package private imports.
	 */
	useImportRestrictions?: RuleConfiguration_for_Null;
	/**
	 * Promotes the use of import type for types.
	 */
	useImportType?: RuleConfiguration_for_Null;
	/**
	 * Disallow missing key props in iterators/collection literals.
	 */
	useJsxKeyInIterable?: RuleConfiguration_for_Null;
	/**
	 * Promotes the usage of node:assert/strict over node:assert.
	 */
	useNodeAssertStrict?: RuleConfiguration_for_Null;
	/**
	 * Enforces using the node: protocol for Node.js builtin modules.
	 */
	useNodejsImportProtocol?: RuleConfiguration_for_Null;
	/**
	 * Use the Number properties instead of global ones.
	 */
	useNumberNamespace?: RuleConfiguration_for_Null;
	/**
	 * Enforce using function types instead of object type with call signatures.
	 */
	useShorthandFunctionType?: RuleConfiguration_for_Null;
	/**
	 * Enforce the sorting of CSS utility classes.
	 */
	useSortedClasses?: RuleConfiguration_for_UtilityClassSortingOptions;
}
/**
 * A list of rules that belong to this group
 */
export interface Performance {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Disallow the use of spread (...) syntax on accumulators.
	 */
	noAccumulatingSpread?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of the delete operator.
	 */
	noDelete?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
}
/**
 * A list of rules that belong to this group
 */
export interface Security {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Prevent the usage of dangerous JSX props
	 */
	noDangerouslySetInnerHtml?: RuleConfiguration_for_Null;
	/**
	 * Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop.
	 */
	noDangerouslySetInnerHtmlWithChildren?: RuleConfiguration_for_Null;
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
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
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
	 * Disallow implicit true values on JSX boolean attributes
	 */
	noImplicitBoolean?: RuleConfiguration_for_Null;
	/**
	 * Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
	 */
	noInferrableTypes?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of TypeScript's namespaces.
	 */
	noNamespace?: RuleConfiguration_for_Null;
	/**
	 * Disallow negation in the condition of an if statement if it has an else clause.
	 */
	noNegationElse?: RuleConfiguration_for_Null;
	/**
	 * Disallow non-null assertions using the ! postfix operator.
	 */
	noNonNullAssertion?: RuleConfiguration_for_Null;
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
	noShoutyConstants?: RuleConfiguration_for_Null;
	/**
	 * Disallow template literals if interpolation and special-character handling are not needed
	 */
	noUnusedTemplateLiteral?: RuleConfiguration_for_Null;
	/**
	 * Disallow else block when the if block breaks early.
	 */
	noUselessElse?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of var
	 */
	noVar?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce the use of as const over literal type and type annotation.
	 */
	useAsConstAssertion?: RuleConfiguration_for_Null;
	/**
	 * Requires following curly brace conventions.
	 */
	useBlockStatements?: RuleConfiguration_for_Null;
	/**
	 * Enforce using else if instead of nested if in else clauses.
	 */
	useCollapsedElseIf?: RuleConfiguration_for_Null;
	/**
	 * Require const declarations for variables that are never reassigned after declared.
	 */
	useConst?: RuleConfiguration_for_Null;
	/**
	 * Enforce default function parameters and optional function parameters to be last.
	 */
	useDefaultParameterLast?: RuleConfiguration_for_Null;
	/**
	 * Require that each enum member value be explicitly initialized.
	 */
	useEnumInitializers?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of Math.pow in favor of the ** operator.
	 */
	useExponentiationOperator?: RuleConfiguration_for_Null;
	/**
	 * This rule enforces the use of <>...</> over <Fragment>...</Fragment>.
	 */
	useFragmentSyntax?: RuleConfiguration_for_Null;
	/**
	 * Require all enum members to be literal values.
	 */
	useLiteralEnumMembers?: RuleConfiguration_for_Null;
	/**
	 * Enforce naming conventions for everything across a codebase.
	 */
	useNamingConvention?: RuleConfiguration_for_NamingConventionOptions;
	/**
	 * Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals
	 */
	useNumericLiterals?: RuleConfiguration_for_Null;
	/**
	 * Prevent extra closing tags for components without children
	 */
	useSelfClosingElements?: RuleConfiguration_for_Null;
	/**
	 * When expressing array types, this rule promotes the usage of T[] shorthand instead of Array<T>.
	 */
	useShorthandArrayType?: RuleConfiguration_for_Null;
	/**
	 * Require assignment operator shorthand where possible.
	 */
	useShorthandAssign?: RuleConfiguration_for_Null;
	/**
	 * Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block.
	 */
	useSingleCaseStatement?: RuleConfiguration_for_Null;
	/**
	 * Disallow multiple variable declarations in the same variable statement
	 */
	useSingleVarDeclarator?: RuleConfiguration_for_Null;
	/**
	 * Prefer template literals over string concatenation.
	 */
	useTemplate?: RuleConfiguration_for_Null;
	/**
	 * Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed.
	 */
	useWhile?: RuleConfiguration_for_Null;
}
/**
 * A list of rules that belong to this group
 */
export interface Suspicious {
	/**
	 * It enables ALL rules for this group.
	 */
	all?: boolean;
	/**
	 * Use standard constants instead of approximated literals.
	 */
	noApproximativeNumericConstant?: RuleConfiguration_for_Null;
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
	noCommentText?: RuleConfiguration_for_Null;
	/**
	 * Disallow comparing against -0
	 */
	noCompareNegZero?: RuleConfiguration_for_Null;
	/**
	 * Disallow labeled statements that are not loops.
	 */
	noConfusingLabels?: RuleConfiguration_for_Null;
	/**
	 * Disallow void type outside of generic or return types.
	 */
	noConfusingVoidType?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of console.log
	 */
	noConsoleLog?: RuleConfiguration_for_Null;
	/**
	 * Disallow TypeScript const enum
	 */
	noConstEnum?: RuleConfiguration_for_Null;
	/**
	 * Prevents from having control characters and some escape sequences that match control characters in regular expressions.
	 */
	noControlCharactersInRegex?: RuleConfiguration_for_Null;
	/**
	 * Disallow the use of debugger
	 */
	noDebugger?: RuleConfiguration_for_Null;
	/**
	 * Require the use of === and !==
	 */
	noDoubleEquals?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate case labels.
	 */
	noDuplicateCase?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate class members.
	 */
	noDuplicateClassMembers?: RuleConfiguration_for_Null;
	/**
	 * Prevents JSX properties to be assigned multiple times.
	 */
	noDuplicateJsxProps?: RuleConfiguration_for_Null;
	/**
	 * Prevents object literals having more than one property declaration for the same name.
	 */
	noDuplicateObjectKeys?: RuleConfiguration_for_Null;
	/**
	 * Disallow duplicate function parameter name.
	 */
	noDuplicateParameters?: RuleConfiguration_for_Null;
	/**
	 * Disallow the declaration of empty interfaces.
	 */
	noEmptyInterface?: RuleConfiguration_for_Null;
	/**
	 * Disallow the any type usage.
	 */
	noExplicitAny?: RuleConfiguration_for_Null;
	/**
	 * Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files.
	 */
	noExtraNonNullAssertion?: RuleConfiguration_for_Null;
	/**
	 * Disallow fallthrough of switch clauses.
	 */
	noFallthroughSwitchClause?: RuleConfiguration_for_Null;
	/**
	 * Disallow reassigning function declarations.
	 */
	noFunctionAssign?: RuleConfiguration_for_Null;
	/**
	 * Use Number.isFinite instead of global isFinite.
	 */
	noGlobalIsFinite?: RuleConfiguration_for_Null;
	/**
	 * Use Number.isNaN instead of global isNaN.
	 */
	noGlobalIsNan?: RuleConfiguration_for_Null;
	/**
	 * Disallow use of implicit any type on variable declarations.
	 */
	noImplicitAnyLet?: RuleConfiguration_for_Null;
	/**
	 * Disallow assigning to imported bindings
	 */
	noImportAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow labels that share a name with a variable
	 */
	noLabelVar?: RuleConfiguration_for_Null;
	/**
	 * Enforce proper usage of new and constructor.
	 */
	noMisleadingInstantiator?: RuleConfiguration_for_Null;
	/**
	 * Disallow shorthand assign when variable appears on both sides.
	 */
	noMisrefactoredShorthandAssign?: RuleConfiguration_for_Null;
	/**
	 * Disallow direct use of Object.prototype builtins.
	 */
	noPrototypeBuiltins?: RuleConfiguration_for_Null;
	/**
	 * Disallow variable, function, class, and type redeclarations in the same scope.
	 */
	noRedeclare?: RuleConfiguration_for_Null;
	/**
	 * Prevents from having redundant "use strict".
	 */
	noRedundantUseStrict?: RuleConfiguration_for_Null;
	/**
	 * Disallow comparisons where both sides are exactly the same.
	 */
	noSelfCompare?: RuleConfiguration_for_Null;
	/**
	 * Disallow identifiers from shadowing restricted names.
	 */
	noShadowRestrictedNames?: RuleConfiguration_for_Null;
	/**
	 * Disallow sparse arrays
	 */
	noSparseArray?: RuleConfiguration_for_Null;
	/**
	 * Disallow unsafe declaration merging between interfaces and classes.
	 */
	noUnsafeDeclarationMerging?: RuleConfiguration_for_Null;
	/**
	 * Disallow using unsafe negation.
	 */
	noUnsafeNegation?: RuleConfiguration_for_Null;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce default clauses in switch statements to be last
	 */
	useDefaultSwitchClauseLast?: RuleConfiguration_for_Null;
	/**
	 * Enforce get methods to always return a value.
	 */
	useGetterReturn?: RuleConfiguration_for_Null;
	/**
	 * Use Array.isArray() instead of instanceof Array.
	 */
	useIsArray?: RuleConfiguration_for_Null;
	/**
	 * Require using the namespace keyword over the module keyword to declare TypeScript namespaces.
	 */
	useNamespaceKeyword?: RuleConfiguration_for_Null;
	/**
	 * This rule verifies the result of typeof $expr unary expressions is being compared to valid values, either string literals containing valid type names or other typeof expressions
	 */
	useValidTypeof?: RuleConfiguration_for_Null;
}
export interface OverrideFormatterConfiguration {
	/**
	 * The attribute position style.
	 */
	attributePosition?: AttributePosition;
	enabled?: boolean;
	/**
	 * Stores whether formatting should be allowed to proceed if a given file has syntax errors
	 */
	formatWithErrors?: boolean;
	/**
	 * The size of the indentation, 2 by default (deprecated, use `indent-width`)
	 */
	indentSize?: number;
	/**
	 * The indent style.
	 */
	indentStyle?: PlainIndentStyle;
	/**
	 * The size of the indentation, 2 by default
	 */
	indentWidth?: number;
	/**
	 * The type of line ending.
	 */
	lineEnding?: LineEnding;
	/**
	 * What's the max width of a line. Defaults to 80.
	 */
	lineWidth?: LineWidth;
}
export interface OverrideLinterConfiguration {
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: boolean;
	/**
	 * List of rules
	 */
	rules?: Rules;
}
export interface OverrideOrganizeImportsConfiguration {
	/**
	 * if `false`, it disables the feature and the linter won't be executed. `true` by default
	 */
	enabled?: boolean;
}
export type RuleConfiguration_for_Null =
	| RulePlainConfiguration
	| RuleWithOptions_for_Null;
export type RuleConfiguration_for_ValidAriaRoleOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_ValidAriaRoleOptions;
export type RuleConfiguration_for_ComplexityOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_ComplexityOptions;
export type RuleConfiguration_for_HooksOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_HooksOptions;
export type RuleConfiguration_for_DeprecatedHooksOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_DeprecatedHooksOptions;
export type RuleConfiguration_for_RestrictedImportsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_RestrictedImportsOptions;
export type RuleConfiguration_for_ConsistentArrayTypeOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_ConsistentArrayTypeOptions;
export type RuleConfiguration_for_FilenamingConventionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_FilenamingConventionOptions;
export type RuleConfiguration_for_UtilityClassSortingOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_UtilityClassSortingOptions;
export type RuleConfiguration_for_RestrictedGlobalsOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_RestrictedGlobalsOptions;
export type RuleConfiguration_for_NamingConventionOptions =
	| RulePlainConfiguration
	| RuleWithOptions_for_NamingConventionOptions;
export type RulePlainConfiguration = "warn" | "error" | "off";
export interface RuleWithOptions_for_Null {
	level: RulePlainConfiguration;
	options: null;
}
export interface RuleWithOptions_for_ValidAriaRoleOptions {
	level: RulePlainConfiguration;
	options: ValidAriaRoleOptions;
}
export interface RuleWithOptions_for_ComplexityOptions {
	level: RulePlainConfiguration;
	options: ComplexityOptions;
}
export interface RuleWithOptions_for_HooksOptions {
	level: RulePlainConfiguration;
	options: HooksOptions;
}
export interface RuleWithOptions_for_DeprecatedHooksOptions {
	level: RulePlainConfiguration;
	options: DeprecatedHooksOptions;
}
export interface RuleWithOptions_for_RestrictedImportsOptions {
	level: RulePlainConfiguration;
	options: RestrictedImportsOptions;
}
export interface RuleWithOptions_for_ConsistentArrayTypeOptions {
	level: RulePlainConfiguration;
	options: ConsistentArrayTypeOptions;
}
export interface RuleWithOptions_for_FilenamingConventionOptions {
	level: RulePlainConfiguration;
	options: FilenamingConventionOptions;
}
export interface RuleWithOptions_for_UtilityClassSortingOptions {
	level: RulePlainConfiguration;
	options: UtilityClassSortingOptions;
}
export interface RuleWithOptions_for_RestrictedGlobalsOptions {
	level: RulePlainConfiguration;
	options: RestrictedGlobalsOptions;
}
export interface RuleWithOptions_for_NamingConventionOptions {
	level: RulePlainConfiguration;
	options: NamingConventionOptions;
}
export interface ValidAriaRoleOptions {
	allowInvalidRoles: string[];
	ignoreNonDom: boolean;
}
/**
 * Options for the rule `noExcessiveCognitiveComplexity`.
 */
export interface ComplexityOptions {
	/**
	 * The maximum complexity score that we allow. Anything higher is considered excessive.
	 */
	maxAllowedComplexity: number;
}
/**
 * Options for the rule `useExhaustiveDependencies`
 */
export interface HooksOptions {
	/**
	 * List of safe hooks
	 */
	hooks: Hooks[];
}
/**
 * Options for the `useHookAtTopLevel` rule have been deprecated, since we now use the React hook naming convention to determine whether a function is a hook.
 */
export interface DeprecatedHooksOptions {}
/**
 * Options for the rule `noRestrictedImports`.
 */
export interface RestrictedImportsOptions {
	/**
	 * A list of names that should trigger the rule
	 */
	paths: {};
}
export interface ConsistentArrayTypeOptions {
	syntax: ConsistentArrayType;
}
/**
 * Rule's options.
 */
export interface FilenamingConventionOptions {
	/**
	 * Allowed cases for _TypeScript_ `enum` member names.
	 */
	filenameCases: FilenameCases;
	/**
	 * If `false`, then non-ASCII characters are allowed.
	 */
	requireAscii: boolean;
	/**
	 * If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases. This does not affect other [Case].
	 */
	strictCase: boolean;
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
/**
 * Options for the rule `noRestrictedGlobals`.
 */
export interface RestrictedGlobalsOptions {
	/**
	 * A list of names that should trigger the rule
	 */
	deniedGlobals: string[];
}
/**
 * Rule's options.
 */
export interface NamingConventionOptions {
	/**
	 * Allowed cases for _TypeScript_ `enum` member names.
	 */
	enumMemberCase: EnumMemberCase;
	/**
	 * If `false`, then non-ASCII characters are allowed.
	 */
	requireAscii: boolean;
	/**
	 * If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases. This does not affect other [Case].
	 */
	strictCase: boolean;
}
export interface Hooks {
	/**
	* The "position" of the closure function, starting from zero.

### Example 
	 */
	closureIndex?: number;
	/**
	 * The "position" of the array of dependencies, starting from zero.
	 */
	dependenciesIndex?: number;
	/**
	 * The name of the hook
	 */
	name: string;
}
export type ConsistentArrayType = "shorthand" | "generic";
export type FilenameCases = FilenameCase[];
/**
 * Supported cases for TypeScript `enum` member names.
 */
export type EnumMemberCase = "PascalCase" | "CONSTANT_CASE" | "camelCase";
/**
 * Supported cases for TypeScript `enum` member names.
 */
export type FilenameCase =
	| "camelCase"
	| "export"
	| "kebab-case"
	| "PascalCase"
	| "snake_case";
export interface UpdateProjectParams {
	path: BiomePath;
}
export interface OpenProjectParams {
	content: string;
	path: BiomePath;
	version: number;
}
export interface OpenFileParams {
	content: string;
	language_hint?: Language;
	path: BiomePath;
	version: number;
}
/**
 * Supported languages by Biome
 */
export type Language =
	| "Astro"
	| "Vue"
	| "Svelte"
	| "JavaScript"
	| "JavaScriptReact"
	| "TypeScript"
	| "TypeScriptReact"
	| "Json"
	| "Jsonc"
	| "Css"
	| "Unknown";
export interface ChangeFileParams {
	content: string;
	path: BiomePath;
	version: number;
}
export interface CloseFileParams {
	path: BiomePath;
}
export interface GetSyntaxTreeParams {
	path: BiomePath;
}
export interface GetSyntaxTreeResult {
	ast: string;
	cst: string;
}
export interface OrganizeImportsParams {
	path: BiomePath;
}
export interface OrganizeImportsResult {
	code: string;
}
export interface GetFileContentParams {
	path: BiomePath;
}
export interface GetControlFlowGraphParams {
	cursor: TextSize;
	path: BiomePath;
}
export type TextSize = number;
export interface GetFormatterIRParams {
	path: BiomePath;
}
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	max_diagnostics: number;
	path: BiomePath;
}
export type RuleCategories = RuleCategory[];
export type RuleCategory = "Syntax" | "Lint" | "Action" | "Transformation";
export interface PullDiagnosticsResult {
	diagnostics: Diagnostic[];
	errors: number;
	skipped_diagnostics: number;
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
	verbose_advices: Advices;
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
	| "lint/a11y/useHeadingContent"
	| "lint/a11y/useHtmlLang"
	| "lint/a11y/useIframeTitle"
	| "lint/a11y/useKeyWithClickEvents"
	| "lint/a11y/useKeyWithMouseEvents"
	| "lint/a11y/useMediaCaption"
	| "lint/a11y/useValidAnchor"
	| "lint/a11y/useValidAriaProps"
	| "lint/a11y/useValidAriaRole"
	| "lint/a11y/useValidAriaValues"
	| "lint/a11y/useValidLang"
	| "lint/complexity/noBannedTypes"
	| "lint/complexity/noExcessiveCognitiveComplexity"
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
	| "lint/complexity/noUselessRename"
	| "lint/complexity/noUselessSwitchCase"
	| "lint/complexity/noUselessThisAlias"
	| "lint/complexity/noUselessTypeConstraint"
	| "lint/complexity/noVoid"
	| "lint/complexity/noWith"
	| "lint/complexity/useArrowFunction"
	| "lint/complexity/useFlatMap"
	| "lint/complexity/useLiteralKeys"
	| "lint/complexity/useOptionalChain"
	| "lint/complexity/useRegexLiterals"
	| "lint/complexity/useSimpleNumberKeys"
	| "lint/complexity/useSimplifiedLogicExpression"
	| "lint/correctness/noChildrenProp"
	| "lint/correctness/noConstAssign"
	| "lint/correctness/noConstantCondition"
	| "lint/correctness/noConstructorReturn"
	| "lint/correctness/noEmptyCharacterClassInRegex"
	| "lint/correctness/noEmptyPattern"
	| "lint/correctness/noGlobalObjectCalls"
	| "lint/correctness/noInnerDeclarations"
	| "lint/correctness/noInvalidConstructorSuper"
	| "lint/correctness/noInvalidNewBuiltin"
	| "lint/correctness/noNewSymbol"
	| "lint/correctness/noNonoctalDecimalEscape"
	| "lint/correctness/noPrecisionLoss"
	| "lint/correctness/noRenderReturnValue"
	| "lint/correctness/noSelfAssign"
	| "lint/correctness/noSetterReturn"
	| "lint/correctness/noStringCaseMismatch"
	| "lint/correctness/noSwitchDeclarations"
	| "lint/correctness/noUndeclaredVariables"
	| "lint/correctness/noUnnecessaryContinue"
	| "lint/correctness/noUnreachable"
	| "lint/correctness/noUnreachableSuper"
	| "lint/correctness/noUnsafeFinally"
	| "lint/correctness/noUnsafeOptionalChaining"
	| "lint/correctness/noUnusedLabels"
	| "lint/correctness/noUnusedVariables"
	| "lint/correctness/noVoidElementsWithChildren"
	| "lint/correctness/noVoidTypeReturn"
	| "lint/correctness/useExhaustiveDependencies"
	| "lint/correctness/useHookAtTopLevel"
	| "lint/correctness/useIsNan"
	| "lint/correctness/useValidForDirection"
	| "lint/correctness/useYield"
	| "lint/nursery/noApproximativeNumericConstant"
	| "lint/nursery/noConsole"
	| "lint/nursery/noDuplicateJsonKeys"
	| "lint/nursery/noDuplicateTestHooks"
	| "lint/nursery/noEmptyBlockStatements"
	| "lint/nursery/noEmptyTypeParameters"
	| "lint/nursery/noExcessiveNestedTestSuites"
	| "lint/nursery/noExportsInTest"
	| "lint/nursery/noFocusedTests"
	| "lint/nursery/noGlobalAssign"
	| "lint/nursery/noGlobalEval"
	| "lint/nursery/noInvalidUseBeforeDeclaration"
	| "lint/nursery/noMisleadingCharacterClass"
	| "lint/nursery/noNamespaceImport"
	| "lint/nursery/noNodejsModules"
	| "lint/nursery/noReExportAll"
	| "lint/nursery/noRestrictedImports"
	| "lint/nursery/noSemicolonInJsx"
	| "lint/nursery/noSkippedTests"
	| "lint/nursery/noThenProperty"
	| "lint/nursery/noTypeOnlyImportAttributes"
	| "lint/nursery/noUndeclaredDependencies"
	| "lint/nursery/noUnusedImports"
	| "lint/nursery/noUnusedPrivateClassMembers"
	| "lint/nursery/noUselessLoneBlockStatements"
	| "lint/nursery/noUselessTernary"
	| "lint/nursery/useAwait"
	| "lint/nursery/useBiomeSuppressionComment"
	| "lint/nursery/useConsistentArrayType"
	| "lint/nursery/useExportType"
	| "lint/nursery/useFilenamingConvention"
	| "lint/nursery/useForOf"
	| "lint/nursery/useGroupedTypeImport"
	| "lint/nursery/useImportRestrictions"
	| "lint/nursery/useImportType"
	| "lint/nursery/useJsxKeyInIterable"
	| "lint/nursery/useNodeAssertStrict"
	| "lint/nursery/useNodejsImportProtocol"
	| "lint/nursery/useNumberNamespace"
	| "lint/nursery/useShorthandFunctionType"
	| "lint/nursery/useSortedClasses"
	| "lint/performance/noAccumulatingSpread"
	| "lint/performance/noDelete"
	| "lint/security/noDangerouslySetInnerHtml"
	| "lint/security/noDangerouslySetInnerHtmlWithChildren"
	| "lint/style/noArguments"
	| "lint/style/noCommaOperator"
	| "lint/style/noDefaultExport"
	| "lint/style/noImplicitBoolean"
	| "lint/style/noInferrableTypes"
	| "lint/style/noNamespace"
	| "lint/style/noNegationElse"
	| "lint/style/noNonNullAssertion"
	| "lint/style/noParameterAssign"
	| "lint/style/noParameterProperties"
	| "lint/style/noRestrictedGlobals"
	| "lint/style/noShoutyConstants"
	| "lint/style/noUnusedTemplateLiteral"
	| "lint/style/noUselessElse"
	| "lint/style/noVar"
	| "lint/style/useAsConstAssertion"
	| "lint/style/useBlockStatements"
	| "lint/style/useCollapsedElseIf"
	| "lint/style/useConst"
	| "lint/style/useDefaultParameterLast"
	| "lint/style/useEnumInitializers"
	| "lint/style/useExponentiationOperator"
	| "lint/style/useFragmentSyntax"
	| "lint/style/useLiteralEnumMembers"
	| "lint/style/useNamingConvention"
	| "lint/style/useNumericLiterals"
	| "lint/style/useSelfClosingElements"
	| "lint/style/useShorthandArrayType"
	| "lint/style/useShorthandAssign"
	| "lint/style/useSingleCaseStatement"
	| "lint/style/useSingleVarDeclarator"
	| "lint/style/useTemplate"
	| "lint/style/useWhile"
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
	| "lint/suspicious/noConsoleLog"
	| "lint/suspicious/noConstEnum"
	| "lint/suspicious/noControlCharactersInRegex"
	| "lint/suspicious/noDebugger"
	| "lint/suspicious/noDoubleEquals"
	| "lint/suspicious/noDuplicateCase"
	| "lint/suspicious/noDuplicateClassMembers"
	| "lint/suspicious/noDuplicateJsxProps"
	| "lint/suspicious/noDuplicateObjectKeys"
	| "lint/suspicious/noDuplicateParameters"
	| "lint/suspicious/noEmptyInterface"
	| "lint/suspicious/noExplicitAny"
	| "lint/suspicious/noExtraNonNullAssertion"
	| "lint/suspicious/noFallthroughSwitchClause"
	| "lint/suspicious/noFunctionAssign"
	| "lint/suspicious/noGlobalIsFinite"
	| "lint/suspicious/noGlobalIsNan"
	| "lint/suspicious/noImplicitAnyLet"
	| "lint/suspicious/noImportAssign"
	| "lint/suspicious/noLabelVar"
	| "lint/suspicious/noMisleadingInstantiator"
	| "lint/suspicious/noMisrefactoredShorthandAssign"
	| "lint/suspicious/noPrototypeBuiltins"
	| "lint/suspicious/noRedeclare"
	| "lint/suspicious/noRedundantUseStrict"
	| "lint/suspicious/noSelfCompare"
	| "lint/suspicious/noShadowRestrictedNames"
	| "lint/suspicious/noSparseArray"
	| "lint/suspicious/noUnsafeDeclarationMerging"
	| "lint/suspicious/noUnsafeNegation"
	| "lint/suspicious/useDefaultSwitchClauseLast"
	| "lint/suspicious/useGetterReturn"
	| "lint/suspicious/useIsArray"
	| "lint/suspicious/useNamespaceKeyword"
	| "lint/suspicious/useValidTypeof"
	| "files/missingHandler"
	| "format"
	| "check"
	| "ci"
	| "configuration"
	| "organizeImports"
	| "migrate"
	| "deserialize"
	| "project"
	| "internalError/io"
	| "internalError/fs"
	| "internalError/panic"
	| "parse"
	| "parse/noSuperWithoutExtends"
	| "parse/noInitializerWithDefinite"
	| "parse/noDuplicatePrivateClassMembers"
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
	| "suppressions/deprecatedSuppressionComment"
	| "args/fileNotFound"
	| "flags/invalid"
	| "semanticTests";
export interface Location {
	path?: Resource_for_String;
	source_code?: string;
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
	| { Log: [LogCategory, MarkupBuf] }
	| { List: MarkupBuf[] }
	| { Frame: Location }
	| { Diff: TextEdit }
	| { Backtrace: [MarkupBuf, Backtrace] }
	| { Command: string }
	| { Group: [MarkupBuf, Advices] };
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
export type LogCategory = "None" | "Info" | "Warn" | "Error";
export interface TextEdit {
	dictionary: string;
	ops: CompressedOp[];
}
export type Backtrace = BacktraceFrame[];
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
	| { DiffOp: DiffOp }
	| { EqualLines: { line_count: number } };
/**
 * Serializable representation of a backtrace frame.
 */
export interface BacktraceFrame {
	ip: number;
	symbols: BacktraceSymbol[];
}
export type DiffOp =
	| { Equal: { range: TextRange } }
	| { Insert: { range: TextRange } }
	| { Delete: { range: TextRange } };
/**
 * Serializable representation of a backtrace frame symbol.
 */
export interface BacktraceSymbol {
	colno?: number;
	filename?: string;
	lineno?: number;
	name?: string;
}
export interface PullActionsParams {
	path: BiomePath;
	range: TextRange;
}
export interface PullActionsResult {
	actions: CodeAction[];
}
export interface CodeAction {
	category: ActionCategory;
	rule_name?: [string, string];
	suggestion: CodeSuggestion;
}
/**
	* The category of a code action, this type maps directly to the [CodeActionKind] type in the Language Server Protocol specification

[CodeActionKind]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#codeActionKind 
	 */
export type ActionCategory =
	| "QuickFix"
	| { Refactor: RefactorKind }
	| { Source: SourceActionKind }
	| { Other: string };
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
 * The sub-category of a refactor code action
 */
export type RefactorKind =
	| "None"
	| "Extract"
	| "Inline"
	| "Rewrite"
	| { Other: string };
/**
 * The sub-category of a source code action
 */
export type SourceActionKind =
	| "FixAll"
	| "None"
	| "OrganizeImports"
	| { Other: string };
/**
 * Indicates how a tool should manage this suggestion.
 */
export type Applicability = "Always" | "MaybeIncorrect";
export interface FormatFileParams {
	path: BiomePath;
}
export interface Printed {
	code: string;
	range?: TextRange;
	sourcemap: SourceMarker[];
	verbatim_ranges: TextRange[];
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
	range: TextRange;
}
export interface FormatOnTypeParams {
	offset: TextSize;
	path: BiomePath;
}
export interface FixFileParams {
	fix_file_mode: FixFileMode;
	path: BiomePath;
	should_format: boolean;
}
/**
 * Which fixes should be applied during the analyzing phase
 */
export type FixFileMode = "SafeFixes" | "SafeAndUnsafeFixes";
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
	skipped_suggested_fixes: number;
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
	new_name: string;
	path: BiomePath;
	symbol_at: TextSize;
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
export type Configuration = PartialConfiguration;
export interface Workspace {
	fileFeatures(params: SupportsFeatureParams): Promise<SupportsFeatureResult>;
	updateSettings(params: UpdateSettingsParams): Promise<void>;
	updateCurrentProject(params: UpdateProjectParams): Promise<void>;
	openProject(params: OpenProjectParams): Promise<void>;
	openFile(params: OpenFileParams): Promise<void>;
	changeFile(params: ChangeFileParams): Promise<void>;
	closeFile(params: CloseFileParams): Promise<void>;
	getSyntaxTree(params: GetSyntaxTreeParams): Promise<GetSyntaxTreeResult>;
	organizeImports(
		params: OrganizeImportsParams,
	): Promise<OrganizeImportsResult>;
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
		updateCurrentProject(params) {
			return transport.request("biome/update_current_project", params);
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
		organizeImports(params) {
			return transport.request("biome/organize_imports", params);
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
		destroy() {
			transport.destroy();
		},
	};
}
