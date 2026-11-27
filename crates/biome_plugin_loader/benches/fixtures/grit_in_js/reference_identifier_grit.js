import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";

export const referenceIdentifier = defineRule({
	query: grit("JsReferenceIdentifier()"),
	run(node) {
		registerDiagnostic(node, "error", "Reference identifier");
	},
});
