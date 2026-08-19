import type { Severity } from "./diagnostics";
import type { JsAstNode } from "./js_ast";

export * from "./diagnostics";
export * from "./js_ast";

export function registerDiagnostic(
	node: JsAstNode,
	severity: Severity,
	message: string,
): void;
