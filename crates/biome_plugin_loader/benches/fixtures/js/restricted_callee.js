import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

const RESTRICTED = new Set(["eval", "setTimeout", "setInterval"]);

export const restrictedCallee = defineRule({
	query: ast("JS_CALL_EXPRESSION"),
	run(node) {
		const callee = node.callee;
		if (callee?.kind !== "JS_IDENTIFIER_EXPRESSION") return;
		if (!RESTRICTED.has(callee.text)) return;
		registerDiagnostic(callee, "error", "Restricted callee");
	},
});
