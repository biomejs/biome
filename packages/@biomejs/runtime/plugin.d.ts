import type { Severity } from "./diagnostics";
import type {
	AnyJsAstNode,
	JsAstElement,
	JsAstNode,
	JsAstToken,
	JsNodeByKind,
	JsTokenKind,
} from "./js_ast";
import type { SemanticModel } from "./semantic";

export * from "./diagnostics";
export * from "./js_ast";
export * from "./semantic";

declare const queriedNode: unique symbol;
declare const nativeMutation: unique symbol;

/**
 * A query matching AST nodes by their syntax kinds, created with {@link ast}.
 *
 * `N` is the union of the node types matched by the query; it only exists at
 * the type level, to infer the argument type of {@link Rule#run}.
 */
interface Query<N extends JsAstNode, K extends keyof ContextByQuery> {
	readonly type: K;
	readonly kinds: readonly (keyof JsNodeByKind)[];
	readonly [queriedNode]?: N;
}

export type AstQuery<N extends JsAstNode> = Query<N, "ast">;
export type SemanticQuery<N extends JsAstNode> = Query<N, "semantic">;

/** File information supplied to each invocation of a rule. */
export interface RuleContext {
	/** The analyzed file's path, as supplied by Biome. */
	readonly filePath: string;
	/** The parsing mode of the analyzed source, including embedded snippets. */
	readonly sourceType: JsFileSource;
}

/** File information and semantic analysis supplied to a semantic rule. */
export interface SemanticRuleContext extends RuleContext {
	/** Bindings and references in the original analyzed source. */
	readonly model: SemanticModel;
}

type ContextByQuery = {
	ast: RuleContext;
	semantic: SemanticRuleContext;
};

/** JavaScript or TypeScript parsing settings for the analyzed source. */
export interface JsFileSource {
	readonly language:
		| { readonly kind: "javascript" }
		| { readonly kind: "typescript"; readonly definitionFile: boolean };
	/** Syntax extensions permitted in this source. */
	readonly variant: "standard" | "standardRestricted" | "jsx";
	readonly moduleKind: "module" | "script";
	readonly version: "es2022" | "esNext";
	readonly embeddingKind: JsEmbeddingKind;
}

/** The host framework and parsing mode of embedded JavaScript or TypeScript. */
export type JsEmbeddingKind =
	| { readonly kind: "none" }
	| {
			readonly kind: "astro";
			readonly frontmatter: boolean;
			readonly isClassAttribute: boolean;
	  }
	| {
			readonly kind: "vue";
			readonly setup: boolean;
			readonly isSource: boolean;
			readonly eventHandler: boolean;
	  }
	| {
			readonly kind: "svelte";
			readonly fileKind: "component" | "sourceModule";
			readonly embeddingKind:
				| "source"
				| "expression"
				| "snippetSignature"
				| "legacyConst"
				| "declaration";
	  };

/**
 * A lint rule, created with {@link defineRule} and exported from the plugin
 * with `export const`. The name of the export is used as the rule name.
 */
export interface Rule<
	N extends JsAstNode,
	K extends keyof ContextByQuery = "ast",
> {
	/**
	 * The query selecting the nodes the rule runs on.
	 */
	readonly query: Query<N, K>;

	/**
	 * Called with every node matching the query.
	 * `context` information is read-only
	 *
	 * For example, inspect declarations only in TypeScript sources:
	 * ```ts
	 * export const myRule = defineRule({
	 *   query: ast("JS_VARIABLE_STATEMENT"),
	 *   run(node, context) {
	 *     if (context.sourceType.language.kind !== "typescript") return;
	 *     registerDiagnostic(node, "information", `Declaration in ${context.filePath}.`);
	 *   },
	 * });
	 * ```
	 */
	readonly run: (node: NoInfer<N>, context: NoInfer<ContextByQuery[K]>) => void;
}

/**
 * Creates a query matching every node of the given syntax kinds.
 */
export function ast<K extends readonly (keyof JsNodeByKind)[]>(
	...kinds: K
): AstQuery<JsNodeByKind[K[number]]>;

/**
 * Matches nodes by their syntax kinds and provides a semantic model in `context`.
 *
 * For example, inspect every read of a declared binding:
 * ```ts
 * export const inspectReads = defineRule({
 *   query: semantic("JS_IDENTIFIER_BINDING"),
 *   run(node, context) {
 *     const binding = context.model.asBinding(node);
 *     for (const reference of binding?.allReads() ?? []) {
 *       registerDiagnostic(reference.syntax(), "information", "Binding read here.");
 *     }
 *   },
 * });
 * ```
 */
export function semantic<K extends readonly (keyof JsNodeByKind)[]>(
	...kinds: K
): SemanticQuery<JsNodeByKind[K[number]]>;

/**
 * Defines a lint rule. Export the returned rule with `export const` to
 * register it to the analyzer.
 */
export function defineRule<
	N extends JsAstNode,
	K extends keyof ContextByQuery = "ast",
>(rule: Rule<N, K>): Rule<N, K>;

/**
 * Collects replacements and removals for one code fix without changing the
 * source being analyzed. Create one with {@link createMutation}.
 *
 * Replacements can reuse existing nodes and tokens or use newly created ones.
 * You are responsible for making sure the resulting code is valid. For example,
 * removing an argument does not automatically remove its comma.
 *
 * Replacements preserve the original node or token's leading and trailing
 * comments and whitespace. Removals delete them along with the node or token.
 *
 * Edits to the same node, or to a node, and its children can overwrite each
 * other. Combine these changes into a single replacement for a predictable
 * result.
 *
 * To attach the fix to a diagnostic, set {@link CodeFix.mutation} to this object
 * and pass the fix to {@link registerDiagnostic}. Finish adding changes first:
 * once registered, this mutation cannot be changed or attached to another
 * diagnostic.
 *
 * You can keep nodes, tokens, and unfinished mutations between `run()` calls.
 * Report a fix only for the same source the mutation was created from.
 */
export interface JsMutation {
	readonly [nativeMutation]: true;
	/**
	 * Replaces a node, keeping its leading and trailing comments and whitespace.
	 * The replacement can be an existing node or one created by a field-update method.
	 *
	 * For example, change a numeric literal to `2`:
	 * ```ts
	 * const next = literal.withValueToken(factory.token("JS_NUMBER_LITERAL", "2"));
	 * mutation.replaceNode(literal, next);
	 * ```
	 */
	replaceNode(previous: AnyJsAstNode, next: AnyJsAstNode): void;
	/**
	 * Replaces a token, keeping its leading and trailing comments and whitespace.
	 * Pass token objects, not the strings returned by token fields.
	 *
	 * For example, change a declaration's keyword to `let`:
	 * ```ts
	 * const keyword = declaration.token("kindToken");
	 * if (keyword) {
	 *   mutation.replaceToken(keyword, factory.token("LET_KW"));
	 * }
	 * ```
	 */
	replaceToken(previous: JsAstToken, next: JsAstToken): void;
	/**
	 * Replaces a node with a node, or a token with a token, keeping the original's
	 * leading and trailing comments and whitespace. Useful with childrenWithTokens().
	 *
	 * For example, replace a binary expression's operator with `===`:
	 * ```ts
	 * const operator = binary.childrenWithTokens()[1];
	 * if (operator) {
	 *   mutation.replaceElement(operator, factory.token("EQ3"));
	 * }
	 * ```
	 */
	replaceElement(previous: JsAstElement, next: JsAstElement): void;
	/**
	 * Removes a node and its leading and trailing comments and whitespace.
	 * Adjacent commas and other separators are not removed automatically.
	 *
	 * For example, remove a debugger statement from a block:
	 * ```ts
	 * mutation.removeNode(debuggerStatement);
	 * ```
	 */
	removeNode(node: AnyJsAstNode): void;
	/**
	 * Removes a token and its leading and trailing comments and whitespace.
	 * Use token(field) to get the token object rather than its text.
	 *
	 * For example, remove a statement's optional semicolon:
	 * ```ts
	 * const semicolon = statement.token("semicolonToken");
	 * if (semicolon) mutation.removeToken(semicolon);
	 * ```
	 */
	removeToken(token: JsAstToken): void;
	/**
	 * Removes a node or token and its leading and trailing comments and whitespace.
	 * Useful when iterating over the result of childrenWithTokens().
	 *
	 * For example, remove the first argument and its following comma:
	 * ```ts
	 * const [argument, comma] = argumentList.childrenWithTokens();
	 * if (argument && comma?.kind === "COMMA") {
	 *   mutation.removeElement(argument);
	 *   mutation.removeElement(comma);
	 * }
	 * ```
	 */
	removeElement(element: JsAstElement): void;
}

/**
 * Starts collecting edits for the source containing `node`. You can pass any
 * node from that source, not just the top-level node.
 *
 * Adding edits does not change the source or the nodes you already have. To
 * execute the edits as a fix, pass the mutation to {@link registerDiagnostic}
 * as part of a {@link CodeFix}.
 *
 * For example, prepare a fix that removes a statement:
 * ```ts
 * const mutation = createMutation(statement);
 * mutation.removeNode(statement);
 * ```
 */
export function createMutation(node: AnyJsAstNode): JsMutation;

/** Helpers for creating replacement tokens. */
export const factory: {
	/**
	 * Creates a token to use as a replacement in a code fix.
	 *
	 * `kind` names the token type. Keywords and punctuation have a fixed spelling,
	 * so `text` can be omitted or set to that spelling. For identifiers and literals,
	 * supply the desired text, including quotes when creating a string literal.
	 * The text is used as written, without adding spaces or comments.
	 *
	 * For example, create a keyword, an identifier, and a string literal:
	 * ```ts
	 * const keyword = factory.token("LET_KW");
	 * const name = factory.token("IDENT", "newName");
	 * const message = factory.token("JS_STRING_LITERAL", '"hello"');
	 * ```
	 */
	readonly token: (kind: JsTokenKind, text?: string) => JsAstToken;
};

/** A suggested source edit attached to a diagnostic. */
export interface CodeFix {
	readonly mutation: JsMutation;
	/** A message describing the suggested edit. */
	readonly message: string;
	/** The kind of fix. */
	readonly kind: "safe" | "unsafe";
}

/**
 * Reports an issue found by a plugin rule. Biome highlights the code represented
 * by `node` and displays `message` at the requested severity.
 *
 * Provide `fix` to include a suggested code change alongside the diagnostic.
 *
 * For example, report a warning on a variable declaration:
 * ```ts
 * registerDiagnostic(declaration, "warning", "Use let or const instead of var.");
 * ```
 */
export function registerDiagnostic(
	node: JsAstNode,
	severity: Severity,
	message: string,
	fix?: CodeFix,
): void;
