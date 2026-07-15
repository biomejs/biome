export type Severity = "fatal" | "error" | "warning" | "information" | "hint";

/** A JavaScript AST node exposed to a plugin. */
export interface JsAstNode {
	readonly kind: string;
	readonly text: string;
	readonly children: readonly JsAstNode[];
}

/** The root node of a JavaScript AST exposed to a plugin. */
export type AnyJsRoot = JsAstNode;

export function registerDiagnostic(severity: Severity, message: string): void;
