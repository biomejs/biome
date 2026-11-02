import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

export const context = defineRule({
	query: ast("JS_MODULE", "JS_SCRIPT"),
	run(node, context) {
		registerDiagnostic(
			node,
			"information",
			`${context.sourceType.language.kind}:${context.sourceType.moduleKind}:${context.filePath.replaceAll("\\", "/")}`,
		);
	},
});
