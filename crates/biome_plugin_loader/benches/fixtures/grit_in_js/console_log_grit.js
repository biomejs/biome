import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";

export const consoleLog = defineRule({
	query: grit("`console.log($args)`"),
	run(node) {
		registerDiagnostic(node, "error", "Unexpected console.log");
	},
});
