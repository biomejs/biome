import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

let invocation = 0;

export const stateful = defineRule({
	query: ast("JS_MODULE"),
	run(node) {
		invocation += 1;
		registerDiagnostic(node, "hint", `Invocation ${invocation}`);
	},
});
