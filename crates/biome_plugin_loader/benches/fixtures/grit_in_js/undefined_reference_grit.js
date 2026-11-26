import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";

export const undefinedReference = defineRule({
	query: grit("`undefined`"),
	run(node) {
		registerDiagnostic(node, "error", "Unexpected undefined");
	},
});
