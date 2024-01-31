// Generated file, do not edit by hand, see `xtask/codegen`
import type { Transport } from "./transport";
export interface SupportsFeatureParams {
	feature: FeatureName[];
	path: RomePath;
}
export type FeatureName = "Format" | "Lint" | "OrganizeImports";
export interface RomePath {
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
	noAccessKey?: RuleConfiguration;
	/**
	 * Enforce that aria-hidden="true" is not set on focusable elements.
	 */
	noAriaHiddenOnFocusable?: RuleConfiguration;
	/**
	 * Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
	 */
	noAriaUnsupportedElements?: RuleConfiguration;
	/**
	 * Enforce that autoFocus prop is not used on elements.
	 */
	noAutofocus?: RuleConfiguration;
	/**
	 * Disallow target="_blank" attribute without rel="noreferrer"
	 */
	noBlankTarget?: RuleConfiguration;
	/**
	 * Enforces that no distracting elements are used.
	 */
	noDistractingElements?: RuleConfiguration;
	/**
	 * The scope prop should be used only on <th> elements.
	 */
	noHeaderScope?: RuleConfiguration;
	/**
	 * Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
	 */
	noInteractiveElementToNoninteractiveRole?: RuleConfiguration;
	/**
	 * Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveElementToInteractiveRole?: RuleConfiguration;
	/**
	 * Enforce that tabIndex is not assigned to non-interactive HTML elements.
	 */
	noNoninteractiveTabindex?: RuleConfiguration;
	/**
	 * Prevent the usage of positive integers on tabIndex property
	 */
	noPositiveTabindex?: RuleConfiguration;
	/**
	 * Enforce img alt prop does not contain the word "image", "picture", or "photo".
	 */
	noRedundantAlt?: RuleConfiguration;
	/**
	 * Enforce explicit role property is not the same as implicit/default role property on an element.
	 */
	noRedundantRoles?: RuleConfiguration;
	/**
	 * Enforces the usage of the title element for the svg element.
	 */
	noSvgWithoutTitle?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce that all elements that require alternative text have meaningful information to relay back to the end user.
	 */
	useAltText?: RuleConfiguration;
	/**
	 * Enforce that anchors have content and that the content is accessible to screen readers.
	 */
	useAnchorContent?: RuleConfiguration;
	/**
	 * Enforce that tabIndex is assigned to non-interactive HTML elements with aria-activedescendant.
	 */
	useAriaActivedescendantWithTabindex?: RuleConfiguration;
	/**
	 * Enforce that elements with ARIA roles must have all required ARIA attributes for that role.
	 */
	useAriaPropsForRole?: RuleConfiguration;
	/**
	 * Enforces the usage of the attribute type for the element button
	 */
	useButtonType?: RuleConfiguration;
	/**
	 * Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers. Accessible means that it is not hidden using the aria-hidden prop.
	 */
	useHeadingContent?: RuleConfiguration;
	/**
	 * Enforce that html element has lang attribute.
	 */
	useHtmlLang?: RuleConfiguration;
	/**
	 * Enforces the usage of the attribute title for the element iframe.
	 */
	useIframeTitle?: RuleConfiguration;
	/**
	 * Enforce onClick is accompanied by at least one of the following: onKeyUp, onKeyDown, onKeyPress.
	 */
	useKeyWithClickEvents?: RuleConfiguration;
	/**
	 * Enforce onMouseOver / onMouseOut are accompanied by onFocus / onBlur.
	 */
	useKeyWithMouseEvents?: RuleConfiguration;
	/**
	 * Enforces that audio and video elements must have a track for captions.
	 */
	useMediaCaption?: RuleConfiguration;
	/**
	 * Enforce that all anchors are valid, and they are navigable elements.
	 */
	useValidAnchor?: RuleConfiguration;
	/**
	 * Ensures that ARIA properties aria-* are all valid.
	 */
	useValidAriaProps?: RuleConfiguration;
	/**
	 * Elements with ARIA roles must use a valid, non-abstract ARIA role.
	 */
	useValidAriaRole?: RuleConfiguration;
	/**
	 * Enforce that ARIA state and property values are valid.
	 */
	useValidAriaValues?: RuleConfiguration;
	/**
	 * Ensure that the attribute passed to the lang attribute is a correct ISO language and/or country.
	 */
	useValidLang?: RuleConfiguration;
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
	noBannedTypes?: RuleConfiguration;
	/**
	 * Disallow functions that exceed a given Cognitive Complexity score.
	 */
	noExcessiveCognitiveComplexity?: RuleConfiguration;
	/**
	 * Disallow unnecessary boolean casts
	 */
	noExtraBooleanCast?: RuleConfiguration;
	/**
	 * Prefer for...of statement instead of Array.forEach.
	 */
	noForEach?: RuleConfiguration;
	/**
	 * Disallow unclear usage of consecutive space characters in regular expression literals
	 */
	noMultipleSpacesInRegularExpressionLiterals?: RuleConfiguration;
	/**
	 * This rule reports when a class has no non-static members, such as for a class used exclusively as a static namespace.
	 */
	noStaticOnlyClass?: RuleConfiguration;
	/**
	 * Disallow this and super in static contexts.
	 */
	noThisInStatic?: RuleConfiguration;
	/**
	 * Disallow unnecessary catch clauses.
	 */
	noUselessCatch?: RuleConfiguration;
	/**
	 * Disallow unnecessary constructors.
	 */
	noUselessConstructor?: RuleConfiguration;
	/**
	 * Disallow empty exports that don't change anything in a module file.
	 */
	noUselessEmptyExport?: RuleConfiguration;
	/**
	 * Disallow unnecessary fragments
	 */
	noUselessFragments?: RuleConfiguration;
	/**
	 * Disallow unnecessary labels.
	 */
	noUselessLabel?: RuleConfiguration;
	/**
	 * Disallow renaming import, export, and destructured assignments to the same name.
	 */
	noUselessRename?: RuleConfiguration;
	/**
	 * Disallow useless case in switch statements.
	 */
	noUselessSwitchCase?: RuleConfiguration;
	/**
	 * Disallow useless this aliasing.
	 */
	noUselessThisAlias?: RuleConfiguration;
	/**
	 * Disallow using any or unknown as type constraint.
	 */
	noUselessTypeConstraint?: RuleConfiguration;
	/**
	 * Disallow the use of void operators, which is not a familiar operator.
	 */
	noVoid?: RuleConfiguration;
	/**
	 * Disallow with statements in non-strict contexts.
	 */
	noWith?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Use arrow functions over function expressions.
	 */
	useArrowFunction?: RuleConfiguration;
	/**
	 * Promotes the use of .flatMap() when map().flat() are used together.
	 */
	useFlatMap?: RuleConfiguration;
	/**
	 * Enforce the usage of a literal access to properties over computed property access.
	 */
	useLiteralKeys?: RuleConfiguration;
	/**
	 * Enforce using concise optional chain instead of chained logical expressions.
	 */
	useOptionalChain?: RuleConfiguration;
	/**
	 * Enforce the use of the regular expression literals instead of the RegExp constructor if possible.
	 */
	useRegexLiterals?: RuleConfiguration;
	/**
	 * Disallow number literal object member names which are not base10 or uses underscore as separator
	 */
	useSimpleNumberKeys?: RuleConfiguration;
	/**
	 * Discard redundant terms from logical expressions.
	 */
	useSimplifiedLogicExpression?: RuleConfiguration;
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
	noChildrenProp?: RuleConfiguration;
	/**
	 * Prevents from having const variables being re-assigned.
	 */
	noConstAssign?: RuleConfiguration;
	/**
	 * Disallow constant expressions in conditions
	 */
	noConstantCondition?: RuleConfiguration;
	/**
	 * Disallow returning a value from a constructor.
	 */
	noConstructorReturn?: RuleConfiguration;
	/**
	 * Disallow empty character classes in regular expression literals.
	 */
	noEmptyCharacterClassInRegex?: RuleConfiguration;
	/**
	 * Disallows empty destructuring patterns.
	 */
	noEmptyPattern?: RuleConfiguration;
	/**
	 * Disallow calling global object properties as functions
	 */
	noGlobalObjectCalls?: RuleConfiguration;
	/**
	 * Disallow function and var declarations that are accessible outside their block.
	 */
	noInnerDeclarations?: RuleConfiguration;
	/**
	 * Prevents the incorrect use of super() inside classes. It also checks whether a call super() is missing from classes that extends other constructors.
	 */
	noInvalidConstructorSuper?: RuleConfiguration;
	/**
	 * Disallow new operators with global non-constructor functions.
	 */
	noInvalidNewBuiltin?: RuleConfiguration;
	/**
	 * Disallow new operators with the Symbol object.
	 */
	noNewSymbol?: RuleConfiguration;
	/**
	 * Disallow \8 and \9 escape sequences in string literals.
	 */
	noNonoctalDecimalEscape?: RuleConfiguration;
	/**
	 * Disallow literal numbers that lose precision
	 */
	noPrecisionLoss?: RuleConfiguration;
	/**
	 * Prevent the usage of the return value of React.render.
	 */
	noRenderReturnValue?: RuleConfiguration;
	/**
	 * Disallow assignments where both sides are exactly the same.
	 */
	noSelfAssign?: RuleConfiguration;
	/**
	 * Disallow returning a value from a setter
	 */
	noSetterReturn?: RuleConfiguration;
	/**
	 * Disallow comparison of expressions modifying the string case with non-compliant value.
	 */
	noStringCaseMismatch?: RuleConfiguration;
	/**
	 * Disallow lexical declarations in switch clauses.
	 */
	noSwitchDeclarations?: RuleConfiguration;
	/**
	 * Prevents the usage of variables that haven't been declared inside the document.
	 */
	noUndeclaredVariables?: RuleConfiguration;
	/**
	 * Avoid using unnecessary continue.
	 */
	noUnnecessaryContinue?: RuleConfiguration;
	/**
	 * Disallow unreachable code
	 */
	noUnreachable?: RuleConfiguration;
	/**
	 * Ensures the super() constructor is called exactly once on every code  path in a class constructor before this is accessed if the class has a superclass
	 */
	noUnreachableSuper?: RuleConfiguration;
	/**
	 * Disallow control flow statements in finally blocks.
	 */
	noUnsafeFinally?: RuleConfiguration;
	/**
	 * Disallow the use of optional chaining in contexts where the undefined value is not allowed.
	 */
	noUnsafeOptionalChaining?: RuleConfiguration;
	/**
	 * Disallow unused labels.
	 */
	noUnusedLabels?: RuleConfiguration;
	/**
	 * Disallow unused variables.
	 */
	noUnusedVariables?: RuleConfiguration;
	/**
	 * This rules prevents void elements (AKA self-closing elements) from having children.
	 */
	noVoidElementsWithChildren?: RuleConfiguration;
	/**
	 * Disallow returning a value from a function with the return type 'void'
	 */
	noVoidTypeReturn?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce all dependencies are correctly specified in a React hook.
	 */
	useExhaustiveDependencies?: RuleConfiguration;
	/**
	 * Enforce that all React hooks are being called from the Top Level component functions.
	 */
	useHookAtTopLevel?: RuleConfiguration;
	/**
	 * Require calls to isNaN() when checking for NaN.
	 */
	useIsNan?: RuleConfiguration;
	/**
	 * Enforce "for" loop update clause moving the counter in the right direction.
	 */
	useValidForDirection?: RuleConfiguration;
	/**
	 * Require generator functions to contain yield.
	 */
	useYield?: RuleConfiguration;
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
	 * Disallow two keys with the same name inside a JSON object.
	 */
	noDuplicateJsonKeys?: RuleConfiguration;
	/**
	 * Disallow empty block statements and static blocks.
	 */
	noEmptyBlockStatements?: RuleConfiguration;
	/**
	 * Disallow empty type parameters in type aliases and interfaces.
	 */
	noEmptyTypeParameters?: RuleConfiguration;
	/**
	 * Disallow focused tests.
	 */
	noFocusedTests?: RuleConfiguration;
	/**
	 * Disallow assignments to native objects and read-only global variables.
	 */
	noGlobalAssign?: RuleConfiguration;
	/**
	 * Disallow the use of global eval().
	 */
	noGlobalEval?: RuleConfiguration;
	/**
	 * Disallow the use of variables and function parameters before their declaration
	 */
	noInvalidUseBeforeDeclaration?: RuleConfiguration;
	/**
	 * Disallow characters made with multiple code points in character class syntax.
	 */
	noMisleadingCharacterClass?: RuleConfiguration;
	/**
	 * Forbid the use of Node.js builtin modules.
	 */
	noNodejsModules?: RuleConfiguration;
	/**
	 * Avoid re-export all
	 */
	noReExportAll?: RuleConfiguration;
	/**
	 * Disallow disabled tests.
	 */
	noSkippedTests?: RuleConfiguration;
	/**
	 * Disallow then property.
	 */
	noThenProperty?: RuleConfiguration;
	/**
	 * Disallow unused imports.
	 */
	noUnusedImports?: RuleConfiguration;
	/**
	 * Disallow unused private class members
	 */
	noUnusedPrivateClassMembers?: RuleConfiguration;
	/**
	 * Disallow unnecessary nested block statements.
	 */
	noUselessLoneBlockStatements?: RuleConfiguration;
	/**
	 * Disallow ternary operators when simpler alternatives exist.
	 */
	noUselessTernary?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Ensure async functions utilize await.
	 */
	useAwait?: RuleConfiguration;
	/**
	 * Require consistently using either T[] or Array<T>
	 */
	useConsistentArrayType?: RuleConfiguration;
	/**
	 * Promotes the use of export type for types.
	 */
	useExportType?: RuleConfiguration;
	/**
	 * Enforce naming conventions for JavaScript and TypeScript filenames.
	 */
	useFilenamingConvention?: RuleConfiguration;
	/**
	 * This rule recommends a for-of loop when in a for loop, the index used to extract an item from the iterated array.
	 */
	useForOf?: RuleConfiguration;
	/**
	 * Enforce the use of import type when an import only has specifiers with type qualifier.
	 */
	useGroupedTypeImport?: RuleConfiguration;
	/**
	 * Disallows package private imports.
	 */
	useImportRestrictions?: RuleConfiguration;
	/**
	 * Promotes the use of import type for types.
	 */
	useImportType?: RuleConfiguration;
	/**
	 * Enforces using the node: protocol for Node.js builtin modules.
	 */
	useNodejsImportProtocol?: RuleConfiguration;
	/**
	 * Use the Number properties instead of global ones.
	 */
	useNumberNamespace?: RuleConfiguration;
	/**
	 * Enforce using function types instead of object type with call signatures.
	 */
	useShorthandFunctionType?: RuleConfiguration;
	/**
	 * Enforce the sorting of CSS utility classes.
	 */
	useSortedClasses?: RuleConfiguration;
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
	noAccumulatingSpread?: RuleConfiguration;
	/**
	 * Disallow the use of the delete operator.
	 */
	noDelete?: RuleConfiguration;
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
	noDangerouslySetInnerHtml?: RuleConfiguration;
	/**
	 * Report when a DOM element or a component uses both children and dangerouslySetInnerHTML prop.
	 */
	noDangerouslySetInnerHtmlWithChildren?: RuleConfiguration;
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
	noArguments?: RuleConfiguration;
	/**
	 * Disallow comma operator.
	 */
	noCommaOperator?: RuleConfiguration;
	/**
	 * Disallow default exports.
	 */
	noDefaultExport?: RuleConfiguration;
	/**
	 * Disallow implicit true values on JSX boolean attributes
	 */
	noImplicitBoolean?: RuleConfiguration;
	/**
	 * Disallow type annotations for variables, parameters, and class properties initialized with a literal expression.
	 */
	noInferrableTypes?: RuleConfiguration;
	/**
	 * Disallow the use of TypeScript's namespaces.
	 */
	noNamespace?: RuleConfiguration;
	/**
	 * Disallow negation in the condition of an if statement if it has an else clause.
	 */
	noNegationElse?: RuleConfiguration;
	/**
	 * Disallow non-null assertions using the ! postfix operator.
	 */
	noNonNullAssertion?: RuleConfiguration;
	/**
	 * Disallow reassigning function parameters.
	 */
	noParameterAssign?: RuleConfiguration;
	/**
	 * Disallow the use of parameter properties in class constructors.
	 */
	noParameterProperties?: RuleConfiguration;
	/**
	 * This rule allows you to specify global variable names that you don’t want to use in your application.
	 */
	noRestrictedGlobals?: RuleConfiguration;
	/**
	 * Disallow the use of constants which its value is the upper-case version of its name.
	 */
	noShoutyConstants?: RuleConfiguration;
	/**
	 * Disallow template literals if interpolation and special-character handling are not needed
	 */
	noUnusedTemplateLiteral?: RuleConfiguration;
	/**
	 * Disallow else block when the if block breaks early.
	 */
	noUselessElse?: RuleConfiguration;
	/**
	 * Disallow the use of var
	 */
	noVar?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce the use of as const over literal type and type annotation.
	 */
	useAsConstAssertion?: RuleConfiguration;
	/**
	 * Requires following curly brace conventions.
	 */
	useBlockStatements?: RuleConfiguration;
	/**
	 * Enforce using else if instead of nested if in else clauses.
	 */
	useCollapsedElseIf?: RuleConfiguration;
	/**
	 * Require const declarations for variables that are never reassigned after declared.
	 */
	useConst?: RuleConfiguration;
	/**
	 * Enforce default function parameters and optional function parameters to be last.
	 */
	useDefaultParameterLast?: RuleConfiguration;
	/**
	 * Require that each enum member value be explicitly initialized.
	 */
	useEnumInitializers?: RuleConfiguration;
	/**
	 * Disallow the use of Math.pow in favor of the ** operator.
	 */
	useExponentiationOperator?: RuleConfiguration;
	/**
	 * This rule enforces the use of <>...</> over <Fragment>...</Fragment>.
	 */
	useFragmentSyntax?: RuleConfiguration;
	/**
	 * Require all enum members to be literal values.
	 */
	useLiteralEnumMembers?: RuleConfiguration;
	/**
	 * Enforce naming conventions for everything across a codebase.
	 */
	useNamingConvention?: RuleConfiguration;
	/**
	 * Disallow parseInt() and Number.parseInt() in favor of binary, octal, and hexadecimal literals
	 */
	useNumericLiterals?: RuleConfiguration;
	/**
	 * Prevent extra closing tags for components without children
	 */
	useSelfClosingElements?: RuleConfiguration;
	/**
	 * When expressing array types, this rule promotes the usage of T[] shorthand instead of Array<T>.
	 */
	useShorthandArrayType?: RuleConfiguration;
	/**
	 * Require assignment operator shorthand where possible.
	 */
	useShorthandAssign?: RuleConfiguration;
	/**
	 * Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block.
	 */
	useSingleCaseStatement?: RuleConfiguration;
	/**
	 * Disallow multiple variable declarations in the same variable statement
	 */
	useSingleVarDeclarator?: RuleConfiguration;
	/**
	 * Prefer template literals over string concatenation.
	 */
	useTemplate?: RuleConfiguration;
	/**
	 * Enforce the use of while loops instead of for loops when the initializer and update expressions are not needed.
	 */
	useWhile?: RuleConfiguration;
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
	noApproximativeNumericConstant?: RuleConfiguration;
	/**
	 * Discourage the usage of Array index in keys.
	 */
	noArrayIndexKey?: RuleConfiguration;
	/**
	 * Disallow assignments in expressions.
	 */
	noAssignInExpressions?: RuleConfiguration;
	/**
	 * Disallows using an async function as a Promise executor.
	 */
	noAsyncPromiseExecutor?: RuleConfiguration;
	/**
	 * Disallow reassigning exceptions in catch clauses.
	 */
	noCatchAssign?: RuleConfiguration;
	/**
	 * Disallow reassigning class members.
	 */
	noClassAssign?: RuleConfiguration;
	/**
	 * Prevent comments from being inserted as text nodes
	 */
	noCommentText?: RuleConfiguration;
	/**
	 * Disallow comparing against -0
	 */
	noCompareNegZero?: RuleConfiguration;
	/**
	 * Disallow labeled statements that are not loops.
	 */
	noConfusingLabels?: RuleConfiguration;
	/**
	 * Disallow void type outside of generic or return types.
	 */
	noConfusingVoidType?: RuleConfiguration;
	/**
	 * Disallow the use of console.log
	 */
	noConsoleLog?: RuleConfiguration;
	/**
	 * Disallow TypeScript const enum
	 */
	noConstEnum?: RuleConfiguration;
	/**
	 * Prevents from having control characters and some escape sequences that match control characters in regular expressions.
	 */
	noControlCharactersInRegex?: RuleConfiguration;
	/**
	 * Disallow the use of debugger
	 */
	noDebugger?: RuleConfiguration;
	/**
	 * Require the use of === and !==
	 */
	noDoubleEquals?: RuleConfiguration;
	/**
	 * Disallow duplicate case labels.
	 */
	noDuplicateCase?: RuleConfiguration;
	/**
	 * Disallow duplicate class members.
	 */
	noDuplicateClassMembers?: RuleConfiguration;
	/**
	 * Prevents JSX properties to be assigned multiple times.
	 */
	noDuplicateJsxProps?: RuleConfiguration;
	/**
	 * Prevents object literals having more than one property declaration for the same name.
	 */
	noDuplicateObjectKeys?: RuleConfiguration;
	/**
	 * Disallow duplicate function parameter name.
	 */
	noDuplicateParameters?: RuleConfiguration;
	/**
	 * Disallow the declaration of empty interfaces.
	 */
	noEmptyInterface?: RuleConfiguration;
	/**
	 * Disallow the any type usage.
	 */
	noExplicitAny?: RuleConfiguration;
	/**
	 * Prevents the wrong usage of the non-null assertion operator (!) in TypeScript files.
	 */
	noExtraNonNullAssertion?: RuleConfiguration;
	/**
	 * Disallow fallthrough of switch clauses.
	 */
	noFallthroughSwitchClause?: RuleConfiguration;
	/**
	 * Disallow reassigning function declarations.
	 */
	noFunctionAssign?: RuleConfiguration;
	/**
	 * Use Number.isFinite instead of global isFinite.
	 */
	noGlobalIsFinite?: RuleConfiguration;
	/**
	 * Use Number.isNaN instead of global isNaN.
	 */
	noGlobalIsNan?: RuleConfiguration;
	/**
	 * Disallow use of implicit any type on variable declarations.
	 */
	noImplicitAnyLet?: RuleConfiguration;
	/**
	 * Disallow assigning to imported bindings
	 */
	noImportAssign?: RuleConfiguration;
	/**
	 * Disallow labels that share a name with a variable
	 */
	noLabelVar?: RuleConfiguration;
	/**
	 * Enforce proper usage of new and constructor.
	 */
	noMisleadingInstantiator?: RuleConfiguration;
	/**
	 * Disallow shorthand assign when variable appears on both sides.
	 */
	noMisrefactoredShorthandAssign?: RuleConfiguration;
	/**
	 * Disallow direct use of Object.prototype builtins.
	 */
	noPrototypeBuiltins?: RuleConfiguration;
	/**
	 * Disallow variable, function, class, and type redeclarations in the same scope.
	 */
	noRedeclare?: RuleConfiguration;
	/**
	 * Prevents from having redundant "use strict".
	 */
	noRedundantUseStrict?: RuleConfiguration;
	/**
	 * Disallow comparisons where both sides are exactly the same.
	 */
	noSelfCompare?: RuleConfiguration;
	/**
	 * Disallow identifiers from shadowing restricted names.
	 */
	noShadowRestrictedNames?: RuleConfiguration;
	/**
	 * Disallow sparse arrays
	 */
	noSparseArray?: RuleConfiguration;
	/**
	 * Disallow unsafe declaration merging between interfaces and classes.
	 */
	noUnsafeDeclarationMerging?: RuleConfiguration;
	/**
	 * Disallow using unsafe negation.
	 */
	noUnsafeNegation?: RuleConfiguration;
	/**
	 * It enables the recommended rules for this group
	 */
	recommended?: boolean;
	/**
	 * Enforce default clauses in switch statements to be last
	 */
	useDefaultSwitchClauseLast?: RuleConfiguration;
	/**
	 * Enforce get methods to always return a value.
	 */
	useGetterReturn?: RuleConfiguration;
	/**
	 * Use Array.isArray() instead of instanceof Array.
	 */
	useIsArray?: RuleConfiguration;
	/**
	 * Require using the namespace keyword over the module keyword to declare TypeScript namespaces.
	 */
	useNamespaceKeyword?: RuleConfiguration;
	/**
	 * This rule verifies the result of typeof $expr unary expressions is being compared to valid values, either string literals containing valid type names or other typeof expressions
	 */
	useValidTypeof?: RuleConfiguration;
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
export type RuleConfiguration = RulePlainConfiguration | RuleWithOptions;
export type RulePlainConfiguration = "warn" | "error" | "off";
export interface RuleWithOptions {
	level: RulePlainConfiguration;
	options?: PossibleOptions;
}
export type PossibleOptions =
	| ComplexityOptions
	| ConsistentArrayTypeOptions
	| FilenamingConventionOptions
	| HooksOptions
	| DeprecatedHooksOptions
	| NamingConventionOptions
	| RestrictedGlobalsOptions
	| ValidAriaRoleOptions
	| UtilityClassSortingOptions;
/**
 * Options for the rule `noExcessiveCognitiveComplexity`.
 */
export interface ComplexityOptions {
	/**
	 * The maximum complexity score that we allow. Anything higher is considered excessive.
	 */
	maxAllowedComplexity: number;
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
	 * If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases. This does not affect other [Case].
	 */
	strictCase: boolean;
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
 * Rule's options.
 */
export interface NamingConventionOptions {
	/**
	 * Allowed cases for _TypeScript_ `enum` member names.
	 */
	enumMemberCase: EnumMemberCase;
	/**
	 * If `false`, then consecutive uppercase are allowed in _camel_ and _pascal_ cases. This does not affect other [Case].
	 */
	strictCase: boolean;
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
export interface ValidAriaRoleOptions {
	allowInvalidRoles: string[];
	ignoreNonDom: boolean;
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
export type ConsistentArrayType = "shorthand" | "generic";
export type FilenameCases = FilenameCase[];
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
export interface ProjectFeaturesParams {
	manifest_path: RomePath;
}
export interface ProjectFeaturesResult {}
export interface OpenFileParams {
	content: string;
	language_hint?: Language;
	path: RomePath;
	version: number;
}
/**
 * Supported languages by Biome
 */
export type Language =
	| "Astro"
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
	path: RomePath;
	version: number;
}
export interface CloseFileParams {
	path: RomePath;
}
export interface GetSyntaxTreeParams {
	path: RomePath;
}
export interface GetSyntaxTreeResult {
	ast: string;
	cst: string;
}
export interface OrganizeImportsParams {
	path: RomePath;
}
export interface OrganizeImportsResult {
	code: string;
}
export interface GetFileContentParams {
	path: RomePath;
}
export interface GetControlFlowGraphParams {
	cursor: TextSize;
	path: RomePath;
}
export type TextSize = number;
export interface GetFormatterIRParams {
	path: RomePath;
}
export interface PullDiagnosticsParams {
	categories: RuleCategories;
	max_diagnostics: number;
	path: RomePath;
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
	| "lint/nursery/noDuplicateJsonKeys"
	| "lint/nursery/noEmptyBlockStatements"
	| "lint/nursery/noEmptyTypeParameters"
	| "lint/nursery/noFocusedTests"
	| "lint/nursery/noGlobalAssign"
	| "lint/nursery/noGlobalEval"
	| "lint/nursery/noInvalidUseBeforeDeclaration"
	| "lint/nursery/noMisleadingCharacterClass"
	| "lint/nursery/noNodejsModules"
	| "lint/nursery/noReExportAll"
	| "lint/nursery/noSkippedTests"
	| "lint/nursery/noThenProperty"
	| "lint/nursery/noTypeOnlyImportAttributes"
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
	path: RomePath;
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
	path: RomePath;
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
	path: RomePath;
	range: TextRange;
}
export interface FormatOnTypeParams {
	offset: TextSize;
	path: RomePath;
}
export interface FixFileParams {
	fix_file_mode: FixFileMode;
	path: RomePath;
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
	path: RomePath;
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
	projectFeatures(
		params: ProjectFeaturesParams,
	): Promise<ProjectFeaturesResult>;
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
		projectFeatures(params) {
			return transport.request("biome/project_features", params);
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
