import {
	defineRule,
	registerDiagnostic,
	semantic,
} from "@biomejs/runtime/plugin";

export const reads = defineRule({
	query: semantic("JS_IDENTIFIER_BINDING"),
	run(node, context) {
		for (const reference of context.model.asBinding(node)?.allReads() ?? []) {
			registerDiagnostic(reference.syntax(), "warning", "Binding read.");
		}
	},
});
