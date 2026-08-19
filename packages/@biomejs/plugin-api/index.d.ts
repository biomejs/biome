import type { Severity } from "./diagnostics";
import type { JsAstNode, JsNodeByKind } from "./js_ast";

export * from "./diagnostics";
export * from "./js_ast";

declare const queriedNode: unique symbol;

/**
 * A query matching AST nodes by their syntax kinds, created with {@link ast}.
 *
 * `N` is the union of the node types matched by the query; it only exists at
 * the type level, to infer the argument type of {@link Rule#run}.
 */
export interface AstQuery<N extends JsAstNode> {
	readonly type: "ast";
	readonly kinds: readonly (keyof JsNodeByKind)[];
	readonly [queriedNode]?: N;
}

/**
 * A lint rule, created with {@link defineRule} and exported from the plugin
 * with `export const`. The name of the export is used as the rule name.
 */
export interface Rule<N extends JsAstNode> {
	/**
	 * The query selecting the nodes the rule runs on.
	 */
	readonly query: AstQuery<N>;

	/**
	 * Called with every node matching the query.
	 */
	run(node: N): void;
}

/**
 * Creates a query matching every node of the given syntax kinds.
 */
export function ast<K extends readonly (keyof JsNodeByKind)[]>(
	...kinds: K
): AstQuery<JsNodeByKind[K[number]]>;

/**
 * Defines a lint rule. Export the returned rule with `export const` to
 * register it to the analyzer.
 */
export function defineRule<N extends JsAstNode>(rule: Rule<N>): Rule<N>;

export function registerDiagnostic(
	node: JsAstNode,
	severity: Severity,
	message: string,
): void;
