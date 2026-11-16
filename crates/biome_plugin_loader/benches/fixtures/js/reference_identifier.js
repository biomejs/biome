import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

export const referenceIdentifier = defineRule({
	query: ast("JS_REFERENCE_IDENTIFIER"),
	run(node) {
		registerDiagnostic(node, "error", "Reference identifier");
	},
});
