import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

export const undefinedReference = defineRule({
	query: ast("JS_IDENTIFIER_EXPRESSION"),
	run(node) {
		if (node.text !== "undefined") return;
		registerDiagnostic(node, "error", "Unexpected undefined");
	},
});
