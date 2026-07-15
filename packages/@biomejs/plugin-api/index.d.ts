import type { JsAstNode } from "./js_ast";

export type Severity = "fatal" | "error" | "warning" | "information" | "hint";

export * from "./js_ast";

export function registerDiagnostic(
	node: JsAstNode,
	severity: Severity,
	message: string,
): void;
