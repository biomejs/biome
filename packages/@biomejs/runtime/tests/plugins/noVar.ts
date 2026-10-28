import {
	ast,
	createMutation,
	defineRule,
	factory,
	registerDiagnostic,
} from "@biomejs/runtime/plugin";

export const noVar = defineRule({
	query: ast("JS_VARIABLE_DECLARATION"),
	run(node) {
		const kindToken = node.token("kindToken");
		if (node.kindToken !== "var" || kindToken === undefined) return;
		const mutation = createMutation(node);
		mutation.replaceToken(kindToken, factory.token("LET_KW"));
		registerDiagnostic(node, "warning", "Use let.", {
			mutation,
			message: "Replace var",
			kind: "unsafe",
		});
	},
});
