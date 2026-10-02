import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

export const consoleLog = defineRule({
	query: ast("JS_CALL_EXPRESSION"),
	run(node) {
		const callee = node.callee;
		if (callee?.kind !== "JS_STATIC_MEMBER_EXPRESSION") return;
		if (callee.object?.text !== "console" || callee.member?.text !== "log") return;
		registerDiagnostic(node, "error", "Unexpected console.log");
	},
});
