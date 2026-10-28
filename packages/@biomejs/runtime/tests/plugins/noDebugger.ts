import {
	ast,
	createMutation,
	defineRule,
	registerDiagnostic,
} from "@biomejs/runtime/plugin";

export const noDebugger = defineRule({
	query: ast("JS_DEBUGGER_STATEMENT"),
	run(node) {
		const mutation = createMutation(node);
		mutation.removeNode(node);
		registerDiagnostic(node, "error", "Remove debugger.", {
			mutation,
			message: "Remove debugger",
			kind: "safe",
		});
	},
});
