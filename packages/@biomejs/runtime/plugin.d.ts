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
declare const queryContext: unique symbol;
declare const nativeMutation: unique symbol;
declare const nodeKindSet: unique symbol;
declare const tokenKindSet: unique symbol;

/**
 * A query created with {@link ast} or {@link grit}.
 *
 * `N` is the union of the node types matched by the query; it only exists at
 * the type level, to infer the argument type of {@link Rule#run}.
 */
interface Query<N extends JsAstNode | JsAstToken, C> {
	readonly type: "ast" | "grit";
	readonly [queriedNode]?: N;
	readonly [queryContext]?: C;
}

export interface AstQuery<N extends JsAstNode, C = RuleContext>
	extends Query<N, C> {
	readonly type: "ast";
	readonly kinds: readonly (keyof JsNodeByKind)[];
}

/**
 * A query matching the nodes and tokens selected by a GritQL pattern, created
 * with {@link grit}.
 */
export interface GritQuery<C = RuleContext> extends Query<JsAstElement, C> {
	readonly type: "grit";
	readonly pattern: string;
}

/**
 * A Grit query created with `semantic: true`, whose `run` receives a semantic
 * model.
 */
export type SemanticGritQuery = GritQuery<SemanticRuleContext>;

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
				| "textInterpolation"
				| "snippetSignature"
				| "legacyConst"
				| "declaration";
	  };

/**
 * A lint rule, created with {@link defineRule} and exported from the plugin
 * with `export const`. The name of the export is used as the rule name.
 */
export interface Rule<N extends JsAstElement, C = RuleContext> {
	/**
	 * The query selecting the nodes the rule runs on.
	 */
	readonly query: Query<N, C>;

	/**
	 * Called with every node, or token for Grit queries, matching the query.
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
	readonly run: (node: NoInfer<N>, context: NoInfer<C>) => void;
}

/**
 * Creates a query matching every node of the given syntax kinds.
 *
 * Pass `{ semantic: true }` to provide a semantic model in `context`.
 *
 * For example, inspect every read of a declared binding:
 * ```ts
 * export const inspectReads = defineRule({
 *   query: ast("JS_IDENTIFIER_BINDING", { semantic: true }),
 *   run(node, context) {
 *     const binding = context.model.asBinding(node);
 *     for (const reference of binding?.allReads() ?? []) {
 *       registerDiagnostic(reference.syntax(), "information", "Binding read here.");
 *     }
 *   },
 * });
 * ```
 */
export function ast<K extends readonly (keyof JsNodeByKind)[]>(
	...kinds: K
): AstQuery<JsNodeByKind[K[number]]>;
export function ast<K extends readonly (keyof JsNodeByKind)[]>(
	...kinds: [...K, { semantic: true }]
): AstQuery<JsNodeByKind[K[number]], SemanticRuleContext>;
export function ast<K extends readonly (keyof JsNodeByKind)[]>(
	...kinds: [...K, { semantic?: false }]
): AstQuery<JsNodeByKind[K[number]]>;

/**
 * Creates a query matching the nodes selected by a GritQL pattern.
 *
 * The pattern runs against the analyzed file and `run` is called once for each
 * match. When the pattern matches a list, such as several sibling statements,
 * `run` is called for each node in the list. Patterns may also match leaf
 * tokens, in which case `run` receives a token.
 *
 * The pattern cannot contain a rewrite. Report diagnostics from `run`
 * instead. When the pattern matches a token, `run` receives a token and
 * diagnostics can be reported on its `parent` node.
 *
 * Pass `{ semantic: true }` to provide a semantic model in `context`.
 *
 * For example, inspect console calls without a fixed list of syntax kinds:
 * ```ts
 * export const noConsole = defineRule({
 *   query: grit("`console.log($args)`"),
 *   run(element) {
 *     if (element.kind !== "JS_CALL_EXPRESSION") return;
 *     registerDiagnostic(element, "warning", "Unexpected console.log.");
 *   },
 * });
 * ```
 */
export function grit(pattern: string): GritQuery;
export function grit(
	pattern: string,
	options: { semantic: true },
): SemanticGritQuery;
export function grit(
	pattern: string,
	options?: { semantic?: false },
): GritQuery;

/**
 * Defines a lint rule. Export the returned rule with `export const` to
 * register it to the analyzer.
 */
export function defineRule<N extends JsAstElement, C = RuleContext>(
	rule: Rule<N, C>,
): Rule<N, C>;

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

/**
 * A set of node kinds, created with {@link nodeKinds}.
 *
 * Testing membership with {@link NodeKindSet#has} reads the node's kind
 * natively, which is cheaper than comparing `node.kind` strings. Build sets
 * once at module scope and reuse them across `run()` calls.
 *
 * `K` is the exact union of kinds in the set, so a set of some kinds is not
 * assignable to a set of other or more kinds. Functions that accept any node
 * kind set should be generic, as in
 * `function f<K extends keyof JsNodeByKind>(set: NodeKindSet<K>)`.
 */
export interface NodeKindSet<
	in out K extends keyof JsNodeByKind = keyof JsNodeByKind,
> {
	readonly [nodeKindSet]: true;
	/**
	 * Returns whether `element` is a node whose kind is in this set.
	 *
	 * Returns `false` for tokens and for `undefined`, so optional fields and the
	 * results of childrenWithTokens() can be passed directly.
	 *
	 * For example, report calls and constructor calls:
	 * ```ts
	 * const CALLS = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
	 *
	 * if (CALLS.has(statement.expression)) {
	 *   registerDiagnostic(statement, "information", "This statement is a call.");
	 * }
	 * ```
	 *
	 * @throws {TypeError} If `element` is not a node, a token, or `undefined`.
	 */
	has(element: JsAstElement | undefined): element is JsNodeByKind[K];
	/**
	 * Returns a new set with the kinds of this set and `other`. Neither set
	 * changes.
	 *
	 * For example, extend a shared set of calls:
	 * ```ts
	 * const CALLS = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
	 * const CALLS_AND_IMPORTS = CALLS.union(nodeKinds("JS_IMPORT_CALL_EXPRESSION"));
	 * ```
	 *
	 * @throws {TypeError} If `other` is not a set returned by {@link nodeKinds},
	 * including when it is a token kind set.
	 */
	union<J extends keyof JsNodeByKind>(
		other: NodeKindSet<J>,
	): NodeKindSet<K | J>;
}

/**
 * A set of token kinds, created with {@link tokenKinds}.
 *
 * Testing membership with {@link TokenKindSet#has} reads the token's kind
 * natively, which is cheaper than comparing `token.kind` strings. Build sets
 * once at module scope and reuse them across `run()` calls.
 *
 * `K` is the exact union of kinds in the set, so a set of some kinds is not
 * assignable to a set of other or more kinds. Functions that accept any token
 * kind set should be generic, as in
 * `function f<K extends JsAstToken["kind"]>(set: TokenKindSet<K>)`.
 */
export interface TokenKindSet<
	in out K extends JsAstToken["kind"] = JsAstToken["kind"],
> {
	readonly [tokenKindSet]: true;
	/**
	 * Returns whether `element` is a token whose kind is in this set.
	 *
	 * Returns `false` for nodes and for `undefined`, so the results of
	 * childrenWithTokens() can be passed directly.
	 *
	 * For example, report loose equality operators:
	 * ```ts
	 * const LOOSE_EQUALITY = tokenKinds("EQ2", "NEQ");
	 *
	 * for (const element of binary.childrenWithTokens()) {
	 *   if (LOOSE_EQUALITY.has(element)) {
	 *     registerDiagnostic(binary, "warning", `Use strict equality instead of ${element.text}.`);
	 *   }
	 * }
	 * ```
	 *
	 * @throws {TypeError} If `element` is not a node, a token, or `undefined`.
	 */
	has(
		element: JsAstElement | undefined,
	): element is JsAstToken & { readonly kind: K };
	/**
	 * Returns a new set with the kinds of this set and `other`. Neither set
	 * changes.
	 *
	 * For example, combine equality and relational operators:
	 * ```ts
	 * const EQUALITY = tokenKinds("EQ2", "EQ3", "NEQ", "NEQ2");
	 * const COMPARISON = EQUALITY.union(tokenKinds("L_ANGLE", "R_ANGLE", "LTEQ", "GTEQ"));
	 * ```
	 *
	 * @throws {TypeError} If `other` is not a set returned by {@link tokenKinds},
	 * including when it is a node kind set.
	 */
	union<J extends JsAstToken["kind"]>(
		other: TokenKindSet<J>,
	): TokenKindSet<K | J>;
}

/**
 * Creates a set of node kinds. Pass names listed in {@link JsNodeByKind}.
 *
 * Node kinds and token kinds are separate: passing a token kind such as
 * `"IDENT"` throws. Use {@link tokenKinds} to build a set of token kinds.
 *
 * @throws {TypeError} If a kind is not a string or does not name a node kind.
 */
export function nodeKinds<K extends readonly (keyof JsNodeByKind)[]>(
	...kinds: K
): NodeKindSet<K[number]>;

/**
 * Creates a set of token kinds. Pass any name that {@link JsAstToken#kind}
 * can return, such as `"IDENT"` or `"LET_KW"`.
 *
 * Node kinds and token kinds are separate: passing a node kind such as
 * `"JS_CALL_EXPRESSION"` throws. Use {@link nodeKinds} to build a set of node
 * kinds.
 *
 * @throws {TypeError} If a kind is not a string or does not name a token kind.
 */
export function tokenKinds<K extends readonly JsAstToken["kind"][]>(
	...kinds: K
): TokenKindSet<K[number]>;

declare module "./js_ast" {
	interface JsAstNode {
		/**
		 * Returns a fresh array of the nodes inside this node whose kind is in
		 * `kinds`, in source order. This node itself is never included, even
		 * when its kind is in `kinds`.
		 *
		 * Biome searches the tree natively and creates JavaScript objects only
		 * for the matching nodes, which is much faster than visiting every node
		 * with children(). Returned nodes do not have stable JavaScript object
		 * identity.
		 *
		 * For example, report every `debugger` statement in a file:
		 * ```ts
		 * const DEBUGGER = nodeKinds("JS_DEBUGGER_STATEMENT");
		 *
		 * export const noDebugger = defineRule({
		 *   query: ast("JS_MODULE"),
		 *   run(root) {
		 *     for (const statement of root.descendants(DEBUGGER)) {
		 *       registerDiagnostic(statement, "warning", "Remove this debugger statement.");
		 *     }
		 *   },
		 * });
		 * ```
		 *
		 * @throws {TypeError} If `kinds` is not a set created with {@link nodeKinds}.
		 */
		descendants<K extends keyof JsNodeByKind>(
			kinds: NodeKindSet<K>,
		): readonly JsNodeByKind[K][];
		/**
		 * Visits the nodes inside this node whose kind is in `kinds`, in source
		 * order. This node itself is never visited, even when its kind is in
		 * `kinds`.
		 *
		 * `enter` is called with each matching node before the nodes inside it,
		 * and `exit`, when provided, is called with the same object after them.
		 * Use the pair to track nesting, such as a stack of enclosing elements.
		 * Return `false` from `enter` to skip the nodes inside that node; `exit`
		 * is then not called for it. Any other return value continues the walk.
		 * An exception thrown by a callback stops the walk and is rethrown.
		 *
		 * Biome walks the tree natively and calls the functions only for the
		 * matching nodes, which is much faster than visiting every node with
		 * children().
		 *
		 * For example, report calls nested inside another call:
		 * ```ts
		 * const CALLS = nodeKinds("JS_CALL_EXPRESSION");
		 *
		 * let depth = 0;
		 * root.walk(
		 *   CALLS,
		 *   (call) => {
		 *     if (depth > 0) registerDiagnostic(call, "information", "Nested call.");
		 *     depth++;
		 *   },
		 *   () => {
		 *     depth--;
		 *   },
		 * );
		 * ```
		 *
		 * @throws {TypeError} If `kinds` is not a set created with {@link nodeKinds},
		 * or `enter` or `exit` is not a function.
		 */
		walk<K extends keyof JsNodeByKind>(
			kinds: NodeKindSet<K>,
			enter: (node: JsNodeByKind[K]) => unknown,
			exit?: (node: JsNodeByKind[K]) => void,
		): void;
	}
}

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
